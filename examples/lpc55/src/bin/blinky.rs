#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_lpc55::gpio;
use gpio::{Level, Output};
use panic_halt as _;

use cortex_m::asm::nop;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_lpc55::init(Default::default());
    let led = Output::new(p.LED_GREEN, Level::Low);

    led.toggle();
    // let x = pac::Peripherals::take().unwrap();
    // x.GPIO.dir[1].write(|w| unsafe { w.bits(1 << 6) });

    loop {
        led.set_high();
        // x.GPIO.not[1].write(|w| unsafe { w.bits(1 << 6) });

        for _ in 0 .. 50_000 {
            nop();
        }

        led.set_low();

        
        for _ in 0 .. 50_000 {
            nop();
        }
    }
}
