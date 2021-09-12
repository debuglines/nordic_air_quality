use macaddr::MacAddr6;

#[derive(Debug)]
pub struct SensorData {
    pub relative_humidity_percent: f32,
    pub radon_short_term: Option<u32>,
    pub radon_long_term: Option<u32>,
    pub temperature_celsius: f32,
    pub relative_atmospheric_pressure: f32,
    pub co2: f32,
    pub voc: f32,
}

#[derive(Debug)]
pub struct SensorMetadata {
    pub mac_address: MacAddr6,
    pub serial_number: Option<String>,
    pub measurements: SensorData,
}

pub enum SensorMeasurementKind {
    RelativeHumidity,
    RadonShortTerm,
    RadonLongTerm,
    TemperatureCelsius,
    RelativeAtmosphericPressure,
    Co2,
    Voc,
}

pub mod consts {
    pub const RELATIVE_HUMIDITY_UNIT_STR: &str = "%rH";
    pub const RADON_UNIT_STR: &str = "Bq/m3";
    pub const TEMPERATURE_CELSIUS_UNIT_STR: &str = "degC";
    /// Alternative: `hPA`
    pub const RELATIVE_ATMOSPHERIC_PRESSURE_UNIT_STR: &str = "mbar";
    pub const CO2_UNIT_STR: &str = "ppm";
    pub const VOC_UNIT_STR: &str = "ppb";
}

impl SensorMeasurementKind {
    pub fn unit_str(&self) -> String {
        match self {
            Self::RelativeHumidity => consts::RELATIVE_HUMIDITY_UNIT_STR,
            Self::RadonShortTerm => consts::RADON_UNIT_STR,
            Self::RadonLongTerm => consts::RADON_UNIT_STR,
            Self::TemperatureCelsius => consts::TEMPERATURE_CELSIUS_UNIT_STR,
            Self::RelativeAtmosphericPressure => consts::RELATIVE_ATMOSPHERIC_PRESSURE_UNIT_STR,
            Self::Co2 => consts::CO2_UNIT_STR,
            Self::Voc => consts::VOC_UNIT_STR,
        }
        .to_string()
    }
}
