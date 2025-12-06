use crate::domain::models::{AirQualityData, Location};
use crate::domain::services::AqiConverter;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait AirQualityRepository: Send + Sync {
    async fn get_air_quality(&self, location: &Location) -> Result<RawAirQualityData>;
}

pub struct RawAirQualityData {
    pub city: String,
    pub state: String,
    pub aqi: i32,
    pub temperature: i32,
    pub humidity: i32,
}

pub struct CheckAirQuality<R: AirQualityRepository> {
    repository: R,
}

impl<R: AirQualityRepository> Clone for CheckAirQuality<R>
where
    R: Clone,
{
    fn clone(&self) -> Self {
        Self {
            repository: self.repository.clone(),
        }
    }
}

impl<R: AirQualityRepository> CheckAirQuality<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, location: Location) -> Result<AirQualityData> {
        let raw_data = self.repository.get_air_quality(&location).await?;
        let pm25 = AqiConverter::estimate_pm25_from_aqi(raw_data.aqi);

        Ok(AirQualityData {
            location: Location::from_city(raw_data.city, raw_data.state, "Thailand"),
            aqi: raw_data.aqi,
            pm25,
            temperature: raw_data.temperature,
            humidity: raw_data.humidity,
        })
    }
}
