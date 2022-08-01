//! Watchdog timer configuration.
use crate::pac::WDT;

use core::marker::PhantomData;

/// The state of the [`Watchdog`].
pub trait WatchdogState {}

/// The [`Watchdog`] is disabled.
pub enum Disabled {}
/// The [`Watchdog`] is in reset and has yet to be configured. It will
/// trigger a system reset ~15 seconds from system start.
pub enum Reset {}

impl WatchdogState for Disabled {}
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
        self.watchdog.wdt_mr.write(|w| w.wddis().set_bit());
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
