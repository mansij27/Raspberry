extern crate rppal;
use rppal::gpio::{Gpio, Mode};
use rppal::system::DeviceInfo;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("Running on: {:?}", DeviceInfo::new().unwrap());

    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(17).unwrap().into_output();

    loop {
        // Send start signal to the DHT22 sensor
        pin.set_mode(Mode::Output);
        pin.write(rppal::gpio::Level::Low);
        sleep(Duration::from_millis(18)); // Hold low for at least 18ms
        pin.write(rppal::gpio::Level::High);
        pin.set_mode(Mode::Input);

        // Wait for the DHT22 sensor to respond
        while pin.read() == rppal::gpio::Level::High {}
        while pin.read() == rppal::gpio::Level::Low {}

        // Read data from the DHT22 sensor
        let mut data = [0u8; 5];
        for i in 0..5 {
            data[i] = read_byte(&mut pin);
        }

        let humidity = ((data[0] as u16) << 8 | (data[1] as u16)) as f32 / 10.0;
        let temperature = ((data[2] as u16 & 0x7F) << 8 | (data[3] as u16)) as f32 / 10.0;
        let temperature_sign = if data[2] & 0x80 != 0 { "-" } else { "" };

        println!(
            "Temperature: {}{:.1}°C, Humidity: {:.1}%",
            temperature_sign, temperature, humidity
        );

        sleep(Duration::from_secs(2)); // Delay between readings
    }
}

fn read_byte(pin: &mut rppal::gpio::OutputPin) -> u8 {
    let mut byte = 0;
    for _ in 0..8 {
        while pin.read() == rppal::gpio::Level::Low {}
        sleep(Duration::from_micros(30)); // Bit start time
        if pin.read() == rppal::gpio::Level::High {
            byte |= 1;
        }
        byte <<= 1;
        while pin.read() == rppal::gpio::Level::High {}
    }
    byte
}
