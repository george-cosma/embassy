#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_lpc55::hal::{self as hal, traits::wg::digital::v2::OutputPin};
use panic_halt as _;
use defmt_rtt as _;

use cortex_m::asm::nop;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::new();
    
    let mut syscon = p.syscon;
    let mut gpio = p.gpio.enabled(&mut syscon);
    let mut iocon = p.iocon.enabled(&mut syscon);

    let pins = hal::Pins::take().unwrap();

    let mut led = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output_high();

    loop {
        info!("led off!");
        led.set_high().unwrap();

        for _ in 0 .. 200_000 {
            nop();
        }

        info!("led on!");
        led.set_low().unwrap();
        
        for _ in 0 .. 200_000 {
            nop();
        }
    }
}
