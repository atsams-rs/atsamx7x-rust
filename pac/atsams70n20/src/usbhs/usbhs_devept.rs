#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USBHS_DEVEPT {
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
pub type EPEN0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN0W<'a> {
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
pub type EPEN1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN1W<'a> {
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
pub type EPEN2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN2W<'a> {
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
pub type EPEN3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN3W<'a> {
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
pub type EPEN4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN4W<'a> {
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
pub type EPEN5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN5W<'a> {
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
pub type EPEN6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN6W<'a> {
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
pub type EPEN7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN7W<'a> {
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
pub type EPEN8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN8W<'a> {
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
pub type EPEN9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPEN9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPEN9W<'a> {
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
pub type EPRST0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST0W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST0W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST1W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST1W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST2W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST2W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST3W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST3W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST4W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST4W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST5W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST5W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST6W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST6W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST7W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST7W<'a> {
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
#[doc = r"Reader of the field"]
pub type EPRST8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST8W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST8W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type EPRST9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _EPRST9W<'a> {
    w: &'a mut W,
}
impl<'a> _EPRST9W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&self) -> EPEN0_R {
        EPEN0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&self) -> EPEN1_R {
        EPEN1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&self) -> EPEN2_R {
        EPEN2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&self) -> EPEN3_R {
        EPEN3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&self) -> EPEN4_R {
        EPEN4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&self) -> EPEN5_R {
        EPEN5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&self) -> EPEN6_R {
        EPEN6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&self) -> EPEN7_R {
        EPEN7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&self) -> EPEN8_R {
        EPEN8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    pub fn epen9(&self) -> EPEN9_R {
        EPEN9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&self) -> EPRST0_R {
        EPRST0_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&self) -> EPRST1_R {
        EPRST1_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&self) -> EPRST2_R {
        EPRST2_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&self) -> EPRST3_R {
        EPRST3_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&self) -> EPRST4_R {
        EPRST4_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&self) -> EPRST5_R {
        EPRST5_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&self) -> EPRST6_R {
        EPRST6_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&self) -> EPRST7_R {
        EPRST7_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&self) -> EPRST8_R {
        EPRST8_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    pub fn eprst9(&self) -> EPRST9_R {
        EPRST9_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Endpoint 0 Enable"]
    #[inline(always)]
    pub fn epen0(&mut self) -> _EPEN0W {
        _EPEN0W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 Enable"]
    #[inline(always)]
    pub fn epen1(&mut self) -> _EPEN1W {
        _EPEN1W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 Enable"]
    #[inline(always)]
    pub fn epen2(&mut self) -> _EPEN2W {
        _EPEN2W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 Enable"]
    #[inline(always)]
    pub fn epen3(&mut self) -> _EPEN3W {
        _EPEN3W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 Enable"]
    #[inline(always)]
    pub fn epen4(&mut self) -> _EPEN4W {
        _EPEN4W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 Enable"]
    #[inline(always)]
    pub fn epen5(&mut self) -> _EPEN5W {
        _EPEN5W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 Enable"]
    #[inline(always)]
    pub fn epen6(&mut self) -> _EPEN6W {
        _EPEN6W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 Enable"]
    #[inline(always)]
    pub fn epen7(&mut self) -> _EPEN7W {
        _EPEN7W { w: self }
    }
    #[doc = "Bit 8 - Endpoint 8 Enable"]
    #[inline(always)]
    pub fn epen8(&mut self) -> _EPEN8W {
        _EPEN8W { w: self }
    }
    #[doc = "Bit 9 - Endpoint 9 Enable"]
    #[inline(always)]
    pub fn epen9(&mut self) -> _EPEN9W {
        _EPEN9W { w: self }
    }
    #[doc = "Bit 16 - Endpoint 0 Reset"]
    #[inline(always)]
    pub fn eprst0(&mut self) -> _EPRST0W {
        _EPRST0W { w: self }
    }
    #[doc = "Bit 17 - Endpoint 1 Reset"]
    #[inline(always)]
    pub fn eprst1(&mut self) -> _EPRST1W {
        _EPRST1W { w: self }
    }
    #[doc = "Bit 18 - Endpoint 2 Reset"]
    #[inline(always)]
    pub fn eprst2(&mut self) -> _EPRST2W {
        _EPRST2W { w: self }
    }
    #[doc = "Bit 19 - Endpoint 3 Reset"]
    #[inline(always)]
    pub fn eprst3(&mut self) -> _EPRST3W {
        _EPRST3W { w: self }
    }
    #[doc = "Bit 20 - Endpoint 4 Reset"]
    #[inline(always)]
    pub fn eprst4(&mut self) -> _EPRST4W {
        _EPRST4W { w: self }
    }
    #[doc = "Bit 21 - Endpoint 5 Reset"]
    #[inline(always)]
    pub fn eprst5(&mut self) -> _EPRST5W {
        _EPRST5W { w: self }
    }
    #[doc = "Bit 22 - Endpoint 6 Reset"]
    #[inline(always)]
    pub fn eprst6(&mut self) -> _EPRST6W {
        _EPRST6W { w: self }
    }
    #[doc = "Bit 23 - Endpoint 7 Reset"]
    #[inline(always)]
    pub fn eprst7(&mut self) -> _EPRST7W {
        _EPRST7W { w: self }
    }
    #[doc = "Bit 24 - Endpoint 8 Reset"]
    #[inline(always)]
    pub fn eprst8(&mut self) -> _EPRST8W {
        _EPRST8W { w: self }
    }
    #[doc = "Bit 25 - Endpoint 9 Reset"]
    #[inline(always)]
    pub fn eprst9(&mut self) -> _EPRST9W {
        _EPRST9W { w: self }
    }
}
