use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub query: LocationQuery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationQuery {
    City {
        city: String,
        state: String,
        country: String,
    },
    Coordinates {
        lat: f64,
        lon: f64,
    },
}

impl Location {
    pub fn from_city(
        city: impl Into<String>,
        state: impl Into<String>,
        country: impl Into<String>,
    ) -> Self {
        let city = city.into();
        Self {
            name: city.clone(),
            query: LocationQuery::City {
                city,
                state: state.into(),
                country: country.into(),
            },
        }
    }

    pub fn from_coordinates(name: impl Into<String>, lat: f64, lon: f64) -> Self {
        Self {
            name: name.into(),
            query: LocationQuery::Coordinates { lat, lon },
        }
    }

    pub fn city_state(&self) -> (String, String) {
        match &self.query {
            LocationQuery::City { city, state, .. } => (city.clone(), state.clone()),
            LocationQuery::Coordinates { .. } => (self.name.clone(), String::new()),
        }
    }

    pub fn city_state_country(&self) -> (String, String, String) {
        match &self.query {
            LocationQuery::City { city, state, country } => {
                (city.clone(), state.clone(), country.clone())
            }
            LocationQuery::Coordinates { .. } => {
                (self.name.clone(), String::new(), String::new())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct AirQualityData {
    pub location: Location,
    pub aqi: i32,
    pub pm25: i32,
    pub temperature: i32,
    pub humidity: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AirQualityLevel {
    Good,
    Moderate,
    UnhealthyForSensitive,
    Unhealthy,
    VeryUnhealthy,
}

impl AirQualityLevel {
    pub fn from_aqi(aqi: i32) -> Self {
        match aqi {
            0..=50 => Self::Good,
            51..=100 => Self::Moderate,
            101..=150 => Self::UnhealthyForSensitive,
            151..=200 => Self::Unhealthy,
            _ => Self::VeryUnhealthy,
        }
    }

    pub fn thai_description(&self) -> &'static str {
        match self {
            Self::Good => "อากาศดี",
            Self::Moderate => "พอใช้ได้",
            Self::UnhealthyForSensitive => "เริ่มแย่",
            Self::Unhealthy => "มีผลกระทบต่อสุขภาพ",
            Self::VeryUnhealthy => "มีผลกระทบต่อสุขภาพมาก",
        }
    }

    pub fn emoji(&self) -> &'static str {
        match self {
            Self::Good => "🟢",
            Self::Moderate => "🟡",
            Self::UnhealthyForSensitive => "🟠",
            Self::Unhealthy => "🔴",
            Self::VeryUnhealthy => "🟣",
        }
    }

    pub fn health_warning(&self) -> &'static str {
        match self {
            Self::Good => "ออกไปข้างนอกได้สบายๆ 👍",
            Self::Moderate => "ออกไปได้ แต่คนแพ้ง่ายควรระวัง",
            Self::UnhealthyForSensitive => {
                "เด็ก คนแก่ คนป่วย ไม่ควรออกไปข้างนอก"
            }
            Self::Unhealthy => {
                "อันตราย! ถ้าต้องออกไป ใส่ N95"
            }
            Self::VeryUnhealthy => {
                "อันตรายมาก! อยู่ในบ้าน ปิดหน้าต่าง เปิดเครื่องฟอก"
            }
        }
    }
}
