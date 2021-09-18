#![doc = include_str!("../README.md")]

use btleplug::api::{BDAddr, Central, Characteristic, Manager as _, Peripheral as _};
use btleplug::platform::{Adapter, Manager, Peripheral};
use lazy_static::lazy_static;
use naq_domain::{SensorData, SensorMetadata};
use std::time::Duration;
use std::{error::Error, fmt};
use tokio::time;
use uuid::Uuid;

const SENSOR_WAVE_PLUS_READ_UUID_STRING: &str = "b42e2a68-ade7-11e4-89d3-123b93f75cba";
lazy_static! {
    static ref SENSOR_WAVE_PLUS_READ_UUID: Uuid =
        Uuid::parse_str(SENSOR_WAVE_PLUS_READ_UUID_STRING).unwrap();
}

#[derive(Debug)]
pub enum ConnectorError {
    BluetoothAdapterNotFound,
    SensorDeviceNotFound,
    SensorReadCharacteristicNotFound,
}

impl Error for ConnectorError {}

impl fmt::Display for ConnectorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::BluetoothAdapterNotFound => "Bluetooth adapter not found",
            Self::SensorDeviceNotFound => "Sensor device not found",
            Self::SensorReadCharacteristicNotFound => "Unable to read sensor data",
        };

        write!(f, "{}", message)
    }
}

pub async fn check_sensor_data_by_mac_address(
    mac_address: &macaddr::MacAddr6,
    scan_duration: Duration,
) -> Result<SensorMetadata, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let central = get_central_adapter(&manager).await?;

    central.start_scan().await?;
    time::sleep(scan_duration).await;

    let sensor = find_device_by_mac_address(&central, &BDAddr::from(mac_address.into_array()))
        .await?
        .ok_or(ConnectorError::SensorDeviceNotFound)?;

    sensor.connect().await?;
    let chars = sensor.discover_characteristics().await?;

    let sensor_data = read_sensor_data(&sensor, &chars).await?;
    let sensor_metadata = SensorMetadata {
        mac_address: mac_address.clone(),
        serial_number: None,
        measurements: sensor_data,
    };

    Ok(sensor_metadata)
}

#[derive(Debug, serde::Deserialize)]
struct SensorDataRaw {
    version: u8,
    humidity: u8,
    unknown1: u8,
    unknown2: u8,
    radon_short_term: u16,
    radon_long_term: u16,
    temperature: u16,
    atmospheric_pressure: u16,
    co2: u16,
    voc: u16,
}

impl From<SensorDataRaw> for SensorData {
    fn from(raw: SensorDataRaw) -> Self {
        Self {
            relative_humidity_percent: raw.humidity as f32 / 2.0,
            radon_short_term: parse_raw_radon(raw.radon_short_term),
            radon_long_term: parse_raw_radon(raw.radon_long_term),
            temperature_celsius: raw.temperature as f32 / 100.0,
            relative_atmospheric_pressure: raw.atmospheric_pressure as f32 / 50.0,
            co2: raw.co2 as f32,
            voc: raw.voc as f32,
        }
    }
}

fn parse_raw_radon(input: u16) -> Option<u32> {
    (0..u16::MAX).contains(&input).then(|| input as u32)
}

async fn read_sensor_data(
    sensor: &Peripheral,
    chars: &[Characteristic],
) -> Result<SensorData, Box<dyn Error>> {
    let sensor_char = chars
        .iter()
        .find(|c| c.uuid == *SENSOR_WAVE_PLUS_READ_UUID)
        .ok_or(ConnectorError::SensorReadCharacteristicNotFound)?;

    let sensor_raw = sensor.read(sensor_char).await?;
    let decoded: SensorDataRaw = bincode::deserialize(&sensor_raw[..]).unwrap();

    Ok(SensorData::from(decoded))
}

async fn get_central_adapter(manager: &Manager) -> Result<Adapter, Box<dyn Error>> {
    let adapters = manager.adapters().await?;
    adapters
        .into_iter()
        .next()
        .ok_or_else(|| ConnectorError::BluetoothAdapterNotFound.into())
}

async fn find_device_by_mac_address(
    central: &Adapter,
    mac_address: &BDAddr,
) -> Result<Option<Peripheral>, Box<dyn Error>> {
    for p in central.peripherals().await? {
        if let Some(properties) = p.properties().await? {
            if properties.address == *mac_address {
                return Ok(Some(p));
            }
        }
    }
    Ok(None)
}
