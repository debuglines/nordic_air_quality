# Nordic air quality device connector lib

Project name: `naq_device_connector`.

This library uses bluetooth to read sensor data.
It can connect to the sensor devices with either the serial number or the mac address.

## Usage

```rust
use macaddr::MacAddr6;
use naq_device_connector::check_sensor_data_by_mac_address;
use naq_domain::SensorData;

async fn check_by_mac_address(mac_address: &MacAddr6) -> SensorMetadata {
    println!("> Checking for sensor data. Please wait a few seconds ...");
    check_sensor_data_by_mac_address(
        &mac_address, 
        Duration::from_secs(6)
    )
        .await
        .unwrap()
}
```