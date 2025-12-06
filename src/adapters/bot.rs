use crate::domain::models::Location;
use crate::use_cases::CheckAirQuality;
use crate::use_cases::check_air_quality::AirQualityRepository;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::utils::command::BotCommands;
use tracing::{error, info};

use crate::domain::models::AirQualityLevel;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "คำสั่งที่ใช้ได้:")]
pub enum Command {
    #[command(description = "แสดงคำสั่งทั้งหมด")]
    Help,
    #[command(description = "ดูคุณภาพอากาศทุกพื้นที่")]
    Pm25,
    #[command(description = "ดูคุณภาพอากาศ เช่น /check Ban Suan")]
    Check(String),
}

pub struct BotHandler<R: AirQualityRepository + Clone + 'static> {
    bot: Bot,
    checker: Arc<CheckAirQuality<R>>,
    locations: Arc<Vec<Location>>,
    country: String,
    state: String,
}

impl<R: AirQualityRepository + Clone + 'static> BotHandler<R> {
    pub fn new(
        token: String,
        checker: CheckAirQuality<R>,
        locations: Vec<Location>,
        state: String,
        country: String,
    ) -> Self {
        Self {
            bot: Bot::new(token),
            checker: Arc::new(checker),
            locations: Arc::new(locations),
            country,
            state,
        }
    }

    pub async fn run(self) {
        let checker = self.checker;
        let locations = self.locations;
        let state = self.state;
        let country = self.country;

        let handler = Update::filter_message().filter_command::<Command>().endpoint(
            move |bot: Bot, msg: Message, cmd: Command| {
                let checker = checker.clone();
                let locations = locations.clone();
                let state = state.clone();
                let country = country.clone();

                async move {
                    match cmd {
                        Command::Help => {
                            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                                .await?;
                        }
                        Command::Pm25 => {
                            handle_pm25(&bot, &msg, &checker, &locations).await;
                        }
                        Command::Check(city) => {
                            let city = city.trim();
                            if city.is_empty() {
                                bot.send_message(msg.chat.id, "กรุณาระบุชื่อเมือง เช่น /check Ban Suan")
                                    .await?;
                            } else {
                                let location = Location::new(city, &state, &country);
                                handle_check(&bot, &msg, &checker, &location).await;
                            }
                        }
                    }
                    Ok::<(), teloxide::RequestError>(())
                }
            },
        );

        info!("Starting Telegram bot...");
        Dispatcher::builder(self.bot, handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

async fn handle_pm25<R: AirQualityRepository>(
    bot: &Bot,
    msg: &Message,
    checker: &CheckAirQuality<R>,
    locations: &[Location],
) {
    for location in locations {
        handle_check(bot, msg, checker, location).await;
    }
}

async fn handle_check<R: AirQualityRepository>(
    bot: &Bot,
    msg: &Message,
    checker: &CheckAirQuality<R>,
    location: &Location,
) {
    match checker.execute(location.clone()).await {
        Ok(data) => {
            let level = AirQualityLevel::from_aqi(data.aqi);
            let message = format!(
                "{} <b>{}</b>\n\n\
                📍 {}, {}\n\
                AQI <b>{}</b> · PM2.5 {} µg/m³\n\
                🌡️ {}°C · 💧 {}%\n\n\
                {}",
                level.emoji(),
                level.thai_description(),
                data.location.city,
                data.location.state,
                data.aqi,
                data.pm25,
                data.temperature,
                data.humidity,
                level.health_warning(),
            );

            if let Err(e) = bot
                .send_message(msg.chat.id, message)
                .parse_mode(teloxide::types::ParseMode::Html)
                .await
            {
                error!("Failed to send message: {}", e);
            }
        }
        Err(e) => {
            let error_msg = format!("❌ ไม่สามารถดึงข้อมูล {} ได้: {}", location.city, e);
            if let Err(e) = bot.send_message(msg.chat.id, error_msg).await {
                error!("Failed to send error message: {}", e);
            }
        }
    }
}
