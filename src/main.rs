use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::{error, info};

mod adapters;
mod domain;
mod use_cases;

use adapters::bot::BotHandler;
use adapters::config::Config;
use adapters::iqair::IQAirClient;
use adapters::telegram::TelegramClient;
use use_cases::{CheckAirQuality, NotifyAirQuality};

struct AirQualityService {
    _scheduler: JobScheduler,
}

#[shuttle_runtime::async_trait]
impl shuttle_runtime::Service for AirQualityService {
    async fn bind(self, _addr: std::net::SocketAddr) -> Result<(), shuttle_runtime::Error> {
        info!("Worker started successfully");

        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_runtime::Secrets] secrets: shuttle_runtime::SecretStore,
) -> Result<AirQualityService, shuttle_runtime::Error> {
    let config = Config::from_secrets(secrets).map_err(shuttle_runtime::Error::Custom)?;

    info!("Starting Air Quality Notifier");
    info!("Monitoring {} locations", config.locations.len());
    for loc in &config.locations {
        info!("  - {}", loc.name);
    }
    info!("Schedule: {}", config.cron_schedule);

    let iqair_client = IQAirClient::new(config.iqair_token.clone());
    let telegram_client = TelegramClient::new(config.telegram_token.clone());

    let check_air_quality = CheckAirQuality::new(iqair_client.clone());
    let notify_air_quality = NotifyAirQuality::new(telegram_client);

    // Start Telegram bot in background
    let bot_checker = CheckAirQuality::new(iqair_client);

    let bot_handler = BotHandler::new(
        config.telegram_token.clone(),
        bot_checker,
        config.locations.clone(),
    );
    tokio::spawn(async move {
        bot_handler.run().await;
    });

    let scheduler = JobScheduler::new()
        .await
        .map_err(|e| shuttle_runtime::Error::Custom(e.into()))?;

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
                            data.location.name, data.pm25, data.aqi
                        );
                        if let Err(e) = notifier.execute(&chat_id, &data).await {
                            error!(
                                "Failed to send notification for {}: {}",
                                data.location.name, e
                            );
                        }
                    }
                    Err(e) => {
                        error!("Failed to check air quality for {}: {}", location.name, e);
                    }
                }
            }
        })
    })
    .map_err(|e| shuttle_runtime::Error::Custom(e.into()))?;

    scheduler
        .add(job)
        .await
        .map_err(|e| shuttle_runtime::Error::Custom(e.into()))?;

    scheduler
        .start()
        .await
        .map_err(|e| shuttle_runtime::Error::Custom(e.into()))?;

    Ok(AirQualityService {
        _scheduler: scheduler,
    })
}
