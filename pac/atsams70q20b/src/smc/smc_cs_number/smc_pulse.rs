#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMC_PULSE {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct NWE_PULSER {
    bits: u8,
}
impl NWE_PULSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NCS_WR_PULSER {
    bits: u8,
}
impl NCS_WR_PULSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NRD_PULSER {
    bits: u8,
}
impl NRD_PULSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NCS_RD_PULSER {
    bits: u8,
}
impl NCS_RD_PULSER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _NWE_PULSEW<'a> {
    w: &'a mut W,
}
impl<'a> _NWE_PULSEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NCS_WR_PULSEW<'a> {
    w: &'a mut W,
}
impl<'a> _NCS_WR_PULSEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NRD_PULSEW<'a> {
    w: &'a mut W,
}
impl<'a> _NRD_PULSEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NCS_RD_PULSEW<'a> {
    w: &'a mut W,
}
impl<'a> _NCS_RD_PULSEW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline]
    pub fn nwe_pulse(&self) -> NWE_PULSER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NWE_PULSER { bits }
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline]
    pub fn ncs_wr_pulse(&self) -> NCS_WR_PULSER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NCS_WR_PULSER { bits }
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline]
    pub fn nrd_pulse(&self) -> NRD_PULSER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NRD_PULSER { bits }
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline]
    pub fn ncs_rd_pulse(&self) -> NCS_RD_PULSER {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NCS_RD_PULSER { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - NWE Pulse Length"]
    #[inline]
    pub fn nwe_pulse(&mut self) -> _NWE_PULSEW {
        _NWE_PULSEW { w: self }
    }
    #[doc = "Bits 8:14 - NCS Pulse Length in WRITE Access"]
    #[inline]
    pub fn ncs_wr_pulse(&mut self) -> _NCS_WR_PULSEW {
        _NCS_WR_PULSEW { w: self }
    }
    #[doc = "Bits 16:22 - NRD Pulse Length"]
    #[inline]
    pub fn nrd_pulse(&mut self) -> _NRD_PULSEW {
        _NRD_PULSEW { w: self }
    }
    #[doc = "Bits 24:30 - NCS Pulse Length in READ Access"]
    #[inline]
    pub fn ncs_rd_pulse(&mut self) -> _NCS_RD_PULSEW {
        _NCS_RD_PULSEW { w: self }
    }
}
