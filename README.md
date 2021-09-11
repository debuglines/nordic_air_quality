# Nordic air quality

Workspace for reading and managing sensor data from air quality devices. 

Supported devices:

- Airthings Wave plus

## Projects

The workspace contains several projects, including a device connector lib, domain lib, and command line interfaces. 

### Device connector

<dl>
    <dt>Project name</dt>
    <dd>naq_device_connector</dd>
    <dt>Type</dt>
    <dd>Library</dd>
</dl>

[Project README.md](./crates/naq_device_connector/README.md).
This library uses bluetooth to read sensor data. 
It can connect to the sensor devices with either the serial number or the mac address.  
