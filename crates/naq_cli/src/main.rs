#![doc = include_str!("../README.md")]

use macaddr::MacAddr6;
use naq_device_connector::check_sensor_data_by_mac_address;
use naq_domain::{SensorData, SensorMeasurementKind};
use std::process;
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "naq",
    about = "Nordic air quality cli that can get data from sensor devices"
)]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(subcommand)]
    /// Check the sensor device
    check: CheckCommand,
}

#[derive(Debug, StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum CheckCommand {
    /// Check sensor device using serial number
    Serial { serial_number: String },
    /// Check sensor device using mac address.
    /// Example: 12:34:56:78:9A:BC
    Mac {
        #[structopt(help = "Example: 12:34:56:78:9A:BC")]
        mac_address: String,
    },
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();
    match opt.check {
        CheckCommand::Serial { serial_number } => {}
        CheckCommand::Mac { mac_address } => {
            check_by_mac_address(&mac_address).await;
        }
    }
}

async fn check_by_mac_address<S: AsRef<str>>(mac_address_input: S) {
    let mac_address = parse_mac_address_or_exit(mac_address_input);
    println!("> Checking for sensor data. Please wait a few seconds ...");
    let sensor_metadata = check_sensor_data_by_mac_address(&mac_address, Duration::from_secs(6))
        .await
        .unwrap();

    println!("> Found sensor data");
    println!();
    println!("T- Timestamp: {}", timestamp_now_formatted());
    println!("|- MAC address: {}", sensor_metadata.mac_address);
    println!(
        "|- Serial number: {}",
        sensor_metadata
            .serial_number
            .unwrap_or_else(|| "<unable to extract>".to_string())
    );
    println!("`- Measurements");
    print_sensor_measurements(&sensor_metadata.measurements);
    println!();
}

fn print_sensor_measurements(sensor_data: &SensorData) {
    println!(
        " |- CO2: {} {}",
        sensor_data.co2,
        SensorMeasurementKind::Co2.unit_str()
    );
    println!(
        " |- TVOC: {} {}",
        sensor_data.voc,
        SensorMeasurementKind::Voc.unit_str()
    );

    if let Some(radon) = sensor_data.radon_short_term {
        println!(
            " |- Radon: {} {}",
            radon,
            SensorMeasurementKind::RadonShortTerm.unit_str()
        );
    } else {
        println!(" |- Radon: Not available");
    }
    println!(
        " |- Temperature: {} {}",
        sensor_data.temperature_celsius,
        SensorMeasurementKind::TemperatureCelsius.unit_str()
    );
    println!(
        " |- Humidity: {} {}",
        sensor_data.relative_humidity_percent,
        SensorMeasurementKind::RelativeHumidity.unit_str()
    );
    println!(
        " `- A. pressure: {} {}",
        sensor_data.relative_atmospheric_pressure,
        SensorMeasurementKind::RelativeAtmosphericPressure.unit_str()
    );
}

/// For the current time, returns date and time string such as 1996-12-19 16:39:57 (+0930).
fn timestamp_now_formatted() -> String {
    use chrono::prelude::{DateTime, Local};

    let system_time = std::time::SystemTime::now();
    let datetime: DateTime<Local> = system_time.into();
    format!("{}", datetime.format("%F %T (%z)"))
}

fn parse_mac_address_or_exit<S: AsRef<str>>(mac_address_input: S) -> MacAddr6 {
    match macaddr::MacAddr6::from_str(mac_address_input.as_ref()) {
        Ok(mac_address) => mac_address,
        Err(_error) => {
            eprintln!("Invalid mac address");
            process::exit(1);
        }
    }
}
