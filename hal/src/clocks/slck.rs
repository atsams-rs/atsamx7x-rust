use super::*;

/// The source of the [`MainClock`].
///
/// Refer to §23.4.2 and §60.2.1.
pub trait SlowClockSource {}

impl SlowClockSource for InternalRC {}
impl SlowClockSource for ExternalNormal {}
impl SlowClockSource for ExternalBypass {}

/// SCLK, sourced from the [`InternalRC`], or an external clock:
/// [`ExternalNormal`] or [`ExternalBypass`].
pub struct SlowClock<S: SlowClockSource> {
    source: PhantomData<S>,
}

impl<S: SlowClockSource> SlowClock<S> {
    pub(crate) const FREQ: Hertz = Hertz::from_raw(32_768);
}

impl<S: SlowClockSource> Clock for SlowClock<S> {
    fn freq(&self) -> Hertz {
        Self::FREQ
    }
}

impl Token<SlowClock<InternalRC>> {
    /// Configure [`SlowClock`] for the [`InternalRC`] source.
    pub fn configure_internal(self) -> SlowClock<InternalRC> {
        // clock is in this mode already, nothing needs to be done
        SlowClock {
            source: PhantomData,
        }
    }

    /// Configure [`SlowClock`] for the [`ExternalNormal`] source.
    pub fn configure_external_normal(self) -> SlowClock<ExternalNormal> {
        self.supc().supc_cr.write(|w| {
            w.xtalsel().set_bit();
            w.key().passwd()
        });

        SlowClock {
            source: PhantomData,
        }
    }

    /// Configure [`SlowClock`] for the [`ExternalBypass`] source.
    pub fn configure_external_bypass(self) -> SlowClock<ExternalBypass> {
        self.supc().supc_mr.modify(|_, w| {
            w.oscbypass().set_bit();
            w.key().passwd()
        });
        self.supc().supc_cr.write(|w| {
            w.xtalsel().set_bit();
            w.key().passwd()
        });

        SlowClock {
            source: PhantomData,
        }
    }
}
