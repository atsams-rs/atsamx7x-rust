/*!
Flash controller configuration
---

The Enhanced Embedded Flash Controller peripheral (EFC) or (EEFC) provides the interface of the Flash block with the 32-bit internal bus.

Two functions of the EFC are exposed in this HAL, the ability to set wait states, and the ability to reconfigure the flash memory.

The wait states need to be reconfigured to allow for correct operation at higher clock frequencies and is normally handled by the [clocks](crate::clocks) module.


*/

use crate::clocks::{ClockError, Megahertz};
use crate::pac::{efc, EFC};
use core::slice;
use embedded_storage::nor_flash::*;

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

/// The Base Address of the Flash Memory.
pub const BASE_ADDRESS: u32 = 0x00400000;
///The Capacity in bytes of the Flash Memory.
#[cfg(feature = "flash-2M")]
pub const CAPACITY: u32 = 0x00200000;
///The Capacity in bytes of the Flash Memory.
#[cfg(feature = "flash-1M")]
pub const CAPACITY: u32 = 0x00100000;
///The Capacity in bytes of the Flash Memory.
#[cfg(feature = "flash-512K")]
pub const CAPACITY: u32 = 0x00080000;
/// The Size in bytes of a page in the Flash Memory.
pub const PAGE_SIZE: u32 = 512;
/// The Size in bytes of a sector in the Flash Memory.
pub const SECTOR_SIZE: u32 = 0x00020000;

/// An iterator over the Flash Sectors.
struct SectorIterator {
    start_offset: u32,
    end_offset: u32,
}

/// A struct representing a Sector in Flash Memory.
#[derive(Clone, Copy)]
struct Sector {
    offset: u32,
}

impl Iterator for SectorIterator {
    type Item = Sector;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_offset >= self.end_offset {
            return None;
        }
        let sector = Sector {
            offset: self.start_offset,
        };
        self.start_offset += SECTOR_SIZE;

        Some(sector)
    }
}

/// A struct representing a Page in Flash memory
#[derive(Clone, Copy)]
struct Page {
    offset: u32,
}

/// An iterator over the Flash Pages.
struct PageIterator {
    start_offset: u32,
    end_offset: u32,
}

impl Iterator for PageIterator {
    type Item = Page;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start_offset >= self.end_offset {
            return None;
        }
        let page = Page {
            offset: self.start_offset,
        };
        self.start_offset += PAGE_SIZE;

        Some(page)
    }
}

/// Flash Memory Read/Write/Erase Errors.
#[derive(Debug, Copy, Clone)]
pub enum Error {
    /// The arguments are not properly aligned.
    NotAligned,
    /// The arguments are out of bounds.
    OutOfBounds,
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
    /// OtherError
    OtherError,
}

