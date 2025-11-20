use crate::domain::models::Location;
use anyhow::{Context, Result};
use std::env;

#[derive(Debug)]
pub struct Config {
    pub iqair_token: String,
    pub telegram_token: String,
    pub telegram_channel: String,
    pub locations: Vec<Location>,
    pub cron_schedule: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let cities = env::var("CITIES").unwrap_or_else(|_| "Ban Suan".to_string());
        let state = env::var("STATE").unwrap_or_else(|_| "Chon Buri".to_string());
        let country = env::var("COUNTRY").unwrap_or_else(|_| "Thailand".to_string());

        let locations = cities
            .split(',')
            .map(|city| Location::new(city.trim(), &state, &country))
            .collect();

        Ok(Self {
            iqair_token: env::var("IQAIR_API_KEY").context("IQAIR_API_KEY not set")?,
            telegram_token: env::var("TELEGRAM_TOKEN").context("TELEGRAM_TOKEN not set")?,
            telegram_channel: env::var("TELEGRAM_CHANNEL").context("TELEGRAM_CHANNEL not set")?,
            locations,
            cron_schedule: env::var("CRON_SCHEDULE")
                .unwrap_or_else(|_| "0 0 8,12,18 * * *".to_string()),
        })
    }
}
