use std::time::Duration;

use serde::Deserialize;

use crate::domain::currency::{ExchangeRateSource, RateError};

/// Frankfurter serves the European Central Bank's daily reference rates:
/// free, no API key, and HTTPS from Rust means no CSP/capability changes.
const FRANKFURTER_URL: &str = "https://api.frankfurter.dev/v1/latest";
const TIMEOUT: Duration = Duration::from_secs(10);

pub struct FrankfurterRateSource {
    agent: ureq::Agent,
}

impl FrankfurterRateSource {
    pub fn new() -> Self {
        let config = ureq::Agent::config_builder()
            .timeout_global(Some(TIMEOUT))
            .build();
        Self {
            agent: config.into(),
        }
    }
}

#[derive(Deserialize)]
struct LatestResponse {
    rates: std::collections::HashMap<String, f64>,
}

impl ExchangeRateSource for FrankfurterRateSource {
    fn rate(&self, from: &str, to: &str) -> Result<f64, RateError> {
        let unsupported = || RateError::Unsupported {
            from: from.to_string(),
            to: to.to_string(),
        };
        let body = self
            .agent
            .get(FRANKFURTER_URL)
            .query("base", from)
            .query("symbols", to)
            .call()
            .and_then(|mut res| res.body_mut().read_to_string())
            .map_err(|e| match e {
                // Frankfurter answers 404 for a currency the ECB doesn't publish.
                ureq::Error::StatusCode(404 | 422) => unsupported(),
                ureq::Error::StatusCode(code) => {
                    RateError::Network(format!("the service answered HTTP {code}"))
                }
                other => RateError::Network(other.to_string()),
            })?;
        parse_rate(&body, to).ok_or_else(|| {
            if body.contains("\"rates\"") {
                unsupported()
            } else {
                RateError::InvalidResponse(body.chars().take(120).collect())
            }
        })
    }
}

fn parse_rate(body: &str, to: &str) -> Option<f64> {
    let parsed: LatestResponse = serde_json::from_str(body).ok()?;
    parsed
        .rates
        .get(to)
        .copied()
        .filter(|r| r.is_finite() && *r > 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_requested_rate() {
        let body = r#"{"amount":1.0,"base":"EUR","date":"2026-10-20","rates":{"USD":1.1215}}"#;
        assert_eq!(parse_rate(body, "USD"), Some(1.1215));
    }

    #[test]
    fn missing_or_nonsensical_rates_are_rejected() {
        assert_eq!(parse_rate(r#"{"rates":{}}"#, "USD"), None);
        assert_eq!(parse_rate(r#"{"rates":{"USD":0}}"#, "USD"), None);
        assert_eq!(parse_rate(r#"{"message":"not found"}"#, "USD"), None);
    }
}
