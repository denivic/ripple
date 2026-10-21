//! Resend and Postmark: both are "POST a JSON body with an auth header",
//! so they share one small `HttpPost` seam that tests replace with a fake.

use std::time::Duration;

use serde_json::{json, Value};

use crate::domain::email::{EmailTransport, OutgoingEmail};

const RESEND_URL: &str = "https://api.resend.com/emails";
const POSTMARK_URL: &str = "https://api.postmarkapp.com/email";
const TIMEOUT: Duration = Duration::from_secs(15);
/// Display name on the From header; the address is the user's own.
pub(super) const SENDER_NAME: &str = "Ripple";

#[derive(Debug, Clone, PartialEq)]
pub enum HttpOutcome {
    /// Any HTTP response, success or not, with its body for error details.
    Status(u16, String),
    /// No response at all (DNS, TLS, refused, timed out).
    Network(String),
}

/// The only network operation the API transports need.
pub trait HttpPost: Send + Sync {
    fn post_json(&self, url: &str, headers: &[(&str, String)], body: &Value) -> HttpOutcome;
}

pub struct UreqHttp {
    agent: ureq::Agent,
}

impl UreqHttp {
    pub fn new() -> Self {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(TIMEOUT))
            // Read 4xx bodies ourselves: they carry the provider's reason.
            .http_status_as_error(false)
            .build();
        Self {
            agent: config.into(),
        }
    }
}

impl HttpPost for UreqHttp {
    fn post_json(&self, url: &str, headers: &[(&str, String)], body: &Value) -> HttpOutcome {
        let mut req = self
            .agent
            .post(url)
            .header("Content-Type", "application/json")
            .header("Accept", "application/json");
        for (name, value) in headers {
            req = req.header(*name, value.as_str());
        }
        match req.send(body.to_string()) {
            Ok(mut res) => {
                let status = res.status().as_u16();
                let text = res.body_mut().read_to_string().unwrap_or_default();
                HttpOutcome::Status(status, text)
            }
            // ureq's error text describes the connection, never our headers.
            Err(ureq::Error::Timeout(_)) => HttpOutcome::Network("the request timed out".to_string()),
            Err(e) => HttpOutcome::Network(e.to_string()),
        }
    }
}

fn from_header(from: &str) -> String {
    format!("{SENDER_NAME} <{from}>")
}

/// Pulls the provider's own explanation out of an error body, capped so a
/// misbehaving server can't flood the UI. Resend uses `message`, Postmark
/// `Message`.
fn provider_reason(body: &str) -> Option<String> {
    let v: Value = serde_json::from_str(body).ok()?;
    let msg = v.get("message").or_else(|| v.get("Message"))?.as_str()?;
    Some(msg.chars().take(160).collect())
}

/// Shared status → readable message mapping; `key_name` is what the user
/// calls the credential in the UI.
fn interpret(provider: &str, key_name: &str, outcome: HttpOutcome) -> Result<(), String> {
    match outcome {
        HttpOutcome::Status(200..=299, _) => Ok(()),
        HttpOutcome::Status(401 | 403, _) => {
            Err(format!("Authentication failed — check the {provider} {key_name}"))
        }
        HttpOutcome::Status(429, _) => {
            Err(format!("{provider} rate limit reached — try again in a minute"))
        }
        HttpOutcome::Status(code @ 500..=599, _) => {
            Err(format!("{provider} is having problems (HTTP {code}) — try again later"))
        }
        HttpOutcome::Status(code, body) => Err(match provider_reason(&body) {
            Some(reason) => format!("{provider} rejected the email: {reason}"),
            None => format!("{provider} rejected the email (HTTP {code})"),
        }),
        HttpOutcome::Network(reason) => {
            Err(format!("Couldn't reach {provider} — check your internet connection ({reason})"))
        }
    }
}

pub struct ResendTransport<H: HttpPost = UreqHttp> {
    http: H,
    api_key: String,
}

impl<H: HttpPost> ResendTransport<H> {
    pub fn new(http: H, api_key: &str) -> Self {
        Self {
            http,
            api_key: api_key.to_string(),
        }
    }

    fn body(email: &OutgoingEmail) -> Value {
        let mut body = json!({
            "from": from_header(&email.from),
            "to": [email.to],
            "subject": email.subject,
            "text": email.text,
        });
        if let Some(html) = &email.html {
            body["html"] = json!(html);
        }
        body
    }
}

impl<H: HttpPost> EmailTransport for ResendTransport<H> {
    fn send(&self, email: &OutgoingEmail) -> Result<(), String> {
        let headers = [("Authorization", format!("Bearer {}", self.api_key))];
        let outcome = self.http.post_json(RESEND_URL, &headers, &Self::body(email));
        interpret("Resend", "API key", outcome)
    }
}

pub struct PostmarkTransport<H: HttpPost = UreqHttp> {
    http: H,
    server_token: String,
}

impl<H: HttpPost> PostmarkTransport<H> {
    pub fn new(http: H, server_token: &str) -> Self {
        Self {
            http,
            server_token: server_token.to_string(),
        }
    }

