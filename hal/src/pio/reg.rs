use crate::pac::pioa::RegisterBlock;

use super::dynpin::*;

pub(in crate::pio) trait RegisterInterface {
    fn id(&self) -> DynPinId;

    /// Return the `u32` mask to set/clear a bit for this
    /// [`DynPinId`].
    #[inline]
    fn mask(&self) -> u32 {
        1 << self.id().num
    }

    #[inline]
    fn reg(&self) -> &RegisterBlock {
        unsafe { &*self.id().bank.ptr() }
    }

    #[inline]
    fn change_mode(&mut self, mode: DynPinMode) {
        match mode {
            DynPinMode::Reset => unimplemented!(),
            DynPinMode::Peripheral(a) => self.as_peripheral(a),
            DynPinMode::Output => self.as_output(),
            DynPinMode::OpenDrain => self.as_opendrain(),
            DynPinMode::Input => self.as_input(),
        }
    }

    #[inline]
    fn as_peripheral(&mut self, cfg: DynPeripheral) {
        use DynPeripheral::*;
        let (sr0, sr1) = match cfg {
            A => (false, false),
            B => (true, false),
            C => (false, true),
            D => (true, true),
        };
        let idx = self.id().num;

        // configure function, preserving other pin bits
        for (i, bit) in (0..=1).zip([sr0, sr1]) {
            self.reg().abcdsr[i].modify(|r, w| unsafe {
                w.bits(match bit {
                    true => r.bits() | 1 << idx,
                    false => r.bits() & !(1 << idx),
                })
            });
        }

        // give pin to peripheral
        self.reg().pdr.write(|w| unsafe { w.bits(self.mask()) });

        // disable multidrive
        self.reg().mddr.write(|w| unsafe { w.bits(self.mask()) });
    }

    #[inline]
    fn as_output(&mut self) {
        // take pin from peripheral
        self.reg().per.write(|w| unsafe { w.bits(self.mask()) });

        // disable multidrive
        self.reg().mddr.write(|w| unsafe { w.bits(self.mask()) });

        // enable pin output
        self.reg().oer.write(|w| unsafe { w.bits(self.mask()) });
    }

    #[inline]
    fn as_opendrain(&mut self) {
        // take pin from peripheral
        self.reg().per.write(|w| unsafe { w.bits(self.mask()) });

        // Enable multidrive
        self.reg().mder.write(|w| unsafe { w.bits(self.mask()) });

        // disable pin output, driver should not be connected
        self.reg().odr.write(|w| unsafe { w.bits(self.mask()) });
    }

    #[inline]
    fn as_input(&mut self) {
        // take pin from peripheral
        self.reg().per.write(|w| unsafe { w.bits(self.mask()) });

        // disable multidrive
        self.reg().mddr.write(|w| unsafe { w.bits(self.mask()) });

        // disable pin output: pin is a pure input
        self.reg().odr.write(|w| unsafe { w.bits(self.mask()) });
    }

    /// Write the logic level of an output pin
    #[inline]
    fn write_pin(&mut self, bit: bool) {
        if bit {
            self.reg().sodr.write(|w| unsafe { w.bits(self.mask()) });
        } else {
            self.reg().codr.write(|w| unsafe { w.bits(self.mask()) });
        }
    }

    /// Read the logic level of an output pin.
    ///
    /// Returns `true` if the pin is driven high.
    #[inline]
    fn read_out_pin(&self) -> bool {
        self.reg().odsr.read().bits() & self.mask() != 0
    }

    /// Read the logic level of an input pin.
    ///
    /// Returns `true` if the pin is at level 1 (high).
    #[inline]
    fn read_in_pin(&self) -> bool {
        self.reg().pdsr.read().bits() & self.mask() != 0
    }

    /// Toggle the logic level of an output pin.
    #[inline]
    fn toggle_out_pin(&mut self) {
        self.write_pin(!self.read_out_pin());
    }

