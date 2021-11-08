use std::thread;
use std::time::Duration;
use rppal::gpio::{OutputPin};

pub const ENGINE_PIN: u8 = 23;

pub fn flip_flop_pin(pin: &mut OutputPin, secs: u64) {
  pin.set_high();
  thread::sleep(Duration::from_secs(secs));
  pin.set_low();
}