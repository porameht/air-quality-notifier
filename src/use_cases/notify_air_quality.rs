use crate::domain::models::{AirQualityData, AirQualityLevel};
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait NotificationGateway: Send + Sync {
    async fn send(&self, channel_id: &str, message: &str) -> Result<()>;
}

pub struct NotifyAirQuality<N: NotificationGateway> {
    gateway: N,
}

impl<N: NotificationGateway> Clone for NotifyAirQuality<N>
where
    N: Clone,
{
    fn clone(&self) -> Self {
        Self {
            gateway: self.gateway.clone(),
        }
    }
}

impl<N: NotificationGateway> NotifyAirQuality<N> {
    pub fn new(gateway: N) -> Self {
        Self { gateway }
    }

    pub async fn execute(&self, channel_id: &str, data: &AirQualityData) -> Result<()> {
        let message = self.format_message(data);
        self.gateway.send(channel_id, &message).await
    }

    fn format_message(&self, data: &AirQualityData) -> String {
        let level = AirQualityLevel::from_pm25(data.pm25);

        format!(
            "🌫️ *รายงานคุณภาพอากาศ*\n\n\
            {} *ระดับ: {}*\n\
            ━━━━━━━━━━━━━━━━━━\n\
            📍 {}, {}\n\
            🌫️ PM2.5: *{} µg/m³*\n\
            📊 AQI: {}\n\
            🌡️ {}°C | 💧 {}%\n\
            ━━━━━━━━━━━━━━━━━━\n\
            {}\n\n\
            ⏰ {}",
            level.emoji(),
            level.thai_description(),
            data.location.city,
            data.location.state,
            data.pm25,
            data.aqi,
            data.temperature,
            data.humidity,
            level.health_warning(),
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        )
    }
}
