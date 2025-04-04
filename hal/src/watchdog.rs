//! Watchdog timer configuration.
use crate::{generics, pac::WDT};

use core::convert::From;
use core::marker::PhantomData;

/// The state of the [`Watchdog`].
pub trait WatchdogState: generics::Sealed {}

/// The [`Watchdog`] is disabled.
pub enum Disabled {}
/// The [`Watchdog`] is in reset and has yet to be configured. It will
/// trigger a system reset ~15 seconds from system start.
pub enum Reset {}

impl generics::Sealed for Disabled {}
impl WatchdogState for Disabled {}
impl generics::Sealed for Reset {}
impl WatchdogState for Reset {}

/// [`WDT`] abstraction.
pub struct Watchdog<S: WatchdogState> {
    watchdog: WDT,
    state: PhantomData<S>,
}

impl Watchdog<Reset> {
    /// Creates a [`Watchdog`] in [`Reset`] from the device's [`WDT`].
    pub fn new(watchdog: WDT) -> Self {
        Self {
            watchdog,
            state: PhantomData,
        }
    }

    /// Disables the watchdog completely.
    pub fn disable(self) -> Watchdog<Disabled> {
        self.watchdog.mr.write(|w| w.wddis().set_bit());
        Watchdog {
            watchdog: self.watchdog,
            state: PhantomData,
        }
    }
}

impl From<WDT> for Watchdog<Disabled> {
    fn from(wd: WDT) -> Self {
        Watchdog::new(wd).disable()
    }
}

impl crate::ehal::watchdog::Watchdog for Watchdog<Disabled> {
    /// Stub implementation. Does nothing and immediately returns.
    fn feed(&mut self) {}
}
