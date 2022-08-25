/*! 
Flash controller configuration
---

The Enhanced Embedded Flash Controller peripheral (EFC) or (EEFC) provides the interface of the Flash block with the 32-bit internal bus.

Two functions of the EFC are exposed in this HAL, the ability to set wait states, and the ability to reconfigure the flash memory.

The wait states need to be reconfigured to allow for correct operation at higher clock frequencies and is normally handled by the [`Clock`] module.


*/

use crate::clocks::{ClockError, Megahertz};
use crate::pac::{efc, EFC};
use core::ptr;

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

/// Set of [`Token`]s for device Flash Memory.
#[allow(missing_docs)]
pub struct Tokens {
    pub sector0: Sector<0>,
    pub sector1: Sector<1>,
    pub sector2: Sector<2>,
    pub sector3: Sector<3>,
    #[cfg(not(feature = "flash-512K"))]
    pub sector4: Sector<4>,
    #[cfg(not(feature = "flash-512K"))]
    pub sector5: Sector<5>,
    #[cfg(not(feature = "flash-512K"))]
    pub sector6: Sector<6>,
    #[cfg(not(feature = "flash-512K"))]
    pub sector7: Sector<7>,
    #[cfg(feature = "flash-2M")]
    pub sector8: Sector<8>,
    #[cfg(feature = "flash-2M")]
    pub sector9: Sector<9>,
    #[cfg(feature = "flash-2M")]
    pub sector10: Sector<10>,
    #[cfg(feature = "flash-2M")]
    pub sector11: Sector<11>,
    #[cfg(feature = "flash-2M")]
    pub sector12: Sector<12>,
    #[cfg(feature = "flash-2M")]
    pub sector13: Sector<13>,
    #[cfg(feature = "flash-2M")]
    pub sector14: Sector<14>,
    #[cfg(feature = "flash-2M")]
    pub sector15: Sector<15>,
}

impl Tokens {
    /// Create the set of all Flash Sector [`Token`]s.
    pub fn new(_efc: EFC) -> Self {
        Self {
            sector0: Sector::new(),
            sector1: Sector::new(),
            sector2: Sector::new(),
            sector3: Sector::new(),
            #[cfg(not(feature = "flash-512K"))]
            sector4: Sector::new(),
            #[cfg(not(feature = "flash-512K"))]
            sector5: Sector::new(),
            #[cfg(not(feature = "flash-512K"))]
            sector6: Sector::new(),
            #[cfg(not(feature = "flash-512K"))]
            sector7: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector8: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector9: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector10: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector11: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector12: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector13: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector14: Sector::new(),
            #[cfg(feature = "flash-2M")]
            sector15: Sector::new(),
        }
    }
}

/// Struct representing a sector of Flash Memory
pub struct Sector<const N: u8> {}
/// Error Enum, Big WIP on this one.
#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// Access Outside Of the Flash Sector
    SectorRangeError,
    /// Attempted to execute a command when eefc is busy
    FlashBusyError,
    /// An invalid Command and/or a bad keyword was/were written in EEFC_FMR
    FlashCommandError,
    /// Programming/erase of at least one locked region has happened.
    FlashLockError,
    /// A Flash memory error occured at the end of programming (EraseVerify or WriteVerify
    /// test has failed).
    FlashError,
    /// MultiError MSB
    MultipleEccErrorMsb,
    /// MultiEccErrorLsb
    MultipleEccErrorLsb,
    /// UniqueError MSB
    UniqueEccErrorMsb,
    /// UniqueEccErrorLsb
    UniqueEccErrorLsb,
}

impl From<u32> for Error {
    fn from(x: u32) -> Error {
        if x & 0x00000002 != 0 {
            Error::FlashCommandError
        } else if x & 0x00000004 != 0 {
            Error::FlashLockError
        } else if x & 0x00000008 != 0 {
            Error::FlashError
        } else if x & 0x00010000 != 0 {
            Error::UniqueEccErrorLsb
        } else if x & 0x00020000 != 0 {
            Error::MultipleEccErrorLsb
        } else if x & 0x00040000 != 0 {
            Error::UniqueEccErrorMsb
        } else if x & 0x00080000 != 0 {
            Error::MultipleEccErrorMsb
        } else {
            panic!("Unknown Error")
        }
    }
}

/// Token Representing a sector in flash memory
impl<const N: u8> Sector<N> {
    /// Read a single 32 bit word from a flash sector.
    pub fn read_word(&self, offset: usize) -> Result<usize, Error> {
        // Check that offset is still within range of the sector
        if offset > 32767 {
            return Err(Error::SectorRangeError);
        }
        Ok(unsafe { ptr::read_volatile(self.addr().offset(offset.try_into().unwrap())) })
    }

