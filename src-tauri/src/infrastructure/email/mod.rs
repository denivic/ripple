//! Email adapters: one `EmailTransport` per provider plus the OS-keychain
//! `SecretStore`.

pub mod http_api;
pub mod keychain;
pub mod smtp;

use crate::domain::email::{EmailProvider, EmailSettings, EmailTransport, EmailTransportFactory};

use http_api::{PostmarkTransport, ResendTransport, UreqHttp};
use smtp::SmtpEmailTransport;

/// The production factory: picks the real transport for the provider.
pub struct ProviderTransports;

impl EmailTransportFactory for ProviderTransports {
    fn transport(
        &self,
        settings: &EmailSettings,
        secret: &str,
    ) -> Result<Box<dyn EmailTransport>, String> {
        Ok(match settings.provider {
            EmailProvider::None => return Err("Email isn't set up".to_string()),
            EmailProvider::Smtp => Box::new(SmtpEmailTransport::connect(settings, secret)?),
            EmailProvider::Resend => Box::new(ResendTransport::new(UreqHttp::new(), secret)),
            EmailProvider::Postmark => Box::new(PostmarkTransport::new(UreqHttp::new(), secret)),
        })
    }
}
