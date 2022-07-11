//! PIO bank abstraction, each bank containing up to 32 [`Pin`]s

use core::marker::PhantomData;
use fugit::NanosDurationU32 as NanosDuration;

use super::*;
use crate::pmc::{PeripheralIdentifier, Pmc};

use paste::paste;

/// Type-level enum for [`Pin`] banks
pub trait PinBank: Sealed {
    const DYN: DynBank;
}

/// Zero-data structure that contains the [`Pin`] interrupt lookup procedure
pub struct BankInterrupts<B: PinBank> {
    bank: PhantomData<B>,
}

impl<B: PinBank> BankInterrupts<B> {
    #[inline]
    pub(crate) fn new() -> Self {
        Self { bank: PhantomData }
    }

    /// Yields an [`Iterator`] over the [`Pin`]s in this [`PinBank`]
    /// that have pending interrupts.
    ///
    /// **NOTE**: calling this function marks all [`Pin`] interrupts
    /// as handled in the hardware.
    ///
    /// **NOTE**: The hardware implementation is subject to
    /// false-positives. Refer to [top-level documentation on interrupts].
    ///
    /// ```ignore
    /// let banka = BankA::new(..);
    /// for pin in banka.interrupts.iter() {
    ///     match pin {
    ///         11 => {
    ///             // interrupt was pending on PA11
    ///         },
    ///         pin => {
    ///             // interrupt was pending on another pin
    ///         }
    ///     }
    /// }
    /// ````
    ///
    /// [top-level documentation on interrupts]: crate::pio#Interrupts
    #[inline]
    #[must_use]
    pub fn iter(&mut self) -> BankInterruptsIter<B> {
        BankInterruptsIter::new(unsafe {
            let reg = &*B::DYN.ptr();

            // IMR contains the bits/pins of enables interrupts, ISR
            // contains the bits/pins which have detected an event.
            // AND'ing there registers gives us the bits/pins of
            // interrupts that have been explicitly enabled and
            // detected an event, thus triggering the corresponding
            // PIOA/B/C/D/E device interrupt.
            reg.pio_isr.read().bits() & reg.pio_imr.read().bits()
        })
    }
}

/// Iterates over all [`Pin`]s in [`PinBank`] that had pending
/// interrupts, following the call to [`BankInterrupts::iter`]
pub struct BankInterruptsIter<B: PinBank> {
    bank: PhantomData<B>,
    irq: u32,
    idx: u8,
}

impl<B: PinBank> BankInterruptsIter<B> {
    #[inline]
    pub(crate) fn new(irq: u32) -> Self {
        Self {
            bank: PhantomData,
            irq,
            idx: 0,
        }
    }
}

impl<B: PinBank> Iterator for BankInterruptsIter<B> {
    /// The number of the [`Pin`] that had a pending interrupt.
    ///
    /// For example, a pending interrupt on [`Pin<PA11, _>`] would
    /// yield a `Some(11)` on [`BankInterruptsIter::next`].
    type Item = u8;

    /// Returns the next pin number in this [`PinBank`] that had a
    /// pending interrupt.
    fn next(&mut self) -> Option<Self::Item> {
        match self.idx {
            32.. => {
                // We have iterated over all pins: nothing more to do.
                return None;
            }
            idx if self.irq & (1 << idx) != 0 => {
                // Pin number `idx` had a pending interrupt.
                let pin = self.idx;
                self.idx = self.idx + 1;
                Some(pin)
            }
            _ => {
                // Pin number `idx` did not have a pending interrupt:
                // advance to the next pin.
                self.idx = self.idx + 1;
                self.next()
            }
        }
    }
}

