#![no_std]
#![no_main]

use embassy_executor::Spawner;
// use embassy_lpc::gpio;
// use gpio::{Level, Output};
use panic_halt as _;

use cortex_m::asm::nop;

use lpc55_pac as pac;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    // let p = embassy_lpc::init(Default::default());
    // let mut led = Output::new(p.PIN_25, Level::Low);

    let x = pac::Peripherals::take().unwrap();
    x.GPIO.dir[1].write(|w| unsafe { w.bits(1 << 6) });

    loop {
        // led.toggle();
        x.GPIO.not[1].write(|w| unsafe { w.bits(1 << 6) });

        for _ in 0 .. 200_000 {
            nop();
        }
    }
}
