use anyhow::Result;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

mod adapters;
mod domain;
mod use_cases;

use adapters::config::Config;
use adapters::iqair::IQAirClient;
use adapters::telegram::TelegramClient;
use use_cases::{CheckAirQuality, NotifyAirQuality};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;
    info!("Starting Air Quality Notifier");
    info!("Monitoring {} locations", config.locations.len());
    for loc in &config.locations {
        info!("  - {}, {}", loc.city, loc.state);
    }
    info!("Schedule: {}", config.cron_schedule);

    let iqair_client = IQAirClient::new(config.iqair_token.clone());
    let telegram_client = TelegramClient::new(config.telegram_token.clone());

    let check_air_quality = CheckAirQuality::new(iqair_client);
    let notify_air_quality = NotifyAirQuality::new(telegram_client);

    let scheduler = JobScheduler::new().await?;

    let locations = config.locations.clone();
    let channel = config.telegram_channel.clone();

    let job = Job::new_async(config.cron_schedule.as_str(), move |_uuid, _lock| {
        let checker = check_air_quality.clone();
        let notifier = notify_air_quality.clone();
        let locs = locations.clone();
        let chat_id = channel.clone();

        Box::pin(async move {
            for location in locs {
                match checker.execute(location.clone()).await {
                    Ok(data) => {
                        info!(
                            "Air quality checked: {} PM2.5={} AQI={}",
                            data.location.city, data.pm25, data.aqi
                        );
                        if let Err(e) = notifier.execute(&chat_id, &data).await {
                            error!(
                                "Failed to send notification for {}: {}",
                                data.location.city, e
                            );
                        }
                    }
                    Err(e) => {
                        error!("Failed to check air quality for {}: {}", location.city, e);
                    }
                }
            }
        })
    })?;

    scheduler.add(job).await?;
    scheduler.start().await?;

    info!("Worker started successfully");

    tokio::signal::ctrl_c().await?;
    info!("Shutting down gracefully");

    Ok(())
}
