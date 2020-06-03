#[doc = "Reader of register LOCKBIT_WORD2"]
pub type R = crate::R<u32, super::LOCKBIT_WORD2>;
#[doc = "Writer for register LOCKBIT_WORD2"]
pub type W = crate::W<u32, super::LOCKBIT_WORD2>;
#[doc = "Register LOCKBIT_WORD2 `reset()`'s with value 0"]
impl crate::ResetValue for super::LOCKBIT_WORD2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCK_REGION_64`"]
pub type LOCK_REGION_64_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_64`"]
pub struct LOCK_REGION_64_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_64_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_65`"]
pub type LOCK_REGION_65_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_65`"]
pub struct LOCK_REGION_65_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_65_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_66`"]
pub type LOCK_REGION_66_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_66`"]
pub struct LOCK_REGION_66_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_66_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_67`"]
pub type LOCK_REGION_67_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_67`"]
pub struct LOCK_REGION_67_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_67_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_68`"]
pub type LOCK_REGION_68_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_68`"]
pub struct LOCK_REGION_68_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_68_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_69`"]
pub type LOCK_REGION_69_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_69`"]
pub struct LOCK_REGION_69_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_69_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_70`"]
pub type LOCK_REGION_70_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_70`"]
pub struct LOCK_REGION_70_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_70_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_71`"]
pub type LOCK_REGION_71_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_71`"]
pub struct LOCK_REGION_71_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_71_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_72`"]
pub type LOCK_REGION_72_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_72`"]
pub struct LOCK_REGION_72_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_72_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_73`"]
pub type LOCK_REGION_73_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_73`"]
pub struct LOCK_REGION_73_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_73_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_74`"]
pub type LOCK_REGION_74_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_74`"]
pub struct LOCK_REGION_74_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_74_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_75`"]
pub type LOCK_REGION_75_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_75`"]
pub struct LOCK_REGION_75_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_75_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_76`"]
pub type LOCK_REGION_76_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_76`"]
pub struct LOCK_REGION_76_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_76_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_77`"]
pub type LOCK_REGION_77_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_77`"]
pub struct LOCK_REGION_77_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_77_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_78`"]
pub type LOCK_REGION_78_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_78`"]
pub struct LOCK_REGION_78_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_78_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_79`"]
pub type LOCK_REGION_79_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_79`"]
pub struct LOCK_REGION_79_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_79_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_80`"]
pub type LOCK_REGION_80_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_80`"]
pub struct LOCK_REGION_80_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_80_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_81`"]
pub type LOCK_REGION_81_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_81`"]
pub struct LOCK_REGION_81_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_81_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_82`"]
pub type LOCK_REGION_82_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_82`"]
pub struct LOCK_REGION_82_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_82_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_83`"]
pub type LOCK_REGION_83_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_83`"]
pub struct LOCK_REGION_83_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_83_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_84`"]
pub type LOCK_REGION_84_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_84`"]
pub struct LOCK_REGION_84_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_84_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_85`"]
pub type LOCK_REGION_85_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_85`"]
pub struct LOCK_REGION_85_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_85_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_86`"]
pub type LOCK_REGION_86_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_86`"]
pub struct LOCK_REGION_86_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_86_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_87`"]
pub type LOCK_REGION_87_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_87`"]
pub struct LOCK_REGION_87_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_87_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_88`"]
pub type LOCK_REGION_88_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_88`"]
pub struct LOCK_REGION_88_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_88_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_89`"]
pub type LOCK_REGION_89_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_89`"]
pub struct LOCK_REGION_89_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_89_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_90`"]
pub type LOCK_REGION_90_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_90`"]
pub struct LOCK_REGION_90_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_90_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_91`"]
pub type LOCK_REGION_91_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_91`"]
pub struct LOCK_REGION_91_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_91_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_92`"]
pub type LOCK_REGION_92_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_92`"]
pub struct LOCK_REGION_92_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_92_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_93`"]
pub type LOCK_REGION_93_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_93`"]
pub struct LOCK_REGION_93_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_93_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_94`"]
pub type LOCK_REGION_94_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_94`"]
pub struct LOCK_REGION_94_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_94_W<'a> {
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
#[doc = "Reader of field `LOCK_REGION_95`"]
pub type LOCK_REGION_95_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCK_REGION_95`"]
pub struct LOCK_REGION_95_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCK_REGION_95_W<'a> {
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
    #[doc = "Bit 0 - Lock Region 64"]
    #[inline(always)]
    pub fn lock_region_64(&self) -> LOCK_REGION_64_R {
        LOCK_REGION_64_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Lock Region 65"]
    #[inline(always)]
    pub fn lock_region_65(&self) -> LOCK_REGION_65_R {
        LOCK_REGION_65_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Lock Region 66"]
    #[inline(always)]
    pub fn lock_region_66(&self) -> LOCK_REGION_66_R {
        LOCK_REGION_66_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Lock Region 67"]
    #[inline(always)]
    pub fn lock_region_67(&self) -> LOCK_REGION_67_R {
        LOCK_REGION_67_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Lock Region 68"]
    #[inline(always)]
    pub fn lock_region_68(&self) -> LOCK_REGION_68_R {
        LOCK_REGION_68_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Lock Region 69"]
    #[inline(always)]
    pub fn lock_region_69(&self) -> LOCK_REGION_69_R {
        LOCK_REGION_69_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Lock Region 70"]
    #[inline(always)]
    pub fn lock_region_70(&self) -> LOCK_REGION_70_R {
        LOCK_REGION_70_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Lock Region 71"]
    #[inline(always)]
    pub fn lock_region_71(&self) -> LOCK_REGION_71_R {
        LOCK_REGION_71_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Lock Region 72"]
    #[inline(always)]
    pub fn lock_region_72(&self) -> LOCK_REGION_72_R {
        LOCK_REGION_72_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Lock Region 73"]
    #[inline(always)]
    pub fn lock_region_73(&self) -> LOCK_REGION_73_R {
        LOCK_REGION_73_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Lock Region 74"]
    #[inline(always)]
    pub fn lock_region_74(&self) -> LOCK_REGION_74_R {
        LOCK_REGION_74_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Lock Region 75"]
    #[inline(always)]
    pub fn lock_region_75(&self) -> LOCK_REGION_75_R {
        LOCK_REGION_75_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Lock Region 76"]
    #[inline(always)]
    pub fn lock_region_76(&self) -> LOCK_REGION_76_R {
        LOCK_REGION_76_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Lock Region 77"]
    #[inline(always)]
    pub fn lock_region_77(&self) -> LOCK_REGION_77_R {
        LOCK_REGION_77_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Lock Region 78"]
    #[inline(always)]
    pub fn lock_region_78(&self) -> LOCK_REGION_78_R {
        LOCK_REGION_78_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Lock Region 79"]
    #[inline(always)]
    pub fn lock_region_79(&self) -> LOCK_REGION_79_R {
        LOCK_REGION_79_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Lock Region 80"]
    #[inline(always)]
    pub fn lock_region_80(&self) -> LOCK_REGION_80_R {
        LOCK_REGION_80_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Lock Region 81"]
    #[inline(always)]
    pub fn lock_region_81(&self) -> LOCK_REGION_81_R {
        LOCK_REGION_81_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Lock Region 82"]
    #[inline(always)]
    pub fn lock_region_82(&self) -> LOCK_REGION_82_R {
        LOCK_REGION_82_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Lock Region 83"]
    #[inline(always)]
    pub fn lock_region_83(&self) -> LOCK_REGION_83_R {
        LOCK_REGION_83_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Lock Region 84"]
    #[inline(always)]
    pub fn lock_region_84(&self) -> LOCK_REGION_84_R {
        LOCK_REGION_84_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Lock Region 85"]
    #[inline(always)]
    pub fn lock_region_85(&self) -> LOCK_REGION_85_R {
        LOCK_REGION_85_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Lock Region 86"]
    #[inline(always)]
    pub fn lock_region_86(&self) -> LOCK_REGION_86_R {
        LOCK_REGION_86_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Lock Region 87"]
    #[inline(always)]
    pub fn lock_region_87(&self) -> LOCK_REGION_87_R {
        LOCK_REGION_87_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Lock Region 88"]
    #[inline(always)]
    pub fn lock_region_88(&self) -> LOCK_REGION_88_R {
        LOCK_REGION_88_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Lock Region 89"]
    #[inline(always)]
    pub fn lock_region_89(&self) -> LOCK_REGION_89_R {
        LOCK_REGION_89_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Lock Region 90"]
    #[inline(always)]
    pub fn lock_region_90(&self) -> LOCK_REGION_90_R {
        LOCK_REGION_90_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Lock Region 91"]
    #[inline(always)]
    pub fn lock_region_91(&self) -> LOCK_REGION_91_R {
        LOCK_REGION_91_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Lock Region 92"]
    #[inline(always)]
    pub fn lock_region_92(&self) -> LOCK_REGION_92_R {
        LOCK_REGION_92_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Lock Region 93"]
    #[inline(always)]
    pub fn lock_region_93(&self) -> LOCK_REGION_93_R {
        LOCK_REGION_93_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Lock Region 94"]
    #[inline(always)]
    pub fn lock_region_94(&self) -> LOCK_REGION_94_R {
        LOCK_REGION_94_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Lock Region 95"]
    #[inline(always)]
    pub fn lock_region_95(&self) -> LOCK_REGION_95_R {
        LOCK_REGION_95_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock Region 64"]
    #[inline(always)]
    pub fn lock_region_64(&mut self) -> LOCK_REGION_64_W {
        LOCK_REGION_64_W { w: self }
    }
    #[doc = "Bit 1 - Lock Region 65"]
    #[inline(always)]
    pub fn lock_region_65(&mut self) -> LOCK_REGION_65_W {
        LOCK_REGION_65_W { w: self }
    }
    #[doc = "Bit 2 - Lock Region 66"]
    #[inline(always)]
    pub fn lock_region_66(&mut self) -> LOCK_REGION_66_W {
        LOCK_REGION_66_W { w: self }
    }
    #[doc = "Bit 3 - Lock Region 67"]
    #[inline(always)]
    pub fn lock_region_67(&mut self) -> LOCK_REGION_67_W {
        LOCK_REGION_67_W { w: self }
    }
    #[doc = "Bit 4 - Lock Region 68"]
    #[inline(always)]
    pub fn lock_region_68(&mut self) -> LOCK_REGION_68_W {
        LOCK_REGION_68_W { w: self }
    }
    #[doc = "Bit 5 - Lock Region 69"]
    #[inline(always)]
    pub fn lock_region_69(&mut self) -> LOCK_REGION_69_W {
        LOCK_REGION_69_W { w: self }
    }
    #[doc = "Bit 6 - Lock Region 70"]
    #[inline(always)]
    pub fn lock_region_70(&mut self) -> LOCK_REGION_70_W {
        LOCK_REGION_70_W { w: self }
    }
    #[doc = "Bit 7 - Lock Region 71"]
    #[inline(always)]
    pub fn lock_region_71(&mut self) -> LOCK_REGION_71_W {
        LOCK_REGION_71_W { w: self }
    }
    #[doc = "Bit 8 - Lock Region 72"]
    #[inline(always)]
    pub fn lock_region_72(&mut self) -> LOCK_REGION_72_W {
        LOCK_REGION_72_W { w: self }
    }
    #[doc = "Bit 9 - Lock Region 73"]
    #[inline(always)]
    pub fn lock_region_73(&mut self) -> LOCK_REGION_73_W {
        LOCK_REGION_73_W { w: self }
    }
    #[doc = "Bit 10 - Lock Region 74"]
    #[inline(always)]
    pub fn lock_region_74(&mut self) -> LOCK_REGION_74_W {
        LOCK_REGION_74_W { w: self }
    }
    #[doc = "Bit 11 - Lock Region 75"]
    #[inline(always)]
    pub fn lock_region_75(&mut self) -> LOCK_REGION_75_W {
        LOCK_REGION_75_W { w: self }
    }
    #[doc = "Bit 12 - Lock Region 76"]
    #[inline(always)]
    pub fn lock_region_76(&mut self) -> LOCK_REGION_76_W {
        LOCK_REGION_76_W { w: self }
    }
    #[doc = "Bit 13 - Lock Region 77"]
    #[inline(always)]
    pub fn lock_region_77(&mut self) -> LOCK_REGION_77_W {
        LOCK_REGION_77_W { w: self }
    }
    #[doc = "Bit 14 - Lock Region 78"]
    #[inline(always)]
    pub fn lock_region_78(&mut self) -> LOCK_REGION_78_W {
        LOCK_REGION_78_W { w: self }
    }
    #[doc = "Bit 15 - Lock Region 79"]
    #[inline(always)]
    pub fn lock_region_79(&mut self) -> LOCK_REGION_79_W {
        LOCK_REGION_79_W { w: self }
    }
    #[doc = "Bit 16 - Lock Region 80"]
    #[inline(always)]
    pub fn lock_region_80(&mut self) -> LOCK_REGION_80_W {
        LOCK_REGION_80_W { w: self }
    }
    #[doc = "Bit 17 - Lock Region 81"]
    #[inline(always)]
    pub fn lock_region_81(&mut self) -> LOCK_REGION_81_W {
        LOCK_REGION_81_W { w: self }
    }
    #[doc = "Bit 18 - Lock Region 82"]
    #[inline(always)]
    pub fn lock_region_82(&mut self) -> LOCK_REGION_82_W {
        LOCK_REGION_82_W { w: self }
    }
    #[doc = "Bit 19 - Lock Region 83"]
    #[inline(always)]
    pub fn lock_region_83(&mut self) -> LOCK_REGION_83_W {
        LOCK_REGION_83_W { w: self }
    }
    #[doc = "Bit 20 - Lock Region 84"]
    #[inline(always)]
    pub fn lock_region_84(&mut self) -> LOCK_REGION_84_W {
        LOCK_REGION_84_W { w: self }
    }
    #[doc = "Bit 21 - Lock Region 85"]
    #[inline(always)]
    pub fn lock_region_85(&mut self) -> LOCK_REGION_85_W {
        LOCK_REGION_85_W { w: self }
    }
    #[doc = "Bit 22 - Lock Region 86"]
    #[inline(always)]
    pub fn lock_region_86(&mut self) -> LOCK_REGION_86_W {
        LOCK_REGION_86_W { w: self }
    }
    #[doc = "Bit 23 - Lock Region 87"]
    #[inline(always)]
    pub fn lock_region_87(&mut self) -> LOCK_REGION_87_W {
        LOCK_REGION_87_W { w: self }
    }
    #[doc = "Bit 24 - Lock Region 88"]
    #[inline(always)]
    pub fn lock_region_88(&mut self) -> LOCK_REGION_88_W {
        LOCK_REGION_88_W { w: self }
    }
    #[doc = "Bit 25 - Lock Region 89"]
    #[inline(always)]
    pub fn lock_region_89(&mut self) -> LOCK_REGION_89_W {
        LOCK_REGION_89_W { w: self }
    }
    #[doc = "Bit 26 - Lock Region 90"]
    #[inline(always)]
    pub fn lock_region_90(&mut self) -> LOCK_REGION_90_W {
        LOCK_REGION_90_W { w: self }
    }
    #[doc = "Bit 27 - Lock Region 91"]
    #[inline(always)]
    pub fn lock_region_91(&mut self) -> LOCK_REGION_91_W {
        LOCK_REGION_91_W { w: self }
    }
    #[doc = "Bit 28 - Lock Region 92"]
    #[inline(always)]
    pub fn lock_region_92(&mut self) -> LOCK_REGION_92_W {
        LOCK_REGION_92_W { w: self }
    }
    #[doc = "Bit 29 - Lock Region 93"]
    #[inline(always)]
    pub fn lock_region_93(&mut self) -> LOCK_REGION_93_W {
        LOCK_REGION_93_W { w: self }
    }
    #[doc = "Bit 30 - Lock Region 94"]
    #[inline(always)]
    pub fn lock_region_94(&mut self) -> LOCK_REGION_94_W {
        LOCK_REGION_94_W { w: self }
    }
    #[doc = "Bit 31 - Lock Region 95"]
    #[inline(always)]
    pub fn lock_region_95(&mut self) -> LOCK_REGION_95_W {
        LOCK_REGION_95_W { w: self }
    }
}
