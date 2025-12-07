use tracing::info;

mod adapters;
mod domain;
mod use_cases;

use adapters::bot::BotHandler;
use adapters::config::Config;
use adapters::iqair::IQAirClient;
use use_cases::CheckAirQuality;

struct AirQualityService;

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

    let iqair_client = IQAirClient::new(config.iqair_token.clone());
    let check_air_quality = CheckAirQuality::new(iqair_client);

    let bot_handler = BotHandler::new(
        config.telegram_token.clone(),
        check_air_quality,
        config.locations.clone(),
    );
    tokio::spawn(async move {
        bot_handler.run().await;
    });

    Ok(AirQualityService)
}
