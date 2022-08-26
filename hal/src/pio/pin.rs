//! [`Pin`]-level configurations

use core::convert::Infallible;
use core::marker::PhantomData;

use crate::ehal::digital::v2::OutputPin;
#[cfg(feature = "unproven")]
use crate::ehal::digital::v2::{InputPin, StatefulOutputPin, ToggleableOutputPin};

use paste::paste;

use super::reg::RegisterInterface;
use super::*;

//================================================================================
// Registers
//================================================================================

pub(in crate::pio) struct Registers<I: PinId> {
    id: PhantomData<I>,
}

impl<I: PinId> RegisterInterface for Registers<I> {
    #[inline]
    fn id(&self) -> DynPinId {
        I::DYN
    }
}

impl<I: PinId> Registers<I> {
    #[inline]
    unsafe fn new() -> Self {
        Registers { id: PhantomData }
    }

    #[inline]
    pub(in crate::pio) fn change_mode<M: PinMode>(&mut self) {
        RegisterInterface::change_mode(self, M::DYN);
    }
}

//==============================================================================
//  Pin configurations
//==============================================================================

/// Type indicating that the [`Pin`] is in reset
pub enum Reset {}
impl generics::Sealed for Reset {}

/// Type indicating that the [`Pin`] is an input
pub enum Input {}
impl generics::Sealed for Input {}

/// Type indicating that the [`Pin`] is an output
pub enum Output {}
impl generics::Sealed for Output {}

/// Type indicating that the [`Pin`] is an open drain
pub enum OpenDrain {}
impl generics::Sealed for OpenDrain {}

//================================================================================
// Peripheral configurations
//================================================================================

/// Type-level enum for alternate peripheral function configurations
pub trait PeripheralConfig: generics::Sealed {
    /// Corresponding [`DynPeripheral`](super::DynPeripheral)
    const DYN: DynPeripheral;
}

macro_rules! peripheral {
    (
        $(
            $Letter:ident
        ),+
    ) => {
        paste! {
            $(
                impl PeripheralConfig for $Letter {
                    const DYN: DynPeripheral = DynPeripheral::$Letter;
                }
                #[
                    doc = "Type-level variant of [`PinMode`] for alternate \
                           peripheral function [`" $Letter "`]"
                ]
                pub type [<Peripheral $Letter>] = Peripheral<$Letter>;
            )+
        }
    }
}

peripheral!(A, B, C, D);

/// Type-level variant of [`PinMode`] for [`Peripheral`] functions
pub struct Peripheral<C: PeripheralConfig> {
    cfg: PhantomData<C>,
}

impl<C: PeripheralConfig> generics::Sealed for Peripheral<C> {}

//================================================================================
// Pin modes
//================================================================================

/// Type-level enum representing [`Pin`] modes
#[allow(missing_docs)]
pub trait PinMode: generics::Sealed {
    const DYN: DynPinMode;
}

impl PinMode for Reset {
    const DYN: DynPinMode = DynPinMode::Reset;
}

impl<C: PeripheralConfig> PinMode for Peripheral<C> {
    const DYN: DynPinMode = DynPinMode::Peripheral(C::DYN);
}

impl PinMode for Output {
    const DYN: DynPinMode = DynPinMode::Output;
}

impl PinMode for OpenDrain {
    const DYN: DynPinMode = DynPinMode::OpenDrain;
}

impl PinMode for Input {
    const DYN: DynPinMode = DynPinMode::Input;
}

//================================================================================
// Pin definition
//================================================================================

#[allow(missing_docs)]
pub trait PinId: generics::Sealed {
    const DYN: DynPinId;
}

/// A type-level GPIO pin, parameterized by [`PinId`] and [`PinMode`] types
pub struct Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    pub(in crate::pio) regs: Registers<I>,
    mode: PhantomData<M>,
}
impl<I, M> generics::Sealed for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
}

