//! Flash controller configuration

use crate::pmc::{Megahertz, PmcError};
use crate::target_device::EFC;

/// The voltage which drives the MCU.
///
/// Refer to §58 and §59.
#[derive(Eq, PartialEq)]
pub enum VddioLevel {
    /// VDDIO = 3.3V, typical
    V3,
    /// VDDIO = 1.7V, minimal
    V1,
}

pub struct Efc {
    pub(crate) periph: EFC,
    vddio: VddioLevel,
}

impl Efc {
    pub fn new(periph: EFC, vddio: VddioLevel) -> Self {
        periph.eefc_wpmr.modify(|_r, w| {
            w.wpkey().passwd();
            w.wpen().clear_bit();
            w
        });

        Self { periph, vddio }
    }

    /// Calculates and sets the lowest possible number of flash wait
    /// states from a given master clock frequency in MHz.
    ///
    /// The max mck frequency supported is 150MHz. This is *not* the CPU frequency,
    /// which may go up to 300MHz.
    pub fn set_wait_states(&mut self, freq: Megahertz) -> Result<(), PmcError> {
        let fws = FlashWaitStates::calculate(freq, &self.vddio)?;

        self.periph
            .eefc_fmr
            .modify(|_r, w| unsafe { w.fws().bits(fws as u8) });

        Ok(())
    }
}

/// The number of flash wait states for a read operation.
///
/// Note: The number of cycles a read takes is 1 + FWS.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
enum FlashWaitStates {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl FlashWaitStates {
    pub fn calculate(freq: Megahertz, vddio: &VddioLevel) -> Result<Self, PmcError> {
        #[cfg(any(feature = "v70", feature = "v71"))]
        if vddio == &VddioLevel::V1 {
            // V70/V71 must be driven with VDDIO = 3.3V, typical
            return Err(PmcError::InvalidConfiguration);
        }

        Self::fws_from_freq(freq, vddio)
    }

    fn fws_from_freq(freq: Megahertz, vddio: &VddioLevel) -> Result<Self, PmcError> {
        match vddio {
            VddioLevel::V1 => {
                // References:
                // - Table 59-50 (p. 1850) Embedded Flash Wait States for Worst-Case Conditions (E70/S70; VDDIO = 1.7V)
                Ok(match freq.to_MHz() {
                    0..=21 => Self::Zero,
                    22..=42 => Self::One,
                    43..=63 => Self::Two,
                    64..=84 => Self::Three,
                    85..=106 => Self::Four,
                    107..=125 => Self::Five,
                    126..=137 => Self::Six,
                    _ => return Err(PmcError::InvalidConfiguration),
                })
            }
            VddioLevel::V3 => {
                // References:
                // - Table 58-50 (p. 1804) Embedded Flash Wait States for Worst-Case Conditions (V70/V71)
                // - Table 59-50 (p. 1850) Embedded Flash Wait States for Worst-Case Conditions (E70/S70; VDDIO = 3.0V)
                Ok(match freq.to_MHz() {
                    0..=23 => Self::Zero,
                    24..=46 => Self::One,
                    47..=69 => Self::Two,
                    70..=92 => Self::Three,
                    93..=115 => Self::Four,
                    116..=138 => Self::Five,
                    139..=150 => Self::Six,
                    _ => return Err(PmcError::InvalidConfiguration),
                })
            }
        }
    }
}
