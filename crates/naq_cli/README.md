# Nordic air quality CLI

Project name: `naq_cli`.

CLI to check sensor data for air quality devices. 
It currently supports Airthings Wave plus.

## Usage

```shell
$ naq_cli mac <mac_address>
```
Result
```text
> Checking for sensor data. Please wait a few seconds ...
> Found sensor data

T- Timestamp: 2021-09-12 13:33:57 (+0200)
|- MAC address: <mac_address>
|- Serial number: <serial_number>
`- Measurements
 |- CO2: 478 ppm
 |- TVOC: 138 ppb
 |- Radon: 15 Bq/m3
 |- Temperature: 24.81 degC
 |- Humidity: 53.5 %rH
 `- A. pressure: 986.58 mbar
```