/// Configuration common to all [`Pin`]s in the bank
pub struct BankConfiguration {
    /// Minimum length of a pulse to accept it as an input.
    ///
    /// `min_debounce_duration` denotes the minimum duration the pin
    /// level must be asserted for the pulse to be accepted as an
    /// input. If the pin-level is asserted for `<
    /// min_debounce_duration / 2`, the pulse is always rejected. If
    /// the pin-level is asserted between `min_debounce_duration / 2 <
    /// d < min_debounce_duration` (where `d` denotes this duration),
    /// the pulse may or may not be rejected, depending on the precise
    /// timing of its occurance, in relation to the SLCK sampling
    /// clock.
    ///
    /// Wanted minimum duration is clamped between `0.61µs <=
    /// min_debounce_duration <= 4s`.
    ///
    /// # Input delay
    ///
    /// When using the debounce filter, an accepted pulse is subject
    /// to an input delay of approximately `1.5 *
    /// min_debounce_duration`, not counting the peripheral clock
    /// cycles required to register the input.
    ///
    /// # Additional reading
    ///
    /// Refer to §32.5.9, particularly Figure 32-5.
    pub min_debounce_duration: NanosDuration,
}

impl Default for BankConfiguration {
    fn default() -> Self {
        Self {
            min_debounce_duration: NanosDuration::from_ticks(0),
        }
    }
}

trait Bank {
    fn reg(&self) -> &RegisterBlock;

    /// Calculates the SLCK prescaler used for the debounce input
    /// filter. Refer to §32.5.9 and §32.6.29.
    fn set_min_duration_on_debounce(&mut self, min_duration: NanosDuration) {
        let slck: NanosDuration = crate::pmc::Pmc::SLCK_FREQ.into_duration();
        let div = min_duration / (2 * slck);
        let div: u16 = div.clamp(0, u16::MAX.into()) as u16;

        self.reg().pio_scdr.write(|w| unsafe {
            // NOTE: hardware adds 1 to written value
            w.div().bits(div.checked_sub(1).unwrap_or(div))
        });
    }
}

macro_rules! pin_id {
    ($Bank:ident, $Id:ident, $NUM:literal) => {
        paste! {
            #[doc = "Pin ID representing pin " $Id]
            pub enum $Id {}
            impl Sealed for $Id {}
            impl PinId for $Id {
                const DYN: DynPinId = DynPinId {
                    bank: DynBank::$Bank,
                    num: $NUM,
                };
            }
        }
    };
}

