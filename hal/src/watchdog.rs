//! Watchdog timer configuration.
use crate::{hal, target_device};

pub struct Watchdog(target_device::WDT);

impl Watchdog {
    pub fn new(wdt: target_device::WDT) -> Self {
        Self(wdt)
    }
}

impl hal::watchdog::WatchdogDisable for Watchdog {
    fn disable(&mut self) {
        self.0.wdt_mr.write(|w| w.wddis().set_bit());
    }
}
