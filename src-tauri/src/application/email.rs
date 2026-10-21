//! Sends notification emails with the user's configured provider, and the
//! use cases behind the Settings page's email section. Everything here is
//! blocking: the scheduler calls it from its own thread, and commands wrap
//! it in `spawn_blocking`.

pub mod templates;

use std::sync::Arc;

use crate::domain::email::{
    EmailProvider, EmailSettings, EmailSettingsRepository, EmailTransportFactory, OutgoingEmail,
    SecretStore, ALL_SECRET_ACCOUNTS,
};
use crate::infrastructure::db::{Db, SqliteEmailSettingsRepository};
use crate::infrastructure::email::keychain::KeychainSecretStore;
use crate::infrastructure::email::ProviderTransports;

/// One rendered email: a subject, a plain-text body and an optional HTML body.
#[derive(Debug, Clone, PartialEq)]
pub struct EmailMessage {
    pub subject: String,
    pub text: String,
    pub html: Option<String>,
}

/// Loads email_settings + the keychain secret and sends. Err(String) with a
/// user-readable reason; Ok(false) when provider is 'none' (not configured).
pub fn send_notification_email(db: &Arc<Db>, msg: &EmailMessage) -> Result<bool, String> {
    send_email(
        &SqliteEmailSettingsRepository::new(Arc::clone(db)),
        &KeychainSecretStore::new(),
        &ProviderTransports,
        msg,
    )
}

/// What the UI calls each provider's secret, for messages.
pub fn secret_label(provider: EmailProvider) -> &'static str {
    match provider {
        EmailProvider::None => "secret",
        EmailProvider::Smtp => "SMTP password",
        EmailProvider::Resend => "Resend API key",
        EmailProvider::Postmark => "Postmark server token",
    }
}

/// `send_notification_email` with its ports injected, so tests never touch
/// SQLite-on-disk, the keychain or the network.
pub fn send_email(
    repo: &dyn EmailSettingsRepository,
    secrets: &dyn SecretStore,
    transports: &dyn EmailTransportFactory,
    msg: &EmailMessage,
) -> Result<bool, String> {
    let settings = repo
        .get()
        .map_err(|e| format!("Couldn't read the email settings: {e}"))?;
    let Some(account) = settings.provider.secret_account() else {
        return Ok(false);
    };
    settings.validate()?;
    let secret = secrets
        .get(account)
        .map_err(|e| e.to_string())?
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            format!(
                "No {} is saved — add it in Settings",
                secret_label(settings.provider)
            )
        })?;
    let transport = transports.transport(&settings, &secret)?;
    transport.send(&OutgoingEmail {
        from: settings.from_address.unwrap_or_default().trim().to_string(),
        to: settings.to_address.unwrap_or_default().trim().to_string(),
        subject: msg.subject.clone(),
        text: msg.text.clone(),
        html: msg.html.clone(),
    })?;
    Ok(true)
}

/// Settings as the UI sees them: whether a secret exists, never the secret.
#[derive(Debug, Clone, PartialEq)]
pub struct EmailSettingsView {
    pub settings: EmailSettings,
    pub has_secret: bool,
}

pub fn get_settings(
    repo: &dyn EmailSettingsRepository,
    secrets: &dyn SecretStore,
) -> Result<EmailSettingsView, String> {
    let settings = repo.get().map_err(|e| e.to_string())?;
    // A keychain hiccup shouldn't take the whole Settings page down; the
    // user just sees "no secret saved" and can re-enter it.
    let has_secret = settings
        .provider
        .secret_account()
        .and_then(|account| secrets.get(account).ok().flatten())
        .is_some_and(|s| !s.is_empty());
    Ok(EmailSettingsView {
        settings,
        has_secret,
    })
}

