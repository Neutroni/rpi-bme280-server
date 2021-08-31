# rpi-bme280-server
Web server for Raspberry Pi computers written in Rust serving JSON data from a Bosch BME280 sensor.

## Requirements
[RPPAL](https://docs.golemparts.com/rppal/) compatible operating system and hardware

## Configuration
Configuration is loaded from a [TOML](https://toml.io/) file that contains parameters
required for the operation. [Sample of the configuration](samples/config.toml)

Address of the sensor, path the server should respond to, port server should 
reserve and limit of measurement considered stale can be customized.

## Startup using systemd
 A sample of systemd service file is provided at  [sample.service](samples/sample.service)
 That must be modified to match compiled programs location
  - `ExecStart` Defines the program that is to be run. Sample assumes that binary 
    is located in `/home/Pi/Programs/rpi-bme280-server/` directory and named 
    `rpi-bme280-server` and that `config.toml` file is present in the working directory.
  - `WorkingDirectory` Sets the working directory for the program, if `ExecStart`
    line does not contain the configuration file location `config.toml` file must
    exist in the working directory.

The service file can be copied to target systems `/etc/systemd/system/` folder with 
appropriate name. After file has been  copied the server can be started by using `$sudo systemctl start sample.service` command.

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

