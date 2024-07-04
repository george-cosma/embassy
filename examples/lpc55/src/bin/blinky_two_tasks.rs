#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_lpc55::hal::drivers::pins::direction::Output;
use embassy_lpc55::hal::drivers::pins::Pio1_6;
use embassy_lpc55::hal::traits::wg::digital::v2::ToggleableOutputPin;
use embassy_lpc55::hal::typestates::pin::state::Gpio;
use embassy_lpc55::hal::Pin;
use embassy_lpc55::hal::{self as hal, traits::wg::digital::v2::OutputPin};
use panic_probe as _;
use defmt_rtt as _;

use cortex_m::asm::nop;

use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex::Mutex;
type LedType = Mutex<ThreadModeRawMutex, Option<Pin<Pio1_6, Gpio<Output>>>>;
static LED: LedType = Mutex::new(None);

use embassy_time::{Duration, Ticker};

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = hal::new();
    
    let mut syscon = p.syscon;
    let mut gpio = p.gpio.enabled(&mut syscon);
    let mut iocon = p.iocon.enabled(&mut syscon);

    let pins = hal::Pins::take().unwrap();

    let led = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output_high();

    {
        *(LED.lock().await) = Some(led);
    }

    let dt = 100 * 1_000_000;
    let k = 1.003;

    info!("Blinking with period {} ms", dt / 1_000_000);

    spawner.spawn(toggle_led(&LED, Duration::from_nanos(dt))).unwrap();
    // spawner.spawn(toggle_led(&LED, Duration::from_nanos((dt as f64 * k) as u64))).unwrap();
}

#[embassy_executor::task(pool_size = 2)]
async fn toggle_led(led: &'static LedType, delay: Duration) {
    let mut ticker = Ticker::every(delay);
    loop {
        info!("Toggling led");
        {
            let mut led_unlocked = led.lock().await;
            if let Some(pin_ref) = led_unlocked.as_mut() {
                pin_ref.toggle().unwrap();
            }
        }
        ticker.next().await;
    }
}