/// Validates and stores the settings; a non-blank `new_secret` replaces the
/// chosen provider's keychain secret, `None` leaves it alone.
pub fn save_settings(
    repo: &dyn EmailSettingsRepository,
    secrets: &dyn SecretStore,
    settings: &EmailSettings,
    new_secret: Option<&str>,
) -> Result<(), String> {
    settings.validate()?;
    let new_secret = new_secret.map(str::trim).filter(|s| !s.is_empty());
    if let Some(secret) = new_secret {
        let account = settings
            .provider
            .secret_account()
            .ok_or("Choose an email provider before saving a password or key")?;
        secrets.set(account, secret).map_err(|e| e.to_string())?;
    }
    repo.save(settings).map_err(|e| e.to_string())
}

/// Removes the current provider's secret from the keychain.
pub fn clear_secret(
    repo: &dyn EmailSettingsRepository,
    secrets: &dyn SecretStore,
) -> Result<(), String> {
    let settings = repo.get().map_err(|e| e.to_string())?;
    match settings.provider.secret_account() {
        Some(account) => secrets.delete(account).map_err(|e| e.to_string()),
        None => Ok(()),
    }
}

/// For a full data reset: every secret Ripple may have stored, whichever
/// provider is current. Absent ones are skipped by `SecretStore::delete`.
pub fn delete_all_secrets(secrets: &dyn SecretStore) -> Result<(), String> {
    for account in ALL_SECRET_ACCOUNTS {
        secrets.delete(account).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Sends `templates::test_email()` with the saved settings.
pub fn send_test(
    repo: &dyn EmailSettingsRepository,
    secrets: &dyn SecretStore,
    transports: &dyn EmailTransportFactory,
) -> Result<(), String> {
    match send_email(repo, secrets, transports, &templates::test_email())? {
        true => Ok(()),
        false => Err("Choose an email provider and save first".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::domain::email::{EmailTransport, SmtpSecurity};
    use crate::domain::repository::RepoResult;
    use crate::infrastructure::email::keychain::MemorySecretStore;

    #[derive(Default)]
    struct MemoryRepo(Mutex<EmailSettings>);

    impl EmailSettingsRepository for MemoryRepo {
        fn get(&self) -> RepoResult<EmailSettings> {
            Ok(self.0.lock().unwrap().clone())
        }
        fn save(&self, settings: &EmailSettings) -> RepoResult<()> {
            *self.0.lock().unwrap() = settings.clone();
            Ok(())
        }
    }

    /// Records what would have been sent and which secret was used.
    #[derive(Default)]
    struct FakeTransports {
        sent: Arc<Mutex<Vec<(String, OutgoingEmail)>>>,
        fail_with: Option<String>,
    }

    struct FakeTransport {
        secret: String,
        sent: Arc<Mutex<Vec<(String, OutgoingEmail)>>>,
        fail_with: Option<String>,
    }

    impl EmailTransport for FakeTransport {
        fn send(&self, email: &OutgoingEmail) -> Result<(), String> {
            if let Some(e) = &self.fail_with {
                return Err(e.clone());
            }
            self.sent
                .lock()
                .unwrap()
                .push((self.secret.clone(), email.clone()));
            Ok(())
        }
    }

    impl EmailTransportFactory for FakeTransports {
        fn transport(
            &self,
            _settings: &EmailSettings,
            secret: &str,
        ) -> Result<Box<dyn EmailTransport>, String> {
            Ok(Box::new(FakeTransport {
                secret: secret.to_string(),
                sent: Arc::clone(&self.sent),
                fail_with: self.fail_with.clone(),
            }))
        }
    }

    fn resend() -> EmailSettings {
        EmailSettings {
            provider: EmailProvider::Resend,
            from_address: Some("ripple@example.com".to_string()),
            to_address: Some(" me@example.com ".to_string()),
            smtp_host: None,
            smtp_port: None,
            smtp_username: None,
            smtp_security: SmtpSecurity::StartTls,
        }
    }

    fn msg() -> EmailMessage {
        EmailMessage {
            subject: "Hi".to_string(),
            text: "Body".to_string(),
            html: None,
        }
    }

    #[test]
    fn provider_none_is_a_quiet_no_op() {
        let transports = FakeTransports::default();
        let sent = send_email(
            &MemoryRepo::default(),
            &MemorySecretStore::default(),
            &transports,
            &msg(),
        );
        assert_eq!(sent, Ok(false));
        assert!(transports.sent.lock().unwrap().is_empty());
    }

    #[test]
    fn sends_with_the_providers_secret_and_trimmed_addresses() {
        let repo = MemoryRepo(Mutex::new(resend()));
        let secrets = MemorySecretStore::default();
        secrets.set("resend-api-key", "re_123").unwrap();
        secrets.set("postmark-token", "wrong one").unwrap();
        let transports = FakeTransports::default();
        assert_eq!(send_email(&repo, &secrets, &transports, &msg()), Ok(true));
        let sent = transports.sent.lock().unwrap();
        assert_eq!(sent[0].0, "re_123");
        assert_eq!(sent[0].1.to, "me@example.com");
        assert_eq!(sent[0].1.subject, "Hi");
    }

    #[test]
    fn missing_secret_and_transport_errors_are_readable() {
        let repo = MemoryRepo(Mutex::new(resend()));
        let secrets = MemorySecretStore::default();
        let err = send_email(&repo, &secrets, &FakeTransports::default(), &msg()).unwrap_err();
        assert_eq!(err, "No Resend API key is saved — add it in Settings");

        secrets.set("resend-api-key", "re_123").unwrap();
        let failing = FakeTransports {
            fail_with: Some("Authentication failed — check the Resend API key".to_string()),
            ..Default::default()
        };
        assert_eq!(
            send_test(&repo, &secrets, &failing).unwrap_err(),
            "Authentication failed — check the Resend API key"
        );
    }

    #[test]
    fn send_test_without_a_provider_explains_itself() {
        let err = send_test(
            &MemoryRepo::default(),
            &MemorySecretStore::default(),
            &FakeTransports::default(),
        )
        .unwrap_err();
        assert_eq!(err, "Choose an email provider and save first");
    }

    #[test]
    fn save_stores_secret_in_keychain_only_and_view_reports_has_secret() {
        let repo = MemoryRepo::default();
        let secrets = MemorySecretStore::default();
        assert!(!get_settings(&repo, &secrets).unwrap().has_secret);

        save_settings(&repo, &secrets, &resend(), Some("  re_123  ")).unwrap();
        assert_eq!(secrets.get("resend-api-key").unwrap().as_deref(), Some("re_123"));
        let view = get_settings(&repo, &secrets).unwrap();
        assert!(view.has_secret);
        assert_eq!(view.settings, resend());

        // Saving again without a new secret keeps the stored one.
        save_settings(&repo, &secrets, &resend(), None).unwrap();
        assert!(get_settings(&repo, &secrets).unwrap().has_secret);

        clear_secret(&repo, &secrets).unwrap();
        assert!(!get_settings(&repo, &secrets).unwrap().has_secret);
    }

    #[test]
    fn save_rejects_invalid_settings_and_orphan_secrets() {
        let repo = MemoryRepo::default();
        let secrets = MemorySecretStore::default();
        let bad = EmailSettings {
            to_address: None,
            ..resend()
        };
        assert!(save_settings(&repo, &secrets, &bad, Some("k")).is_err());
        assert_eq!(secrets.get("resend-api-key").unwrap(), None);
        assert!(save_settings(&repo, &secrets, &EmailSettings::default(), Some("k")).is_err());
    }

    #[test]
    fn delete_all_secrets_clears_every_account() {
        let secrets = MemorySecretStore::default();
        for account in ALL_SECRET_ACCOUNTS {
            secrets.set(account, "x").unwrap();
        }
        delete_all_secrets(&secrets).unwrap();
        // Absent ones are fine too.
        delete_all_secrets(&secrets).unwrap();
        for account in ALL_SECRET_ACCOUNTS {
            assert_eq!(secrets.get(account).unwrap(), None);
        }
    }

    #[test]
    fn real_db_with_default_settings_sends_nothing() {
        let db = Arc::new(Db::open_in_memory().unwrap());
        assert_eq!(send_notification_email(&db, &msg()), Ok(false));
    }
}
