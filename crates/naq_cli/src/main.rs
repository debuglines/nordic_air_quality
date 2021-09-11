use macaddr::{MacAddr, MacAddr6};
use naq_device_connector::check_sensor_data_by_mac_address;
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
    println!("Checking for sensor data ...");
    let sensor_data = check_sensor_data_by_mac_address(&mac_address, Duration::from_secs(6))
        .await
        .unwrap();

    println!("Sensor data: {:#?}", sensor_data);
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
