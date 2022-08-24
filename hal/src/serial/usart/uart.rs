/*!
Universal asynchronous receiver-transmitter (UART) mode.
---

[`embedded_hal::serial`](`crate::ehal::serial`) implementations for
[`Uart`].
*/

use super::*;
use crate::ehal::{self, blocking};

/// Possible [Uart] errors
#[derive(Debug, Eq, PartialEq)]
pub enum UartError {
    /// A frame's stop bit was read as zero.
    Framing,
    /// Parity check of the recived frame failed.
    Parity,
    /// A word was received before the previous word was read.
    Overrun,
    /// A [`Uart`] function was called, but the [`Usart`] is in [`Spi`] mode
    ModeError,
}

impl From<UartError> for UsartError {
    fn from(e: UartError) -> Self {
        Self::Uart(e)
    }
}

/// Transmit component of a [`Uart`].
pub struct Tx<M: UsartMeta> {
    _meta: PhantomData<M>,
}

impl<M: UsartMeta> RegisterAccess<M> for Tx<M> {}

/// Receive component of a [`Uart`].
pub struct Rx<M: UsartMeta> {
    _meta: PhantomData<M>,
}

impl<M: UsartMeta> RegisterAccess<M> for Rx<M> {}

/// UART handle for [`Usart`].
/// ---
///
/// This Uart implementation implements the embedded hal [`serial`](ehal::serial) traits
pub struct Uart<M: UsartMeta> {
    cfg: UartConfiguration,
    usclks: UsartClockSource,
    pres: Prescaler,
    _meta: PhantomData<M>,
}

impl<M: UsartMeta> RegisterAccess<M> for Uart<M> {}

impl<M: UsartMeta> Uart<M> {
    /// Crate level since it is used by [`Usart`]
    pub(crate) fn new<C: UsartUartClock>(
        clk: &C,
        cfg: UartConfiguration,
    ) -> Result<Uart<M>, UsartError> {
        // Ensure a valid prescaler can be calculated
        let pres = Usart::<M>::calc_pres(clk.freq(), cfg.baud_rate, cfg.oversample)?;

        Ok(Self {
            cfg,
            usclks: C::SRC,
            pres,
            _meta: PhantomData,
        })
    }

    /// Split the [`Uart`] into two separate [`Tx`] and [`Rx`]
    /// components.
    pub fn split(self) -> (Tx<M>, Rx<M>) {
        (Tx { _meta: PhantomData }, Rx { _meta: PhantomData })
    }
}

impl<M: UsartMeta> UsartHandle<M> for Uart<M> {
    const MODE: UsartMode = UsartMode::Uart;

    #[inline(always)]
    unsafe fn reset(&self, usart: &mut Usart<M>) {
        usart.reg().us_mr_usart_mode().reset();
    }

    unsafe fn configure(&self, usart: &mut Usart<M>) -> Prescaler {
        usart.reg().us_mr_usart_mode().write(|w| {
            w.usclks().variant(self.usclks);
            w.clko().clear_bit(); // Do not drive the clk pin
            w.usart_mode().normal();
            w.sync().clear_bit(); // Uart mode
            w.par().variant(self.cfg.parity.into());
            w.nbstop()._1_bit();

            // NOTE(negation): in HW, 0 => 16X oversample, 1 => 8X
            // oversample. In this API, oversample means 16X.
            w.filter().bit(!self.cfg.oversample);

            w.chrl()._8_bit();
            w.chmode().variant(self.cfg.mode.into());
            w
        });

        self.pres
    }
}

macro_rules! impl_common {
    ($($T:ty,)+) => {
        $(
            impl<M: UsartMeta> $T {
                #[inline(always)]
                fn ensure_mode(&self) -> Result<(), UartError> {
                    if self.mode() != UsartMode::Uart {
                        return Err(UartError::ModeError);
                    }
                    Ok(())
                }
            }
        )+
    }
}

macro_rules! impl_read {
    ($($T:ty,)+) => {
        $(
            impl<M: UsartMeta> ehal::serial::Read<u8> for $T {
                type Error = UartError;

                /// Reads a single word from the [`Usart`] peripheral.
                ///
                /// # Errors
                ///
                /// The read will fail with [`nb::Error::WouldBlock`] if a word
                /// has yet to be received, and will report any read [`UartError`].
                /// It will also fail if [`Usart`] is not in [`Uart`] mode.
                fn read(&mut self) -> nb::Result<u8, Self::Error> {
                    self.ensure_mode()?;

                    let sr = self.reg().us_csr_usart_mode().read();
                    if sr.rxrdy().bit_is_clear() {
                        return Err(nb::Error::WouldBlock);
                    }
                    if sr.ovre().bit_is_set() {
                        return Err(nb::Error::Other(UartError::Overrun));
                    }
                    // TODO PARE and FRAME are missing in the SVD
                    // files. Refer to
                    // <https://git.grepit.se/embedded-rust/atsamx7x-hal/-/issues/28>/
                    //
                    //
                    // if sr.pare().bit_is_set() {
                    //     return Err(nb::Error::Other(UartError::Parity));
                    // }
                    // if sr.frame().bit_is_set() {
                    //     return Err(nb::Error::Other(UartError::Framing));
                    // }

                    Ok(self.reg().us_rhr.read().rxchr().bits() as u8)
                }
            }

            impl<M: UsartMeta> $T {
                /// Clear the [`UartError`] flags in hardware.
                #[inline(always)]
                pub fn clear_errors(&mut self) {
                    <Self as RegisterAccess<M>>::clear_errors(self)
                }
            }
        )+
    }
}

macro_rules! impl_write {
    ($($T:ty,)+) => {
        $(
            impl<M: UsartMeta> ehal::serial::Write<u8> for $T {
                type Error = UartError;

                /// Writes a single word to the [`Usart`] peripheral.
                ///
                /// # Errors
                ///
                /// Returns [`nb::Error::WouldBlock`] if the last word has yet to
                /// be processed. Also returns [`UartError::ModeError`] if the [`Usart`]
                /// is in any other mode than [`Uart`]
                fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
                    self.ensure_mode()?;

                    if self.reg().us_csr_usart_mode().read().txrdy().bit_is_clear() {
                        return Err(nb::Error::WouldBlock);
                    }

                    self.reg()
                        .us_thr
                        .write(|w| unsafe { w.txchr().bits(byte as u16) });
                    Ok(())
                }

                /// Ensures that all of the usart transfer data has been sent.
                ///
                /// # Errors
                ///
                /// Returns [`nb::Error::WouldBlock`] if the last word has yet to
                /// be processed. Also returns [`UartError::ModeError`] if the [`Usart`]
                /// is in any other mode than [`Uart`]
                fn flush(&mut self) -> nb::Result<(), Self::Error> {
                    self.ensure_mode()?;

                    if self.reg().us_csr_usart_mode().read().txempty().bit_is_clear() {
                        Err(nb::Error::WouldBlock)
                    } else {
                        Ok(())
                    }
                }
            }

            // Provide a blocking impl, derived from the non-blocking impl above.
            impl<M: UsartMeta> blocking::serial::write::Default<u8> for $T {}
        )+
    }
}

impl_common!(Uart<M>, Tx<M>, Rx<M>,);
impl_write!(Uart<M>, Tx<M>,);
impl_read!(Uart<M>, Rx<M>,);
