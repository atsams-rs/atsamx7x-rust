use super::*;

/// Abstraction that counts events occuring on a [`ChannelInputPin<_, _, A>`].
pub struct Counter<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> {
    channel: Channel<M, I, Capture<C, FREQ_HZ>>,
}

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32> Counter<M, I, C, FREQ_HZ> {
    /// Sample the frequency on the previously specified
    /// [`ChannelInputPin<_, _, A>`] using the previously specified
    /// event criteria.
    ///
    /// # Timeout
    ///
    /// If the [`Counter`] did not sample the specified events within
    /// `timeout`, a [`CounterError::Timeout`] is returned.
    pub fn sample_freq(
        &mut self,
        timeout: fugit::Duration<u32, 1, FREQ_HZ>,
    ) -> Result<Hertz, CounterError> {
        // Ensure any previous state is cleared.
        self.channel.read_status();

        // Load RC compare value with timeout.
        let ticks: u16 = timeout
            .ticks()
            .try_into()
            .map_err(|_| CounterError::TimeoutOverflow {
                wanted: timeout.convert(),
                maximum: fugit::Duration::<u32, 1, FREQ_HZ>::from_ticks(u16::MAX.into()).convert(),
            })?;
        self.channel.set_compare(CompareRegister::Rc(ticks));

        self.channel.reset_enable();

        let mut loaded = [false, false];
        while loaded.iter().any(|f| !f) {
            let sr = self.channel.read_status();

            if sr.cpcs().bit_is_set() {
                // NOTE: timeout, no edges where sampled
                return Err(CounterError::Timeout);
            }

            // Has RA/RB loaded?
            if sr.ldras().bit_is_set() {
                loaded[0] = true;
            }
            if sr.ldrbs().bit_is_set() {
                loaded[1] = true;
            }
        }

        // Both edges have been sampled: calculate measured frequency.
        let ra = self.channel.channel().tc_ra.read().ra().bits();
        let rb = self.channel.channel().tc_rb.read().rb().bits();

        // Read-back the sampling ratio
        let ratio = self
            .channel
            .channel()
            .tc_cmr_capture_mode()
            .read()
            .sbsmplr()
            .variant()
            .map(|v| v.multiplier())
            .unwrap();

        Ok(
            fugit::Duration::<u32, 1, FREQ_HZ>::from_ticks(rb.saturating_sub(ra))
                .try_into_rate()
                .unwrap_or(Hertz::from_raw(0))
                * ratio,
        )
    }
}

/// Capture modes for external input
#[derive(Debug, Clone, Copy)]
pub enum CaptureMode {
    /// Capture rising edges
    Rising,
    /// Capture falling edges
    Falling,
    /// Capture rising and falling edges
    Either,
}

impl From<CaptureMode> for LDRA_A {
    fn from(mode: CaptureMode) -> Self {
        match mode {
            CaptureMode::Rising => Self::RISING,
            CaptureMode::Falling => Self::FALLING,
            CaptureMode::Either => Self::EDGE,
        }
    }
}

impl From<CaptureMode> for LDRB_A {
    fn from(mode: CaptureMode) -> Self {
        match mode {
            CaptureMode::Rising => Self::RISING,
            CaptureMode::Falling => Self::FALLING,
            CaptureMode::Either => Self::EDGE,
        }
    }
}

impl From<CaptureSamplingRatio> for SBSMPLR_A {
    fn from(ratio: CaptureSamplingRatio) -> Self {
        match ratio {
            CaptureSamplingRatio::One => Self::ONE,
            CaptureSamplingRatio::Two => Self::HALF,
            CaptureSamplingRatio::Four => Self::FOURTH,
            CaptureSamplingRatio::Eight => Self::EIGHTH,
            CaptureSamplingRatio::Sixteen => Self::SIXTEENTH,
        }
    }
}

trait CaptureSamplingRatioMultiplier {
    fn multiplier(&self) -> u32;
}

impl CaptureSamplingRatioMultiplier for SBSMPLR_A {
    fn multiplier(&self) -> u32 {
        match self {
            Self::ONE => 1,
            Self::HALF => 2,
            Self::FOURTH => 4,
            Self::EIGHTH => 8,
            Self::SIXTEENTH => 16,
        }
    }
}

/// Possible [`Counter`] errors.
#[derive(Debug)]
pub enum CounterError {
    /// The specified timeout is too large and cannot be handled by
    /// hardware.
    TimeoutOverflow {
        /// The specified timeout.
        wanted: NanosDuration,
        /// The maximum possible timeout.
        maximum: NanosDuration,
    },
    /// Timeout was reached before [`Counter`] could sample the
    /// specified events.
    Timeout,
}

/// Possible [`ChannelInputPin<_, _, A>`] input sampling ratios:
/// denotes how many [`CaptureMode`] events should be sampled before
/// the criteria is fulfilled.
#[allow(missing_docs)]
pub enum CaptureSamplingRatio {
    One,
    Two,
    Four,
    Eight,
    Sixteen,
}

/// [`Counter`] configuration, describing how the signal on
/// [`ChannelInputPin<_, _, A>`] should be handled.
pub struct CounterConfiguration {
    /// The sampling ratio of the input signal.
    pub sampling: CaptureSamplingRatio,
    /// The starting event: the [`Counter`] starts counting internal
    /// clock ticks from this event.
    pub leading_criteria: CaptureMode,
    /// The ending event: the [`Counter`] stops counting internal
    /// clock ticks from this event.
    pub trailing_criteria: CaptureMode,
}

impl<M: TcMeta, I: ChannelId, C: ChannelClock, const FREQ_HZ: u32>
    Channel<M, I, Capture<C, FREQ_HZ>>
{
    /// Transform the [`Channel`] into a [`Counter`] implementation
    /// that can measure frequencies.
    pub fn into_freq_measure(self, cfg: CounterConfiguration) -> Counter<M, I, C, FREQ_HZ> {
        self.channel().tc_cmr_capture_mode().modify(|_, w| {
            w.ldra().variant(cfg.leading_criteria.into());
            w.ldrb().variant(cfg.trailing_criteria.into());
            w.sbsmplr().variant(cfg.sampling.into());

            // stop/disable clock on RB load; this allows us to read
            // our one-shot measurements without race conditions.
            w.ldbdis().set_bit();
            w.ldbstop().set_bit();

            // RC compare no effect on counter/clock
            w.cpctrg().clear_bit();
            // use TIOBx as external trigger
            w.abetrg().clear_bit();
            // clock not gated by external signal
            w.etrgedg().none();

            w.burst().none();

            // increment counter on rising clock edge
            w.clki().clear_bit();

            w
        });

        // disable all interrupts
        self.channel().tc_idr.write(|w| unsafe { w.bits(u32::MAX) });

        Counter { channel: self }
    }
}
