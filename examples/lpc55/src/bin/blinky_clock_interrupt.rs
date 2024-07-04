#![no_std]
#![no_main]

use core::arch::asm;

use cortex_m::asm;
use defmt::*;
use embassy_executor::Spawner;
use embassy_lpc55::hal::{self as hal, raw::syscon::ctimerclksel1, traits::wg::digital::v2::ToggleableOutputPin};
use hal::prelude::*;
use nb::block;
use panic_probe as _;
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

    info!("TC (Timer register) initial: {:?}", ctimer.tc.read().bits());
    info!("TCR (Timer control register) initial: {:?}", ctimer.tcr.read().bits());
    info!("Mcr (Match control register) initial: {:?}", ctimer.mcr.read().bits());
    info!("MR[0] (Match register) initial: {:?}", ctimer.mr[0].read().bits());
    info!("IR (Interrupt register) initial: {:?}", ctimer.ir.read().bits());
    
    ctimer.mcr.write(|w| w
        .mr0i().set_bit()
        .mr0r().set_bit());
    ctimer.mr[0].write(|w| unsafe { w.bits(5_000_000) } );
    ctimer.tcr.write(|w| w.cen().set_bit() );
    ctimer.ir.write(|w| w.mr0int().set_bit());

    info!("TCR (Timer control register) initial: {:?}", ctimer.tcr.read().bits());
    info!("TC after: {:?}", ctimer.tc.read().bits());
    info!("Mcr after: {:?}", ctimer.mcr.read().bits());
    info!("MR[0] after: {:?}", ctimer.mr[0].read().bits());
    info!("IR (Interrupt register) after: {:?}", ctimer.ir.read().bits());


    let mut gpio = p.gpio.enabled(&mut syscon);
    let mut iocon = p.iocon.enabled(&mut syscon);
    
    let pins = hal::Pins::take().unwrap();

    let mut led = pins
        .pio1_6
        .into_gpio_pin(&mut iocon, &mut gpio)
        .into_output_high();

    loop {
        // for _ in 0..100_000 {
        //     asm::nop();
        // }
        // info!("TC loop: {:?}", ctimer.tc.read().bits());
        // info!("IR loop: {:?}", ctimer.ir.read().bits());
        if ctimer.ir.read().mr0int().bit_is_set() {
            info!("Tick!");
            led.toggle().unwrap();
            ctimer.ir.write(|w| w.mr0int().set_bit());
        }
    }
}
