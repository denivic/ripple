//! SMTP delivery via `lettre`'s blocking transport. TLS is always required:
//! STARTTLS mode refuses servers that don't offer the upgrade, and TLS mode
//! wraps the socket from the first byte. There is no plaintext path.

use std::time::Duration;

use lettre::message::{header::ContentType, Mailbox, MultiPart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::domain::email::{EmailSettings, EmailTransport, OutgoingEmail, SmtpSecurity};

use super::http_api::SENDER_NAME;

const TIMEOUT: Duration = Duration::from_secs(15);

/// Generic over `lettre::Transport` so tests can drive it with lettre's
/// `StubTransport` instead of a live server.
pub struct SmtpEmailTransport<T: Transport = SmtpTransport> {
    inner: T,
}

impl SmtpEmailTransport<SmtpTransport> {
    pub fn connect(settings: &EmailSettings, password: &str) -> Result<Self, String> {
        let host = settings
            .smtp_host
            .as_deref()
            .map(str::trim)
            .filter(|h| !h.is_empty())
            .ok_or("SMTP host is required")?;
        let username = settings
            .smtp_username
            .as_deref()
            .map(str::trim)
            .filter(|u| !u.is_empty())
            .ok_or("SMTP username is required")?;
        // `relay` = implicit TLS, `starttls_relay` = STARTTLS required; both
        // verify the server certificate against webpki roots.
        let builder = match settings.smtp_security {
            SmtpSecurity::Tls => SmtpTransport::relay(host),
            SmtpSecurity::StartTls => SmtpTransport::starttls_relay(host),
        }
        .map_err(|_| format!("SMTP host '{host}' isn't valid"))?;
        let inner = builder
            .port(settings.effective_smtp_port())
            .credentials(Credentials::new(username.to_string(), password.to_string()))
            .timeout(Some(TIMEOUT))
            .build();
        Ok(Self { inner })
    }
}

/// Plain text alone, or multipart/alternative when there's an HTML version.
pub fn build_message(email: &OutgoingEmail) -> Result<Message, String> {
    let from = Mailbox::new(
        Some(SENDER_NAME.to_string()),
        email
            .from
            .parse()
            .map_err(|_| format!("From address '{}' isn't valid", email.from))?,
    );
    let to: Mailbox = email
        .to
        .parse()
        .map_err(|_| format!("To address '{}' isn't valid", email.to))?;
    let builder = Message::builder().from(from).to(to).subject(&email.subject);
    let built = match &email.html {
        Some(html) => builder.multipart(MultiPart::alternative_plain_html(
            email.text.clone(),
            html.clone(),
        )),
        None => builder
            .header(ContentType::TEXT_PLAIN)
            .body(email.text.clone()),
    };
    built.map_err(|e| format!("Couldn't build the email: {e}"))
}

/// Maps lettre's errors onto what the user can do about them. The server's
/// own reply text is dropped: it is noisy and can echo the username.
pub fn readable_smtp_error(e: &lettre::transport::smtp::Error) -> String {
    let code = e.status().map(|c| c.to_string());
    match code.as_deref() {
        Some("535" | "534" | "530" | "454") => {
            return "Authentication failed — check the username and app password".to_string()
        }
        Some(c) if c.starts_with('5') => {
            return format!("The SMTP server refused the email (code {c}) — check the From address is allowed")
        }
        Some(c) => return format!("The SMTP server is temporarily unavailable (code {c}) — try again later"),
        None => {}
    }
    if e.is_timeout() {
        "Timed out talking to the SMTP server — check the host and port".to_string()
    } else if e.is_tls() {
        "Couldn't set up a secure connection — check the port and security setting (587 = STARTTLS, 465 = TLS)".to_string()
    } else {
        "Couldn't connect to the SMTP server — check the host, port and security setting".to_string()
    }
}

/// Lets the generic transport turn whichever lettre error type it gets into
/// a readable message.
pub trait ReadableError {
    fn readable(&self) -> String;
}

impl ReadableError for lettre::transport::smtp::Error {
    fn readable(&self) -> String {
        readable_smtp_error(self)
    }
}

#[cfg(test)]
impl ReadableError for lettre::transport::stub::Error {
    fn readable(&self) -> String {
        "stub transport failure".to_string()
    }
}

impl<T> EmailTransport for SmtpEmailTransport<T>
where
    T: Transport,
    T::Error: ReadableError,
{
    fn send(&self, email: &OutgoingEmail) -> Result<(), String> {
        let message = build_message(email)?;
        self.inner.send(&message).map(|_| ()).map_err(|e| e.readable())
    }
}

#[cfg(test)]
mod tests {
    use lettre::transport::stub::StubTransport;

    use super::*;
    use crate::domain::email::EmailProvider;

    fn email(html: Option<&str>) -> OutgoingEmail {
        OutgoingEmail {
            from: "ripple@example.com".to_string(),
            to: "me@example.com".to_string(),
            subject: "Your week in Ripple".to_string(),
            text: "plain body".to_string(),
            html: html.map(str::to_string),
        }
    }

    #[test]
    fn plain_message_has_headers_and_text_body() {
        let raw = String::from_utf8(build_message(&email(None)).unwrap().formatted()).unwrap();
        assert!(raw.contains("From: Ripple <ripple@example.com>"));
        assert!(raw.contains("To: me@example.com"));
        assert!(raw.contains("Subject: Your week in Ripple"));
        assert!(raw.contains("Content-Type: text/plain"));
        assert!(raw.contains("plain body"));
    }

    #[test]
    fn html_message_is_multipart_alternative() {
        let raw = String::from_utf8(
            build_message(&email(Some("<p>html body</p>"))).unwrap().formatted(),
        )
        .unwrap();
        assert!(raw.contains("multipart/alternative"));
        assert!(raw.contains("text/plain"));
        assert!(raw.contains("text/html"));
        assert!(raw.contains("<p>html body</p>"));
    }

    #[test]
    fn sends_through_the_inner_transport() {
        let stub = StubTransport::new_ok();
        let transport = SmtpEmailTransport { inner: stub.clone() };
        transport.send(&email(None)).unwrap();
        let sent = stub.messages();
        assert_eq!(sent.len(), 1);
        assert_eq!(sent[0].0.to()[0].to_string(), "me@example.com");

        let failing = SmtpEmailTransport { inner: StubTransport::new_error() };
        assert_eq!(failing.send(&email(None)).unwrap_err(), "stub transport failure");
    }

    #[test]
    fn invalid_addresses_fail_before_sending() {
        let bad = OutgoingEmail { to: "nope".to_string(), ..email(None) };
        assert!(build_message(&bad).unwrap_err().contains("To address"));
    }

    #[test]
    fn connect_builds_both_tls_modes_without_network() {
        // Building the transport opens no socket; it only validates config.
        let mut settings = EmailSettings {
            provider: EmailProvider::Smtp,
            from_address: Some("ripple@example.com".to_string()),
            to_address: Some("me@example.com".to_string()),
            smtp_host: Some("smtp.example.com".to_string()),
            smtp_port: None,
            smtp_username: Some("me".to_string()),
            smtp_security: SmtpSecurity::StartTls,
        };
        assert!(SmtpEmailTransport::connect(&settings, "pw").is_ok());
        settings.smtp_security = SmtpSecurity::Tls;
        assert!(SmtpEmailTransport::connect(&settings, "pw").is_ok());
        settings.smtp_username = None;
        assert_eq!(
            SmtpEmailTransport::connect(&settings, "pw").err().unwrap(),
            "SMTP username is required"
        );
    }
}
