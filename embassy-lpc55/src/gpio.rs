use embassy_hal_internal::{impl_peripheral, into_ref};

use crate::{pac, pac_instance, peripherals, Peripheral, PeripheralRef};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Level {
    /// Logical low.
    Low,
    /// Logical high.
    High,
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Bank {
    /// Bank 0.
    Bank0 = 0,
    /// Bank 1.
    Bank1 = 1,
}

impl Bank {
    pub fn to_index(&self) -> usize {
        match &self {
            Bank::Bank0 => 0,
            Bank::Bank1 => 1,
        }
    }
}

pub struct Output<'d> {
    pin: Flex<'d>,
}

impl<'d> Output<'d> {
    /// Create GPIO output driver for a [Pin] with the provided [Level].
    #[inline]
    pub fn new(pin: impl Peripheral<P = impl Pin> + 'd, initial_output: Level) -> Self {
        let pin = Flex::new(pin);
        pin.set_as_output();
        let result = Self { pin };

        match initial_output {
            Level::High => result.set_high(),
            Level::Low => result.set_low(),
        };

        result
    }    
    
    pub fn set_high(&self) {
        pac_instance.unwrap().GPIO.set[self.pin.pin_bank().to_index()].write(
            |w| unsafe {
                w.bits(self.pin.bit())
            }
        )
    }
    
    pub fn set_low(&self) {
        let x = pac::Peripherals::take().unwrap();
        x.GPIO.clr[self.pin.pin_bank().to_index()].write(
            |w| unsafe {
                w.bits(self.pin.bit())
            }
        )
    }

    pub fn toggle(&self) {
        let x = pac::Peripherals::take().unwrap();
        x.GPIO.not[self.pin.pin_bank().to_index()].write(
            |w| unsafe {
                w.bits(self.pin.bit())
            }
        )
    }
}

pub struct Flex<'d> {
    pin: PeripheralRef<'d, AnyPin>,
}

impl<'d> Flex<'d> {
    /// Wrap the pin in a `Flex`.
    ///
    /// The pin remains disconnected. The initial output level is unspecified, but can be changed
    /// before the pin is put into output mode.
    #[inline]
    pub fn new(pin: impl Peripheral<P = impl Pin> + 'd) -> Self {
        into_ref!(pin);

        Self { pin: pin.map_into() }
    }

    pub fn pin_bank(&self) -> Bank {
        self.pin.pin_bank()
    }

    pub fn pin_number(&self) -> u8 {
        self.pin.pin_number()
    }

    pub fn bit(&self) -> u32 {
        1 << self.pin.pin_number()
    }
    
    pub fn set_as_output(&self) {
        let x = pac::Peripherals::take().unwrap();
        x.GPIO.dir[self.pin.pin_bank().to_index()].write(
            |w| unsafe {
                w.bits(self.bit())
            }
        )
    }
}

pub(crate) trait SealedPin: Sized {
    fn pin_bank(&self) -> Bank;
    fn pin_number(&self) -> u8;
}


/// Interface for a Pin that can be configured by an [Input] or [Output] driver, or converted to an [AnyPin].
#[allow(private_bounds)]
pub trait Pin: Peripheral<P = Self> + Into<AnyPin> + SealedPin + Sized + 'static {
    /// Degrade to a generic pin struct
    fn degrade(self) -> AnyPin {
        AnyPin {
            pin_bank: self.pin_bank(),
            pin_number: self.pin_number(),
        }
    }

    /// Returns the pin number within a bank
    #[inline]
    fn pin(&self) -> u8 {
        self.pin_number()
    }

    /// Returns the bank of this pin
    #[inline]
    fn bank(&self) -> Bank {
        self.pin_bank()
    }
}

/// Type-erased GPIO pin
pub struct AnyPin {
    pin_bank: Bank,
    pin_number: u8,
}

impl AnyPin {
    /// Unsafely create a new type-erased pin.
    ///
    /// # Safety
    ///
    /// You must ensure that you’re only using one instance of this type at a time.
    pub unsafe fn steal(pin_bank: Bank, pin_number: u8) -> Self {
        Self { pin_bank, pin_number }
    }
}

impl_peripheral!(AnyPin);

impl Pin for AnyPin {}
impl SealedPin for AnyPin {
    #[inline]
    fn pin_bank(&self) -> Bank {
        self.pin_bank
    }

    #[inline]
    fn pin_number(&self) -> u8 {
        self.pin_number
    }
}

macro_rules! impl_pin {
    ($name:ident, $bank:expr, $pin_num:expr) => {
        impl Pin for peripherals::$name {}
        impl SealedPin for peripherals::$name {
            #[inline]
            fn pin_bank(&self) -> Bank {
                $bank
            }

            #[inline]
            fn pin_number(&self) -> u8 {
                $pin_num
            }
        }

        impl From<peripherals::$name> for crate::gpio::AnyPin {
            fn from(val: peripherals::$name) -> Self {
                crate::gpio::Pin::degrade(val)
            }
        }
    };
}

impl_pin!(LED_RED, Bank::Bank1, 6);
impl_pin!(LED_GREEN, Bank::Bank1, 7);

pub(crate) unsafe fn init() {}