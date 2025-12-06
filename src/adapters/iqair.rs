use crate::domain::models::Location;
use crate::use_cases::check_air_quality::{AirQualityRepository, RawAirQualityData};
use anyhow::{Context, Result};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct IQAirClient {
    api_key: String,
    client: reqwest::Client,
    city_coordinates: HashMap<String, (f64, f64)>,
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
        let mut city_coordinates = HashMap::new();
        // Cities not directly supported by IQAir API but available via coordinates
        city_coordinates.insert("phan thong".to_string(), (13.4617, 101.0817));
        city_coordinates.insert("phanthong".to_string(), (13.4617, 101.0817));

        Self {
            api_key,
            client: reqwest::Client::new(),
            city_coordinates,
        }
    }

    fn build_city_url(&self, city: &str, state: &str, country: &str) -> String {
        format!(
            "https://api.airvisual.com/v2/city?city={}&state={}&country={}&key={}",
            urlencoding::encode(city),
            urlencoding::encode(state),
            urlencoding::encode(country),
            self.api_key
        )
    }

    fn build_coords_url(&self, lat: f64, lon: f64) -> String {
        format!(
            "https://api.airvisual.com/v2/nearest_city?lat={}&lon={}&key={}",
            lat, lon, self.api_key
        )
    }

    async fn fetch_api(&self, url: &str) -> Result<ApiResponse> {
        let response: ApiResponse = self
            .client
            .get(url)
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

        Ok(response)
    }
}

#[async_trait]
impl AirQualityRepository for IQAirClient {
    async fn get_air_quality(&self, location: &Location) -> Result<RawAirQualityData> {
        let (city, state, country) = location.city_state_country();

        // Try city name first
        let city_url = self.build_city_url(&city, &state, &country);
        let response = match self.fetch_api(&city_url).await {
            Ok(resp) => resp,
            Err(_) => {
                // Fallback: check if we have coordinates for this city
                let city_lower = city.to_lowercase();
                if let Some(&(lat, lon)) = self.city_coordinates.get(&city_lower) {
                    let coords_url = self.build_coords_url(lat, lon);
                    self.fetch_api(&coords_url).await?
                } else {
                    // No fallback available, return original error
                    anyhow::bail!("City '{}' not found in IQAir API", city);
                }
            }
        };

        Ok(RawAirQualityData {
            city: response.data.city,
            state: response.data.state,
            aqi: response.data.current.pollution.aqi_us,
            temperature: response.data.current.weather.tp,
            humidity: response.data.current.weather.hu,
        })
    }
}
