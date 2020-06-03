#[doc = "Reader of register LOCKBIT_WORD1"]
pub type R = crate::R<u32, super::LOCKBIT_WORD1>;
#[doc = "Writer for register LOCKBIT_WORD1"]
pub type W = crate::W<u32, super::LOCKBIT_WORD1>;
#[doc = "Register LOCKBIT_WORD1 `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCKBIT_WORD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK_REGION_32`"]
pub type LOCK_REGION_32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_32`"]
pub struct LOCK_REGION_32_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_32_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_33`"]
pub type LOCK_REGION_33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_33`"]
pub struct LOCK_REGION_33_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_33_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_34`"]
pub type LOCK_REGION_34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_34`"]
pub struct LOCK_REGION_34_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_34_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_35`"]
pub type LOCK_REGION_35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_35`"]
pub struct LOCK_REGION_35_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_35_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_36`"]
pub type LOCK_REGION_36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_36`"]
pub struct LOCK_REGION_36_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_36_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_37`"]
pub type LOCK_REGION_37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_37`"]
pub struct LOCK_REGION_37_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_37_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_38`"]
pub type LOCK_REGION_38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_38`"]
pub struct LOCK_REGION_38_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_38_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_39`"]
pub type LOCK_REGION_39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_39`"]
pub struct LOCK_REGION_39_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_39_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_40`"]
pub type LOCK_REGION_40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_40`"]
pub struct LOCK_REGION_40_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_40_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_41`"]
pub type LOCK_REGION_41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_41`"]
pub struct LOCK_REGION_41_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_41_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_42`"]
pub type LOCK_REGION_42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_42`"]
pub struct LOCK_REGION_42_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_42_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_43`"]
pub type LOCK_REGION_43_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_43`"]
pub struct LOCK_REGION_43_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_43_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_44`"]
pub type LOCK_REGION_44_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_44`"]
pub struct LOCK_REGION_44_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_44_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_45`"]
pub type LOCK_REGION_45_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_45`"]
pub struct LOCK_REGION_45_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_45_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_46`"]
pub type LOCK_REGION_46_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_46`"]
pub struct LOCK_REGION_46_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_46_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_47`"]
pub type LOCK_REGION_47_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_47`"]
pub struct LOCK_REGION_47_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_47_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_48`"]
pub type LOCK_REGION_48_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_48`"]
pub struct LOCK_REGION_48_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_48_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_49`"]
pub type LOCK_REGION_49_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_49`"]
pub struct LOCK_REGION_49_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_49_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_50`"]
pub type LOCK_REGION_50_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_50`"]
pub struct LOCK_REGION_50_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_50_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_51`"]
pub type LOCK_REGION_51_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_51`"]
pub struct LOCK_REGION_51_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_51_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_52`"]
pub type LOCK_REGION_52_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_52`"]
pub struct LOCK_REGION_52_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_52_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_53`"]
pub type LOCK_REGION_53_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_53`"]
pub struct LOCK_REGION_53_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_53_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_54`"]
pub type LOCK_REGION_54_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_54`"]
pub struct LOCK_REGION_54_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_54_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_55`"]
pub type LOCK_REGION_55_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_55`"]
pub struct LOCK_REGION_55_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_55_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_56`"]
pub type LOCK_REGION_56_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_56`"]
pub struct LOCK_REGION_56_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_56_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_57`"]
pub type LOCK_REGION_57_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_57`"]
pub struct LOCK_REGION_57_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_57_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_58`"]
pub type LOCK_REGION_58_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_58`"]
pub struct LOCK_REGION_58_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_58_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_59`"]
pub type LOCK_REGION_59_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_59`"]
pub struct LOCK_REGION_59_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_59_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_60`"]
pub type LOCK_REGION_60_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_60`"]
pub struct LOCK_REGION_60_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_60_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_61`"]
pub type LOCK_REGION_61_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_61`"]
pub struct LOCK_REGION_61_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_61_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_62`"]
pub type LOCK_REGION_62_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_62`"]
pub struct LOCK_REGION_62_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_62_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_63`"]
pub type LOCK_REGION_63_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_63`"]
pub struct LOCK_REGION_63_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_63_W<'a> {
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
    #[doc = "Bit 0 - Lock Region 32"]
    #[inline(always)]
    pub fn lock_region_32(&self) -> LOCK_REGION_32_R {
        LOCK_REGION_32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 33"]
    #[inline(always)]
    pub fn lock_region_33(&self) -> LOCK_REGION_33_R {
        LOCK_REGION_33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 34"]
    #[inline(always)]
    pub fn lock_region_34(&self) -> LOCK_REGION_34_R {
        LOCK_REGION_34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 35"]
    #[inline(always)]
    pub fn lock_region_35(&self) -> LOCK_REGION_35_R {
        LOCK_REGION_35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 36"]
    #[inline(always)]
    pub fn lock_region_36(&self) -> LOCK_REGION_36_R {
        LOCK_REGION_36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 37"]
    #[inline(always)]
    pub fn lock_region_37(&self) -> LOCK_REGION_37_R {
        LOCK_REGION_37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 38"]
    #[inline(always)]
    pub fn lock_region_38(&self) -> LOCK_REGION_38_R {
        LOCK_REGION_38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 39"]
    #[inline(always)]
    pub fn lock_region_39(&self) -> LOCK_REGION_39_R {
        LOCK_REGION_39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 40"]
    #[inline(always)]
    pub fn lock_region_40(&self) -> LOCK_REGION_40_R {
        LOCK_REGION_40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 41"]
    #[inline(always)]
    pub fn lock_region_41(&self) -> LOCK_REGION_41_R {
        LOCK_REGION_41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 42"]
    #[inline(always)]
    pub fn lock_region_42(&self) -> LOCK_REGION_42_R {
        LOCK_REGION_42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 43"]
    #[inline(always)]
    pub fn lock_region_43(&self) -> LOCK_REGION_43_R {
        LOCK_REGION_43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 44"]
    #[inline(always)]
    pub fn lock_region_44(&self) -> LOCK_REGION_44_R {
        LOCK_REGION_44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 45"]
    #[inline(always)]
    pub fn lock_region_45(&self) -> LOCK_REGION_45_R {
        LOCK_REGION_45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 46"]
    #[inline(always)]
    pub fn lock_region_46(&self) -> LOCK_REGION_46_R {
        LOCK_REGION_46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 47"]
    #[inline(always)]
    pub fn lock_region_47(&self) -> LOCK_REGION_47_R {
        LOCK_REGION_47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 48"]
    #[inline(always)]
    pub fn lock_region_48(&self) -> LOCK_REGION_48_R {
        LOCK_REGION_48_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 49"]
    #[inline(always)]
    pub fn lock_region_49(&self) -> LOCK_REGION_49_R {
        LOCK_REGION_49_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 50"]
    #[inline(always)]
    pub fn lock_region_50(&self) -> LOCK_REGION_50_R {
        LOCK_REGION_50_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 51"]
    #[inline(always)]
    pub fn lock_region_51(&self) -> LOCK_REGION_51_R {
        LOCK_REGION_51_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 52"]
    #[inline(always)]
    pub fn lock_region_52(&self) -> LOCK_REGION_52_R {
        LOCK_REGION_52_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 53"]
    #[inline(always)]
    pub fn lock_region_53(&self) -> LOCK_REGION_53_R {
        LOCK_REGION_53_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 54"]
    #[inline(always)]
    pub fn lock_region_54(&self) -> LOCK_REGION_54_R {
        LOCK_REGION_54_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 55"]
    #[inline(always)]
    pub fn lock_region_55(&self) -> LOCK_REGION_55_R {
        LOCK_REGION_55_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 56"]
    #[inline(always)]
    pub fn lock_region_56(&self) -> LOCK_REGION_56_R {
        LOCK_REGION_56_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 57"]
    #[inline(always)]
    pub fn lock_region_57(&self) -> LOCK_REGION_57_R {
        LOCK_REGION_57_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 58"]
    #[inline(always)]
    pub fn lock_region_58(&self) -> LOCK_REGION_58_R {
        LOCK_REGION_58_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 59"]
    #[inline(always)]
    pub fn lock_region_59(&self) -> LOCK_REGION_59_R {
        LOCK_REGION_59_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 60"]
    #[inline(always)]
    pub fn lock_region_60(&self) -> LOCK_REGION_60_R {
        LOCK_REGION_60_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 61"]
    #[inline(always)]
    pub fn lock_region_61(&self) -> LOCK_REGION_61_R {
        LOCK_REGION_61_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 62"]
    #[inline(always)]
    pub fn lock_region_62(&self) -> LOCK_REGION_62_R {
        LOCK_REGION_62_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 63"]
    #[inline(always)]
    pub fn lock_region_63(&self) -> LOCK_REGION_63_R {
        LOCK_REGION_63_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Region 32"]
    #[inline(always)]
    pub fn lock_region_32(&mut self) -> LOCK_REGION_32_W {
        LOCK_REGION_32_W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 33"]
    #[inline(always)]
    pub fn lock_region_33(&mut self) -> LOCK_REGION_33_W {
        LOCK_REGION_33_W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 34"]
    #[inline(always)]
    pub fn lock_region_34(&mut self) -> LOCK_REGION_34_W {
        LOCK_REGION_34_W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 35"]
    #[inline(always)]
    pub fn lock_region_35(&mut self) -> LOCK_REGION_35_W {
        LOCK_REGION_35_W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 36"]
    #[inline(always)]
    pub fn lock_region_36(&mut self) -> LOCK_REGION_36_W {
        LOCK_REGION_36_W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 37"]
    #[inline(always)]
    pub fn lock_region_37(&mut self) -> LOCK_REGION_37_W {
        LOCK_REGION_37_W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 38"]
    #[inline(always)]
    pub fn lock_region_38(&mut self) -> LOCK_REGION_38_W {
        LOCK_REGION_38_W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 39"]
    #[inline(always)]
    pub fn lock_region_39(&mut self) -> LOCK_REGION_39_W {
        LOCK_REGION_39_W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 40"]
    #[inline(always)]
    pub fn lock_region_40(&mut self) -> LOCK_REGION_40_W {
        LOCK_REGION_40_W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 41"]
    #[inline(always)]
    pub fn lock_region_41(&mut self) -> LOCK_REGION_41_W {
        LOCK_REGION_41_W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 42"]
    #[inline(always)]
    pub fn lock_region_42(&mut self) -> LOCK_REGION_42_W {
        LOCK_REGION_42_W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 43"]
    #[inline(always)]
    pub fn lock_region_43(&mut self) -> LOCK_REGION_43_W {
        LOCK_REGION_43_W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 44"]
    #[inline(always)]
    pub fn lock_region_44(&mut self) -> LOCK_REGION_44_W {
        LOCK_REGION_44_W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 45"]
    #[inline(always)]
    pub fn lock_region_45(&mut self) -> LOCK_REGION_45_W {
        LOCK_REGION_45_W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 46"]
    #[inline(always)]
    pub fn lock_region_46(&mut self) -> LOCK_REGION_46_W {
        LOCK_REGION_46_W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 47"]
    #[inline(always)]
    pub fn lock_region_47(&mut self) -> LOCK_REGION_47_W {
        LOCK_REGION_47_W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 48"]
    #[inline(always)]
    pub fn lock_region_48(&mut self) -> LOCK_REGION_48_W {
        LOCK_REGION_48_W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 49"]
    #[inline(always)]
    pub fn lock_region_49(&mut self) -> LOCK_REGION_49_W {
        LOCK_REGION_49_W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 50"]
    #[inline(always)]
    pub fn lock_region_50(&mut self) -> LOCK_REGION_50_W {
        LOCK_REGION_50_W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 51"]
    #[inline(always)]
    pub fn lock_region_51(&mut self) -> LOCK_REGION_51_W {
        LOCK_REGION_51_W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 52"]
    #[inline(always)]
    pub fn lock_region_52(&mut self) -> LOCK_REGION_52_W {
        LOCK_REGION_52_W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 53"]
    #[inline(always)]
    pub fn lock_region_53(&mut self) -> LOCK_REGION_53_W {
        LOCK_REGION_53_W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 54"]
    #[inline(always)]
    pub fn lock_region_54(&mut self) -> LOCK_REGION_54_W {
        LOCK_REGION_54_W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 55"]
    #[inline(always)]
    pub fn lock_region_55(&mut self) -> LOCK_REGION_55_W {
        LOCK_REGION_55_W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 56"]
    #[inline(always)]
    pub fn lock_region_56(&mut self) -> LOCK_REGION_56_W {
        LOCK_REGION_56_W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 57"]
    #[inline(always)]
    pub fn lock_region_57(&mut self) -> LOCK_REGION_57_W {
        LOCK_REGION_57_W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 58"]
    #[inline(always)]
    pub fn lock_region_58(&mut self) -> LOCK_REGION_58_W {
        LOCK_REGION_58_W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 59"]
    #[inline(always)]
    pub fn lock_region_59(&mut self) -> LOCK_REGION_59_W {
        LOCK_REGION_59_W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 60"]
    #[inline(always)]
    pub fn lock_region_60(&mut self) -> LOCK_REGION_60_W {
        LOCK_REGION_60_W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 61"]
    #[inline(always)]
    pub fn lock_region_61(&mut self) -> LOCK_REGION_61_W {
        LOCK_REGION_61_W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 62"]
    #[inline(always)]
    pub fn lock_region_62(&mut self) -> LOCK_REGION_62_W {
        LOCK_REGION_62_W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 63"]
    #[inline(always)]
    pub fn lock_region_63(&mut self) -> LOCK_REGION_63_W {
        LOCK_REGION_63_W { w: self }
    }
}
