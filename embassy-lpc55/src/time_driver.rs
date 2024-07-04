use atomic_polyfill::{AtomicU64, AtomicU8, Ordering};
use defmt::{info, trace};
use embassy_time_driver::{AlarmHandle, Driver};
use lpc55_hal::typestates::ClocksSupport1MhzFroToken;
use core::cell::Cell;
use critical_section::CriticalSection;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use crate::hal;
use hal::prelude::*;
use hal::drivers::Timer;

struct AlarmState {
    timestamp: Cell<u64>,
    callback: Cell<Option<(fn(*mut ()), *mut ())>>,
}
unsafe impl Send for AlarmState {}

const ALARM_COUNT: usize = 5;
const DUMMY_ALARM: AlarmState = AlarmState {
    timestamp: Cell::new(0),
    callback: Cell::new(None),
};

// Dumbass driber
struct TimerDriver {
    now: AtomicU64,
    next_alarm: AtomicU8,
    alarms: Mutex<CriticalSectionRawMutex, [AlarmState; ALARM_COUNT]>,
    clocks_configured: bool
}

embassy_time_driver::time_driver_impl!(static DRIVER: TimerDriver = TimerDriver{
    now: AtomicU64::new(0),
    next_alarm: AtomicU8::new(0),
    alarms:  Mutex::const_new(CriticalSectionRawMutex::new(), [DUMMY_ALARM; ALARM_COUNT]),
    clocks_configured: false
});

impl Driver for TimerDriver {
    fn now(&self) -> u64 {
        self.now.fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {
            info!("now() -> {}", x);
            Some(x + 1)
        }).unwrap()
    }

    unsafe fn allocate_alarm(&self) -> Option<AlarmHandle> {
        let id = self.next_alarm.fetch_update(Ordering::AcqRel, Ordering::Acquire, |x| {
            if x < ALARM_COUNT as u8 {
                info!("allocate_alarm -> Some({})", x);
                Some(x + 1)
            } else {
                info!("allocate_alarm -> None");
                None
            }
        });

        match id {
            Ok(id) => Some(AlarmHandle::new(id)),
            Err(_) => None,
        }
    }

    fn set_alarm_callback(&self, alarm: AlarmHandle, callback: fn(*mut ()), ctx: *mut ()) {
        info!("set_alarm_callback({}, _, _)", alarm.id());
        
        let n = alarm.id() as usize;
        critical_section::with(|cs| {
            let alarm = &self.alarms.borrow(cs)[n];
            alarm.callback.set(Some((callback, ctx)));
        })
    }

    fn set_alarm(&self, alarm: AlarmHandle, timestamp: u64) -> bool {
        info!("set_alarm({}, {})", alarm.id(), timestamp);
        let n = alarm.id() as usize;
        critical_section::with(|cs| {
            let alarm = &self.alarms.borrow(cs)[n];
            alarm.timestamp.set(timestamp);

            // Arm it.
            // Note that we're not checking the high bits at all. This means the irq may fire early
            // if the alarm is more than 72 minutes (2^32 us) in the future. This is OK, since on irq fire
            // it is checked if the alarm time has passed.
            let mut p = unsafe { hal::Peripherals::steal() };
            let token : ClocksSupport1MhzFroToken = unsafe { core::mem::transmute(()) };

            match n {
                0 => {
                    let ctimer = p.ctimer.0.enabled(&mut p.syscon, token);
                    ctimer.mcr.write(|w| w
                        .mr0i().set_bit());
                    ctimer.mr[0].write(|w| unsafe { w.bits((timestamp * 1) as u32) });
                    ctimer.tcr.write(|w| w.cen().set_bit());
                    ctimer.ir.write(|w| w.mr0int().set_bit());
                }
                1 => {
                    let ctimer = p.ctimer.1.enabled(&mut p.syscon, token);
                    ctimer.mcr.write(|w| w
                        .mr0i().set_bit());
                    ctimer.mr[0].write(|w| unsafe { w.bits((timestamp * 1) as u32) });
                    ctimer.tcr.write(|w| w.cen().set_bit());
                    ctimer.ir.write(|w| w.mr0int().set_bit());
                }
                2 => {
                    let ctimer = p.ctimer.2.enabled(&mut p.syscon, token);
                    ctimer.mcr.write(|w| w
                        .mr0i().set_bit());
                    ctimer.mr[0].write(|w| unsafe { w.bits((timestamp * 1) as u32) });
                    ctimer.tcr.write(|w| w.cen().set_bit());
                    ctimer.ir.write(|w| w.mr0int().set_bit());
                }
                3 => {
                    let ctimer = p.ctimer.3.enabled(&mut p.syscon, token);
                    ctimer.mcr.write(|w| w
                        .mr0i().set_bit());
                    ctimer.mr[0].write(|w| unsafe { w.bits((timestamp * 1) as u32) });
                    ctimer.tcr.write(|w| w.cen().set_bit());
                    ctimer.ir.write(|w| w.mr0int().set_bit());
                }
                4 => {
                    let ctimer = p.ctimer.4.enabled(&mut p.syscon, token);
                    ctimer.mcr.write(|w| w
                        .mr0i().set_bit());
                    ctimer.mr[0].write(|w| unsafe { w.bits((timestamp * 1) as u32) });
                    ctimer.tcr.write(|w| w.cen().set_bit());
                    ctimer.ir.write(|w| w.mr0int().set_bit());
                }
                _ => {}
            };

            true
            // let now = self.now();
            // if timestamp <= now {
            //     // If alarm timestamp has passed the alarm will not fire.
            //     // Disarm the alarm and return `false` to indicate that.
            //     pac::TIMER.armed().write(|w| w.set_armed(1 << n));

            //     alarm.timestamp.set(u64::MAX);

            //     false
            // } else {
            //     true
            // }
        })
    }
}

// /// safety: must be called exactly once at bootup
// pub unsafe fn init() {
//     // init alarms
//     critical_section::with(|cs| {
//         let alarms = DRIVER.alarms.borrow(cs);
//         for a in alarms {
//             a.timestamp.set(u64::MAX);
//         }
//     });

//     // enable all irqs
//     pac::TIMER.inte().write(|w| {
//         w.set_alarm(0, true);
//         w.set_alarm(1, true);
//         w.set_alarm(2, true);
//         w.set_alarm(3, true);
//     });
//     interrupt::TIMER_IRQ_0.enable();
//     interrupt::TIMER_IRQ_1.enable();
//     interrupt::TIMER_IRQ_2.enable();
//     interrupt::TIMER_IRQ_3.enable();
// }