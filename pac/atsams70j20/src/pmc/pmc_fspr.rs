#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PMC_FSPR {
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
pub type FSTP0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP0W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP0W<'a> {
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
pub type FSTP1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP1W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP1W<'a> {
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
pub type FSTP2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP2W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP2W<'a> {
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
pub type FSTP3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP3W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP3W<'a> {
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
pub type FSTP4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP4W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP4W<'a> {
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
pub type FSTP5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP5W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP5W<'a> {
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
pub type FSTP6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP6W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP6W<'a> {
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
pub type FSTP7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP7W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP7W<'a> {
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
pub type FSTP8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP8W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP8W<'a> {
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
pub type FSTP9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP9W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP9W<'a> {
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
pub type FSTP10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP10W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP10W<'a> {
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
pub type FSTP11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP11W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP11W<'a> {
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
#[doc = r"Reader of the field"]
pub type FSTP12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP12W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP12W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSTP13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP13W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP13W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSTP14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP14W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP14W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type FSTP15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSTP15W<'a> {
    w: &'a mut W,
}
impl<'a> _FSTP15W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&self) -> FSTP0_R {
        FSTP0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&self) -> FSTP1_R {
        FSTP1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&self) -> FSTP2_R {
        FSTP2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&self) -> FSTP3_R {
        FSTP3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&self) -> FSTP4_R {
        FSTP4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&self) -> FSTP5_R {
        FSTP5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&self) -> FSTP6_R {
        FSTP6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&self) -> FSTP7_R {
        FSTP7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&self) -> FSTP8_R {
        FSTP8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&self) -> FSTP9_R {
        FSTP9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&self) -> FSTP10_R {
        FSTP10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&self) -> FSTP11_R {
        FSTP11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&self) -> FSTP12_R {
        FSTP12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&self) -> FSTP13_R {
        FSTP13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&self) -> FSTP14_R {
        FSTP14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&self) -> FSTP15_R {
        FSTP15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Fast Startup Input Polarity 0"]
    #[inline(always)]
    pub fn fstp0(&mut self) -> _FSTP0W {
        _FSTP0W { w: self }
    }
    #[doc = "Bit 1 - Fast Startup Input Polarity 1"]
    #[inline(always)]
    pub fn fstp1(&mut self) -> _FSTP1W {
        _FSTP1W { w: self }
    }
    #[doc = "Bit 2 - Fast Startup Input Polarity 2"]
    #[inline(always)]
    pub fn fstp2(&mut self) -> _FSTP2W {
        _FSTP2W { w: self }
    }
    #[doc = "Bit 3 - Fast Startup Input Polarity 3"]
    #[inline(always)]
    pub fn fstp3(&mut self) -> _FSTP3W {
        _FSTP3W { w: self }
    }
    #[doc = "Bit 4 - Fast Startup Input Polarity 4"]
    #[inline(always)]
    pub fn fstp4(&mut self) -> _FSTP4W {
        _FSTP4W { w: self }
    }
    #[doc = "Bit 5 - Fast Startup Input Polarity 5"]
    #[inline(always)]
    pub fn fstp5(&mut self) -> _FSTP5W {
        _FSTP5W { w: self }
    }
    #[doc = "Bit 6 - Fast Startup Input Polarity 6"]
    #[inline(always)]
    pub fn fstp6(&mut self) -> _FSTP6W {
        _FSTP6W { w: self }
    }
    #[doc = "Bit 7 - Fast Startup Input Polarity 7"]
    #[inline(always)]
    pub fn fstp7(&mut self) -> _FSTP7W {
        _FSTP7W { w: self }
    }
    #[doc = "Bit 8 - Fast Startup Input Polarity 8"]
    #[inline(always)]
    pub fn fstp8(&mut self) -> _FSTP8W {
        _FSTP8W { w: self }
    }
    #[doc = "Bit 9 - Fast Startup Input Polarity 9"]
    #[inline(always)]
    pub fn fstp9(&mut self) -> _FSTP9W {
        _FSTP9W { w: self }
    }
    #[doc = "Bit 10 - Fast Startup Input Polarity 10"]
    #[inline(always)]
    pub fn fstp10(&mut self) -> _FSTP10W {
        _FSTP10W { w: self }
    }
    #[doc = "Bit 11 - Fast Startup Input Polarity 11"]
    #[inline(always)]
    pub fn fstp11(&mut self) -> _FSTP11W {
        _FSTP11W { w: self }
    }
    #[doc = "Bit 12 - Fast Startup Input Polarity 12"]
    #[inline(always)]
    pub fn fstp12(&mut self) -> _FSTP12W {
        _FSTP12W { w: self }
    }
    #[doc = "Bit 13 - Fast Startup Input Polarity 13"]
    #[inline(always)]
    pub fn fstp13(&mut self) -> _FSTP13W {
        _FSTP13W { w: self }
    }
    #[doc = "Bit 14 - Fast Startup Input Polarity 14"]
    #[inline(always)]
    pub fn fstp14(&mut self) -> _FSTP14W {
        _FSTP14W { w: self }
    }
    #[doc = "Bit 15 - Fast Startup Input Polarity 15"]
    #[inline(always)]
    pub fn fstp15(&mut self) -> _FSTP15W {
        _FSTP15W { w: self }
    }
}
