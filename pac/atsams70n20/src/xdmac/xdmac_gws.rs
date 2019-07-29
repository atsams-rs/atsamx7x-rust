#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_GWS {
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
pub type WS0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS0W<'a> {
    w: &'a mut W,
}
impl<'a> _WS0W<'a> {
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
pub type WS1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS1W<'a> {
    w: &'a mut W,
}
impl<'a> _WS1W<'a> {
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
pub type WS2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS2W<'a> {
    w: &'a mut W,
}
impl<'a> _WS2W<'a> {
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
pub type WS3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS3W<'a> {
    w: &'a mut W,
}
impl<'a> _WS3W<'a> {
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
pub type WS4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS4W<'a> {
    w: &'a mut W,
}
impl<'a> _WS4W<'a> {
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
pub type WS5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS5W<'a> {
    w: &'a mut W,
}
impl<'a> _WS5W<'a> {
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
pub type WS6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS6W<'a> {
    w: &'a mut W,
}
impl<'a> _WS6W<'a> {
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
pub type WS7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS7W<'a> {
    w: &'a mut W,
}
impl<'a> _WS7W<'a> {
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
pub type WS8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS8W<'a> {
    w: &'a mut W,
}
impl<'a> _WS8W<'a> {
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
pub type WS9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS9W<'a> {
    w: &'a mut W,
}
impl<'a> _WS9W<'a> {
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
pub type WS10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS10W<'a> {
    w: &'a mut W,
}
impl<'a> _WS10W<'a> {
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
pub type WS11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS11W<'a> {
    w: &'a mut W,
}
impl<'a> _WS11W<'a> {
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
pub type WS12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS12W<'a> {
    w: &'a mut W,
}
impl<'a> _WS12W<'a> {
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
pub type WS13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS13W<'a> {
    w: &'a mut W,
}
impl<'a> _WS13W<'a> {
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
pub type WS14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS14W<'a> {
    w: &'a mut W,
}
impl<'a> _WS14W<'a> {
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
pub type WS15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS15W<'a> {
    w: &'a mut W,
}
impl<'a> _WS15W<'a> {
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
pub type WS16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS16W<'a> {
    w: &'a mut W,
}
impl<'a> _WS16W<'a> {
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
pub type WS17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS17W<'a> {
    w: &'a mut W,
}
impl<'a> _WS17W<'a> {
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
pub type WS18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS18W<'a> {
    w: &'a mut W,
}
impl<'a> _WS18W<'a> {
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
pub type WS19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS19W<'a> {
    w: &'a mut W,
}
impl<'a> _WS19W<'a> {
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
pub type WS20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS20W<'a> {
    w: &'a mut W,
}
impl<'a> _WS20W<'a> {
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
pub type WS21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS21W<'a> {
    w: &'a mut W,
}
impl<'a> _WS21W<'a> {
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
pub type WS22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS22W<'a> {
    w: &'a mut W,
}
impl<'a> _WS22W<'a> {
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
pub type WS23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WS23W<'a> {
    w: &'a mut W,
}
impl<'a> _WS23W<'a> {
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
    #[doc = "Bit 0 - XDMAC Channel 0 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws0(&self) -> WS0_R {
        WS0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws1(&self) -> WS1_R {
        WS1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws2(&self) -> WS2_R {
        WS2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws3(&self) -> WS3_R {
        WS3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws4(&self) -> WS4_R {
        WS4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws5(&self) -> WS5_R {
        WS5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws6(&self) -> WS6_R {
        WS6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws7(&self) -> WS7_R {
        WS7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws8(&self) -> WS8_R {
        WS8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws9(&self) -> WS9_R {
        WS9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws10(&self) -> WS10_R {
        WS10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws11(&self) -> WS11_R {
        WS11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws12(&self) -> WS12_R {
        WS12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws13(&self) -> WS13_R {
        WS13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws14(&self) -> WS14_R {
        WS14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws15(&self) -> WS15_R {
        WS15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws16(&self) -> WS16_R {
        WS16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws17(&self) -> WS17_R {
        WS17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws18(&self) -> WS18_R {
        WS18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws19(&self) -> WS19_R {
        WS19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws20(&self) -> WS20_R {
        WS20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws21(&self) -> WS21_R {
        WS21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws22(&self) -> WS22_R {
        WS22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws23(&self) -> WS23_R {
        WS23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - XDMAC Channel 0 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws0(&mut self) -> _WS0W {
        _WS0W { w: self }
    }
    #[doc = "Bit 1 - XDMAC Channel 1 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws1(&mut self) -> _WS1W {
        _WS1W { w: self }
    }
    #[doc = "Bit 2 - XDMAC Channel 2 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws2(&mut self) -> _WS2W {
        _WS2W { w: self }
    }
    #[doc = "Bit 3 - XDMAC Channel 3 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws3(&mut self) -> _WS3W {
        _WS3W { w: self }
    }
    #[doc = "Bit 4 - XDMAC Channel 4 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws4(&mut self) -> _WS4W {
        _WS4W { w: self }
    }
    #[doc = "Bit 5 - XDMAC Channel 5 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws5(&mut self) -> _WS5W {
        _WS5W { w: self }
    }
    #[doc = "Bit 6 - XDMAC Channel 6 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws6(&mut self) -> _WS6W {
        _WS6W { w: self }
    }
    #[doc = "Bit 7 - XDMAC Channel 7 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws7(&mut self) -> _WS7W {
        _WS7W { w: self }
    }
    #[doc = "Bit 8 - XDMAC Channel 8 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws8(&mut self) -> _WS8W {
        _WS8W { w: self }
    }
    #[doc = "Bit 9 - XDMAC Channel 9 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws9(&mut self) -> _WS9W {
        _WS9W { w: self }
    }
    #[doc = "Bit 10 - XDMAC Channel 10 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws10(&mut self) -> _WS10W {
        _WS10W { w: self }
    }
    #[doc = "Bit 11 - XDMAC Channel 11 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws11(&mut self) -> _WS11W {
        _WS11W { w: self }
    }
    #[doc = "Bit 12 - XDMAC Channel 12 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws12(&mut self) -> _WS12W {
        _WS12W { w: self }
    }
    #[doc = "Bit 13 - XDMAC Channel 13 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws13(&mut self) -> _WS13W {
        _WS13W { w: self }
    }
    #[doc = "Bit 14 - XDMAC Channel 14 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws14(&mut self) -> _WS14W {
        _WS14W { w: self }
    }
    #[doc = "Bit 15 - XDMAC Channel 15 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws15(&mut self) -> _WS15W {
        _WS15W { w: self }
    }
    #[doc = "Bit 16 - XDMAC Channel 16 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws16(&mut self) -> _WS16W {
        _WS16W { w: self }
    }
    #[doc = "Bit 17 - XDMAC Channel 17 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws17(&mut self) -> _WS17W {
        _WS17W { w: self }
    }
    #[doc = "Bit 18 - XDMAC Channel 18 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws18(&mut self) -> _WS18W {
        _WS18W { w: self }
    }
    #[doc = "Bit 19 - XDMAC Channel 19 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws19(&mut self) -> _WS19W {
        _WS19W { w: self }
    }
    #[doc = "Bit 20 - XDMAC Channel 20 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws20(&mut self) -> _WS20W {
        _WS20W { w: self }
    }
    #[doc = "Bit 21 - XDMAC Channel 21 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws21(&mut self) -> _WS21W {
        _WS21W { w: self }
    }
    #[doc = "Bit 22 - XDMAC Channel 22 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws22(&mut self) -> _WS22W {
        _WS22W { w: self }
    }
    #[doc = "Bit 23 - XDMAC Channel 23 Write Suspend Bit"]
    #[inline(always)]
    pub fn ws23(&mut self) -> _WS23W {
        _WS23W { w: self }
    }
}
