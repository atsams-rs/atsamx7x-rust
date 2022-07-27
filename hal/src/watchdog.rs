//! Watchdog timer configuration.
use crate::{ehal, target_device};

pub struct Watchdog(target_device::WDT);

impl Watchdog {
    pub fn new(wdt: target_device::WDT) -> Self {
        Self(wdt)
    }
}

impl ehal::watchdog::WatchdogDisable for Watchdog {
    fn disable(&mut self) {
        self.0.mr.write(|w| w.wddis().set_bit());
    }
}