    fn body(email: &OutgoingEmail) -> Value {
        let mut body = json!({
            "From": from_header(&email.from),
            "To": email.to,
            "Subject": email.subject,
            "TextBody": email.text,
            "MessageStream": "outbound",
        });
        if let Some(html) = &email.html {
            body["HtmlBody"] = json!(html);
        }
        body
    }
}

impl<H: HttpPost> EmailTransport for PostmarkTransport<H> {
    fn send(&self, email: &OutgoingEmail) -> Result<(), String> {
        let headers = [("X-Postmark-Server-Token", self.server_token.clone())];
        let outcome = self.http.post_json(POSTMARK_URL, &headers, &Self::body(email));
        interpret("Postmark", "server token", outcome)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    struct Recorded {
        url: String,
        headers: Vec<(String, String)>,
        body: Value,
    }

    /// Records the request and answers with a canned outcome.
    struct FakeHttp {
        reply: HttpOutcome,
        seen: Mutex<Vec<Recorded>>,
    }

    impl FakeHttp {
        fn replying(reply: HttpOutcome) -> Self {
            Self {
                reply,
                seen: Mutex::new(Vec::new()),
            }
        }
    }

    impl HttpPost for &FakeHttp {
        fn post_json(&self, url: &str, headers: &[(&str, String)], body: &Value) -> HttpOutcome {
            self.seen.lock().unwrap().push(Recorded {
                url: url.to_string(),
                headers: headers
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.clone()))
                    .collect(),
                body: body.clone(),
            });
            self.reply.clone()
        }
    }

    fn email(html: Option<&str>) -> OutgoingEmail {
        OutgoingEmail {
            from: "ripple@example.com".to_string(),
            to: "me@example.com".to_string(),
            subject: "Hello".to_string(),
            text: "plain".to_string(),
            html: html.map(str::to_string),
        }
    }

    fn ok() -> HttpOutcome {
        HttpOutcome::Status(200, "{}".to_string())
    }

    #[test]
    fn resend_request_has_bearer_key_and_json_body() {
        let http = FakeHttp::replying(ok());
        ResendTransport::new(&http, "re_secret").send(&email(Some("<p>hi</p>"))).unwrap();
        let seen = http.seen.lock().unwrap();
        assert_eq!(seen[0].url, "https://api.resend.com/emails");
        assert_eq!(
            seen[0].headers,
            vec![("Authorization".to_string(), "Bearer re_secret".to_string())]
        );
        assert_eq!(
            seen[0].body,
            json!({
                "from": "Ripple <ripple@example.com>",
                "to": ["me@example.com"],
                "subject": "Hello",
                "text": "plain",
                "html": "<p>hi</p>",
            })
        );
    }

    #[test]
    fn postmark_request_has_server_token_header_and_pascal_case_body() {
        let http = FakeHttp::replying(ok());
        PostmarkTransport::new(&http, "pm-token").send(&email(None)).unwrap();
        let seen = http.seen.lock().unwrap();
        assert_eq!(seen[0].url, "https://api.postmarkapp.com/email");
        assert_eq!(
            seen[0].headers,
            vec![("X-Postmark-Server-Token".to_string(), "pm-token".to_string())]
        );
        assert_eq!(
            seen[0].body,
            json!({
                "From": "Ripple <ripple@example.com>",
                "To": "me@example.com",
                "Subject": "Hello",
                "TextBody": "plain",
                "MessageStream": "outbound",
            })
        );
    }

    #[test]
    fn errors_are_readable_and_never_contain_the_key() {
        let cases = [
            (HttpOutcome::Status(401, String::new()), "Authentication failed — check the Resend API key"),
            (HttpOutcome::Status(429, String::new()), "Resend rate limit reached — try again in a minute"),
            (HttpOutcome::Status(503, String::new()), "Resend is having problems (HTTP 503) — try again later"),
            (
                HttpOutcome::Status(422, r#"{"message":"The example.com domain is not verified."}"#.to_string()),
                "Resend rejected the email: The example.com domain is not verified.",
            ),
            (HttpOutcome::Status(400, "not json".to_string()), "Resend rejected the email (HTTP 400)"),
        ];
        for (reply, expected) in cases {
            let http = FakeHttp::replying(reply);
            let err = ResendTransport::new(&http, "re_secret").send(&email(None)).unwrap_err();
            assert_eq!(err, expected);
            assert!(!err.contains("re_secret"));
        }
    }

    #[test]
    fn postmark_reasons_and_network_failures_are_reported() {
        let http = FakeHttp::replying(HttpOutcome::Status(
            422,
            r#"{"ErrorCode":400,"Message":"Sender signature not defined"}"#.to_string(),
        ));
        let err = PostmarkTransport::new(&http, "pm").send(&email(None)).unwrap_err();
        assert_eq!(err, "Postmark rejected the email: Sender signature not defined");

        let http = FakeHttp::replying(HttpOutcome::Status(401, String::new()));
        let err = PostmarkTransport::new(&http, "pm").send(&email(None)).unwrap_err();
        assert_eq!(err, "Authentication failed — check the Postmark server token");

        let http = FakeHttp::replying(HttpOutcome::Network("dns failed".to_string()));
        let err = PostmarkTransport::new(&http, "pm").send(&email(None)).unwrap_err();
        assert!(err.starts_with("Couldn't reach Postmark"));
    }
}
