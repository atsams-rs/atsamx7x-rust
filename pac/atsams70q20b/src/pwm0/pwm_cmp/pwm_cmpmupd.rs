#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PWM_CMPMUPD {
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
}
#[doc = r" Proxy"]
pub struct _CENUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CENUPDW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CTRUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CTRUPDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CPRUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CPRUPDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _CUPRUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _CUPRUPDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bit 0 - Comparison x Enable Update"]
    #[inline]
    pub fn cenupd(&mut self) -> _CENUPDW {
        _CENUPDW { w: self }
    }
    #[doc = "Bits 4:7 - Comparison x Trigger Update"]
    #[inline]
    pub fn ctrupd(&mut self) -> _CTRUPDW {
        _CTRUPDW { w: self }
    }
    #[doc = "Bits 8:11 - Comparison x Period Update"]
    #[inline]
    pub fn cprupd(&mut self) -> _CPRUPDW {
        _CPRUPDW { w: self }
    }
    #[doc = "Bits 16:19 - Comparison x Update Period Update"]
    #[inline]
    pub fn cuprupd(&mut self) -> _CUPRUPDW {
        _CUPRUPDW { w: self }
    }
}
