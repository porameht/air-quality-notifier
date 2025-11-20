pub struct AqiConverter;

impl AqiConverter {
    pub fn estimate_pm25_from_aqi(aqi: i32) -> i32 {
        match aqi {
            0..=50 => (aqi as f64 * 12.0 / 50.0) as i32,
            51..=100 => (12.0 + ((aqi - 50) as f64 * 23.4 / 50.0)) as i32,
            101..=150 => (35.4 + ((aqi - 100) as f64 * 19.5 / 50.0)) as i32,
            151..=200 => (55.4 + ((aqi - 150) as f64 * 94.6 / 50.0)) as i32,
            201..=300 => (150.4 + ((aqi - 200) as f64 * 99.6 / 100.0)) as i32,
            _ => (250.4 + ((aqi - 300) as f64 * 249.6 / 200.0)) as i32,
        }
    }
}
