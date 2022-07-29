/*!
Serial Peripheral Interface (SPI) mode.
---
[`embedded-hal::spi`](`crate::ehal::spi`) implementations for [`Spi`].

If SPI without a select pin is required, build the HAL with `features
= [.., "usart-spi-host-without-select"]`.
*/
use super::*;
pub use crate::ehal::spi::*;
use crate::ehal::{self, blocking};

/// Possible [`Spi`]  errors.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SpiError {
    /// The next word was received by the [`Usart`] before the
    /// previous one was [`Spi::read`].
    Overrun,
    /// In [`Client`] mode: a word requested by the bus host was not
    /// sent via [`Spi::send`] in time.
    Underrun,
    /// A [`Spi`] function was called, but the [`Usart`] is in [`Uart`] mode
    ModeError,
    /// Tried to enter [`client-mode`](SpiMode::Client) when client mode was not possible
    InvalidMode,
    /// Calculated prescaler is below 6, which would disable the [`Spi`].
    PrescalerUnderflow,
}
impl From<SpiError> for UsartError {
    fn from(e: SpiError) -> Self {
        Self::Spi(e)
    }
}

/// Possible [`Spi`] modes.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SpiMode {
    /// Denotes Host mode for the [`Spi`]
    Host,
    /// Denotes Client mode for the [`Spi`]
    Client,
}

/// Type-level denotation of a [`Spi`] in [`SpiMode::Host`]-mode.
pub enum Host {}
/// Type-level denotation of a [`Spi`] in [`SpiMode::Host`]-mode.
pub enum Client {}

/// Type marker for the role of the [`Spi`].
pub trait SpiRole {
    /// The roll of the [`Spi`] in the current configuration
    const ROLE: SpiMode;
}

impl SpiRole for Host {
    const ROLE: SpiMode = SpiMode::Host;
}
impl SpiRole for Client {
    const ROLE: SpiMode = SpiMode::Client;
}

/// Holds configuration information for the [`Spi`]
#[derive(PartialEq)]
pub struct SpiConfig {
    /// In [`Host`] mode: the communication bitrate to the bus client.
    /// Ignored in [`Client`] mode.
    pub bitrate: Bps,
    /// See documentation for [`Mode`].
    pub mode: Mode,
}

/// SPI handle for the [`Usart`].
/// ---
///
/// This Spi abstraction implements the embedded hal [spi](`ehal::spi`) traits
pub struct Spi<M: UsartMeta, SpiRole> {
    cfg: SpiConfig,
    pres: Prescaler,
    _meta: PhantomData<M>,
    _role: PhantomData<SpiRole>,
}

impl<M: UsartMeta, R: SpiRole> RegisterAccess<M> for Spi<M, R> {}

impl<M: UsartMeta, R: SpiRole> Spi<M, R> {
    pub(crate) fn new(usart: &Usart<M>, cfg: SpiConfig) -> Result<Self, UsartError> {
        // Ensure a valid prescaler can be calculated
        let pres = {
            // USART does not seem to support oversampling in SPI mode.
            const NO_OVERSAMPLING: bool = false;
            let pres = usart.calc_pres(cfg.bitrate, NO_OVERSAMPLING)?;

            // C.f. §46.6.8.2
            const SPI_MIN_PRESCALER: u16 = 6;
            if pres < SPI_MIN_PRESCALER {
                return Err(SpiError::PrescalerUnderflow.into());
            }

            pres
        };

        Ok(Self {
            cfg,
            pres,
            _meta: PhantomData,
            _role: PhantomData,
        })
    }
}

