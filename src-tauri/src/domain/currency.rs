use thiserror::Error;

/// Why an exchange rate couldn't be had. The messages are user-facing: the
/// Settings page shows them as-is in the conversion dialog.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum RateError {
    #[error("No exchange rate is published for {from} → {to}. Rates come from the European Central Bank, which covers about 30 major currencies.")]
    Unsupported { from: String, to: String },
    #[error("Couldn't reach the exchange-rate service: {0}")]
    Network(String),
    #[error("The exchange-rate service sent an unexpected response: {0}")]
    InvalidResponse(String),
}

/// Port for "how many `to` is one `from` worth today". Kept out of
/// infrastructure so the conversion use case (and its tests) never touch the
/// network.
pub trait ExchangeRateSource {
    fn rate(&self, from: &str, to: &str) -> Result<f64, RateError>;
}

/// Used when the system region is unknown or not in `REGION_CURRENCIES`.
pub const FALLBACK_CURRENCY: &str = "USD";

static SYSTEM_CURRENCY: std::sync::OnceLock<&'static str> = std::sync::OnceLock::new();

/// Records the currency of the user's system region, once, at startup. The
/// domain stays free of OS calls; the interface layer detects the locale.
pub fn set_system_currency(code: &'static str) {
    let _ = SYSTEM_CURRENCY.set(code);
}

/// The currency a fresh (or reset) install starts with: the system region's,
/// so a user in Denmark sees DKK rather than an assumed USD.
pub fn default_currency() -> &'static str {
    SYSTEM_CURRENCY.get().copied().unwrap_or(FALLBACK_CURRENCY)
}

/// Maps a BCP 47 / POSIX locale ("da-DK", "en_DK.UTF-8") to its region's
/// currency. Only the region matters: an English UI in Denmark is still DKK.
pub fn currency_for_locale(locale: &str) -> Option<&'static str> {
    let region = locale
        .split(['-', '_', '.', '@'])
        .skip(1)
        .find(|part| part.len() == 2 && part.bytes().all(|b| b.is_ascii_alphabetic()))?
        .to_ascii_uppercase();
    REGION_CURRENCIES
        .iter()
        .find(|(r, _)| *r == region)
        .map(|(_, currency)| *currency)
}

const REGION_CURRENCIES: &[(&str, &str)] = &[
    // Euro area (incl. Bulgaria since 2026) and states that use the euro.
    ("AT", "EUR"), ("BE", "EUR"), ("BG", "EUR"), ("HR", "EUR"), ("CY", "EUR"),
    ("EE", "EUR"), ("FI", "EUR"), ("FR", "EUR"), ("DE", "EUR"), ("GR", "EUR"),
    ("IE", "EUR"), ("IT", "EUR"), ("LV", "EUR"), ("LT", "EUR"), ("LU", "EUR"),
    ("MT", "EUR"), ("NL", "EUR"), ("PT", "EUR"), ("SK", "EUR"), ("SI", "EUR"),
    ("ES", "EUR"), ("AD", "EUR"), ("MC", "EUR"), ("SM", "EUR"), ("VA", "EUR"),
    ("ME", "EUR"), ("XK", "EUR"),
    // Rest of Europe.
    ("DK", "DKK"), ("FO", "DKK"), ("GL", "DKK"), ("SE", "SEK"), ("NO", "NOK"),
    ("IS", "ISK"), ("CH", "CHF"), ("LI", "CHF"), ("GB", "GBP"), ("PL", "PLN"),
    ("CZ", "CZK"), ("HU", "HUF"), ("RO", "RON"), ("RS", "RSD"), ("UA", "UAH"),
    ("TR", "TRY"), ("RU", "RUB"),
    // Americas.
    ("US", "USD"), ("CA", "CAD"), ("MX", "MXN"), ("BR", "BRL"), ("AR", "ARS"),
    ("CL", "CLP"), ("CO", "COP"), ("PE", "PEN"),
    // Asia-Pacific.
    ("AU", "AUD"), ("NZ", "NZD"), ("JP", "JPY"), ("CN", "CNY"), ("HK", "HKD"),
    ("TW", "TWD"), ("KR", "KRW"), ("SG", "SGD"), ("MY", "MYR"), ("TH", "THB"),
    ("ID", "IDR"), ("PH", "PHP"), ("VN", "VND"), ("IN", "INR"), ("PK", "PKR"),
    ("BD", "BDT"), ("LK", "LKR"),
    // Middle East and Africa.
    ("AE", "AED"), ("SA", "SAR"), ("QA", "QAR"), ("KW", "KWD"), ("IL", "ILS"),
    ("EG", "EGP"), ("ZA", "ZAR"), ("NG", "NGN"), ("KE", "KES"), ("MA", "MAD"),
    ("GH", "GHS"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn currency_comes_from_the_region_not_the_language() {
        assert_eq!(currency_for_locale("en-DK"), Some("DKK"));
        assert_eq!(currency_for_locale("da_DK.UTF-8"), Some("DKK"));
        assert_eq!(currency_for_locale("de-AT"), Some("EUR"));
        assert_eq!(currency_for_locale("en-GB"), Some("GBP"));
        assert_eq!(currency_for_locale("zh-Hans-CN"), Some("CNY"));
    }

    #[test]
    fn unknown_or_regionless_locales_have_no_currency() {
        assert_eq!(currency_for_locale("en"), None);
        assert_eq!(currency_for_locale("xx-ZZ"), None);
    }

    #[test]
    fn region_table_uses_iso_shaped_codes() {
        for (region, currency) in REGION_CURRENCIES {
            assert_eq!(region.len(), 2);
            assert!(crate::domain::is_currency_code(currency));
        }
    }
}
