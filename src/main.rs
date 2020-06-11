extern crate sysfs_gpio;
extern crate serial;

use std::thread::sleep;
use std::time::Duration;

use env_logger::Env;
use log::info;
use sysfs_gpio::{Direction, Pin};


use serial::prelude::*;
use std::io::Write;
use serial::SystemPort;


fn logger( message: &str) {
    let mut port: SystemPort = serial::open("/dev/ttyS0").unwrap();

    info!("{}", message);
    port.write(format!("{}\n", message).as_ref()).unwrap();
}


fn main() {
    env_logger::from_env(Env::default().default_filter_or("info")).init();

    logger("Starting service");

    let my_led = Pin::new(5); // number depends on chip, etc.
    let sleep_time = Duration::from_secs(1);
    my_led
        .with_exported(|| {
            my_led.set_direction(Direction::Out).unwrap();
            loop {
                logger("Blink Low");
                my_led.set_value(0).unwrap();
                sleep(sleep_time);
                logger("Blink High");
                my_led.set_value(1).unwrap();
                sleep(sleep_time);
            }
        })
        .unwrap();
}
