#![no_std]
#![no_main]

use panic_halt as _;
use embedded_hal::spi::FullDuplex;
use arduino_hal::prelude::*;
use arduino_hal::spi;
use nb;
use ufmt;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    
    let (mut spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default()
    );

    loop {
        nb::block!(spi.send(0b00001111)).void_unwrap();
        let data = nb::block!(spi.read()).void_unwrap();

        ufmt::uwriteln!(&mut serial, "data: {}\r", data).void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}
