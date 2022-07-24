#[doc = "Register `WUIR` reader"]
pub struct R(crate::R<WUIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUIR` writer"]
pub struct W(crate::W<WUIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WUIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Wakeup Input Enable 0 to 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN0_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN0_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN0` reader - Wakeup Input Enable 0 to 0"]
pub type WKUPEN0_R = crate::BitReader<WKUPEN0_A>;
impl WKUPEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN0_A {
        match self.bits {
            false => WKUPEN0_A::DISABLE,
            true => WKUPEN0_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN0_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN0_A::ENABLE
    }
}
#[doc = "Field `WKUPEN0` writer - Wakeup Input Enable 0 to 0"]
pub type WKUPEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN0_A, O>;
impl<'a, const O: u8> WKUPEN0_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN0_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN0_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN1_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN1_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN1` reader - Wakeup Input Enable 0 to 1"]
pub type WKUPEN1_R = crate::BitReader<WKUPEN1_A>;
impl WKUPEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN1_A {
        match self.bits {
            false => WKUPEN1_A::DISABLE,
            true => WKUPEN1_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN1_A::ENABLE
    }
}
#[doc = "Field `WKUPEN1` writer - Wakeup Input Enable 0 to 1"]
pub type WKUPEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN1_A, O>;
impl<'a, const O: u8> WKUPEN1_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN1_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN1_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN2_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN2_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN2` reader - Wakeup Input Enable 0 to 2"]
pub type WKUPEN2_R = crate::BitReader<WKUPEN2_A>;
impl WKUPEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN2_A {
        match self.bits {
            false => WKUPEN2_A::DISABLE,
            true => WKUPEN2_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN2_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN2_A::ENABLE
    }
}
#[doc = "Field `WKUPEN2` writer - Wakeup Input Enable 0 to 2"]
pub type WKUPEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN2_A, O>;
impl<'a, const O: u8> WKUPEN2_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN2_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN2_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN3_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN3_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN3` reader - Wakeup Input Enable 0 to 3"]
pub type WKUPEN3_R = crate::BitReader<WKUPEN3_A>;
impl WKUPEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN3_A {
        match self.bits {
            false => WKUPEN3_A::DISABLE,
            true => WKUPEN3_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN3_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN3_A::ENABLE
    }
}
#[doc = "Field `WKUPEN3` writer - Wakeup Input Enable 0 to 3"]
pub type WKUPEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN3_A, O>;
impl<'a, const O: u8> WKUPEN3_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN3_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN3_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN4_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN4_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN4` reader - Wakeup Input Enable 0 to 4"]
pub type WKUPEN4_R = crate::BitReader<WKUPEN4_A>;
impl WKUPEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN4_A {
        match self.bits {
            false => WKUPEN4_A::DISABLE,
            true => WKUPEN4_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN4_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN4_A::ENABLE
    }
}
#[doc = "Field `WKUPEN4` writer - Wakeup Input Enable 0 to 4"]
pub type WKUPEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN4_A, O>;
impl<'a, const O: u8> WKUPEN4_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN4_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN4_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN5_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN5_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN5` reader - Wakeup Input Enable 0 to 5"]
pub type WKUPEN5_R = crate::BitReader<WKUPEN5_A>;
impl WKUPEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN5_A {
        match self.bits {
            false => WKUPEN5_A::DISABLE,
            true => WKUPEN5_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN5_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN5_A::ENABLE
    }
}
#[doc = "Field `WKUPEN5` writer - Wakeup Input Enable 0 to 5"]
pub type WKUPEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN5_A, O>;
impl<'a, const O: u8> WKUPEN5_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN5_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN5_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN6_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN6_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN6` reader - Wakeup Input Enable 0 to 6"]
pub type WKUPEN6_R = crate::BitReader<WKUPEN6_A>;
impl WKUPEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN6_A {
        match self.bits {
            false => WKUPEN6_A::DISABLE,
            true => WKUPEN6_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN6_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN6_A::ENABLE
    }
}
#[doc = "Field `WKUPEN6` writer - Wakeup Input Enable 0 to 6"]
pub type WKUPEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN6_A, O>;
impl<'a, const O: u8> WKUPEN6_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN6_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN6_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN7_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN7_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN7` reader - Wakeup Input Enable 0 to 7"]
pub type WKUPEN7_R = crate::BitReader<WKUPEN7_A>;
impl WKUPEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN7_A {
        match self.bits {
            false => WKUPEN7_A::DISABLE,
            true => WKUPEN7_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN7_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN7_A::ENABLE
    }
}
#[doc = "Field `WKUPEN7` writer - Wakeup Input Enable 0 to 7"]
pub type WKUPEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN7_A, O>;
impl<'a, const O: u8> WKUPEN7_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN7_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN7_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN8_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN8_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN8` reader - Wakeup Input Enable 0 to 8"]
pub type WKUPEN8_R = crate::BitReader<WKUPEN8_A>;
impl WKUPEN8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN8_A {
        match self.bits {
            false => WKUPEN8_A::DISABLE,
            true => WKUPEN8_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN8_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN8_A::ENABLE
    }
}
#[doc = "Field `WKUPEN8` writer - Wakeup Input Enable 0 to 8"]
pub type WKUPEN8_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN8_A, O>;
impl<'a, const O: u8> WKUPEN8_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN8_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN8_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN9_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN9_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN9` reader - Wakeup Input Enable 0 to 9"]
pub type WKUPEN9_R = crate::BitReader<WKUPEN9_A>;
impl WKUPEN9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN9_A {
        match self.bits {
            false => WKUPEN9_A::DISABLE,
            true => WKUPEN9_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN9_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN9_A::ENABLE
    }
}
#[doc = "Field `WKUPEN9` writer - Wakeup Input Enable 0 to 9"]
pub type WKUPEN9_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN9_A, O>;
impl<'a, const O: u8> WKUPEN9_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN9_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN9_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN10_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN10_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN10` reader - Wakeup Input Enable 0 to 10"]
pub type WKUPEN10_R = crate::BitReader<WKUPEN10_A>;
impl WKUPEN10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN10_A {
        match self.bits {
            false => WKUPEN10_A::DISABLE,
            true => WKUPEN10_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN10_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN10_A::ENABLE
    }
}
#[doc = "Field `WKUPEN10` writer - Wakeup Input Enable 0 to 10"]
pub type WKUPEN10_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN10_A, O>;
impl<'a, const O: u8> WKUPEN10_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN10_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN10_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN11_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN11_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN11` reader - Wakeup Input Enable 0 to 11"]
pub type WKUPEN11_R = crate::BitReader<WKUPEN11_A>;
impl WKUPEN11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN11_A {
        match self.bits {
            false => WKUPEN11_A::DISABLE,
            true => WKUPEN11_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN11_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN11_A::ENABLE
    }
}
#[doc = "Field `WKUPEN11` writer - Wakeup Input Enable 0 to 11"]
pub type WKUPEN11_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN11_A, O>;
impl<'a, const O: u8> WKUPEN11_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN11_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN11_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN12_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN12_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN12` reader - Wakeup Input Enable 0 to 12"]
pub type WKUPEN12_R = crate::BitReader<WKUPEN12_A>;
impl WKUPEN12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN12_A {
        match self.bits {
            false => WKUPEN12_A::DISABLE,
            true => WKUPEN12_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN12_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN12_A::ENABLE
    }
}
#[doc = "Field `WKUPEN12` writer - Wakeup Input Enable 0 to 12"]
pub type WKUPEN12_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN12_A, O>;
impl<'a, const O: u8> WKUPEN12_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN12_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN12_A::ENABLE)
    }
}
#[doc = "Wakeup Input Enable 0 to 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPEN13_A {
    #[doc = "0: The corresponding wakeup input has no wakeup effect."]
    DISABLE = 0,
    #[doc = "1: The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    ENABLE = 1,
}
impl From<WKUPEN13_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPEN13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPEN13` reader - Wakeup Input Enable 0 to 13"]
pub type WKUPEN13_R = crate::BitReader<WKUPEN13_A>;
impl WKUPEN13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPEN13_A {
        match self.bits {
            false => WKUPEN13_A::DISABLE,
            true => WKUPEN13_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WKUPEN13_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WKUPEN13_A::ENABLE
    }
}
#[doc = "Field `WKUPEN13` writer - Wakeup Input Enable 0 to 13"]
pub type WKUPEN13_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPEN13_A, O>;
impl<'a, const O: u8> WKUPEN13_W<'a, O> {
    #[doc = "The corresponding wakeup input has no wakeup effect."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WKUPEN13_A::DISABLE)
    }
    #[doc = "The corresponding wakeup input is enabled for a wakeup of the core power supply."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WKUPEN13_A::ENABLE)
    }
}
#[doc = "Wakeup Input Type 0 to 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT0_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT0_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT0` reader - Wakeup Input Type 0 to 0"]
pub type WKUPT0_R = crate::BitReader<WKUPT0_A>;
impl WKUPT0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT0_A {
        match self.bits {
            false => WKUPT0_A::LOW,
            true => WKUPT0_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT0_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT0_A::HIGH
    }
}
#[doc = "Field `WKUPT0` writer - Wakeup Input Type 0 to 0"]
pub type WKUPT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT0_A, O>;
impl<'a, const O: u8> WKUPT0_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT0_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT0_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT1_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT1_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT1` reader - Wakeup Input Type 0 to 1"]
pub type WKUPT1_R = crate::BitReader<WKUPT1_A>;
impl WKUPT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT1_A {
        match self.bits {
            false => WKUPT1_A::LOW,
            true => WKUPT1_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT1_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT1_A::HIGH
    }
}
#[doc = "Field `WKUPT1` writer - Wakeup Input Type 0 to 1"]
pub type WKUPT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT1_A, O>;
impl<'a, const O: u8> WKUPT1_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT1_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT1_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT2_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT2_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT2` reader - Wakeup Input Type 0 to 2"]
pub type WKUPT2_R = crate::BitReader<WKUPT2_A>;
impl WKUPT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT2_A {
        match self.bits {
            false => WKUPT2_A::LOW,
            true => WKUPT2_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT2_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT2_A::HIGH
    }
}
#[doc = "Field `WKUPT2` writer - Wakeup Input Type 0 to 2"]
pub type WKUPT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT2_A, O>;
impl<'a, const O: u8> WKUPT2_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT2_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT2_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT3_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT3_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT3` reader - Wakeup Input Type 0 to 3"]
pub type WKUPT3_R = crate::BitReader<WKUPT3_A>;
impl WKUPT3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT3_A {
        match self.bits {
            false => WKUPT3_A::LOW,
            true => WKUPT3_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT3_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT3_A::HIGH
    }
}
#[doc = "Field `WKUPT3` writer - Wakeup Input Type 0 to 3"]
pub type WKUPT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT3_A, O>;
impl<'a, const O: u8> WKUPT3_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT3_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT3_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT4_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT4_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT4` reader - Wakeup Input Type 0 to 4"]
pub type WKUPT4_R = crate::BitReader<WKUPT4_A>;
impl WKUPT4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT4_A {
        match self.bits {
            false => WKUPT4_A::LOW,
            true => WKUPT4_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT4_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT4_A::HIGH
    }
}
#[doc = "Field `WKUPT4` writer - Wakeup Input Type 0 to 4"]
pub type WKUPT4_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT4_A, O>;
impl<'a, const O: u8> WKUPT4_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT4_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT4_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT5_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT5_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT5` reader - Wakeup Input Type 0 to 5"]
pub type WKUPT5_R = crate::BitReader<WKUPT5_A>;
impl WKUPT5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT5_A {
        match self.bits {
            false => WKUPT5_A::LOW,
            true => WKUPT5_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT5_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT5_A::HIGH
    }
}
#[doc = "Field `WKUPT5` writer - Wakeup Input Type 0 to 5"]
pub type WKUPT5_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT5_A, O>;
impl<'a, const O: u8> WKUPT5_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT5_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT5_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT6_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT6_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT6` reader - Wakeup Input Type 0 to 6"]
pub type WKUPT6_R = crate::BitReader<WKUPT6_A>;
impl WKUPT6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT6_A {
        match self.bits {
            false => WKUPT6_A::LOW,
            true => WKUPT6_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT6_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT6_A::HIGH
    }
}
#[doc = "Field `WKUPT6` writer - Wakeup Input Type 0 to 6"]
pub type WKUPT6_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT6_A, O>;
impl<'a, const O: u8> WKUPT6_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT6_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT6_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT7_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT7_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT7` reader - Wakeup Input Type 0 to 7"]
pub type WKUPT7_R = crate::BitReader<WKUPT7_A>;
impl WKUPT7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT7_A {
        match self.bits {
            false => WKUPT7_A::LOW,
            true => WKUPT7_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT7_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT7_A::HIGH
    }
}
#[doc = "Field `WKUPT7` writer - Wakeup Input Type 0 to 7"]
pub type WKUPT7_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT7_A, O>;
impl<'a, const O: u8> WKUPT7_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT7_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT7_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT8_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT8_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT8_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT8` reader - Wakeup Input Type 0 to 8"]
pub type WKUPT8_R = crate::BitReader<WKUPT8_A>;
impl WKUPT8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT8_A {
        match self.bits {
            false => WKUPT8_A::LOW,
            true => WKUPT8_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT8_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT8_A::HIGH
    }
}
#[doc = "Field `WKUPT8` writer - Wakeup Input Type 0 to 8"]
pub type WKUPT8_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT8_A, O>;
impl<'a, const O: u8> WKUPT8_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT8_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT8_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT9_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT9_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT9_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT9` reader - Wakeup Input Type 0 to 9"]
pub type WKUPT9_R = crate::BitReader<WKUPT9_A>;
impl WKUPT9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT9_A {
        match self.bits {
            false => WKUPT9_A::LOW,
            true => WKUPT9_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT9_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT9_A::HIGH
    }
}
#[doc = "Field `WKUPT9` writer - Wakeup Input Type 0 to 9"]
pub type WKUPT9_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT9_A, O>;
impl<'a, const O: u8> WKUPT9_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT9_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT9_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT10_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT10_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT10_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT10` reader - Wakeup Input Type 0 to 10"]
pub type WKUPT10_R = crate::BitReader<WKUPT10_A>;
impl WKUPT10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT10_A {
        match self.bits {
            false => WKUPT10_A::LOW,
            true => WKUPT10_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT10_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT10_A::HIGH
    }
}
#[doc = "Field `WKUPT10` writer - Wakeup Input Type 0 to 10"]
pub type WKUPT10_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT10_A, O>;
impl<'a, const O: u8> WKUPT10_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT10_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT10_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT11_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT11_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT11_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT11` reader - Wakeup Input Type 0 to 11"]
pub type WKUPT11_R = crate::BitReader<WKUPT11_A>;
impl WKUPT11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT11_A {
        match self.bits {
            false => WKUPT11_A::LOW,
            true => WKUPT11_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT11_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT11_A::HIGH
    }
}
#[doc = "Field `WKUPT11` writer - Wakeup Input Type 0 to 11"]
pub type WKUPT11_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT11_A, O>;
impl<'a, const O: u8> WKUPT11_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT11_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT11_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT12_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT12_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT12_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT12` reader - Wakeup Input Type 0 to 12"]
pub type WKUPT12_R = crate::BitReader<WKUPT12_A>;
impl WKUPT12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT12_A {
        match self.bits {
            false => WKUPT12_A::LOW,
            true => WKUPT12_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT12_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT12_A::HIGH
    }
}
#[doc = "Field `WKUPT12` writer - Wakeup Input Type 0 to 12"]
pub type WKUPT12_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT12_A, O>;
impl<'a, const O: u8> WKUPT12_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT12_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT12_A::HIGH)
    }
}
#[doc = "Wakeup Input Type 0 to 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WKUPT13_A {
    #[doc = "0: A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    LOW = 0,
    #[doc = "1: A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    HIGH = 1,
}
impl From<WKUPT13_A> for bool {
    #[inline(always)]
    fn from(variant: WKUPT13_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WKUPT13` reader - Wakeup Input Type 0 to 13"]
pub type WKUPT13_R = crate::BitReader<WKUPT13_A>;
impl WKUPT13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WKUPT13_A {
        match self.bits {
            false => WKUPT13_A::LOW,
            true => WKUPT13_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == WKUPT13_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == WKUPT13_A::HIGH
    }
}
#[doc = "Field `WKUPT13` writer - Wakeup Input Type 0 to 13"]
pub type WKUPT13_W<'a, const O: u8> = crate::BitWriter<'a, u32, WUIR_SPEC, WKUPT13_A, O>;
impl<'a, const O: u8> WKUPT13_W<'a, O> {
    #[doc = "A falling edge followed by a low level for a period defined by WKUPDBC on the corre-sponding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(WKUPT13_A::LOW)
    }
    #[doc = "A rising edge followed by a high level for a period defined by WKUPDBC on the cor-responding wakeup input forces the wakeup of the core power supply."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(WKUPT13_A::HIGH)
    }
}
impl R {
    #[doc = "Bit 0 - Wakeup Input Enable 0 to 0"]
    #[inline(always)]
    pub fn wkupen0(&self) -> WKUPEN0_R {
        WKUPEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup Input Enable 0 to 1"]
    #[inline(always)]
    pub fn wkupen1(&self) -> WKUPEN1_R {
        WKUPEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup Input Enable 0 to 2"]
    #[inline(always)]
    pub fn wkupen2(&self) -> WKUPEN2_R {
        WKUPEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup Input Enable 0 to 3"]
    #[inline(always)]
    pub fn wkupen3(&self) -> WKUPEN3_R {
        WKUPEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup Input Enable 0 to 4"]
    #[inline(always)]
    pub fn wkupen4(&self) -> WKUPEN4_R {
        WKUPEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup Input Enable 0 to 5"]
    #[inline(always)]
    pub fn wkupen5(&self) -> WKUPEN5_R {
        WKUPEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wakeup Input Enable 0 to 6"]
    #[inline(always)]
    pub fn wkupen6(&self) -> WKUPEN6_R {
        WKUPEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wakeup Input Enable 0 to 7"]
    #[inline(always)]
    pub fn wkupen7(&self) -> WKUPEN7_R {
        WKUPEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wakeup Input Enable 0 to 8"]
    #[inline(always)]
    pub fn wkupen8(&self) -> WKUPEN8_R {
        WKUPEN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wakeup Input Enable 0 to 9"]
    #[inline(always)]
    pub fn wkupen9(&self) -> WKUPEN9_R {
        WKUPEN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup Input Enable 0 to 10"]
    #[inline(always)]
    pub fn wkupen10(&self) -> WKUPEN10_R {
        WKUPEN10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup Input Enable 0 to 11"]
    #[inline(always)]
    pub fn wkupen11(&self) -> WKUPEN11_R {
        WKUPEN11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wakeup Input Enable 0 to 12"]
    #[inline(always)]
    pub fn wkupen12(&self) -> WKUPEN12_R {
        WKUPEN12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wakeup Input Enable 0 to 13"]
    #[inline(always)]
    pub fn wkupen13(&self) -> WKUPEN13_R {
        WKUPEN13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Wakeup Input Type 0 to 0"]
    #[inline(always)]
    pub fn wkupt0(&self) -> WKUPT0_R {
        WKUPT0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wakeup Input Type 0 to 1"]
    #[inline(always)]
    pub fn wkupt1(&self) -> WKUPT1_R {
        WKUPT1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup Input Type 0 to 2"]
    #[inline(always)]
    pub fn wkupt2(&self) -> WKUPT2_R {
        WKUPT2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wakeup Input Type 0 to 3"]
    #[inline(always)]
    pub fn wkupt3(&self) -> WKUPT3_R {
        WKUPT3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wakeup Input Type 0 to 4"]
    #[inline(always)]
    pub fn wkupt4(&self) -> WKUPT4_R {
        WKUPT4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wakeup Input Type 0 to 5"]
    #[inline(always)]
    pub fn wkupt5(&self) -> WKUPT5_R {
        WKUPT5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wakeup Input Type 0 to 6"]
    #[inline(always)]
    pub fn wkupt6(&self) -> WKUPT6_R {
        WKUPT6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wakeup Input Type 0 to 7"]
    #[inline(always)]
    pub fn wkupt7(&self) -> WKUPT7_R {
        WKUPT7_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wakeup Input Type 0 to 8"]
    #[inline(always)]
    pub fn wkupt8(&self) -> WKUPT8_R {
        WKUPT8_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wakeup Input Type 0 to 9"]
    #[inline(always)]
    pub fn wkupt9(&self) -> WKUPT9_R {
        WKUPT9_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wakeup Input Type 0 to 10"]
    #[inline(always)]
    pub fn wkupt10(&self) -> WKUPT10_R {
        WKUPT10_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wakeup Input Type 0 to 11"]
    #[inline(always)]
    pub fn wkupt11(&self) -> WKUPT11_R {
        WKUPT11_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wakeup Input Type 0 to 12"]
    #[inline(always)]
    pub fn wkupt12(&self) -> WKUPT12_R {
        WKUPT12_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wakeup Input Type 0 to 13"]
    #[inline(always)]
    pub fn wkupt13(&self) -> WKUPT13_R {
        WKUPT13_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup Input Enable 0 to 0"]
    #[inline(always)]
    pub fn wkupen0(&mut self) -> WKUPEN0_W<0> {
        WKUPEN0_W::new(self)
    }
    #[doc = "Bit 1 - Wakeup Input Enable 0 to 1"]
    #[inline(always)]
    pub fn wkupen1(&mut self) -> WKUPEN1_W<1> {
        WKUPEN1_W::new(self)
    }
    #[doc = "Bit 2 - Wakeup Input Enable 0 to 2"]
    #[inline(always)]
    pub fn wkupen2(&mut self) -> WKUPEN2_W<2> {
        WKUPEN2_W::new(self)
    }
    #[doc = "Bit 3 - Wakeup Input Enable 0 to 3"]
    #[inline(always)]
    pub fn wkupen3(&mut self) -> WKUPEN3_W<3> {
        WKUPEN3_W::new(self)
    }
    #[doc = "Bit 4 - Wakeup Input Enable 0 to 4"]
    #[inline(always)]
    pub fn wkupen4(&mut self) -> WKUPEN4_W<4> {
        WKUPEN4_W::new(self)
    }
    #[doc = "Bit 5 - Wakeup Input Enable 0 to 5"]
    #[inline(always)]
    pub fn wkupen5(&mut self) -> WKUPEN5_W<5> {
        WKUPEN5_W::new(self)
    }
    #[doc = "Bit 6 - Wakeup Input Enable 0 to 6"]
    #[inline(always)]
    pub fn wkupen6(&mut self) -> WKUPEN6_W<6> {
        WKUPEN6_W::new(self)
    }
    #[doc = "Bit 7 - Wakeup Input Enable 0 to 7"]
    #[inline(always)]
    pub fn wkupen7(&mut self) -> WKUPEN7_W<7> {
        WKUPEN7_W::new(self)
    }
    #[doc = "Bit 8 - Wakeup Input Enable 0 to 8"]
    #[inline(always)]
    pub fn wkupen8(&mut self) -> WKUPEN8_W<8> {
        WKUPEN8_W::new(self)
    }
    #[doc = "Bit 9 - Wakeup Input Enable 0 to 9"]
    #[inline(always)]
    pub fn wkupen9(&mut self) -> WKUPEN9_W<9> {
        WKUPEN9_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup Input Enable 0 to 10"]
    #[inline(always)]
    pub fn wkupen10(&mut self) -> WKUPEN10_W<10> {
        WKUPEN10_W::new(self)
    }
    #[doc = "Bit 11 - Wakeup Input Enable 0 to 11"]
    #[inline(always)]
    pub fn wkupen11(&mut self) -> WKUPEN11_W<11> {
        WKUPEN11_W::new(self)
    }
    #[doc = "Bit 12 - Wakeup Input Enable 0 to 12"]
    #[inline(always)]
    pub fn wkupen12(&mut self) -> WKUPEN12_W<12> {
        WKUPEN12_W::new(self)
    }
    #[doc = "Bit 13 - Wakeup Input Enable 0 to 13"]
    #[inline(always)]
    pub fn wkupen13(&mut self) -> WKUPEN13_W<13> {
        WKUPEN13_W::new(self)
    }
    #[doc = "Bit 16 - Wakeup Input Type 0 to 0"]
    #[inline(always)]
    pub fn wkupt0(&mut self) -> WKUPT0_W<16> {
        WKUPT0_W::new(self)
    }
    #[doc = "Bit 17 - Wakeup Input Type 0 to 1"]
    #[inline(always)]
    pub fn wkupt1(&mut self) -> WKUPT1_W<17> {
        WKUPT1_W::new(self)
    }
    #[doc = "Bit 18 - Wakeup Input Type 0 to 2"]
    #[inline(always)]
    pub fn wkupt2(&mut self) -> WKUPT2_W<18> {
        WKUPT2_W::new(self)
    }
    #[doc = "Bit 19 - Wakeup Input Type 0 to 3"]
    #[inline(always)]
    pub fn wkupt3(&mut self) -> WKUPT3_W<19> {
        WKUPT3_W::new(self)
    }
    #[doc = "Bit 20 - Wakeup Input Type 0 to 4"]
    #[inline(always)]
    pub fn wkupt4(&mut self) -> WKUPT4_W<20> {
        WKUPT4_W::new(self)
    }
    #[doc = "Bit 21 - Wakeup Input Type 0 to 5"]
    #[inline(always)]
    pub fn wkupt5(&mut self) -> WKUPT5_W<21> {
        WKUPT5_W::new(self)
    }
    #[doc = "Bit 22 - Wakeup Input Type 0 to 6"]
    #[inline(always)]
    pub fn wkupt6(&mut self) -> WKUPT6_W<22> {
        WKUPT6_W::new(self)
    }
    #[doc = "Bit 23 - Wakeup Input Type 0 to 7"]
    #[inline(always)]
    pub fn wkupt7(&mut self) -> WKUPT7_W<23> {
        WKUPT7_W::new(self)
    }
    #[doc = "Bit 24 - Wakeup Input Type 0 to 8"]
    #[inline(always)]
    pub fn wkupt8(&mut self) -> WKUPT8_W<24> {
        WKUPT8_W::new(self)
    }
    #[doc = "Bit 25 - Wakeup Input Type 0 to 9"]
    #[inline(always)]
    pub fn wkupt9(&mut self) -> WKUPT9_W<25> {
        WKUPT9_W::new(self)
    }
    #[doc = "Bit 26 - Wakeup Input Type 0 to 10"]
    #[inline(always)]
    pub fn wkupt10(&mut self) -> WKUPT10_W<26> {
        WKUPT10_W::new(self)
    }
    #[doc = "Bit 27 - Wakeup Input Type 0 to 11"]
    #[inline(always)]
    pub fn wkupt11(&mut self) -> WKUPT11_W<27> {
        WKUPT11_W::new(self)
    }
    #[doc = "Bit 28 - Wakeup Input Type 0 to 12"]
    #[inline(always)]
    pub fn wkupt12(&mut self) -> WKUPT12_W<28> {
        WKUPT12_W::new(self)
    }
    #[doc = "Bit 29 - Wakeup Input Type 0 to 13"]
    #[inline(always)]
    pub fn wkupt13(&mut self) -> WKUPT13_W<29> {
        WKUPT13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Supply Controller Wakeup Inputs Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wuir](index.html) module"]
pub struct WUIR_SPEC;
impl crate::RegisterSpec for WUIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wuir::R](R) reader structure"]
impl crate::Readable for WUIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wuir::W](W) writer structure"]
impl crate::Writable for WUIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUIR to value 0"]
impl crate::Resettable for WUIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
