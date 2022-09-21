#![no_std]
#![no_main]

use panic_halt as _;
use arduino_hal::prelude::*;
use arduino_hal::{spi, Delay};
use hx711_spi::Hx711;
use nb;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    let (spi, _) = arduino_hal::Spi::new(
        dp.SPI,
        pins.d13.into_output(),
        pins.d11.into_output(),
        pins.d12.into_pull_up_input(),
        pins.d10.into_output(),
        spi::Settings::default()
    );

    let mut hx711 = Hx711::new(spi,Delay::new());

    loop {
        let data = nb::block!(hx711.read()).void_unwrap();
        ufmt::uwriteln!(&mut serial, "data: {}\r", data).void_unwrap();
        arduino_hal::delay_ms(1000);
    }
}
