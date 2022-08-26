//! Value-level [`Pin`] configuration structures

#[allow(unused_imports)]
use super::*;

//================================================================================
// Value-level end-user Pin configurations
//================================================================================

/// Pull direction for the [`Pin`].
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PullDir {
    /// [`Pin`] is floating: is it not pulled in either direction.
    Floating,
    /// [`Pin`] is pulled down.
    PullDown,
    /// [`Pin`] is pulled up.
    PullUp,
}

/// Input filter to use on the [`Pin`].
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum InputFilter {
    /// Enables the input glitch filter.
    ///
    /// Glitches with a duration of less than 1/2 of [`HostClock`] periods are
    /// filtered out. The frequency of [`HostClock`] is configured via [`HostClockController::configure`].
    ///
    /// [`HostClock`]: crate::clocks::HostClock
    /// [`HostClockController::configure`]: crate::clocks::HostClockController::configure
    Glitch,

    /// Enables the input debouncing filter.
    ///
    /// Minimum debounce duration is bank-global. Refer to
    /// [`BankConfiguration::min_debounce_duration`].
    Debounce,
}

/// Type of signal event for the [`Pin`] to interrupt on.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum InterruptType {
    /// [`Pin`] triggers on rising and falling edge.
    AnyEdge,
    /// [`Pin`] triggers on rising edge.
    RisingEdge,
    /// [`Pin`] triggers on falling edge.
    FallingEdge,
    /// [`Pin`] triggers on low level.
    LowLevel,
    /// [`Pin`] triggers on high level.
    HighLevel,
}

//================================================================================
// DynPinMode & DynPeripheral
//================================================================================

/// Value-level `enum` for [`Peripheral`] function configurations.
#[doc(hidden)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynPeripheral {
    A,
    B,
    C,
    D,
}

/// Value-level `enum` representing [`PinMode`]s.
#[doc(hidden)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DynPinMode {
    Reset,
    Peripheral(DynPeripheral),
    Output,
    OpenDrain,
    Input,
}

//================================================================================
// DynBank & DynPinId
//================================================================================

/// Value-level `enum` regresenting [`Pin`] banks
#[doc(hidden)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum DynBank {
    A,
    B,
    #[cfg(feature = "pins-144")]
    C,
    D,
    #[cfg(feature = "pins-144")]
    E,
}

impl DynBank {
    pub(in crate::pio) fn ptr(&self) -> *const RegisterBlock {
        match self {
            Self::A => PIOA::ptr(),
            Self::B => PIOB::ptr(),
            #[cfg(feature = "pins-144")]
            Self::C => PIOC::ptr(),
            Self::D => PIOD::ptr(),
            #[cfg(feature = "pins-144")]
            Self::E => PIOE::ptr(),
        }
    }
}

/// Value-level `struct` representing [`Pin`] IDs
#[doc(hidden)]
#[derive(Eq, PartialEq, Clone, Copy)]
pub struct DynPinId {
    pub bank: DynBank,
    pub num: u8,
}
