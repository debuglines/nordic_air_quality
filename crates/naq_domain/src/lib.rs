#[derive(Debug)]
pub struct SensorData {
    pub version: u32,
    pub humidity_percent: f32,
    pub radon_short_term: Option<u32>,
    pub radon_long_term: Option<u32>,
    pub temperature_celsius: f32,
    pub relative_atmospheric_pressure: f32,
    pub co2: f32,
    pub voc: f32,
}
