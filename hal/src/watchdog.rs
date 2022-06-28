//! Watchdog timer configuration.
use crate::target_device::WDT;

use core::marker::PhantomData;

pub trait WatchdogState {}

pub enum Disabled {}
pub enum Reset {}

impl WatchdogState for Disabled {}
impl WatchdogState for Reset {}

pub struct Watchdog<S: WatchdogState> {
    watchdog: WDT,
    state: PhantomData<S>,
}

impl Watchdog<Reset> {
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
