#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LOCKBIT_WORD1 {
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
pub type LOCK_REGION_32_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_32W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_32W<'a> {
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
pub type LOCK_REGION_33_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_33W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_33W<'a> {
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
pub type LOCK_REGION_34_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_34W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_34W<'a> {
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
pub type LOCK_REGION_35_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_35W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_35W<'a> {
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
pub type LOCK_REGION_36_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_36W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_36W<'a> {
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
pub type LOCK_REGION_37_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_37W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_37W<'a> {
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
pub type LOCK_REGION_38_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_38W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_38W<'a> {
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
pub type LOCK_REGION_39_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_39W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_39W<'a> {
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
pub type LOCK_REGION_40_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_40W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_40W<'a> {
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
pub type LOCK_REGION_41_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_41W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_41W<'a> {
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
pub type LOCK_REGION_42_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_42W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_42W<'a> {
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
pub type LOCK_REGION_43_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_43W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_43W<'a> {
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
pub type LOCK_REGION_44_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_44W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_44W<'a> {
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
pub type LOCK_REGION_45_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_45W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_45W<'a> {
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
pub type LOCK_REGION_46_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_46W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_46W<'a> {
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
pub type LOCK_REGION_47_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_47W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_47W<'a> {
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
pub type LOCK_REGION_48_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_48W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_48W<'a> {
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
pub type LOCK_REGION_49_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_49W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_49W<'a> {
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
pub type LOCK_REGION_50_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_50W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_50W<'a> {
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
pub type LOCK_REGION_51_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_51W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_51W<'a> {
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
pub type LOCK_REGION_52_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_52W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_52W<'a> {
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
pub type LOCK_REGION_53_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_53W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_53W<'a> {
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
pub type LOCK_REGION_54_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_54W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_54W<'a> {
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
pub type LOCK_REGION_55_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_55W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_55W<'a> {
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
pub type LOCK_REGION_56_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_56W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_56W<'a> {
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
pub type LOCK_REGION_57_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_57W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_57W<'a> {
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
pub type LOCK_REGION_58_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_58W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_58W<'a> {
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
pub type LOCK_REGION_59_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_59W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_59W<'a> {
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
pub type LOCK_REGION_60_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_60W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_60W<'a> {
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
pub type LOCK_REGION_61_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_61W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_61W<'a> {
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
pub type LOCK_REGION_62_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_62W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_62W<'a> {
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
pub type LOCK_REGION_63_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LOCK_REGION_63W<'a> {
    w: &'a mut W,
}
impl<'a> _LOCK_REGION_63W<'a> {
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
    #[doc = "Bit 0 - Lock Region 32"]
    #[inline(always)]
    pub fn lock_region_32(&self) -> LOCK_REGION_32_R {
        LOCK_REGION_32_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 33"]
    #[inline(always)]
    pub fn lock_region_33(&self) -> LOCK_REGION_33_R {
        LOCK_REGION_33_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 34"]
    #[inline(always)]
    pub fn lock_region_34(&self) -> LOCK_REGION_34_R {
        LOCK_REGION_34_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 35"]
    #[inline(always)]
    pub fn lock_region_35(&self) -> LOCK_REGION_35_R {
        LOCK_REGION_35_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 36"]
    #[inline(always)]
    pub fn lock_region_36(&self) -> LOCK_REGION_36_R {
        LOCK_REGION_36_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 37"]
    #[inline(always)]
    pub fn lock_region_37(&self) -> LOCK_REGION_37_R {
        LOCK_REGION_37_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 38"]
    #[inline(always)]
    pub fn lock_region_38(&self) -> LOCK_REGION_38_R {
        LOCK_REGION_38_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 39"]
    #[inline(always)]
    pub fn lock_region_39(&self) -> LOCK_REGION_39_R {
        LOCK_REGION_39_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 40"]
    #[inline(always)]
    pub fn lock_region_40(&self) -> LOCK_REGION_40_R {
        LOCK_REGION_40_R::new(((self.bits() >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 41"]
    #[inline(always)]
    pub fn lock_region_41(&self) -> LOCK_REGION_41_R {
        LOCK_REGION_41_R::new(((self.bits() >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 42"]
    #[inline(always)]
    pub fn lock_region_42(&self) -> LOCK_REGION_42_R {
        LOCK_REGION_42_R::new(((self.bits() >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 43"]
    #[inline(always)]
    pub fn lock_region_43(&self) -> LOCK_REGION_43_R {
        LOCK_REGION_43_R::new(((self.bits() >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 44"]
    #[inline(always)]
    pub fn lock_region_44(&self) -> LOCK_REGION_44_R {
        LOCK_REGION_44_R::new(((self.bits() >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 45"]
    #[inline(always)]
    pub fn lock_region_45(&self) -> LOCK_REGION_45_R {
        LOCK_REGION_45_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 46"]
    #[inline(always)]
    pub fn lock_region_46(&self) -> LOCK_REGION_46_R {
        LOCK_REGION_46_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 47"]
    #[inline(always)]
    pub fn lock_region_47(&self) -> LOCK_REGION_47_R {
        LOCK_REGION_47_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 48"]
    #[inline(always)]
    pub fn lock_region_48(&self) -> LOCK_REGION_48_R {
        LOCK_REGION_48_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 49"]
    #[inline(always)]
    pub fn lock_region_49(&self) -> LOCK_REGION_49_R {
        LOCK_REGION_49_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 50"]
    #[inline(always)]
    pub fn lock_region_50(&self) -> LOCK_REGION_50_R {
        LOCK_REGION_50_R::new(((self.bits() >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 51"]
    #[inline(always)]
    pub fn lock_region_51(&self) -> LOCK_REGION_51_R {
        LOCK_REGION_51_R::new(((self.bits() >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 52"]
    #[inline(always)]
    pub fn lock_region_52(&self) -> LOCK_REGION_52_R {
        LOCK_REGION_52_R::new(((self.bits() >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 53"]
    #[inline(always)]
    pub fn lock_region_53(&self) -> LOCK_REGION_53_R {
        LOCK_REGION_53_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 54"]
    #[inline(always)]
    pub fn lock_region_54(&self) -> LOCK_REGION_54_R {
        LOCK_REGION_54_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 55"]
    #[inline(always)]
    pub fn lock_region_55(&self) -> LOCK_REGION_55_R {
        LOCK_REGION_55_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 56"]
    #[inline(always)]
    pub fn lock_region_56(&self) -> LOCK_REGION_56_R {
        LOCK_REGION_56_R::new(((self.bits() >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 57"]
    #[inline(always)]
    pub fn lock_region_57(&self) -> LOCK_REGION_57_R {
        LOCK_REGION_57_R::new(((self.bits() >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 58"]
    #[inline(always)]
    pub fn lock_region_58(&self) -> LOCK_REGION_58_R {
        LOCK_REGION_58_R::new(((self.bits() >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 59"]
    #[inline(always)]
    pub fn lock_region_59(&self) -> LOCK_REGION_59_R {
        LOCK_REGION_59_R::new(((self.bits() >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 60"]
    #[inline(always)]
    pub fn lock_region_60(&self) -> LOCK_REGION_60_R {
        LOCK_REGION_60_R::new(((self.bits() >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 61"]
    #[inline(always)]
    pub fn lock_region_61(&self) -> LOCK_REGION_61_R {
        LOCK_REGION_61_R::new(((self.bits() >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 62"]
    #[inline(always)]
    pub fn lock_region_62(&self) -> LOCK_REGION_62_R {
        LOCK_REGION_62_R::new(((self.bits() >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 63"]
    #[inline(always)]
    pub fn lock_region_63(&self) -> LOCK_REGION_63_R {
        LOCK_REGION_63_R::new(((self.bits() >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Lock Region 32"]
    #[inline(always)]
    pub fn lock_region_32(&mut self) -> _LOCK_REGION_32W {
        _LOCK_REGION_32W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 33"]
    #[inline(always)]
    pub fn lock_region_33(&mut self) -> _LOCK_REGION_33W {
        _LOCK_REGION_33W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 34"]
    #[inline(always)]
    pub fn lock_region_34(&mut self) -> _LOCK_REGION_34W {
        _LOCK_REGION_34W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 35"]
    #[inline(always)]
    pub fn lock_region_35(&mut self) -> _LOCK_REGION_35W {
        _LOCK_REGION_35W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 36"]
    #[inline(always)]
    pub fn lock_region_36(&mut self) -> _LOCK_REGION_36W {
        _LOCK_REGION_36W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 37"]
    #[inline(always)]
    pub fn lock_region_37(&mut self) -> _LOCK_REGION_37W {
        _LOCK_REGION_37W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 38"]
    #[inline(always)]
    pub fn lock_region_38(&mut self) -> _LOCK_REGION_38W {
        _LOCK_REGION_38W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 39"]
    #[inline(always)]
    pub fn lock_region_39(&mut self) -> _LOCK_REGION_39W {
        _LOCK_REGION_39W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 40"]
    #[inline(always)]
    pub fn lock_region_40(&mut self) -> _LOCK_REGION_40W {
        _LOCK_REGION_40W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 41"]
    #[inline(always)]
    pub fn lock_region_41(&mut self) -> _LOCK_REGION_41W {
        _LOCK_REGION_41W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 42"]
    #[inline(always)]
    pub fn lock_region_42(&mut self) -> _LOCK_REGION_42W {
        _LOCK_REGION_42W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 43"]
    #[inline(always)]
    pub fn lock_region_43(&mut self) -> _LOCK_REGION_43W {
        _LOCK_REGION_43W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 44"]
    #[inline(always)]
    pub fn lock_region_44(&mut self) -> _LOCK_REGION_44W {
        _LOCK_REGION_44W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 45"]
    #[inline(always)]
    pub fn lock_region_45(&mut self) -> _LOCK_REGION_45W {
        _LOCK_REGION_45W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 46"]
    #[inline(always)]
    pub fn lock_region_46(&mut self) -> _LOCK_REGION_46W {
        _LOCK_REGION_46W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 47"]
    #[inline(always)]
    pub fn lock_region_47(&mut self) -> _LOCK_REGION_47W {
        _LOCK_REGION_47W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 48"]
    #[inline(always)]
    pub fn lock_region_48(&mut self) -> _LOCK_REGION_48W {
        _LOCK_REGION_48W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 49"]
    #[inline(always)]
    pub fn lock_region_49(&mut self) -> _LOCK_REGION_49W {
        _LOCK_REGION_49W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 50"]
    #[inline(always)]
    pub fn lock_region_50(&mut self) -> _LOCK_REGION_50W {
        _LOCK_REGION_50W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 51"]
    #[inline(always)]
    pub fn lock_region_51(&mut self) -> _LOCK_REGION_51W {
        _LOCK_REGION_51W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 52"]
    #[inline(always)]
    pub fn lock_region_52(&mut self) -> _LOCK_REGION_52W {
        _LOCK_REGION_52W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 53"]
    #[inline(always)]
    pub fn lock_region_53(&mut self) -> _LOCK_REGION_53W {
        _LOCK_REGION_53W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 54"]
    #[inline(always)]
    pub fn lock_region_54(&mut self) -> _LOCK_REGION_54W {
        _LOCK_REGION_54W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 55"]
    #[inline(always)]
    pub fn lock_region_55(&mut self) -> _LOCK_REGION_55W {
        _LOCK_REGION_55W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 56"]
    #[inline(always)]
    pub fn lock_region_56(&mut self) -> _LOCK_REGION_56W {
        _LOCK_REGION_56W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 57"]
    #[inline(always)]
    pub fn lock_region_57(&mut self) -> _LOCK_REGION_57W {
        _LOCK_REGION_57W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 58"]
    #[inline(always)]
    pub fn lock_region_58(&mut self) -> _LOCK_REGION_58W {
        _LOCK_REGION_58W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 59"]
    #[inline(always)]
    pub fn lock_region_59(&mut self) -> _LOCK_REGION_59W {
        _LOCK_REGION_59W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 60"]
    #[inline(always)]
    pub fn lock_region_60(&mut self) -> _LOCK_REGION_60W {
        _LOCK_REGION_60W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 61"]
    #[inline(always)]
    pub fn lock_region_61(&mut self) -> _LOCK_REGION_61W {
        _LOCK_REGION_61W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 62"]
    #[inline(always)]
    pub fn lock_region_62(&mut self) -> _LOCK_REGION_62W {
        _LOCK_REGION_62W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 63"]
    #[inline(always)]
    pub fn lock_region_63(&mut self) -> _LOCK_REGION_63W {
        _LOCK_REGION_63W { w: self }
    }
}