    /// Erase the entire sector.
    /// Safety: Erasing Flash is fundamentally unsafe.
    pub unsafe fn erase_sector(&self) -> Result<(), Error> {
        if self.efc().eefc_fsr.read().frdy().bit_is_clear() {
            return Err(Error::FlashBusyError);
        }
        self.efc().eefc_fcr.write(|w| {
            w.fkey().passwd();
            w.fcmd().es();
            w.farg().bits((256_u16 * (N as u16)) as u16);
            w
        });
        loop {
            let status = self.efc().eefc_fsr.read();
            // Wait until frdy is set
            if status.bits() == 0x00000001 {
                return Ok(());
            }
            // If an error is detected, return
            else if status.bits() != 0x00000000 {
                return Err(status.bits().into());
            }
        }
    }

    /// Write page to flash memory.
    /// Safety: Writing to Flash is fundamentally unsafe.
    pub unsafe fn write_page(&self, page: u16, data: &[usize]) -> Result<(), Error> {
        if page > 255 {
            return Err(Error::SectorRangeError);
        }
        if self.efc().eefc_fsr.read().frdy().bit_is_clear() {
            return Err(Error::FlashBusyError);
        }
        self.addr()
            .offset((page * 512) as isize)
            .copy_from(data.as_ptr(), 128);

        self.efc().eefc_fcr.write(|w| {
            w.fkey().passwd();
            w.fcmd().wp();
            w.farg().bits(256_u16 * (N as u16) + page);
            w
        });
        loop {
            let status = self.efc().eefc_fsr.read();
            // Wait until frdy is set
            if status.bits() == 0x00000001 {
                return Ok(());
            }
            // If an error is detected, return
            else if status.bits() != 0x00000000 {
                return Err(status.bits().into());
            }
        }
    }

    /// Write single 32-bit word to flash memory.
    /// Safety: Writing to Flash is fundamentally unsafe.
    pub unsafe fn write_word(&self, word: usize, offset: u16) -> Result<(), Error> {
        if offset > 32767 {
            return Err(Error::SectorRangeError);
        }
        if self.efc().eefc_fsr.read().frdy().bit_is_clear() {
            return Err(Error::FlashBusyError);
        }
        self.addr().offset(offset as isize).write_volatile(word);
        self.efc().eefc_fcr.write(|w| {
            w.fkey().passwd();
            w.fcmd().wp();
            w.farg().bits(256_u16 * (N as u16) + (offset/128));
            w
        });
        loop {
            let status = self.efc().eefc_fsr.read();
            // Wait until frdy is set
            if status.bits() == 0x00000001 {
                return Ok(());
            }
            // If an error is detected, return
            else if status.bits() != 0x00000000 {
                return Err(status.bits().into());
            }
        }
    }

    fn new() -> Self {
        Self {}
    }
}

unsafe trait RegisterAccess<const N: u8> {
    #[inline(always)]
    fn addr(&self) -> *mut usize {
        unsafe { (0x400000 + (N as usize) * 0x20000) as *mut usize }
    }

    fn efc(&self) -> &efc::RegisterBlock {
        unsafe { &*EFC::ptr() }
    }
}

unsafe impl<const N: u8> RegisterAccess<N> for Sector<N> {}
/// [`EFC`] abstraction.
pub struct Efc {
    pub(crate) periph: EFC,
    vddio: VddioLevel,
}

impl Efc {
    /// Creates a new [`Efc`], the behavior of which depends on the
    /// voltage, [`VddioLevel`], that drives the MCU.
    pub fn new(periph: EFC, vddio: VddioLevel) -> (Self) {
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
    pub fn set_wait_states(&mut self, freq: Megahertz) -> Result<(), ClockError> {
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
    pub fn calculate(freq: Megahertz, vddio: &VddioLevel) -> Result<Self, ClockError> {
        #[cfg(any(feature = "v70", feature = "v71"))]
        if vddio == &VddioLevel::V1 {
            // V70/V71 must be driven with VDDIO = 3.3V, typical
            return Err(ClockError::InvalidVddioLevel);
        }

        Self::fws_from_freq(freq, vddio)
    }

    fn fws_from_freq(freq: Megahertz, vddio: &VddioLevel) -> Result<Self, ClockError> {
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
                    _ => return Err(ClockError::InvalidHccFreq(freq)),
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
                    _ => return Err(ClockError::InvalidHccFreq(freq)),
                })
            }
        }
    }
}