    /// Set the pull direction of the pin, if any.
    fn set_pull_dir(&mut self, cfg: PullDir) {
        // Refer to §32.5.1
        let pull_up = |b: bool| {
            if b {
                self.reg().puer.write(|w| unsafe { w.bits(self.mask()) });
            } else {
                self.reg().pudr.write(|w| unsafe { w.bits(self.mask()) });
            }
        };
        let pull_down = |b: bool| {
            if b {
                self.reg().ppder.write(|w| unsafe { w.bits(self.mask()) });
            } else {
                self.reg().ppddr.write(|w| unsafe { w.bits(self.mask()) });
            }
        };

        match cfg {
            PullDir::Floating => {
                pull_up(false);
                pull_down(false);
            }

            // Disable the oppisite pin first, otherwise the write is
            // noop, because the pin cannot be pulled down and up at
            // the same time.
            PullDir::PullUp => {
                pull_down(false);
                pull_up(true);
            }
            PullDir::PullDown => {
                pull_up(false);
                pull_down(true);
            }
        }
    }

    fn set_interrupt(&mut self, cfg: Option<InterruptType>) {
        if cfg == None {
            // Disable pin interrupt
            //
            // XXX The peripheral clock is not disabled because it
            // could be used by another pin in this bank. This clock
            // dependency is currently not tracked. Refer to #11.
            self.reg().idr.write(|w| unsafe { w.bits(self.mask()) });
            return;
        }

        // Configure the event detector tree seen in §32.5.10, Figure
        // 32-6.
        if cfg == Some(InterruptType::AnyEdge) {
            self.reg().aimdr.write(|w| unsafe { w.bits(self.mask()) });
        } else {
            self.reg().aimer.write(|w| unsafe { w.bits(self.mask()) });
        }
        match cfg {
            Some(InterruptType::RisingEdge) => {
                self.reg().esr.write(|w| unsafe { w.bits(self.mask()) });
                self.reg().rehlsr.write(|w| unsafe { w.bits(self.mask()) });
            }
            Some(InterruptType::FallingEdge) => {
                self.reg().esr.write(|w| unsafe { w.bits(self.mask()) });
                self.reg().fellsr.write(|w| unsafe { w.bits(self.mask()) });
            }
            Some(InterruptType::LowLevel) => {
                self.reg().lsr.write(|w| unsafe { w.bits(self.mask()) });
                self.reg().fellsr.write(|w| unsafe { w.bits(self.mask()) });
            }
            Some(InterruptType::HighLevel) => {
                self.reg().lsr.write(|w| unsafe { w.bits(self.mask()) });
                self.reg().rehlsr.write(|w| unsafe { w.bits(self.mask()) });
            }
            _ => (),
        }

        self.reg().ier.write(|w| unsafe { w.bits(self.mask()) });
    }

    fn set_filter(&mut self, cfg: Option<InputFilter>) {
        if cfg == None {
            // disable the input filter
            self.reg().ifdr.write(|w| unsafe { w.bits(self.mask()) });
        } else {
            // enable the input filter
            self.reg().ifer.write(|w| unsafe { w.bits(self.mask()) });
        }

        match cfg {
            Some(InputFilter::Glitch) => {
                // enable glitch filter, using the peripheral clock
                self.reg().ifscdr.write(|w| unsafe { w.bits(self.mask()) });
            }
            Some(InputFilter::Debounce) => {
                // enable debounce filter, using the divided slow clock
                self.reg().ifscer.write(|w| unsafe { w.bits(self.mask()) });
            }
            _ => (),
        }
    }

    fn set_schmitt(&mut self, cfg: bool) {
        self.reg().schmitt.modify(|r, w| unsafe {
            // register is r/w: set/clear only the bit for this pin.
            if cfg {
                // enable trigger by clearing the bit
                w.bits(r.bits() & !self.mask())
            } else {
                // disable trigger by setting the bit
                w.bits(r.bits() | self.mask())
            }
        });
    }
}
