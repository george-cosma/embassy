#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_lpc55::hal as hal;
use hal::prelude::*;
use nb::block;
use panic_halt as _;
use defmt_rtt as _;
use hal::drivers::Timer;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = hal::new();
    
    let mut anactrl = p.anactrl;
    let mut pmc = p.pmc;
    let mut syscon = p.syscon;
    
    let clocks = hal::ClockRequirements::default()
        .system_frequency(12.MHz())
        .configure(&mut anactrl, &mut pmc, &mut syscon)
        .unwrap();

    let ctimer = p
        .ctimer
        .1
        .enabled(&mut syscon, clocks.support_1mhz_fro_token().unwrap());
    let mut cdriver = Timer::new(ctimer);

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

        cdriver.start(1_000_000.microseconds());
        block!(cdriver.wait()).unwrap();

        info!("led on!");
        led.set_low().unwrap();
        
        cdriver.start(1_000_000.microseconds());
        block!(cdriver.wait()).unwrap();
    }
}
