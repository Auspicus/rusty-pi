use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

fn start() -> Option<()> {
    let gpio = Gpio::new().ok()?;
    let mut pin = gpio.get(23).ok()?.into_output();

    pin.set_high();
    thread::sleep(Duration::from_secs(1));
    pin.set_low();
    
    return None;
}

fn main() {
    start();
}
