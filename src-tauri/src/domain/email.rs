//! Email delivery settings and the ports the email use cases talk through.
//! Secrets (SMTP password, API keys) are deliberately absent from
//! `EmailSettings`: they only ever live behind `SecretStore`, so no code path
//! that serialises settings can leak one.

use thiserror::Error;

use super::repository::RepoResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailProvider {
    /// Email is off; notification emails are skipped, not failed.
    None,
    Smtp,
    Resend,
    Postmark,
}

impl EmailProvider {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Smtp => "smtp",
            Self::Resend => "resend",
            Self::Postmark => "postmark",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "none" => Some(Self::None),
            "smtp" => Some(Self::Smtp),
            "resend" => Some(Self::Resend),
            "postmark" => Some(Self::Postmark),
            _ => None,
        }
    }

    /// Keychain account holding this provider's secret. `None` has none.
    pub fn secret_account(self) -> Option<&'static str> {
        match self {
            Self::None => None,
            Self::Smtp => Some(SMTP_PASSWORD_ACCOUNT),
            Self::Resend => Some(RESEND_API_KEY_ACCOUNT),
            Self::Postmark => Some(POSTMARK_TOKEN_ACCOUNT),
        }
    }
}

pub const SMTP_PASSWORD_ACCOUNT: &str = "smtp-password";
pub const RESEND_API_KEY_ACCOUNT: &str = "resend-api-key";
pub const POSTMARK_TOKEN_ACCOUNT: &str = "postmark-token";
/// Every account Ripple may have written — what a full reset must delete.
pub const ALL_SECRET_ACCOUNTS: [&str; 3] = [
    SMTP_PASSWORD_ACCOUNT,
    RESEND_API_KEY_ACCOUNT,
    POSTMARK_TOKEN_ACCOUNT,
];

/// How the SMTP connection is encrypted. There is intentionally no plaintext
/// option: credentials and habit data never cross the wire unencrypted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmtpSecurity {
    /// Plain connect, then a mandatory STARTTLS upgrade (usually port 587).
    StartTls,
    /// TLS from the first byte, a.k.a. implicit TLS / SMTPS (usually port 465).
    Tls,
}

impl SmtpSecurity {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StartTls => "starttls",
            Self::Tls => "tls",
        }
    }

    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "starttls" => Some(Self::StartTls),
            "tls" => Some(Self::Tls),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmailSettings {
    pub provider: EmailProvider,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub smtp_host: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_security: SmtpSecurity,
}

impl Default for EmailSettings {
    fn default() -> Self {
        Self {
            provider: EmailProvider::None,
            from_address: None,
            to_address: None,
            smtp_host: None,
            smtp_port: None,
            smtp_username: None,
            smtp_security: SmtpSecurity::StartTls,
        }
    }
}

fn present(field: &Option<String>) -> Option<&str> {
    field.as_deref().map(str::trim).filter(|s| !s.is_empty())
}

/// Shape check, not deliverability: one `@`, a non-empty local part, a
/// dotted domain, and nothing that could smuggle extra headers or
/// recipients (whitespace, commas, angle brackets).
pub fn is_email_address(s: &str) -> bool {
    if s.chars()
        .any(|c| c.is_whitespace() || c.is_control() || matches!(c, ',' | ';' | '<' | '>' | '"'))
    {
        return false;
    }
    let Some((local, domain)) = s.split_once('@') else {
        return false;
    };
    !local.is_empty()
        && !domain.contains('@')
        && domain.contains('.')
        && !domain.starts_with('.')
        && !domain.ends_with('.')
}

impl EmailSettings {
    /// The default port for the chosen security mode, used when the user
    /// leaves the field empty.
    pub fn effective_smtp_port(&self) -> u16 {
        self.smtp_port.unwrap_or(match self.smtp_security {
            SmtpSecurity::StartTls => 587,
            SmtpSecurity::Tls => 465,
        })
    }

    /// Everything a provider needs to actually send. With `None` only the
    /// shape of whatever was typed is checked, so a half-filled form can be
    /// saved while email stays off.
    pub fn validate(&self) -> Result<(), String> {
        let required = self.provider != EmailProvider::None;
        for (label, field) in [("From", &self.from_address), ("To", &self.to_address)] {
            match present(field) {
                Some(addr) if !is_email_address(addr) => {
                    return Err(format!("{label} address '{addr}' isn't a valid email address"))
                }
                None if required => return Err(format!("{label} address is required")),
                _ => {}
            }
        }
        if self.smtp_port == Some(0) {
            return Err("SMTP port must be between 1 and 65535".to_string());
        }
        if self.provider == EmailProvider::Smtp {
            match present(&self.smtp_host) {
                None => return Err("SMTP host is required".to_string()),
                Some(host) if host.contains(char::is_whitespace) || host.contains('/') => {
                    return Err(format!("SMTP host '{host}' isn't a valid host name"))
                }
                _ => {}
            }
            if present(&self.smtp_username).is_none() {
                return Err("SMTP username is required".to_string());
            }
        }
        Ok(())
    }
}

/// Why a keychain operation failed. Messages are user-facing and never
/// contain the secret itself.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum SecretError {
    #[error("The system keychain is unavailable: {0}")]
    Unavailable(String),
    #[error("Couldn't access the system keychain: {0}")]
    Failed(String),
}

