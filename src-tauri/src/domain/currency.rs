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
