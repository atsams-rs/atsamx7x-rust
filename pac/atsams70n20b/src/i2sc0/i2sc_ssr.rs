#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::I2SC_SSR {
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Proxy"]
pub struct _RXORW<'a> {
    w: &'a mut W,
}
impl<'a> _RXORW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TXURW<'a> {
    w: &'a mut W,
}
impl<'a> _TXURW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _RXORCHW<'a> {
    w: &'a mut W,
}
impl<'a> _RXORCHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = r"Proxy"]
pub struct _TXURCHW<'a> {
    w: &'a mut W,
}
impl<'a> _TXURCHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Receive Overrun Status Set"]
    #[inline(always)]
    pub fn rxor(&mut self) -> _RXORW {
        _RXORW { w: self }
    }
    #[doc = "Bit 6 - Transmit Underrun Status Set"]
    #[inline(always)]
    pub fn txur(&mut self) -> _TXURW {
        _TXURW { w: self }
    }
    #[doc = "Bits 8:9 - Receive Overrun Per Channel Status Set"]
    #[inline(always)]
    pub fn rxorch(&mut self) -> _RXORCHW {
        _RXORCHW { w: self }
    }
    #[doc = "Bits 20:21 - Transmit Underrun Per Channel Status Set"]
    #[inline(always)]
    pub fn txurch(&mut self) -> _TXURCHW {
        _TXURCHW { w: self }
    }
}
