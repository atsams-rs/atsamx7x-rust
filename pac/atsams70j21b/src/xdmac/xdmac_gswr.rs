#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_GSWR {
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
pub struct _SWREQ0W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ0W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ1W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ1W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ2W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ2W<'a> {
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
pub struct _SWREQ3W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ3W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ4W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ4W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ5W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ5W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ6W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ6W<'a> {
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
pub struct _SWREQ7W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ7W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ8W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ8W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ9W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ9W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ10W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ10W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ11W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ11W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ12W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ12W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ13W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ13W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ14W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ14W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ15W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ15W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ16W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ16W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ17W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ17W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ18W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ18W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ19W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ19W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ20W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ20W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ21W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ21W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ22W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ22W<'a> {
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
#[doc = r"Proxy"]
pub struct _SWREQ23W<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQ23W<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - XDMAC Channel 0 Software Request Bit"]
    #[inline(always)]
    pub fn swreq0(&mut self) -> _SWREQ0W {
        _SWREQ0W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Software Request Bit"]
    #[inline(always)]
    pub fn swreq1(&mut self) -> _SWREQ1W {
        _SWREQ1W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Software Request Bit"]
    #[inline(always)]
    pub fn swreq2(&mut self) -> _SWREQ2W {
        _SWREQ2W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Software Request Bit"]
    #[inline(always)]
    pub fn swreq3(&mut self) -> _SWREQ3W {
        _SWREQ3W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Software Request Bit"]
    #[inline(always)]
    pub fn swreq4(&mut self) -> _SWREQ4W {
        _SWREQ4W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Software Request Bit"]
    #[inline(always)]
    pub fn swreq5(&mut self) -> _SWREQ5W {
        _SWREQ5W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Software Request Bit"]
    #[inline(always)]
    pub fn swreq6(&mut self) -> _SWREQ6W {
        _SWREQ6W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Software Request Bit"]
    #[inline(always)]
    pub fn swreq7(&mut self) -> _SWREQ7W {
        _SWREQ7W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Software Request Bit"]
    #[inline(always)]
    pub fn swreq8(&mut self) -> _SWREQ8W {
        _SWREQ8W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Software Request Bit"]
    #[inline(always)]
    pub fn swreq9(&mut self) -> _SWREQ9W {
        _SWREQ9W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Software Request Bit"]
    #[inline(always)]
    pub fn swreq10(&mut self) -> _SWREQ10W {
        _SWREQ10W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Software Request Bit"]
    #[inline(always)]
    pub fn swreq11(&mut self) -> _SWREQ11W {
        _SWREQ11W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Software Request Bit"]
    #[inline(always)]
    pub fn swreq12(&mut self) -> _SWREQ12W {
        _SWREQ12W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Software Request Bit"]
    #[inline(always)]
    pub fn swreq13(&mut self) -> _SWREQ13W {
        _SWREQ13W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Software Request Bit"]
    #[inline(always)]
    pub fn swreq14(&mut self) -> _SWREQ14W {
        _SWREQ14W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Software Request Bit"]
    #[inline(always)]
    pub fn swreq15(&mut self) -> _SWREQ15W {
        _SWREQ15W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Software Request Bit"]
    #[inline(always)]
    pub fn swreq16(&mut self) -> _SWREQ16W {
        _SWREQ16W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Software Request Bit"]
    #[inline(always)]
    pub fn swreq17(&mut self) -> _SWREQ17W {
        _SWREQ17W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Software Request Bit"]
    #[inline(always)]
    pub fn swreq18(&mut self) -> _SWREQ18W {
        _SWREQ18W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Software Request Bit"]
    #[inline(always)]
    pub fn swreq19(&mut self) -> _SWREQ19W {
        _SWREQ19W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Software Request Bit"]
    #[inline(always)]
    pub fn swreq20(&mut self) -> _SWREQ20W {
        _SWREQ20W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Software Request Bit"]
    #[inline(always)]
    pub fn swreq21(&mut self) -> _SWREQ21W {
        _SWREQ21W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Software Request Bit"]
    #[inline(always)]
    pub fn swreq22(&mut self) -> _SWREQ22W {
        _SWREQ22W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Software Request Bit"]
    #[inline(always)]
    pub fn swreq23(&mut self) -> _SWREQ23W {
        _SWREQ23W { w: self }
    }
}
