[12:03] Tarun Singh Rawat (Guest)

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

        // Send start signal to the DHT11 sensor

        pin.set_mode(Mode::Output);

        pin.write(rppal::gpio::Level::Low);

        sleep(Duration::from_millis(18)); // Hold low for at least 18ms

        pin.write(rppal::gpio::Level::High);

        sleep(Duration::from_micros(40)); // Hold high for 20-40us

        pin.set_mode(Mode::Input);

 

        // Wait for the DHT11 sensor to respond

        while pin.read() == rppal::gpio::Level::High {}

        while pin.read() == rppal::gpio::Level::Low {}

 

        // Read data from the DHT11 sensor

        let mut data = [0u8; 5];

        for i in 0..5 {

            data[i] = read_byte(&mut pin);

        }

 

        // Interpret the data

        let humidity = data[0] as f32;

        let temperature = data[2] as f32;

 

        println!("Temperature: {:.1}°C, Humidity: {:.1}%", temperature, humidity);

 

        sleep(Duration::from_secs(2)); // Delay between readings

    }

}

 

fn read_byte(pin: &mut rppal::gpio::OutputPin) -> u8 {

    let mut byte = 0;

    for _ in 0..8 {

        while pin.read() == rppal::gpio::Level::Low {}

        sleep(Duration::from_micros(50)); // Bit start time for DHT11

        if pin.read() == rppal::gpio::Level::High {

            byte |= 1;

        }

        byte <<= 1;

        while pin.read() == rppal::gpio::Level::High {}

    }

    byte

}
