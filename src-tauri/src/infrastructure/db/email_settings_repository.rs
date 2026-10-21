use std::sync::Arc;

use rusqlite::{params, OptionalExtension};

use crate::domain::email::{EmailProvider, EmailSettings, EmailSettingsRepository, SmtpSecurity};
use crate::domain::repository::RepoResult;

use super::Db;

/// Only the non-secret half of the email configuration; passwords and API
/// keys are in the OS keychain, never in this table.
pub struct SqliteEmailSettingsRepository {
    db: Arc<Db>,
}

impl SqliteEmailSettingsRepository {
    pub fn new(db: Arc<Db>) -> Self {
        Self { db }
    }
}

impl EmailSettingsRepository for SqliteEmailSettingsRepository {
    fn get(&self) -> RepoResult<EmailSettings> {
        let conn = self.db.lock()?;
        let settings = conn
            .query_row(
                "SELECT provider, from_address, to_address, smtp_host, smtp_port, smtp_username, smtp_security
                 FROM email_settings WHERE id = 1",
                [],
                |row| {
                    let provider: String = row.get(0)?;
                    let port: Option<i64> = row.get(4)?;
                    let security: String = row.get(6)?;
                    Ok(EmailSettings {
                        // An unknown value (hand-edited DB, future version)
                        // degrades to "off" rather than failing every send.
                        provider: EmailProvider::parse(&provider).unwrap_or(EmailProvider::None),
                        from_address: row.get(1)?,
                        to_address: row.get(2)?,
                        smtp_host: row.get(3)?,
                        smtp_port: port.and_then(|p| u16::try_from(p).ok()).filter(|p| *p > 0),
                        smtp_username: row.get(5)?,
                        smtp_security: SmtpSecurity::parse(&security)
                            .unwrap_or(SmtpSecurity::StartTls),
                    })
                },
            )
            .optional()?;
        Ok(settings.unwrap_or_default())
    }

    fn save(&self, s: &EmailSettings) -> RepoResult<()> {
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT INTO email_settings (id, provider, from_address, to_address, smtp_host, smtp_port, smtp_username, smtp_security)
             VALUES (1, ?1, ?2, ?3, ?4, ?5, ?6, ?7)
             ON CONFLICT(id) DO UPDATE SET
               provider = excluded.provider,
               from_address = excluded.from_address,
               to_address = excluded.to_address,
               smtp_host = excluded.smtp_host,
               smtp_port = excluded.smtp_port,
               smtp_username = excluded.smtp_username,
               smtp_security = excluded.smtp_security",
            params![
                s.provider.as_str(),
                s.from_address,
                s.to_address,
                s.smtp_host,
                s.smtp_port,
                s.smtp_username,
                s.smtp_security.as_str(),
            ],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo() -> SqliteEmailSettingsRepository {
        SqliteEmailSettingsRepository::new(Arc::new(Db::open_in_memory().unwrap()))
    }

    #[test]
    fn get_before_any_save_returns_default() {
        assert_eq!(repo().get().unwrap(), EmailSettings::default());
    }

    #[test]
    fn save_then_get_round_trips_and_upserts() {
        let repo = repo();
        let mut settings = EmailSettings {
            provider: EmailProvider::Smtp,
            from_address: Some("me@example.com".to_string()),
            to_address: Some("you@example.com".to_string()),
            smtp_host: Some("smtp.example.com".to_string()),
            smtp_port: Some(465),
            smtp_username: Some("me".to_string()),
            smtp_security: SmtpSecurity::Tls,
        };
        repo.save(&settings).unwrap();
        assert_eq!(repo.get().unwrap(), settings);
        settings.provider = EmailProvider::Postmark;
        repo.save(&settings).unwrap();
        assert_eq!(repo.get().unwrap().provider, EmailProvider::Postmark);
    }

    #[test]
    fn unknown_stored_values_fall_back_to_defaults() {
        let repo = repo();
        repo.db
            .lock()
            .unwrap()
            .execute(
                "INSERT INTO email_settings (id, provider, smtp_port, smtp_security) VALUES (1, 'carrier-pigeon', 99999, 'plaintext')",
                [],
            )
            .unwrap();
        let s = repo.get().unwrap();
        assert_eq!(s.provider, EmailProvider::None);
        assert_eq!(s.smtp_port, None);
        assert_eq!(s.smtp_security, SmtpSecurity::StartTls);
    }
}