/// Port over the OS keychain, so the email use cases (and their tests)
/// never touch the real one.
pub trait SecretStore {
    /// `Ok(None)` when no secret is stored under `account`.
    fn get(&self, account: &str) -> Result<Option<String>, SecretError>;
    fn set(&self, account: &str, secret: &str) -> Result<(), SecretError>;
    /// Deleting an absent secret is not an error.
    fn delete(&self, account: &str) -> Result<(), SecretError>;
}

/// Single-row store for `EmailSettings`. Lives beside the email types
/// rather than in `repository.rs` so the whole email feature stays in one
/// place; a missing row reads as `EmailSettings::default()`.
pub trait EmailSettingsRepository {
    fn get(&self) -> RepoResult<EmailSettings>;
    fn save(&self, settings: &EmailSettings) -> RepoResult<()>;
}

/// One fully addressed email, ready for a transport.
#[derive(Debug, Clone, PartialEq)]
pub struct OutgoingEmail {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub text: String,
    pub html: Option<String>,
}

/// Port for "deliver this email". Each provider (SMTP, Resend, Postmark) is
/// one implementation; errors are short, user-readable and secret-free.
pub trait EmailTransport {
    fn send(&self, email: &OutgoingEmail) -> Result<(), String>;
}

/// Builds the transport for the configured provider. Separate from
/// `EmailTransport` because the transport needs the secret, which is only
/// read at send time.
pub trait EmailTransportFactory {
    fn transport(
        &self,
        settings: &EmailSettings,
        secret: &str,
    ) -> Result<Box<dyn EmailTransport>, String>;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn smtp() -> EmailSettings {
        EmailSettings {
            provider: EmailProvider::Smtp,
            from_address: Some("me@example.com".to_string()),
            to_address: Some("me@example.com".to_string()),
            smtp_host: Some("smtp.example.com".to_string()),
            smtp_port: Some(587),
            smtp_username: Some("me@example.com".to_string()),
            smtp_security: SmtpSecurity::StartTls,
        }
    }

    #[test]
    fn address_shape_check() {
        assert!(is_email_address("a.b+c@mail.example.co"));
        for bad in ["", "plain", "@x.com", "a@", "a@b", "a@.com", "a@b.", "a b@x.com", "a@x.com,b@y.com", "A <a@x.com>", "a@b@c.com"] {
            assert!(!is_email_address(bad), "{bad} should be rejected");
        }
    }

    #[test]
    fn complete_smtp_settings_validate() {
        assert_eq!(smtp().validate(), Ok(()));
    }

    #[test]
    fn providers_require_both_addresses() {
        let s = EmailSettings { to_address: None, ..smtp() };
        assert_eq!(s.validate(), Err("To address is required".to_string()));
        let s = EmailSettings {
            provider: EmailProvider::Resend,
            from_address: Some("  ".to_string()),
            ..smtp()
        };
        assert_eq!(s.validate(), Err("From address is required".to_string()));
    }

    #[test]
    fn malformed_addresses_are_rejected_even_when_off() {
        let s = EmailSettings {
            to_address: Some("not-an-address".to_string()),
            ..EmailSettings::default()
        };
        assert!(s.validate().unwrap_err().contains("isn't a valid email address"));
        assert_eq!(EmailSettings::default().validate(), Ok(()));
    }

    #[test]
    fn smtp_needs_host_username_and_a_real_port() {
        let s = EmailSettings { smtp_host: None, ..smtp() };
        assert_eq!(s.validate(), Err("SMTP host is required".to_string()));
        let s = EmailSettings { smtp_host: Some("smtp example".to_string()), ..smtp() };
        assert!(s.validate().is_err());
        let s = EmailSettings { smtp_username: None, ..smtp() };
        assert_eq!(s.validate(), Err("SMTP username is required".to_string()));
        let s = EmailSettings { smtp_port: Some(0), ..smtp() };
        assert_eq!(s.validate(), Err("SMTP port must be between 1 and 65535".to_string()));
    }

    #[test]
    fn api_providers_ignore_smtp_fields() {
        let s = EmailSettings {
            provider: EmailProvider::Postmark,
            smtp_host: None,
            smtp_username: None,
            ..smtp()
        };
        assert_eq!(s.validate(), Ok(()));
    }

    #[test]
    fn default_port_follows_security_mode() {
        let s = EmailSettings { smtp_port: None, ..smtp() };
        assert_eq!(s.effective_smtp_port(), 587);
        let s = EmailSettings { smtp_port: None, smtp_security: SmtpSecurity::Tls, ..smtp() };
        assert_eq!(s.effective_smtp_port(), 465);
    }

    #[test]
    fn each_provider_maps_to_its_keychain_account() {
        assert_eq!(EmailProvider::None.secret_account(), None);
        assert_eq!(EmailProvider::Smtp.secret_account(), Some("smtp-password"));
        assert_eq!(EmailProvider::Resend.secret_account(), Some("resend-api-key"));
        assert_eq!(EmailProvider::Postmark.secret_account(), Some("postmark-token"));
        for p in ["none", "smtp", "resend", "postmark"] {
            assert_eq!(EmailProvider::parse(p).unwrap().as_str(), p);
        }
    }
}
