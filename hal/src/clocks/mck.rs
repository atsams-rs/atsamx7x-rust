use super::*;

/// MCK, driven by [`SlowClock`], [`MainClock`], [`UpllDivClock`], or
/// [`PllaClock`].
pub struct HostClock {
    freq: Hertz,
}