impl<I, M> Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    #[inline]
    pub(in crate::pio) unsafe fn new() -> Pin<I, M> {
        Pin {
            regs: Registers::new(),
            mode: PhantomData,
        }
    }

    /// Convert the pin to the requested [`PinMode`].
    #[inline]
    pub fn into_mode<N: PinMode>(mut self) -> Pin<I, N> {
        // On reset, PB4/5/6/7/12 have special functions after reset,
        // and the bus matrix must be modified to use these pins are
        // regular (and functional) PIO pins.
        //
        // Refer to §19.4.7.
        #[cfg(feature = "reconfigurable-system-pins")]
        if I::DYN.bank == DynBank::B {
            if let pin @ (4..=7 | 12) = I::DYN.num {
                use crate::pac::{matrix::RegisterBlock, MATRIX};
                const MATRIX: *const RegisterBlock = MATRIX::ptr();

                unsafe {
                    let matrix = &*MATRIX;
                    let mask = 1 << pin;
                    matrix.ccfg_sysio.modify(|_, w| w.bits(mask));
                }
            }
        }

        if N::DYN != M::DYN {
            self.regs.change_mode::<N>();
        }

        // Safe: existing pin is dropped.
        unsafe { Pin::new() }
    }

    /// Configure the pin to operate as the corresponding [`Peripheral`] function.
    ///
    /// The type `C` indicates the desired [`Peripheral`] function.
    #[inline]
    pub fn into_peripheral<C: PeripheralConfig>(self) -> Pin<I, Peripheral<C>> {
        self.into_mode()
    }

    /// Configures this pin to operate as an [`Output`] [`Pin`]. If
    /// `bit` is `true`, the pin output is initially set high;
    /// otherwise low.
    #[inline]
    pub fn into_output(self, bit: bool) -> Pin<I, Output> {
        let mut pin: Pin<I, Output> = self.into_mode();
        pin.regs.write_pin(bit);
        pin
    }

    /// Configures tis pin to operate as an [`OpenDrain`] [`Pin`].
    #[inline]
    pub fn into_opendrain(self, cfg: PullDir) -> Pin<I, OpenDrain> {
        let mut pin: Pin<I, OpenDrain> = self.into_mode();
        pin.set_pull_dir(cfg);

        pin
    }

    /// Configures this [`Pin`] to operate as an [`Input`] [`Pin`].
    #[inline]
    pub fn into_input(self, cfg: PullDir) -> Pin<I, Input> {
        let mut pin: Pin<I, Input> = self.into_mode();
        pin.set_pull_dir(cfg);
        pin
    }

    /// Configures the pull direction for this [`Pin`], or configures
    /// it for floating.
    #[inline]
    pub fn set_pull_dir(&mut self, cfg: PullDir) {
        self.regs.set_pull_dir(cfg);
    }

    /// Configures the interrupt configuration for this [`Pin`].
    ///
    /// A `cfg` of `None` disables interrupts for this [`Pin`].
    #[inline]
    pub fn set_interrupt(&mut self, cfg: Option<InterruptType>) {
        self.regs.set_interrupt(cfg);
    }

    /// Configures the input filter for this [`Pin`].
    ///
    /// A `cfg` of `None` disables the input filter for this [`Pin`].
    #[inline]
    pub fn set_filter(&mut self, cfg: Option<InputFilter>) {
        self.regs.set_filter(cfg);
    }

    /// Enables or disables the Schmitt trigger for tis [`Pin`].
    ///
    /// The Schmitt trigger is enabled by default.
    #[inline]
    pub fn set_schmitt(&mut self, cfg: bool) {
        self.regs.set_schmitt(cfg);
    }

    #[inline]
    pub(in crate::pio) fn _set_high(&mut self) {
        self.regs.write_pin(true);
    }

    #[inline]
    pub(in crate::pio) fn _set_low(&mut self) {
        self.regs.write_pin(false);
    }

    #[inline]
    #[allow(clippy::bool_comparison)]
    pub(in crate::pio) fn _is_written_high(&self) -> bool {
        self.regs.read_out_pin() == true
    }

    #[inline]
    #[allow(clippy::bool_comparison)]
    pub(in crate::pio) fn _is_written_low(&self) -> bool {
        self.regs.read_out_pin() == false
    }

    #[inline]
    #[allow(clippy::bool_comparison)]
    pub(in crate::pio) fn _is_read_high(&self) -> bool {
        self.regs.read_in_pin() == true
    }

    #[inline]
    #[allow(clippy::bool_comparison)]
    pub(in crate::pio) fn _is_read_low(&self) -> bool {
        self.regs.read_in_pin() == false
    }

    #[inline]
    pub(in crate::pio) fn _toggle(&mut self) {
        self.regs.toggle_out_pin();
    }
}