impl<'usart, M: UsartMeta, R: SpiRole> UsartHandle<M> for Spi<M, R> {
    const MODE: UsartMode = UsartMode::Spi(R::ROLE);

    #[inline(always)]
    unsafe fn reset(&self, usart: &mut Usart<M>) {
        usart.reg().us_mr_spi_mode().reset();
    }

    unsafe fn configure(&self, usart: &mut Usart<M>) -> Prescaler {
        usart.reg().us_mr_spi_mode().write(|w| {
            match R::ROLE {
                SpiMode::Host => {
                    w.usart_mode().spi_master();
                    w.clko().set_bit(); // Drive clock pin
                    w.usclks().mck();
                }
                SpiMode::Client => {
                    w.usart_mode().spi_slave();
                    w.clko().clear_bit();
                    w.usclks().sck();
                }
            };

            w.chrl()._8_bit();
            w.wrdbt().clear_bit(); // Don't force read before write

            // C.f. §46.7.4
            match self.cfg.mode.polarity {
                Polarity::IdleLow => w.cpol().clear_bit(),
                Polarity::IdleHigh => w.cpol().set_bit(),
            };
            match self.cfg.mode.phase {
                Phase::CaptureOnFirstTransition => w.cpha().set_bit(),
                Phase::CaptureOnSecondTransition => w.cpha().clear_bit(),
            };

            w
        });

        self.pres
    }
}

impl<M: UsartMeta, R: SpiRole> Spi<M, R> {
    #[inline(always)]
    fn ensure_mode(&self) -> Result<(), SpiError> {
        if self.mode() != UsartMode::Spi(R::ROLE) {
            return Err(SpiError::ModeError);
        }
        Ok(())
    }

    /// Clear the [`SpiError`] flags in hardware.
    #[inline(always)]
    pub fn clear_errors(&mut self) {
        <Self as RegisterAccess<M>>::clear_errors(self)
    }

    fn transfer_inner<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], SpiError> {
        for word in words.iter_mut() {
            nb::block!(self.send(*word))?;
            *word = nb::block!(self.read())?;
        }
        Ok(words)
    }

    fn write_inner(&mut self, words: &[u8]) -> Result<(), SpiError> {
        for word in words.iter() {
            nb::block!(self.send(*word))?;
            nb::block!(self.read())?;
        }
        Ok(())
    }
}

impl<M: UsartMeta, R: SpiRole> ehal::spi::FullDuplex<u8> for Spi<M, R> {
    type Error = SpiError;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.ensure_mode()?;

        let sr = self.reg().us_csr_spi_mode().read();
        if sr.rxrdy().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }
        if sr.ovre().bit_is_set() {
            return Err(nb::Error::Other(SpiError::Overrun));
        }
        // TODO UNRE is missing from the SVD files. Refer to
        // <https://git.grepit.se/embedded-rust/atsamx7x-hal/-/issues/28>.
        //
        // if sr.unre().bit_is_set() {
        //     return Err(nb::Error::Other(SpiError::Underrun));
        // }

        Ok(self.reg().us_rhr.read().rxchr().bits() as u8)
    }

    fn send(&mut self, word: u8) -> nb::Result<(), Self::Error> {
        self.ensure_mode()?;

        let sr = self.reg().us_csr_spi_mode().read();
        if sr.txrdy().bit_is_clear() {
            return Err(nb::Error::WouldBlock);
        }

        self.reg()
            .us_thr
            .write(|w| unsafe { w.txchr().bits(word as u16) });
        Ok(())
    }
}

impl<M: UsartMeta, R: SpiRole> blocking::spi::Transactional<u8> for Spi<M, R> {
    type Error = SpiError;

    fn exec<'a>(
        &mut self,
        operations: &mut [blocking::spi::Operation<'a, u8>],
    ) -> Result<(), Self::Error> {
        use blocking::spi::Operation;

        for o in operations.iter_mut() {
            if let Some(e) = match o {
                Operation::Transfer(words) => self.transfer_inner(words).err(),
                Operation::Write(words) => self.write_inner(words).err(),
            } {
                return Err(e);
            }
        }
        Ok(())
    }
}

impl<M: UsartMeta, R: SpiRole> blocking::spi::Write<u8> for Spi<M, R> {
    type Error = SpiError;

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.write_inner(words)
    }
}

impl<M: UsartMeta, R: SpiRole> blocking::spi::Transfer<u8> for Spi<M, R> {
    type Error = SpiError;

    fn transfer<'w>(&mut self, words: &'w mut [u8]) -> Result<&'w [u8], Self::Error> {
        self.transfer_inner(words)
    }
}
