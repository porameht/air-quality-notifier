use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub city: String,
    pub state: String,
    pub country: String,
}

impl Location {
    pub fn new(
        city: impl Into<String>,
        state: impl Into<String>,
        country: impl Into<String>,
    ) -> Self {
        Self {
            city: city.into(),
            state: state.into(),
            country: country.into(),
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
    pub fn from_pm25(pm25: i32) -> Self {
        match pm25 {
            0..=25 => Self::Good,
            26..=37 => Self::Moderate,
            38..=50 => Self::UnhealthyForSensitive,
            51..=90 => Self::Unhealthy,
            _ => Self::VeryUnhealthy,
        }
    }

    pub fn thai_description(&self) -> &'static str {
        match self {
            Self::Good => "ดีมาก (Good)",
            Self::Moderate => "ปานกลาง (Moderate)",
            Self::UnhealthyForSensitive => "เริ่มมีผลกระทบต่อสุขภาพ",
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
            Self::Good => "✅ คุณภาพอากาศดี ปลอดภัยสำหรับกิจกรรมกลางแจ้ง",
            Self::Moderate => "⚠️ คนไวต่ออากาศควรระวัง",
            Self::UnhealthyForSensitive => "⚠️ ⚠️ กลุ่มเสี่ยงควรลดกิจกรรมกลางแจ้ง\nเด็ก ผู้สูงอายุ ผู้ป่วยโรคหัวใจและปอด",
            Self::Unhealthy => "🚨 อันตราย! ทุกคนควรหลีกเลี่ยงกิจกรรมกลางแจ้ง\nสวมหน้ากาก N95 หากจำเป็นต้องออกไป",
            Self::VeryUnhealthy => "🚨🚨 อันตรายมาก! ห้ามออกกลางแจ้ง\nอยู่ในบ้านและปิดหน้าต่างทุกบาน\nใช้เครื่องฟอกอากาศ",
        }
    }
}