//================================================================================
// PinMode conversions
//================================================================================

/// Use a recursive macro to implement [`From`](core::convert::From) for each
/// pair of [`PinMode`]s. A macro is necessary to avoid conflicting with the
/// reflexive implementation in [`core::convert`], i.e. `impl<T> From<T> for T`.
macro_rules! impl_nxn_convert_from {
    (
        $( #[$cfg1:meta] )?
        $Mode1:ident,
    ) => {};
    (
        #[$cfg1:meta]
        $Mode1:ident,
        $(
            $( #[$cfg2:meta] )?
            $Mode2:ident,
        )*
    ) => {
        #[$cfg1]
        impl_nxn_convert_from!(
            $Mode1,
            $(
                $( #[$cfg2] )?
                $Mode2,
            )*
        );
    };
    (
        $Mode1:ident,
        $(
            $( #[$cfg2:meta] )?
            $Mode2:ident,
        )*
    ) => {
        paste! {
            $(
                $( #[$cfg2] )?
                impl<I> From<Pin<I, $Mode1>> for Pin<I, $Mode2>
                where
                    I: PinId,
                {
                    #[doc = "Convert from [`" $Mode1 "`] to [`" $Mode2 "`]"]
                    #[inline]
                    fn from(pin: Pin<I, $Mode1>) -> Self {
                        pin.into_mode()
                    }
                }
                $( #[$cfg2] )?
                impl<I> From<Pin<I, $Mode2>> for Pin<I, $Mode1>
                where
                    I: PinId,
                {
                    #[doc = "Convert from [`" $Mode2 "`] to [`" $Mode1 "`]"]
                    #[inline]
                    fn from(pin: Pin<I, $Mode2>) -> Self {
                        pin.into_mode()
                    }
                }
            )*
            impl_nxn_convert_from!(
                $(
                    $( #[$cfg2] )?
                    $Mode2,
                )*
            );
        }
    };
}

macro_rules! impl_reset_convert_from {
    (
        $(
            $Mode:ident,
        )+
    ) => {
        paste! {
            $(
                impl<I> From<Pin<I, Reset>> for Pin<I, $Mode>
                where
                    I: PinId,
                {
                    #[doc = "Convert from [`Reset`] to [`" $Mode "`]"]
                    #[inline]
                    fn from(pin: Pin<I, Reset>) -> Self {
                        pin.into_mode()
                    }
                }
            )+
        }
    };
}

// Implements bidirectional From for all permutations
impl_nxn_convert_from!(
    Output,
    Input,
    PeripheralA,
    PeripheralB,
    PeripheralC,
    PeripheralD,
);

// Implements unidirectional From<Reset> for all permutations
impl_reset_convert_from!(
    Output,
    Input,
    PeripheralA,
    PeripheralB,
    PeripheralC,
    PeripheralD,
);

//================================================================================
// Embedded HAL traits
//================================================================================

impl<I> OutputPin for Pin<I, Output>
where
    I: PinId,
{
    type Error = Infallible;

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self._set_high();
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self._set_low();
        Ok(())
    }
}

impl<I> OutputPin for Pin<I, OpenDrain>
where
    I: PinId,
{
    type Error = Infallible;

    #[inline]
    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.set_pull_dir(PullDir::PullUp);
        Ok(())
    }

    #[inline]
    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.set_pull_dir(PullDir::Floating);
        Ok(())
    }
}

#[cfg(feature = "unproven")]
impl<I> ToggleableOutputPin for Pin<I, Output>
where
    I: PinId,
{
    type Error = Infallible;

    #[inline]
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self._toggle();
        Ok(())
    }
}

#[cfg(feature = "unproven")]
impl<I> StatefulOutputPin for Pin<I, Output>
where
    I: PinId,
{
    #[inline]
    fn is_set_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_written_high())
    }

    #[inline]
    fn is_set_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_written_low())
    }
}

#[cfg(feature = "unproven")]
impl<I, M> InputPin for Pin<I, M>
where
    I: PinId,
    M: PinMode,
{
    type Error = Infallible;

    #[inline]
    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_read_high())
    }

    #[inline]
    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_read_low())
    }
}