macro_rules! bank {
    (
        $Bank:ident,
        $(
            $( #[$cfg:meta] )?
            $Id:ident,
        )+
    ) => {
        paste! {
            #[doc = "Collection of all the [`Pin`]s in [`PIO" $Bank "`]"]
            pub struct [<Bank $Bank>] {
                reg: Option<[<PIO $Bank>]>,
                pub interrupts: BankInterrupts<$Bank>,
                $(
                    #[doc = "Pin " $Id]
                    $( #[$cfg] )?
                    pub [<$Id:lower>]: Pin<$Id, Reset>,
                )+
            }

            impl [<Bank $Bank>] {
                #[doc = "Creates a new [`Bank" $Bank "`], starts the [`PIO" $Bank "`] peripheral clock, and applied the given [`BankConfiguration`]."]
                #[inline]
                pub fn new(reg: [<PIO $Bank>], pmc: &mut Pmc, cfg: BankConfiguration) -> Self {
                    // enable the bank's peripheral clock
                    pmc.enable_peripherals(&[PeripheralIdentifier::[<PIO $Bank>]]).unwrap();

                    let mut bank = Self {
                        reg: Some(reg),
                        interrupts: BankInterrupts::new(),
                        $(
                            $( #[$cfg] )?
                            [<$Id:lower>]: unsafe { Pin::new() },
                        )+
                    };

                    bank.set_min_duration_on_debounce(cfg.min_debounce_duration);
                    bank
                }

                /// Steals the PAC peripheral back. The peripheral can
                /// only be taken once: subsequent calls to this
                /// function will panic.
                ///
                /// # Safety
                ///
                /// Direct access to the peripheral could allow
                /// invalidation of the compiler's type-level tracing,
                /// and is thus unsafe.
                #[inline]
                pub unsafe fn steal(&mut self) -> [<PIO $Bank>] {
                    self.reg.take().unwrap()
                }
            }

            impl Bank for [<Bank $Bank>] {
                fn reg(&self) -> &RegisterBlock {
                    self.reg.as_ref().unwrap()
                }
            }
        }
    };
}

macro_rules! banks {
    (
        $(
            $( #[$cfg1:meta] )?
            $Bank:ident {
                $(
                    $( #[$cfg2:meta] )?
                    ($Id:ident, $NUM:literal),
                )+
            }
        )+
    ) => {
        $(
            paste! {
                $( #[$cfg1] )?
                mod [<$Bank:lower _impl>] {
                    use super::*;

                    $(
                        $( #[$cfg2] )?
                            pin_id!($Bank, $Id, $NUM);
                    )+

                    bank!(
                        $Bank,
                        $(
                            $( #[$cfg2] )?
                                $Id,
                        )+
                    );

                    impl PinBank for $Bank {
                        const DYN: DynBank = DynBank::$Bank;
                    }

                }
                $( #[$cfg1] )?
                pub use [<$Bank:lower _impl>]::*;
            }
        )+
    };
}

banks!(
    A {
        (PA0, 0),
        (PA1, 1),
        (PA2, 2),
        (PA3, 3),
        (PA4, 4),
        (PA5, 5),
        (PA6, 6),
        (PA7, 7),
        (PA8, 8),
        (PA9, 9),
        (PA10, 10),
        (PA11, 11),
        (PA12, 12),
        (PA13, 13),
        (PA14, 14),
        (PA15, 15),
        (PA16, 16),
        (PA17, 17),
        (PA18, 18),
        (PA19, 19),
        (PA20, 20),
        (PA21, 21),
        (PA22, 22),
        (PA23, 23),
        (PA24, 24),
        (PA25, 25),
        (PA26, 26),
        (PA27, 27),
        (PA28, 28),
        (PA29, 29),
        (PA30, 30),
        (PA31, 31),
    }

    B {
        (PB0, 0),
        (PB1, 1),
        (PB2, 2),
        (PB3, 3),
        (PB4, 4),
        (PB5, 5),
        (PB6, 6),
        (PB7, 7),
        (PB8, 8),
        (PB9, 9),
        (PB12, 12),
        (PB13, 13),
    }

    #[cfg(feature = "pins-144")]
    C {
        (PC0, 0),
        (PC1, 1),
        (PC2, 2),
        (PC3, 3),
        (PC4, 4),
        (PC5, 5),
        (PC6, 6),
        (PC7, 7),
        (PC8, 8),
        (PC9, 9),
        (PC10, 10),
        (PC11, 11),
        (PC12, 12),
        (PC13, 13),
        (PC14, 14),
        (PC15, 15),
        (PC16, 16),
        (PC17, 17),
        (PC18, 18),
        (PC19, 19),
        (PC20, 20),
        (PC21, 21),
        (PC22, 22),
        (PC23, 23),
        (PC24, 24),
        (PC25, 25),
        (PC26, 26),
        (PC27, 27),
        (PC28, 28),
        (PC29, 29),
        (PC30, 30),
        (PC31, 31),
    }

    D {
        (PD0, 0),
        (PD1, 1),
        (PD2, 2),
        (PD3, 3),
        (PD4, 4),
        (PD5, 5),
        (PD6, 6),
        (PD7, 7),
        (PD8, 8),
        (PD9, 9),
        (PD10, 10),
        (PD11, 11),
        (PD12, 12),
        (PD13, 13),
        (PD14, 14),
        (PD15, 15),
        (PD16, 16),
        (PD17, 17),
        (PD18, 18),
        (PD19, 19),
        (PD20, 20),
        (PD21, 21),
        (PD22, 22),
        (PD23, 23),
        (PD24, 24),
        (PD25, 25),
        (PD26, 26),
        (PD27, 27),
        (PD28, 28),
        (PD29, 29),
        (PD30, 30),
        (PD31, 31),
    }

    #[cfg(feature = "pins-144")]
    E {
        (PE0, 0),
        (PE1, 1),
        (PE2, 2),
        (PE3, 3),
        (PE4, 4),
        (PE5, 5),
    }
);
