#[doc = "Reader of register PMC_FSMR"]
pub type R = crate::R<u32, super::PMC_FSMR>;
#[doc = "Writer for register PMC_FSMR"]
pub type W = crate::W<u32, super::PMC_FSMR>;
#[doc = "Register PMC_FSMR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC_FSMR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FSTT0`"]
pub type FSTT0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT0`"]
pub struct FSTT0_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT0_W<'a> {
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
#[doc = "Reader of field `FSTT1`"]
pub type FSTT1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT1`"]
pub struct FSTT1_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT1_W<'a> {
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
#[doc = "Reader of field `FSTT2`"]
pub type FSTT2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT2`"]
pub struct FSTT2_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT2_W<'a> {
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
#[doc = "Reader of field `FSTT3`"]
pub type FSTT3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT3`"]
pub struct FSTT3_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT3_W<'a> {
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
#[doc = "Reader of field `FSTT4`"]
pub type FSTT4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT4`"]
pub struct FSTT4_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT4_W<'a> {
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
#[doc = "Reader of field `FSTT5`"]
pub type FSTT5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT5`"]
pub struct FSTT5_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT5_W<'a> {
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
#[doc = "Reader of field `FSTT6`"]
pub type FSTT6_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT6`"]
pub struct FSTT6_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT6_W<'a> {
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
#[doc = "Reader of field `FSTT7`"]
pub type FSTT7_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT7`"]
pub struct FSTT7_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT7_W<'a> {
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
#[doc = "Reader of field `FSTT8`"]
pub type FSTT8_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT8`"]
pub struct FSTT8_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT8_W<'a> {
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
#[doc = "Reader of field `FSTT9`"]
pub type FSTT9_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT9`"]
pub struct FSTT9_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT9_W<'a> {
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
#[doc = "Reader of field `FSTT10`"]
pub type FSTT10_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT10`"]
pub struct FSTT10_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT10_W<'a> {
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
#[doc = "Reader of field `FSTT11`"]
pub type FSTT11_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT11`"]
pub struct FSTT11_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT11_W<'a> {
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
#[doc = "Reader of field `FSTT12`"]
pub type FSTT12_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT12`"]
pub struct FSTT12_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT12_W<'a> {
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
#[doc = "Reader of field `FSTT13`"]
pub type FSTT13_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT13`"]
pub struct FSTT13_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT13_W<'a> {
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
#[doc = "Reader of field `FSTT14`"]
pub type FSTT14_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT14`"]
pub struct FSTT14_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT14_W<'a> {
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
#[doc = "Reader of field `FSTT15`"]
pub type FSTT15_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FSTT15`"]
pub struct FSTT15_W<'a> {
    w: &'a mut W,
}
impl<'a> FSTT15_W<'a> {
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
#[doc = "Reader of field `RTTAL`"]
pub type RTTAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTTAL`"]
pub struct RTTAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTTAL_W<'a> {
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
#[doc = "Reader of field `RTCAL`"]
pub type RTCAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCAL`"]
pub struct RTCAL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAL_W<'a> {
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
#[doc = "Reader of field `USBAL`"]
pub type USBAL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBAL`"]
pub struct USBAL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBAL_W<'a> {
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
#[doc = "Reader of field `LPM`"]
pub type LPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPM`"]
pub struct LPM_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_W<'a> {
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
#[doc = "Flash Low-power Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FLPM_A {
    #[doc = "0: Flash is in Standby Mode when system enters Wait Mode"]
    FLASH_STANDBY = 0,
    #[doc = "1: Flash is in Deep-power-down mode when system enters Wait Mode"]
    FLASH_DEEP_POWERDOWN = 1,
    #[doc = "2: Idle mode"]
    FLASH_IDLE = 2,
}
impl From<FLPM_A> for u8 {
    #[inline(always)]
    fn from(variant: FLPM_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FLPM`"]
pub type FLPM_R = crate::R<u8, FLPM_A>;
impl FLPM_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FLPM_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FLPM_A::FLASH_STANDBY),
            1 => Val(FLPM_A::FLASH_DEEP_POWERDOWN),
            2 => Val(FLPM_A::FLASH_IDLE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_STANDBY`"]
    #[inline(always)]
    pub fn is_flash_standby(&self) -> bool {
        *self == FLPM_A::FLASH_STANDBY
    }
    #[doc = "Checks if the value of the field is `FLASH_DEEP_POWERDOWN`"]
    #[inline(always)]
    pub fn is_flash_deep_powerdown(&self) -> bool {
        *self == FLPM_A::FLASH_DEEP_POWERDOWN
    }
    #[doc = "Checks if the value of the field is `FLASH_IDLE`"]
    #[inline(always)]
    pub fn is_flash_idle(&self) -> bool {
        *self == FLPM_A::FLASH_IDLE
    }
}
#[doc = "Write proxy for field `FLPM`"]
pub struct FLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FLPM_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLPM_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Flash is in Standby Mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_standby(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_STANDBY)
    }
    #[doc = "Flash is in Deep-power-down mode when system enters Wait Mode"]
    #[inline(always)]
    pub fn flash_deep_powerdown(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_DEEP_POWERDOWN)
    }
    #[doc = "Idle mode"]
    #[inline(always)]
    pub fn flash_idle(self) -> &'a mut W {
        self.variant(FLPM_A::FLASH_IDLE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 21)) | (((value as u32) & 0x03) << 21);
        self.w
    }
}
#[doc = "Reader of field `FFLPM`"]
pub type FFLPM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FFLPM`"]
pub struct FFLPM_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLPM_W<'a> {
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
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&self) -> FSTT0_R {
        FSTT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&self) -> FSTT1_R {
        FSTT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&self) -> FSTT2_R {
        FSTT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&self) -> FSTT3_R {
        FSTT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&self) -> FSTT4_R {
        FSTT4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&self) -> FSTT5_R {
        FSTT5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&self) -> FSTT6_R {
        FSTT6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&self) -> FSTT7_R {
        FSTT7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&self) -> FSTT8_R {
        FSTT8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&self) -> FSTT9_R {
        FSTT9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&self) -> FSTT10_R {
        FSTT10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&self) -> FSTT11_R {
        FSTT11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&self) -> FSTT12_R {
        FSTT12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&self) -> FSTT13_R {
        FSTT13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&self) -> FSTT14_R {
        FSTT14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&self) -> FSTT15_R {
        FSTT15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&self) -> RTTAL_R {
        RTTAL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&self) -> RTCAL_R {
        RTCAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&self) -> USBAL_R {
        USBAL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&self) -> LPM_R {
        LPM_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&self) -> FLPM_R {
        FLPM_R::new(((self.bits >> 21) & 0x03) as u8)
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&self) -> FFLPM_R {
        FFLPM_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fast Startup Input Enable 0"]
    #[inline(always)]
    pub fn fstt0(&mut self) -> FSTT0_W {
        FSTT0_W { w: self }
    }
    #[doc = "Bit 1 - Fast Startup Input Enable 1"]
    #[inline(always)]
    pub fn fstt1(&mut self) -> FSTT1_W {
        FSTT1_W { w: self }
    }
    #[doc = "Bit 2 - Fast Startup Input Enable 2"]
    #[inline(always)]
    pub fn fstt2(&mut self) -> FSTT2_W {
        FSTT2_W { w: self }
    }
    #[doc = "Bit 3 - Fast Startup Input Enable 3"]
    #[inline(always)]
    pub fn fstt3(&mut self) -> FSTT3_W {
        FSTT3_W { w: self }
    }
    #[doc = "Bit 4 - Fast Startup Input Enable 4"]
    #[inline(always)]
    pub fn fstt4(&mut self) -> FSTT4_W {
        FSTT4_W { w: self }
    }
    #[doc = "Bit 5 - Fast Startup Input Enable 5"]
    #[inline(always)]
    pub fn fstt5(&mut self) -> FSTT5_W {
        FSTT5_W { w: self }
    }
    #[doc = "Bit 6 - Fast Startup Input Enable 6"]
    #[inline(always)]
    pub fn fstt6(&mut self) -> FSTT6_W {
        FSTT6_W { w: self }
    }
    #[doc = "Bit 7 - Fast Startup Input Enable 7"]
    #[inline(always)]
    pub fn fstt7(&mut self) -> FSTT7_W {
        FSTT7_W { w: self }
    }
    #[doc = "Bit 8 - Fast Startup Input Enable 8"]
    #[inline(always)]
    pub fn fstt8(&mut self) -> FSTT8_W {
        FSTT8_W { w: self }
    }
    #[doc = "Bit 9 - Fast Startup Input Enable 9"]
    #[inline(always)]
    pub fn fstt9(&mut self) -> FSTT9_W {
        FSTT9_W { w: self }
    }
    #[doc = "Bit 10 - Fast Startup Input Enable 10"]
    #[inline(always)]
    pub fn fstt10(&mut self) -> FSTT10_W {
        FSTT10_W { w: self }
    }
    #[doc = "Bit 11 - Fast Startup Input Enable 11"]
    #[inline(always)]
    pub fn fstt11(&mut self) -> FSTT11_W {
        FSTT11_W { w: self }
    }
    #[doc = "Bit 12 - Fast Startup Input Enable 12"]
    #[inline(always)]
    pub fn fstt12(&mut self) -> FSTT12_W {
        FSTT12_W { w: self }
    }
    #[doc = "Bit 13 - Fast Startup Input Enable 13"]
    #[inline(always)]
    pub fn fstt13(&mut self) -> FSTT13_W {
        FSTT13_W { w: self }
    }
    #[doc = "Bit 14 - Fast Startup Input Enable 14"]
    #[inline(always)]
    pub fn fstt14(&mut self) -> FSTT14_W {
        FSTT14_W { w: self }
    }
    #[doc = "Bit 15 - Fast Startup Input Enable 15"]
    #[inline(always)]
    pub fn fstt15(&mut self) -> FSTT15_W {
        FSTT15_W { w: self }
    }
    #[doc = "Bit 16 - RTT Alarm Enable"]
    #[inline(always)]
    pub fn rttal(&mut self) -> RTTAL_W {
        RTTAL_W { w: self }
    }
    #[doc = "Bit 17 - RTC Alarm Enable"]
    #[inline(always)]
    pub fn rtcal(&mut self) -> RTCAL_W {
        RTCAL_W { w: self }
    }
    #[doc = "Bit 18 - USB Alarm Enable"]
    #[inline(always)]
    pub fn usbal(&mut self) -> USBAL_W {
        USBAL_W { w: self }
    }
    #[doc = "Bit 20 - Low-power Mode"]
    #[inline(always)]
    pub fn lpm(&mut self) -> LPM_W {
        LPM_W { w: self }
    }
    #[doc = "Bits 21:22 - Flash Low-power Mode"]
    #[inline(always)]
    pub fn flpm(&mut self) -> FLPM_W {
        FLPM_W { w: self }
    }
    #[doc = "Bit 23 - Force Flash Low-power Mode"]
    #[inline(always)]
    pub fn fflpm(&mut self) -> FFLPM_W {
        FFLPM_W { w: self }
    }
}
