#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_OCR {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
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
#[doc = r"Reader of the field"]
pub type CAL4_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CAL4W<'a> {
    w: &'a mut W,
}
impl<'a> _CAL4W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | ((value as u32) & 0x7f);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SEL4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEL4W<'a> {
    w: &'a mut W,
}
impl<'a> _SEL4W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CAL8_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CAL8W<'a> {
    w: &'a mut W,
}
impl<'a> _CAL8W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SEL8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEL8W<'a> {
    w: &'a mut W,
}
impl<'a> _SEL8W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type CAL12_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _CAL12W<'a> {
    w: &'a mut W,
}
impl<'a> _CAL12W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SEL12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SEL12W<'a> {
    w: &'a mut W,
}
impl<'a> _SEL12W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&self) -> CAL4_R {
        CAL4_R::new((self.bits() & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&self) -> SEL4_R {
        SEL4_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&self) -> CAL8_R {
        CAL8_R::new(((self.bits() >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&self) -> SEL8_R {
        SEL8_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&self) -> CAL12_R {
        CAL12_R::new(((self.bits() >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&self) -> SEL12_R {
        SEL12_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn cal4(&mut self) -> _CAL4W {
        _CAL4W { w: self }
    }
    #[doc = "Bit 7 - Selection of Main RC Oscillator Calibration Bits for 4 MHz"]
    #[inline(always)]
    pub fn sel4(&mut self) -> _SEL4W {
        _SEL4W { w: self }
    }
    #[doc = "Bits 8:14 - Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn cal8(&mut self) -> _CAL8W {
        _CAL8W { w: self }
    }
    #[doc = "Bit 15 - Selection of Main RC Oscillator Calibration Bits for 8 MHz"]
    #[inline(always)]
    pub fn sel8(&mut self) -> _SEL8W {
        _SEL8W { w: self }
    }
    #[doc = "Bits 16:22 - Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn cal12(&mut self) -> _CAL12W {
        _CAL12W { w: self }
    }
    #[doc = "Bit 23 - Selection of Main RC Oscillator Calibration Bits for 12 MHz"]
    #[inline(always)]
    pub fn sel12(&mut self) -> _SEL12W {
        _SEL12W { w: self }
    }
}
