#![no_std]

pub mod gpio;

pub use embassy_hal_internal::{into_ref, Peripheral, PeripheralRef};
pub use lpc55_pac as pac;

static mut pac_instance: Option<lpc55_pac::Peripherals> = None;

/// Initialize the `embassy-lpc55` HAL with the provided configuration.
///
/// This returns the peripheral singletons that can be used for creating drivers.
///
/// This should only be called once at startup, otherwise it panics.
pub fn init(_config: config::Config) -> Peripherals {
    // Do this first, so that it panics if user is calling `init` a second time
    // before doing anything important.
    let peripherals = Peripherals::take();
    unsafe { pac_instance = Some(lpc55_pac::Peripherals::take().unwrap()) };
    
    unsafe {
        gpio::init();
    }
    
    peripherals
}

embassy_hal_internal::peripherals! {
    LED_RED,
    LED_GREEN,
}

/// HAL configuration for RP.
pub mod config {
    #[derive(Default)]
    pub struct Config {}
}