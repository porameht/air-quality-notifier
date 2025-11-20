use crate::domain::models::Location;
use anyhow::{Context, Result};
use shuttle_runtime::SecretStore;

#[derive(Debug)]
pub struct Config {
    pub iqair_token: String,
    pub telegram_token: String,
    pub telegram_channel: String,
    pub locations: Vec<Location>,
    pub cron_schedule: String,
}

impl Config {
    pub fn from_secrets(secrets: SecretStore) -> Result<Self> {
        let cities = secrets
            .get("CITIES")
            .unwrap_or_else(|| "Ban Suan".to_string());
        let state = secrets
            .get("STATE")
            .unwrap_or_else(|| "Chon Buri".to_string());
        let country = secrets
            .get("COUNTRY")
            .unwrap_or_else(|| "Thailand".to_string());

        let locations = cities
            .split(',')
            .map(|city| Location::new(city.trim(), &state, &country))
            .collect();

        Ok(Self {
            iqair_token: secrets
                .get("IQAIR_API_KEY")
                .context("IQAIR_API_KEY not set")?,
            telegram_token: secrets
                .get("TELEGRAM_TOKEN")
                .context("TELEGRAM_TOKEN not set")?,
            telegram_channel: secrets
                .get("TELEGRAM_CHANNEL")
                .context("TELEGRAM_CHANNEL not set")?,
            locations,
            cron_schedule: secrets
                .get("CRON_SCHEDULE")
                .unwrap_or_else(|| "0 0 8,12,18 * * *".to_string()),
        })
    }
}
