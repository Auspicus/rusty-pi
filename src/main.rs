use rppal::gpio::{Gpio, OutputPin};
use std::sync::Arc;
use std::{net::TcpListener, sync::Mutex};
use std::thread;
use tungstenite::accept;

use rusty_pi::controller::{flip_flop_pin, ENGINE_PIN};

const BIND_ADDRESS: &'static str = "0.0.0.0:9001";

fn main() {
    let server = TcpListener::bind(BIND_ADDRESS).unwrap();
    println!("Started websocket server @ {}", BIND_ADDRESS);

    let gpio = Gpio::new().expect("Expected an accessible GPIO device but found none.");
    let engine_pin = gpio
        .get(ENGINE_PIN)
        .expect("Expected pin to be accessible but it was not.")
        .into_output();
    println!("Connected to on-board GPIO");

    let pin_arc: Arc<Mutex<OutputPin>> = Arc::new(Mutex::new(engine_pin));

    // This is really stupid.
    // Should be handling connections using threads.
    for stream in server.incoming() {
        let pin_arc_copy: Arc<Mutex<OutputPin>> = Arc::clone(&pin_arc);

        thread::spawn(move || {
            let mut websocket = accept(stream.unwrap()).unwrap();
            
            loop {
                let msg = match websocket.read_message() {
                    Ok(msg) => msg,
                    Err(_) => break,
                };

                println!("Received: {:?}", msg);

                // We do not want to send back ping/pong messages.
                if msg.is_binary() || msg.is_text() {
                    let mut pin = pin_arc_copy.lock().expect("Mutex poisoned by another thread.");
                    flip_flop_pin(&mut pin, 1);
                    println!("Responded: {}", msg);
                    websocket.write_message(msg).unwrap();
                }
            }
        });
    }
}
