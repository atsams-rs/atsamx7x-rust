#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIO_SCHMITT {
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
pub type SCHMITT0_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT0W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT0W<'a> {
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
pub type SCHMITT1_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT1W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT1W<'a> {
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
pub type SCHMITT2_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT2W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT2W<'a> {
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
pub type SCHMITT3_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT3W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT3W<'a> {
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
pub type SCHMITT4_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT4W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT4W<'a> {
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
pub type SCHMITT5_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT5W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT5W<'a> {
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
pub type SCHMITT6_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT6W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT6W<'a> {
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
pub type SCHMITT7_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT7W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT7W<'a> {
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
pub type SCHMITT8_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT8W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT8W<'a> {
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
pub type SCHMITT9_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT9W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT9W<'a> {
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
pub type SCHMITT10_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT10W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT10W<'a> {
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
pub type SCHMITT11_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT11W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT11W<'a> {
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
pub type SCHMITT12_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT12W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT12W<'a> {
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
pub type SCHMITT13_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT13W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT13W<'a> {
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
pub type SCHMITT14_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT14W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT14W<'a> {
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
pub type SCHMITT15_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT15W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT15W<'a> {
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
pub type SCHMITT16_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT16W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT16W<'a> {
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
pub type SCHMITT17_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT17W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT17W<'a> {
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
pub type SCHMITT18_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT18W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT18W<'a> {
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
pub type SCHMITT19_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT19W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT19W<'a> {
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
pub type SCHMITT20_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT20W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT20W<'a> {
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
pub type SCHMITT21_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT21W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT21W<'a> {
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
pub type SCHMITT22_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT22W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT22W<'a> {
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
pub type SCHMITT23_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT23W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT23W<'a> {
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
pub type SCHMITT24_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT24W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT24W<'a> {
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
pub type SCHMITT25_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT25W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT25W<'a> {
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
#[doc = r"Reader of the field"]
pub type SCHMITT26_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT26W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT26W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SCHMITT27_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT27W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT27W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SCHMITT28_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT28W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT28W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SCHMITT29_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT29W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT29W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SCHMITT30_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT30W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT30W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type SCHMITT31_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SCHMITT31W<'a> {
    w: &'a mut W,
}
impl<'a> _SCHMITT31W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&self) -> SCHMITT0_R {
        SCHMITT0_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&self) -> SCHMITT1_R {
        SCHMITT1_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&self) -> SCHMITT2_R {
        SCHMITT2_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&self) -> SCHMITT3_R {
        SCHMITT3_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&self) -> SCHMITT4_R {
        SCHMITT4_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&self) -> SCHMITT5_R {
        SCHMITT5_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&self) -> SCHMITT6_R {
        SCHMITT6_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&self) -> SCHMITT7_R {
        SCHMITT7_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&self) -> SCHMITT8_R {
        SCHMITT8_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&self) -> SCHMITT9_R {
        SCHMITT9_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&self) -> SCHMITT10_R {
        SCHMITT10_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&self) -> SCHMITT11_R {
        SCHMITT11_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&self) -> SCHMITT12_R {
        SCHMITT12_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&self) -> SCHMITT13_R {
        SCHMITT13_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&self) -> SCHMITT14_R {
        SCHMITT14_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&self) -> SCHMITT15_R {
        SCHMITT15_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&self) -> SCHMITT16_R {
        SCHMITT16_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&self) -> SCHMITT17_R {
        SCHMITT17_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&self) -> SCHMITT18_R {
        SCHMITT18_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&self) -> SCHMITT19_R {
        SCHMITT19_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&self) -> SCHMITT20_R {
        SCHMITT20_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&self) -> SCHMITT21_R {
        SCHMITT21_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&self) -> SCHMITT22_R {
        SCHMITT22_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&self) -> SCHMITT23_R {
        SCHMITT23_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&self) -> SCHMITT24_R {
        SCHMITT24_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&self) -> SCHMITT25_R {
        SCHMITT25_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&self) -> SCHMITT26_R {
        SCHMITT26_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&self) -> SCHMITT27_R {
        SCHMITT27_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&self) -> SCHMITT28_R {
        SCHMITT28_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&self) -> SCHMITT29_R {
        SCHMITT29_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&self) -> SCHMITT30_R {
        SCHMITT30_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&self) -> SCHMITT31_R {
        SCHMITT31_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt0(&mut self) -> _SCHMITT0W {
        _SCHMITT0W { w: self }
    }
    #[doc = "Bit 1 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt1(&mut self) -> _SCHMITT1W {
        _SCHMITT1W { w: self }
    }
    #[doc = "Bit 2 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt2(&mut self) -> _SCHMITT2W {
        _SCHMITT2W { w: self }
    }
    #[doc = "Bit 3 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt3(&mut self) -> _SCHMITT3W {
        _SCHMITT3W { w: self }
    }
    #[doc = "Bit 4 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt4(&mut self) -> _SCHMITT4W {
        _SCHMITT4W { w: self }
    }
    #[doc = "Bit 5 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt5(&mut self) -> _SCHMITT5W {
        _SCHMITT5W { w: self }
    }
    #[doc = "Bit 6 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt6(&mut self) -> _SCHMITT6W {
        _SCHMITT6W { w: self }
    }
    #[doc = "Bit 7 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt7(&mut self) -> _SCHMITT7W {
        _SCHMITT7W { w: self }
    }
    #[doc = "Bit 8 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt8(&mut self) -> _SCHMITT8W {
        _SCHMITT8W { w: self }
    }
    #[doc = "Bit 9 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt9(&mut self) -> _SCHMITT9W {
        _SCHMITT9W { w: self }
    }
    #[doc = "Bit 10 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt10(&mut self) -> _SCHMITT10W {
        _SCHMITT10W { w: self }
    }
    #[doc = "Bit 11 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt11(&mut self) -> _SCHMITT11W {
        _SCHMITT11W { w: self }
    }
    #[doc = "Bit 12 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt12(&mut self) -> _SCHMITT12W {
        _SCHMITT12W { w: self }
    }
    #[doc = "Bit 13 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt13(&mut self) -> _SCHMITT13W {
        _SCHMITT13W { w: self }
    }
    #[doc = "Bit 14 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt14(&mut self) -> _SCHMITT14W {
        _SCHMITT14W { w: self }
    }
    #[doc = "Bit 15 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt15(&mut self) -> _SCHMITT15W {
        _SCHMITT15W { w: self }
    }
    #[doc = "Bit 16 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt16(&mut self) -> _SCHMITT16W {
        _SCHMITT16W { w: self }
    }
    #[doc = "Bit 17 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt17(&mut self) -> _SCHMITT17W {
        _SCHMITT17W { w: self }
    }
    #[doc = "Bit 18 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt18(&mut self) -> _SCHMITT18W {
        _SCHMITT18W { w: self }
    }
    #[doc = "Bit 19 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt19(&mut self) -> _SCHMITT19W {
        _SCHMITT19W { w: self }
    }
    #[doc = "Bit 20 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt20(&mut self) -> _SCHMITT20W {
        _SCHMITT20W { w: self }
    }
    #[doc = "Bit 21 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt21(&mut self) -> _SCHMITT21W {
        _SCHMITT21W { w: self }
    }
    #[doc = "Bit 22 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt22(&mut self) -> _SCHMITT22W {
        _SCHMITT22W { w: self }
    }
    #[doc = "Bit 23 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt23(&mut self) -> _SCHMITT23W {
        _SCHMITT23W { w: self }
    }
    #[doc = "Bit 24 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt24(&mut self) -> _SCHMITT24W {
        _SCHMITT24W { w: self }
    }
    #[doc = "Bit 25 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt25(&mut self) -> _SCHMITT25W {
        _SCHMITT25W { w: self }
    }
    #[doc = "Bit 26 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt26(&mut self) -> _SCHMITT26W {
        _SCHMITT26W { w: self }
    }
    #[doc = "Bit 27 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt27(&mut self) -> _SCHMITT27W {
        _SCHMITT27W { w: self }
    }
    #[doc = "Bit 28 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt28(&mut self) -> _SCHMITT28W {
        _SCHMITT28W { w: self }
    }
    #[doc = "Bit 29 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt29(&mut self) -> _SCHMITT29W {
        _SCHMITT29W { w: self }
    }
    #[doc = "Bit 30 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt30(&mut self) -> _SCHMITT30W {
        _SCHMITT30W { w: self }
    }
    #[doc = "Bit 31 - Schmitt Trigger Control"]
    #[inline(always)]
    pub fn schmitt31(&mut self) -> _SCHMITT31W {
        _SCHMITT31W { w: self }
    }
}
