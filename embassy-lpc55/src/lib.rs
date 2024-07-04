#![no_std]

pub mod time_driver;

use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
pub use lpc55_hal as hal;
