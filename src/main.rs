use env_logger::Env;
use log::debug;
use std::sync::{Mutex, Arc};
use warp::{Filter, reply};
use rppal::i2c;
use std::env;
use serde::Deserialize;
use bme280::Measurement;

mod bme280;

#[derive(Deserialize, Debug)]
pub struct Settings {
    sensor_address: u16,
    server_path: String,
    server_port: u16,
    stale_measurement_limit_secs: u64,
}

#[tokio::main]
async fn main() {
// Initialize logger
    let env = Env::default().default_filter_or("warn");
    env_logger::init_from_env(env);

    // Get configuration file location if present as command line argument
    let args: Vec<String> = env::args().collect();
    let config_path: &str = if args.len() > 1 {
        &args[1]
    } else {
        debug!("No configuration file specified, falling back to default.");
        "config.toml"
    };

    // Parse configuration file
    let mut config = config::Config::default();
    config
        .merge(config::File::with_name(config_path))
        .expect("Error parsing configuration file.");
    let settings: Settings = config
        .try_into()
        .expect("Configuration file contains errors.");

    //Initialize sensor, fails are fatal
    //Mutexes to guarantee no clashes of I²c bus
    let sensor_lock = Arc::new(Mutex::new(bme280::Bme280::new(settings.sensor_address).unwrap()));
    let measurement_lock = Arc::new(Mutex::new(sensor_lock.lock().unwrap().make_measurement().unwrap()));

    //Get the limit out of the closure
    let limit:u64 = settings.stale_measurement_limit_secs;

    // GET /weather => 200 OK with body of either new reading or latest reading if sensor in use
    let weather = warp::path(settings.server_path).map(move || {
        let lock_result = sensor_lock.try_lock();
        if let Ok(sensor_guard) = lock_result {
            //Obtained the sensor
            let result: Result<bme280::Measurement, i2c::Error> = sensor_guard.make_measurement();
            if let Ok(measurement) = result {
                //Update cached measurement
                *measurement_lock.lock().unwrap() = measurement;
                return reply::json(&measurement);
            };
        }
        //Check to make sure we do not have too old cached measurement
        let cached_measurement:&Measurement = &*measurement_lock.lock().unwrap();
        if cached_measurement.get_duration_since_creation().as_secs() < limit {
            //Cached value too old, wait for the measurement to complete
            let _ = measurement_lock.lock().unwrap();
            return reply::json(&*measurement_lock.lock().unwrap());
        }
        //Failed to obtain sensor lock or to make a measurement return cached measurement
        return reply::json(cached_measurement);
    });

    warp::serve(weather)
        //Serve the JSON on port 3030
        .run(([0, 0, 0, 0], settings.server_port))
        .await;
}
