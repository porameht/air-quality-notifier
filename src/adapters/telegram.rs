use crate::use_cases::notify_air_quality::NotificationGateway;
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct TelegramClient {
    token: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize)]
struct SendMessageRequest {
    chat_id: String,
    text: String,
    parse_mode: String,
}

impl TelegramClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl NotificationGateway for TelegramClient {
    async fn send(&self, channel_id: &str, message: &str) -> Result<()> {
        let url = format!("https://api.telegram.org/bot{}/sendMessage", self.token);

        let request = SendMessageRequest {
            chat_id: channel_id.to_string(),
            text: message.to_string(),
            parse_mode: "Markdown".to_string(),
        };

        self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send message to Telegram")?
            .error_for_status()
            .context("Telegram API returned error")?;

        Ok(())
    }
}
