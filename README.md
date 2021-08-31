# rpi-bme280-server
Web server for Raspberry Pi computers written in Rust serving JSON data from a Bosch BME280 sensor.

## Requirements
[RPPAL](https://docs.golemparts.com/rppal/) compatible operating system and hardware

## Configuration
Configuration is loaded from a [TOML](https://toml.io/) file that contains parameters
required for the operation. [Sample of the configuration](samples/config.toml)

Address of the sensor, path the server should respond to, port server should 
reserve and limit of measurement considered stale can be customized.

## Ouput
When requested server tries to connect to the sensor over I²C, if the sensor is 
currently in use the most recent measurement that was completed will be returned.

Sample output:

```
{"unix_timestamp":1630406866,
"temperature_c":17.995813178854586,
"pressure_pa":99947.35153765255,
"humidity_relative":67.69223043636826}
```

