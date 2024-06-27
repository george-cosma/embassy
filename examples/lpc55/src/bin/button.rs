#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_lpc55::hal::traits::wg::digital::v2::InputPin;
use embassy_lpc55::hal::{self as hal, traits::wg::digital::v2::OutputPin};
use nb::block;
use panic_halt as _;
use defmt_rtt as _;


use hal::drivers::pins::Level;
use hal::drivers::{
    Pins, Timer,
};
use hal::prelude::*;
pub use hal::typestates::pin::state;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut hal = hal::new();

    let clocks = hal::ClockRequirements::default()
        .system_frequency(96.MHz())
        .configure(&mut hal.anactrl, &mut hal.pmc, &mut hal.syscon)
        .unwrap();
    let fro_token = clocks.support_1mhz_fro_token().unwrap();

    let mut gpio = hal.gpio.enabled(&mut hal.syscon);

    let mut iocon = hal.iocon.enabled(&mut hal.syscon);
    let pins = Pins::take().unwrap();

    let user_button = pins.pio1_9.into_gpio_pin(&mut iocon, &mut gpio).into_input();

    let mut green = pins
        .pio1_7
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);
    let mut red = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);


    let mut delay_timer = Timer::new(hal.ctimer.0.enabled(&mut hal.syscon, fro_token));

    loop {
        delay_timer.start(300_000.microseconds());
        block!(delay_timer.wait()).unwrap();
        
        let button_state = if user_button.is_high().unwrap() {
            "Not Pressed"
        } else {
            "Pressed"
        };

        info!("Value of input: {}", button_state);

        if user_button.is_high().unwrap() { // Not pressed
            red.set_low().unwrap(); // set red on
            green.set_high().unwrap(); // set green off
        } else {
            red.set_high().unwrap(); // set red off
            green.set_low().unwrap(); // set green on
        }
    }
}
