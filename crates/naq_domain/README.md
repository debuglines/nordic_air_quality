# Nordic air quality: domain library

Domain library that defines shared domain data structures. 

## Usage

```rust
use naq_domain::SensorData;

fn main() {
    let sensor_data = SensorData {
        co2: 533.0,
        voc: 72.0,
        relative_atmospheric_pressure: 1005.92,
        relative_humidity_percent: 36.5,
        temperature_celsius: 20.42,
        radon_short_term: Some(1),
        radon_long_term: Some(1),
    };
    
    println!("{:#?}", sensor_data);
}
```
Result
```text
SensorData {
    relative_humidity_percent: 36.5,
    radon_short_term: Some(
        1,
    ),
    radon_long_term: Some(
        1,
    ),
    temperature_celsius: 20.42,
    relative_atmospheric_pressure: 1005.92,
    co2: 533.0,
    voc: 72.0,
}
```