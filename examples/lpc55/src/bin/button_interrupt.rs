#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_lpc55::hal::peripherals::pint::{Mode, Slot};
use embassy_lpc55::hal::{self as hal, traits::wg::digital::v2::OutputPin};
use panic_halt as _;
use defmt_rtt as _;


use hal::drivers::pins::Level;
use hal::drivers::Pins;
pub use hal::typestates::pin::state;


#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut hal = hal::new();

    let mut gpio = hal.gpio.enabled(&mut hal.syscon);

    let mut iocon = hal.iocon.enabled(&mut hal.syscon);
    let pins = Pins::take().unwrap();

    let user_button = pins.pio1_9.into_gpio_pin(&mut iocon, &mut gpio).into_input();

    let mut mux = hal.inputmux.enabled(&mut hal.syscon);
    let mut pint = hal.pint.enabled(&mut hal.syscon);

    pint.enable_interrupt(&mut mux, &user_button, Slot::Slot0, Mode::RisingEdge);
    pint.enable_interrupt(&mut mux, &user_button, Slot::Slot0, Mode::FallingEdge);

    let mut green = pins
        .pio1_7
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);
    let mut red = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output(Level::High);

    // Dont need mux anymore
    mux.disabled(&mut hal.syscon);

    // Clear interrupts initially
    pint.rise.write(|w| unsafe { w.bits(1) });
    pint.fall.write(|w| unsafe { w.bits(1) });

    loop {
        if (pint.fall.read().bits() & 1) != 0 {
            pint.fall.write(|w| unsafe { w.bits(1) });
            info!("Falling edge detected. This means the button was pressed down!");

            red.set_high().unwrap(); // set red off
            green.set_low().unwrap(); // set green on
        }

        if (pint.rise.read().bits() & 1) != 0 {
            pint.rise.write(|w| unsafe { w.bits(1) });
            info!("Rising edge detected. This means the button was released");

            red.set_low().unwrap(); // set red on
            green.set_high().unwrap(); // set green off
        }

    }
}
