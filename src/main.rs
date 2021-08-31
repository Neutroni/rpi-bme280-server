use std::sync::{Mutex, Arc};
use warp::{Filter, reply};
use rppal::i2c;

mod bme280;

#[tokio::main]
async fn main() {
    //Initialize sensor, fails are fatal
    //Mutexes to guarantee no clashes of I2c bus
    let sensor_lock = Arc::new(Mutex::new(bme280::Bme280::new().unwrap()));
    let measurement_lock = Arc::new(Mutex::new(sensor_lock.lock().unwrap().make_measurement().unwrap()));

    // GET /weather => 200 OK with body of either new reading or latest reading if sensor in use
    let weather = warp::path("weather").map(move || {
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
        //Failed to obtain sensor lock or to make a measurement return cached measurement
        return reply::json(&*measurement_lock.lock().unwrap());
    });

    warp::serve(weather)
        //Serve the JSON on port 3030
        .run(([0, 0, 0, 0], 3030))
        .await;
}
