extern crate rppal;use rppal::gpio::{Gpio, Level};use std::thread::sleep;use std::time::Duration;
fn main() {
    // Initialize GPIO    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(17).unwrap().into_output(); // Use GPIO pin 17
    loop {
        // Turn the LED on        pin.set_high();
        sleep(Duration::from_secs(1)); // Wait for 1 second
        // Turn the LED off        pin.set_low();
        sleep(Duration::from_secs(1)); // Wait for 1 second
    }
}
