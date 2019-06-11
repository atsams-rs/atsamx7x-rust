#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SMC_SETUP {
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
pub struct NWE_SETUPR {
    bits: u8,
}
impl NWE_SETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NCS_WR_SETUPR {
    bits: u8,
}
impl NCS_WR_SETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NRD_SETUPR {
    bits: u8,
}
impl NRD_SETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct NCS_RD_SETUPR {
    bits: u8,
}
impl NCS_RD_SETUPR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _NWE_SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NWE_SETUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NCS_WR_SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NCS_WR_SETUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NRD_SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NRD_SETUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _NCS_RD_SETUPW<'a> {
    w: &'a mut W,
}
impl<'a> _NCS_RD_SETUPW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
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
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline]
    pub fn nwe_setup(&self) -> NWE_SETUPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NWE_SETUPR { bits }
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline]
    pub fn ncs_wr_setup(&self) -> NCS_WR_SETUPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NCS_WR_SETUPR { bits }
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline]
    pub fn nrd_setup(&self) -> NRD_SETUPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NRD_SETUPR { bits }
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline]
    pub fn ncs_rd_setup(&self) -> NCS_RD_SETUPR {
        let bits = {
            const MASK: u8 = 63;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        NCS_RD_SETUPR { bits }
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
    #[doc = "Bits 0:5 - NWE Setup Length"]
    #[inline]
    pub fn nwe_setup(&mut self) -> _NWE_SETUPW {
        _NWE_SETUPW { w: self }
    }
    #[doc = "Bits 8:13 - NCS Setup Length in WRITE Access"]
    #[inline]
    pub fn ncs_wr_setup(&mut self) -> _NCS_WR_SETUPW {
        _NCS_WR_SETUPW { w: self }
    }
    #[doc = "Bits 16:21 - NRD Setup Length"]
    #[inline]
    pub fn nrd_setup(&mut self) -> _NRD_SETUPW {
        _NRD_SETUPW { w: self }
    }
    #[doc = "Bits 24:29 - NCS Setup Length in READ Access"]
    #[inline]
    pub fn ncs_rd_setup(&mut self) -> _NCS_RD_SETUPW {
        _NCS_RD_SETUPW { w: self }
    }
}
