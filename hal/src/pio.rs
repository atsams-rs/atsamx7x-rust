//! Parallel Input/Output Controller (PIO)

use core::marker::PhantomData;

use crate::target_device::{PIOA, PIOB, PIOC, PIOD, PIOE};

/// Root trait uted to mark traits with an exhaustive set of
/// implementations.
pub trait Sealed {}

// type FunctionSelect = (bool, bool);
pub struct FunctionSelect {
    sr0: bool,
    sr1: bool,
}
pub trait PinMode: Sealed {
    const ABCD: FunctionSelect;
}

pub enum Reset {}
pub enum FunctionA {}
pub enum FunctionB {}
pub enum FunctionC {}
pub enum FunctionD {}

impl Sealed for Reset {}
impl PinMode for Reset {
    const ABCD: FunctionSelect = functions::A::ABCD;
}

pub mod functions {
    use super::{FunctionSelect, PinMode, Sealed};

    pub enum A {}
    pub enum B {}
    pub enum C {}
    pub enum D {}

    impl Sealed for A {}
    impl Sealed for B {}
    impl Sealed for C {}
    impl Sealed for D {}

    impl PinMode for A {
        const ABCD: FunctionSelect = FunctionSelect {
            sr0: false,
            sr1: false,
        };
    }
    impl PinMode for B {
        const ABCD: FunctionSelect = FunctionSelect {
            sr0: true,
            sr1: false,
        };
    }
    impl PinMode for C {
        const ABCD: FunctionSelect = FunctionSelect {
            sr0: false,
            sr1: true,
        };
    }
    impl PinMode for D {
        const ABCD: FunctionSelect = FunctionSelect {
            sr0: true,
            sr1: true,
        };
    }
}

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    // pub(in crate::gpio) regs: Registers<I>,
    pub regs: PhantomData<I>,
    mode: PhantomData<M>,
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    #[inline]
    pub(crate) unsafe fn new() -> Pin<I, M> {
        Pin {
            mode: PhantomData,
            regs: PhantomData,
        }
    }

    pub fn into_mode<AlternateMode: PinMode>(self) -> Pin<I, AlternateMode> {
        match I::DESC.group {
            PinGroup::A => {
                let pioa: *const PIOA = PIOA::ptr() as *const _;
                let pioa = unsafe { &*pioa };
                let idx = I::DESC.num;

                // configure function
                pioa.pio_abcdsr[0]
                    .modify(|_, w| unsafe { w.bits((AlternateMode::ABCD.sr0 as u32) << idx) });
                pioa.pio_abcdsr[1]
                    .modify(|_, w| unsafe { w.bits((AlternateMode::ABCD.sr1 as u32) << idx) });

                // give pin to peripheral
                pioa.pio_pdr.write(|w| unsafe { w.bits(1 << idx) });

                // disable multidrive
                pioa.pio_mddr.write(|w| unsafe { w.bits(1 << idx) });

                // disable pull-up/down resistors
                pioa.pio_pudr.write(|w| unsafe { w.bits(1 << idx) });
                pioa.pio_ppddr.write(|w| unsafe { w.bits(1 << idx) });
            }
            _ => todo!(),
        }

        unsafe { Pin::new() }
    }
}

pub trait PinId {
    const DESC: PinDesc;
}

pub struct PinDesc {
    pub group: PinGroup,
    pub num: u8,
}

pub enum PinGroup {
    A,
    B,
    C,
    D,
    E,
}

use paste::paste;

macro_rules! pins {
    (
        $(
            $( #[$cfg:meta] )?
            $Id:ident,
        )+
    ) => {
        paste! {
            /// Collection of all the individual [`Pin`]s
            pub struct Pins {
                pioa: Option<PIOA>,
                piob: Option<PIOB>,
                pioc: Option<PIOC>,
                piod: Option<PIOD>,
                pioe: Option<PIOE>,
                $(
                    #[doc = "Pin " $Id]
                    $( #[$cfg] )?
                    pub [<$Id:lower>]: Pin<$Id, Reset>,
                )+
            }

            impl Pins {
                /// Take ownership of the PAC PIO peripherals and split them into discrete [`Pin`]s.
                #[inline]
                pub fn new(pioa: PIOA, piob: PIOB, pioc: PIOC, piod: PIOD, pioe: PIOE) -> Self {
                    Self {
                        pioa: Some(pioa),
                        piob: Some(piob),
                        pioc: Some(pioc),
                        piod: Some(piod),
                        pioe: Some(pioe),

                        $(
                            $( #[$cfg] )?
                            [<$Id:lower>]: unsafe { Pin::new() },
                        )+
                     }
                }

                /// Take the PAC PIO peripherals
                ///
                /// The PIO peripherals can only be stolen once.
                /// Subsequent calls to this function fill panic.
                ///
                /// # Safety
                ///
                /// Direct access to the PIO peripherals allows
                /// invalidation of the compiler's type-level
                /// tracking. Thus unsafe.
                #[inline]
                pub unsafe fn steal(&mut self) -> (PIOA, PIOB, PIOC, PIOD, PIOE) {
                    (
                        self.pioa.take().unwrap(),
                        self.piob.take().unwrap(),
                        self.pioc.take().unwrap(),
                        self.piod.take().unwrap(),
                        self.pioe.take().unwrap(),
                    )
                }
            }
        }
    }
}

macro_rules! pin_id {
    ($Group:ident, $Id:ident, $NUM:literal) => {
        paste! {
            #[doc = "Pin ID representing pin " $Id]
            pub enum $Id {}
            impl PinId for $Id {
                const DESC: PinDesc = PinDesc {
                    group: PinGroup::$Group,
                    num: $NUM,
                };
            }
        }
    };
}

macro_rules! declare_pins {
    (
        $(
            $Group:ident {
                $(
                    $( #[$cfg:meta] )?
                    ($Id:ident, $NUM:literal),
                )+
            }
        )+
    ) => {
        $(
            $(
                $( #[$cfg] )?
                pin_id!($Group, $Id, $NUM);
            )+
        )+
        pins!(
            $(
                $(
                    $( #[$cfg] )?
                    $Id,
                )+
            )+
        );
    };
}

declare_pins!(
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

    E {
        (PE0, 0),
        (PE1, 1),
        (PE2, 2),
        (PE3, 3),
        (PE4, 4),
        (PE5, 5),
    }
);
