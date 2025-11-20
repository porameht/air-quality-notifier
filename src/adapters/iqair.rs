use crate::domain::models::Location;
use crate::use_cases::check_air_quality::{AirQualityRepository, RawAirQualityData};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct IQAirClient {
    api_key: String,
    client: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    status: String,
    data: ApiData,
}

#[derive(Debug, Deserialize)]
struct ApiData {
    city: String,
    state: String,
    current: Current,
}

#[derive(Debug, Deserialize)]
struct Current {
    pollution: Pollution,
    weather: Weather,
}

#[derive(Debug, Deserialize)]
struct Pollution {
    #[serde(rename = "aqius")]
    aqi_us: i32,
}

#[derive(Debug, Deserialize)]
struct Weather {
    tp: i32,
    hu: i32,
}

impl IQAirClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AirQualityRepository for IQAirClient {
    async fn get_air_quality(&self, location: &Location) -> Result<RawAirQualityData> {
        let url = format!(
            "https://api.airvisual.com/v2/city?city={}&state={}&country={}&key={}",
            urlencoding::encode(&location.city),
            urlencoding::encode(&location.state),
            urlencoding::encode(&location.country),
            self.api_key
        );

        let response: ApiResponse = self
            .client
            .get(&url)
            .send()
            .await
            .context("Failed to fetch air quality data")?
            .error_for_status()
            .context("API returned error")?
            .json()
            .await
            .context("Failed to parse response")?;

        if response.status != "success" {
            anyhow::bail!("API error: {}", response.status);
        }

        Ok(RawAirQualityData {
            city: response.data.city,
            state: response.data.state,
            aqi: response.data.current.pollution.aqi_us,
            temperature: response.data.current.weather.tp,
            humidity: response.data.current.weather.hu,
        })
    }
}
