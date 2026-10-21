use crate::domain::email::{SecretError, SecretStore};

/// The tauri.conf.json bundle identifier, so the keychain item is clearly
/// attributable to Ripple in Keychain Access / Credential Manager.
pub const KEYCHAIN_SERVICE: &str = "com.denivic.ripple";

/// `SecretStore` over the OS keychain: Keychain Services on macOS, the
/// Credential Manager on Windows (chosen by `keyring`'s v1 mode).
pub struct KeychainSecretStore {
    service: &'static str,
}

impl KeychainSecretStore {
    pub fn new() -> Self {
        Self {
            service: KEYCHAIN_SERVICE,
        }
    }

    fn entry(&self, account: &str) -> Result<keyring::Entry, SecretError> {
        keyring::Entry::new(self.service, account).map_err(map_error)
    }
}

/// Never formats the error's payload for the encoding variants: those carry
/// the raw stored bytes, i.e. the secret.
fn map_error(e: keyring::Error) -> SecretError {
    match e {
        keyring::Error::NoDefaultStore | keyring::Error::NoStorageAccess(_) => {
            SecretError::Unavailable("access was denied or no keychain is set up".to_string())
        }
        keyring::Error::BadEncoding(_) | keyring::Error::BadDataFormat(_, _) => {
            SecretError::Failed("the saved secret is unreadable — replace it".to_string())
        }
        keyring::Error::PlatformFailure(err) => SecretError::Failed(err.to_string()),
        _ => SecretError::Failed("unexpected keychain error".to_string()),
    }
}

impl SecretStore for KeychainSecretStore {
    fn get(&self, account: &str) -> Result<Option<String>, SecretError> {
        match self.entry(account)?.get_password() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(map_error(e)),
        }
    }

    fn set(&self, account: &str, secret: &str) -> Result<(), SecretError> {
        self.entry(account)?.set_password(secret).map_err(map_error)
    }

    fn delete(&self, account: &str) -> Result<(), SecretError> {
        match self.entry(account)?.delete_credential() {
            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(map_error(e)),
        }
    }
}

/// Test double: a process-local map standing in for the keychain.
#[cfg(test)]
#[derive(Default)]
pub struct MemorySecretStore {
    secrets: std::sync::Mutex<std::collections::HashMap<String, String>>,
}

#[cfg(test)]
impl SecretStore for MemorySecretStore {
    fn get(&self, account: &str) -> Result<Option<String>, SecretError> {
        Ok(self.secrets.lock().unwrap().get(account).cloned())
    }

    fn set(&self, account: &str, secret: &str) -> Result<(), SecretError> {
        self.secrets
            .lock()
            .unwrap()
            .insert(account.to_string(), secret.to_string());
        Ok(())
    }

    fn delete(&self, account: &str) -> Result<(), SecretError> {
        self.secrets.lock().unwrap().remove(account);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_store_sets_gets_and_deletes() {
        let store = MemorySecretStore::default();
        assert_eq!(store.get("smtp-password").unwrap(), None);
        store.set("smtp-password", "hunter2").unwrap();
        assert_eq!(store.get("smtp-password").unwrap().as_deref(), Some("hunter2"));
        store.set("smtp-password", "replaced").unwrap();
        assert_eq!(store.get("smtp-password").unwrap().as_deref(), Some("replaced"));
        store.delete("smtp-password").unwrap();
        assert_eq!(store.get("smtp-password").unwrap(), None);
        // Deleting what isn't there is fine, like the real keychain wrapper.
        store.delete("smtp-password").unwrap();
    }

    #[test]
    fn keychain_errors_never_echo_stored_bytes() {
        let msg = map_error(keyring::Error::BadEncoding(b"hunter2".to_vec())).to_string();
        assert!(!msg.contains("hunter2"));
        assert!(matches!(map_error(keyring::Error::NoDefaultStore), SecretError::Unavailable(_)));
    }
}
