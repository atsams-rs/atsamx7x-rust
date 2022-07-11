//! Serial communication peripherals

use crate::fugit::{HertzU32 as Hertz, RateExtU32};

/// Bits per second
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Debug)]
pub struct Bps(pub(crate) Hertz);

/// [`u32`] type extension that adds convenience methods
pub trait ExtU32 {
    fn bps(self) -> Bps;
}

impl ExtU32 for u32 {
    fn bps(self) -> Bps {
        Bps(self.Hz())
    }
}

pub mod uart;
pub use uart::*;

pub mod twi;
pub use twi::*;

pub mod spi;
pub use spi::*;
