//! Flash block configuration

use crate::target_device::EFC;
use crate::pmc::PmcError;

pub struct Efc {
    pub(crate) periph: EFC,
}

impl Efc {
    pub fn new(periph: EFC) -> Self {
        periph.eefc_wpmr.modify(|_r, w| {
            w.wpkey().passwd();
            w.wpen().clear_bit();
            w
        });

        Self { periph }
    }

    pub fn set_wait_states(&mut self, fws: FlashWaitStates) {
        let fws_bits = fws as u8;

        self.periph
            .eefc_fmr
            .modify(|_r, w| unsafe { w.fws().bits(fws_bits) });
    }
}

/// The number of flash wait states for a read operation.
///
/// Note: The number of cycles a read takes is 1 + FWS.
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum FlashWaitStates {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl FlashWaitStates {
    /// Calculate the lowest possible number of flash wait states from a given
    /// master clock frequency in MHz.
    ///
    /// The max mck frequency supported is 150MHz. This is *not* the CPU frequency,
    /// which may go up to 300MHz.
    ///
    /// Note: This is probably only valid at VDDIO = 3.0V
    pub fn from_mck_mhz(freq: u8) -> Result<Self, PmcError> {
        // Reference: Table 58-51 Embedded Flash Wait States for Worst-Case Conditions
        let fws = match freq {
            0..=23 => Self::Zero,
            24..=46 => Self::One,
            47..=69 => Self::Two,
            70..=92 => Self::Three,
            93..=115 => Self::Four,
            116..=138 => Self::Five,
            139..=150 => Self::Six,
            _ => return Err(PmcError::InvalidConfiguration),
        };

        Ok(fws)
    }
}