impl NorFlashError for Error {
    fn kind(&self) -> NorFlashErrorKind {
        match self {
            Error::NotAligned => NorFlashErrorKind::NotAligned,
            Error::OutOfBounds => NorFlashErrorKind::OutOfBounds,
            _ => NorFlashErrorKind::Other,
        }
    }
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

impl From<NorFlashErrorKind> for Error {
    fn from(e: NorFlashErrorKind) -> Error {
        match e {
            NorFlashErrorKind::NotAligned => Error::NotAligned,
            NorFlashErrorKind::OutOfBounds => Error::OutOfBounds,
            _ => Error::OtherError,
        }
    }
}

impl Sector {
    /// Erase the entire sector.
    fn erase_sector(&self) -> Result<(), Error> {
        if self.efc().eefc_fsr.read().frdy().bit_is_clear() {
            return Err(Error::FlashBusyError);
        }
        self.efc().eefc_fcr.write(|w| {
            w.fkey().passwd();
            w.fcmd().es();
            unsafe { w.farg().bits(self.page_number()) };
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

    // #[inline(always)] fn addr(&self) -> *mut u8 {
    //     (BASE_ADDRESS + self.offset) as *mut u8
    // }

    #[inline(always)]
    fn page_number(&self) -> u16 {
        (self.offset / PAGE_SIZE) as u16
    }

    // #[inline(always)]
    // fn contains(&self, offset: u32) -> bool {
    //     offset >= self.offset
    //         && offset < self.offset + SECTOR_SIZE
    // }
}

impl Page {
    /// Write page to flash memory.
    fn write_page(&self, data: &[u8]) -> Result<(), Error> {
        if data.len() != PAGE_SIZE as usize {
            return Err(Error::NotAligned);
        }
        if self.efc().eefc_fsr.read().frdy().bit_is_clear() {
            return Err(Error::FlashBusyError);
        }
        unsafe { self.addr().copy_from(data.as_ptr(), PAGE_SIZE as usize) };

        self.efc().eefc_fcr.write(|w| {
            w.fkey().passwd();
            w.fcmd().wp();
            unsafe { w.farg().bits(self.page_number()) };
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

    #[inline(always)]
    fn addr(&self) -> *mut u8 {
        (BASE_ADDRESS + self.offset) as *mut u8
    }


    #[inline(always)]
    fn page_number(&self) -> u16 {
        (self.offset / PAGE_SIZE) as u16
    }

    // #[inline(always)]
    // fn contains(&self, offset: u32) -> bool {
    //     offset >= self.offset
    //         && offset < self.offset + PAGE_SIZE as u32
    // }
}

trait RegisterAccess {
    fn efc(&self) -> &efc::RegisterBlock {
        unsafe { &*EFC::ptr() }
    }
}

impl RegisterAccess for Sector {}
impl RegisterAccess for Page {}
/// [`EFC`] abstraction.
pub struct Efc {
    pub(crate) periph: EFC,
    vddio: VddioLevel,
}

impl Efc {
    /// Creates a new [`Efc`], the behavior of which depends on the
    /// voltage, [`VddioLevel`], that drives the MCU.
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
    pub fn set_wait_states(&mut self, freq: Megahertz) -> Result<(), ClockError> {
        let fws = FlashWaitStates::calculate(freq, &self.vddio)?;

        self.periph
            .eefc_fmr
            .modify(|_r, w| unsafe { w.fws().bits(fws as u8) });

        Ok(())
    }
}

trait Flash {
    type SectorIterator;
    type PageIterator;

    fn len(&self) -> u32 {
        CAPACITY
    }

    fn address(&self) -> u32 {
        BASE_ADDRESS
    }

    fn sectors(start_offset: u32, end_offset: u32) -> SectorIterator {
        SectorIterator {
            start_offset,
            end_offset,
        }
    }

    fn pages(start_offset: u32, end_offset: u32) -> PageIterator {
        PageIterator {
            start_offset,
            end_offset,
        }
    }
}

impl Flash for Efc {
    type SectorIterator = SectorIterator;
    type PageIterator = PageIterator;
}

impl ErrorType for Efc {
    type Error = Error;
}

impl ReadNorFlash for Efc {
    const READ_SIZE: usize = 1;

    fn read(&mut self, offset: u32, bytes: &mut [u8]) -> Result<(), Self::Error> {
        check_read(self, offset, bytes.len())?;
        let offset = offset as usize;
        let ptr = self.address() as *const _;
        let capacity = self.capacity();
        let flash_slice = unsafe { slice::from_raw_parts(ptr, capacity) };
        bytes.copy_from_slice(&flash_slice[offset..offset + bytes.len()]);
        Ok(())
    }

    fn capacity(&self) -> usize {
        self.len() as usize
    }
}

impl NorFlash for Efc {
    const WRITE_SIZE: usize = PAGE_SIZE as usize;
    // NOTE: This number is the sector size, there is an option to erase by page as well, but the
    // minimum/maximum erase size for that varies throughout the flash memory.
    const ERASE_SIZE: usize = SECTOR_SIZE as usize;

    fn erase(&mut self, offset: u32, to: u32) -> Result<(), Self::Error> {
        check_erase(self, offset, to)?;
        for sector in Self::sectors(offset, to) {
            sector.erase_sector()?;
        }
        Ok(())
    }

    fn write(&mut self, offset: u32, bytes: &[u8]) -> Result<(), Self::Error> {
        check_write(self, offset, bytes.len())?;
        let mut bytes_written = 0;
        for page in Self::pages(offset, offset + bytes.len() as u32) {
            page.write_page(&bytes[bytes_written..(bytes_written + Self::WRITE_SIZE)])?;
            bytes_written += Self::WRITE_SIZE;
        }
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

