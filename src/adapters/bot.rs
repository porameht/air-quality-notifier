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
}

impl<R: AirQualityRepository + Clone + 'static> BotHandler<R> {
    pub fn new(
        token: String,
        checker: CheckAirQuality<R>,
        locations: Vec<Location>,
    ) -> Self {
        Self {
            bot: Bot::new(token),
            checker: Arc::new(checker),
            locations: Arc::new(locations),
        }
    }

    pub async fn run(self) {
        let checker = self.checker;
        let locations = self.locations;

        let handler = Update::filter_message().filter_command::<Command>().endpoint(
            move |bot: Bot, msg: Message, cmd: Command| {
                let checker = checker.clone();
                let locations = locations.clone();

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
                                bot.send_message(msg.chat.id, "กรุณาระบุชื่อเมือง เช่น /check Ban Suan หรือ /check 13.46,101.09")
                                    .await?;
                            } else {
                                let location = parse_location_input(city);
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

fn parse_location_input(input: &str) -> Location {
    // Check if input looks like coordinates: "13.46,101.09"
    let parts: Vec<&str> = input.split(',').collect();
    if parts.len() == 2 {
        if let (Ok(lat), Ok(lon)) = (parts[0].trim().parse::<f64>(), parts[1].trim().parse::<f64>()) {
            // Validate reasonable lat/lon ranges
            if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon) {
                return Location::from_coordinates(format!("{:.2},{:.2}", lat, lon), lat, lon);
            }
        }
    }
    // Default: treat as city name with default state/country
    Location::from_city(input, "Chon Buri", "Thailand")
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
            let (city, state) = data.location.city_state();
            let location_str = if state.is_empty() {
                city
            } else {
                format!("{}, {}", city, state)
            };
            let message = format!(
                "{} <b>{}</b>\n\n\
                📍 {}\n\
                AQI <b>{}</b> · PM2.5 {} µg/m³\n\
                🌡️ {}°C · 💧 {}%\n\n\
                {}",
                level.emoji(),
                level.thai_description(),
                location_str,
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
            let error_msg = format!("❌ ไม่สามารถดึงข้อมูล {} ได้: {}", location.name, e);
            if let Err(e) = bot.send_message(msg.chat.id, error_msg).await {
                error!("Failed to send error message: {}", e);
            }
        }
    }
}
