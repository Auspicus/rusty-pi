use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use rppal::gpio::{Gpio};

use rusty_pi::controller::{ENGINE_PIN, flip_flop_pin};

const BIND_ADDRESS: &'static str =  "0.0.0.0:9001";

fn main () {
    let server = TcpListener::bind(BIND_ADDRESS).unwrap();
    println!("Started websocket server @ {}", BIND_ADDRESS);

    let gpio = Gpio::new()
        .expect("Expected an accessible GPIO device but found none.");
    let mut engine_pin = gpio.get(ENGINE_PIN)
        .expect("Expected pin to be accessible but it was not.")
        .into_output();
    println!("Connected to on-board GPIO");

    // This is really stupid.
    // Should be handling connections using threads.
    'connectionLoop: for stream in server.incoming() {
        let mut websocket = accept(stream.unwrap()).unwrap();

        loop {
            let msg = match websocket.read_message() {
                Ok(msg) => msg,
                Err(_) => continue 'connectionLoop,
            };

            println!("Received: {:?}", msg);

            // We do not want to send back ping/pong messages.
            if msg.is_binary() || msg.is_text() {
                flip_flop_pin(&mut engine_pin, 1);
                println!("Responded: {}", msg);
                websocket.write_message(msg).unwrap();
            }
        }
    }
}