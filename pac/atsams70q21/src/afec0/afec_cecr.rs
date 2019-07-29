#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AFEC_CECR {
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
pub type ECORR0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR0W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR1W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR2W<'a> {
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
#[doc = r"Reader of the field"]
pub type ECORR3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR4W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR4W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR5W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR5W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR6W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR6W<'a> {
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
#[doc = r"Reader of the field"]
pub type ECORR7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR7W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR7W<'a> {
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
pub type ECORR8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR8W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR8W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR9W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR9W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR10W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR10W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type ECORR11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _ECORR11W<'a> {
    w: &'a mut W,
}
impl<'a> _ECORR11W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    pub fn ecorr0(&self) -> ECORR0_R {
        ECORR0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    pub fn ecorr1(&self) -> ECORR1_R {
        ECORR1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    pub fn ecorr2(&self) -> ECORR2_R {
        ECORR2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    pub fn ecorr3(&self) -> ECORR3_R {
        ECORR3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    pub fn ecorr4(&self) -> ECORR4_R {
        ECORR4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    pub fn ecorr5(&self) -> ECORR5_R {
        ECORR5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    pub fn ecorr6(&self) -> ECORR6_R {
        ECORR6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    pub fn ecorr7(&self) -> ECORR7_R {
        ECORR7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    pub fn ecorr8(&self) -> ECORR8_R {
        ECORR8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    pub fn ecorr9(&self) -> ECORR9_R {
        ECORR9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    pub fn ecorr10(&self) -> ECORR10_R {
        ECORR10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    pub fn ecorr11(&self) -> ECORR11_R {
        ECORR11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Error Correction Enable for channel 0"]
    #[inline(always)]
    pub fn ecorr0(&mut self) -> _ECORR0W {
        _ECORR0W { w: self }
    }
    #[doc = "Bit 1 - Error Correction Enable for channel 1"]
    #[inline(always)]
    pub fn ecorr1(&mut self) -> _ECORR1W {
        _ECORR1W { w: self }
    }
    #[doc = "Bit 2 - Error Correction Enable for channel 2"]
    #[inline(always)]
    pub fn ecorr2(&mut self) -> _ECORR2W {
        _ECORR2W { w: self }
    }
    #[doc = "Bit 3 - Error Correction Enable for channel 3"]
    #[inline(always)]
    pub fn ecorr3(&mut self) -> _ECORR3W {
        _ECORR3W { w: self }
    }
    #[doc = "Bit 4 - Error Correction Enable for channel 4"]
    #[inline(always)]
    pub fn ecorr4(&mut self) -> _ECORR4W {
        _ECORR4W { w: self }
    }
    #[doc = "Bit 5 - Error Correction Enable for channel 5"]
    #[inline(always)]
    pub fn ecorr5(&mut self) -> _ECORR5W {
        _ECORR5W { w: self }
    }
    #[doc = "Bit 6 - Error Correction Enable for channel 6"]
    #[inline(always)]
    pub fn ecorr6(&mut self) -> _ECORR6W {
        _ECORR6W { w: self }
    }
    #[doc = "Bit 7 - Error Correction Enable for channel 7"]
    #[inline(always)]
    pub fn ecorr7(&mut self) -> _ECORR7W {
        _ECORR7W { w: self }
    }
    #[doc = "Bit 8 - Error Correction Enable for channel 8"]
    #[inline(always)]
    pub fn ecorr8(&mut self) -> _ECORR8W {
        _ECORR8W { w: self }
    }
    #[doc = "Bit 9 - Error Correction Enable for channel 9"]
    #[inline(always)]
    pub fn ecorr9(&mut self) -> _ECORR9W {
        _ECORR9W { w: self }
    }
    #[doc = "Bit 10 - Error Correction Enable for channel 10"]
    #[inline(always)]
    pub fn ecorr10(&mut self) -> _ECORR10W {
        _ECORR10W { w: self }
    }
    #[doc = "Bit 11 - Error Correction Enable for channel 11"]
    #[inline(always)]
    pub fn ecorr11(&mut self) -> _ECORR11W {
        _ECORR11W { w: self }
    }
}
