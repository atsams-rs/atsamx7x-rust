#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELMINUS` reader - Selection for Minus Comparator Input"]
pub type SELMINUS_R = crate::FieldReader<u8, SELMINUSSELECT_A>;
#[doc = "Selection for Minus Comparator Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELMINUSSELECT_A {
    #[doc = "0: Select TS"]
    TS = 0,
    #[doc = "1: Select VREFP"]
    VREFP = 1,
    #[doc = "2: Select DAC0"]
    DAC0 = 2,
    #[doc = "3: Select DAC1"]
    DAC1 = 3,
    #[doc = "4: Select AFE0_AD0"]
    AFE0_AD0 = 4,
    #[doc = "5: Select AFE0_AD1"]
    AFE0_AD1 = 5,
    #[doc = "6: Select AFE0_AD2"]
    AFE0_AD2 = 6,
    #[doc = "7: Select AFE0_AD3"]
    AFE0_AD3 = 7,
}
impl From<SELMINUSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMINUSSELECT_A) -> Self {
        variant as _
    }
}
impl SELMINUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELMINUSSELECT_A {
        match self.bits {
            0 => SELMINUSSELECT_A::TS,
            1 => SELMINUSSELECT_A::VREFP,
            2 => SELMINUSSELECT_A::DAC0,
            3 => SELMINUSSELECT_A::DAC1,
            4 => SELMINUSSELECT_A::AFE0_AD0,
            5 => SELMINUSSELECT_A::AFE0_AD1,
            6 => SELMINUSSELECT_A::AFE0_AD2,
            7 => SELMINUSSELECT_A::AFE0_AD3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TS`"]
    #[inline(always)]
    pub fn is_ts(&self) -> bool {
        *self == SELMINUSSELECT_A::TS
    }
    #[doc = "Checks if the value of the field is `VREFP`"]
    #[inline(always)]
    pub fn is_vrefp(&self) -> bool {
        *self == SELMINUSSELECT_A::VREFP
    }
    #[doc = "Checks if the value of the field is `DAC0`"]
    #[inline(always)]
    pub fn is_dac0(&self) -> bool {
        *self == SELMINUSSELECT_A::DAC0
    }
    #[doc = "Checks if the value of the field is `DAC1`"]
    #[inline(always)]
    pub fn is_dac1(&self) -> bool {
        *self == SELMINUSSELECT_A::DAC1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD0`"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        *self == SELMINUSSELECT_A::AFE0_AD0
    }
    #[doc = "Checks if the value of the field is `AFE0_AD1`"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        *self == SELMINUSSELECT_A::AFE0_AD1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD2`"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        *self == SELMINUSSELECT_A::AFE0_AD2
    }
    #[doc = "Checks if the value of the field is `AFE0_AD3`"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        *self == SELMINUSSELECT_A::AFE0_AD3
    }
}
#[doc = "Field `SELMINUS` writer - Selection for Minus Comparator Input"]
pub type SELMINUS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, SELMINUSSELECT_A, 3, O>;
impl<'a, const O: u8> SELMINUS_W<'a, O> {
    #[doc = "Select TS"]
    #[inline(always)]
    pub fn ts(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::TS)
    }
    #[doc = "Select VREFP"]
    #[inline(always)]
    pub fn vrefp(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::VREFP)
    }
    #[doc = "Select DAC0"]
    #[inline(always)]
    pub fn dac0(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::DAC0)
    }
    #[doc = "Select DAC1"]
    #[inline(always)]
    pub fn dac1(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::DAC1)
    }
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::AFE0_AD0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::AFE0_AD1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::AFE0_AD2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut W {
        self.variant(SELMINUSSELECT_A::AFE0_AD3)
    }
}
#[doc = "Field `SELPLUS` reader - Selection For Plus Comparator Input"]
pub type SELPLUS_R = crate::FieldReader<u8, SELPLUSSELECT_A>;
#[doc = "Selection For Plus Comparator Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELPLUSSELECT_A {
    #[doc = "0: Select AFE0_AD0"]
    AFE0_AD0 = 0,
    #[doc = "1: Select AFE0_AD1"]
    AFE0_AD1 = 1,
    #[doc = "2: Select AFE0_AD2"]
    AFE0_AD2 = 2,
    #[doc = "3: Select AFE0_AD3"]
    AFE0_AD3 = 3,
    #[doc = "4: Select AFE0_AD4"]
    AFE0_AD4 = 4,
    #[doc = "5: Select AFE0_AD5"]
    AFE0_AD5 = 5,
    #[doc = "6: Select AFE1_AD0"]
    AFE1_AD0 = 6,
    #[doc = "7: Select AFE1_AD1"]
    AFE1_AD1 = 7,
}
impl From<SELPLUSSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: SELPLUSSELECT_A) -> Self {
        variant as _
    }
}
impl SELPLUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELPLUSSELECT_A {
        match self.bits {
            0 => SELPLUSSELECT_A::AFE0_AD0,
            1 => SELPLUSSELECT_A::AFE0_AD1,
            2 => SELPLUSSELECT_A::AFE0_AD2,
            3 => SELPLUSSELECT_A::AFE0_AD3,
            4 => SELPLUSSELECT_A::AFE0_AD4,
            5 => SELPLUSSELECT_A::AFE0_AD5,
            6 => SELPLUSSELECT_A::AFE1_AD0,
            7 => SELPLUSSELECT_A::AFE1_AD1,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AFE0_AD0`"]
    #[inline(always)]
    pub fn is_afe0_ad0(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE0_AD0
    }
    #[doc = "Checks if the value of the field is `AFE0_AD1`"]
    #[inline(always)]
    pub fn is_afe0_ad1(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE0_AD1
    }
    #[doc = "Checks if the value of the field is `AFE0_AD2`"]
    #[inline(always)]
    pub fn is_afe0_ad2(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE0_AD2
    }
    #[doc = "Checks if the value of the field is `AFE0_AD3`"]
    #[inline(always)]
    pub fn is_afe0_ad3(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE0_AD3
    }
    #[doc = "Checks if the value of the field is `AFE0_AD4`"]
    #[inline(always)]
    pub fn is_afe0_ad4(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE0_AD4
    }
    #[doc = "Checks if the value of the field is `AFE0_AD5`"]
    #[inline(always)]
    pub fn is_afe0_ad5(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE0_AD5
    }
    #[doc = "Checks if the value of the field is `AFE1_AD0`"]
    #[inline(always)]
    pub fn is_afe1_ad0(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE1_AD0
    }
    #[doc = "Checks if the value of the field is `AFE1_AD1`"]
    #[inline(always)]
    pub fn is_afe1_ad1(&self) -> bool {
        *self == SELPLUSSELECT_A::AFE1_AD1
    }
}
#[doc = "Field `SELPLUS` writer - Selection For Plus Comparator Input"]
pub type SELPLUS_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, SELPLUSSELECT_A, 3, O>;
impl<'a, const O: u8> SELPLUS_W<'a, O> {
    #[doc = "Select AFE0_AD0"]
    #[inline(always)]
    pub fn afe0_ad0(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE0_AD0)
    }
    #[doc = "Select AFE0_AD1"]
    #[inline(always)]
    pub fn afe0_ad1(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE0_AD1)
    }
    #[doc = "Select AFE0_AD2"]
    #[inline(always)]
    pub fn afe0_ad2(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE0_AD2)
    }
    #[doc = "Select AFE0_AD3"]
    #[inline(always)]
    pub fn afe0_ad3(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE0_AD3)
    }
    #[doc = "Select AFE0_AD4"]
    #[inline(always)]
    pub fn afe0_ad4(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE0_AD4)
    }
    #[doc = "Select AFE0_AD5"]
    #[inline(always)]
    pub fn afe0_ad5(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE0_AD5)
    }
    #[doc = "Select AFE1_AD0"]
    #[inline(always)]
    pub fn afe1_ad0(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE1_AD0)
    }
    #[doc = "Select AFE1_AD1"]
    #[inline(always)]
    pub fn afe1_ad1(self) -> &'a mut W {
        self.variant(SELPLUSSELECT_A::AFE1_AD1)
    }
}
#[doc = "Field `ACEN` reader - Analog Comparator Enable"]
pub type ACEN_R = crate::BitReader<ACENSELECT_A>;
#[doc = "Analog Comparator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACENSELECT_A {
    #[doc = "0: Analog comparator disabled."]
    DIS = 0,
    #[doc = "1: Analog comparator enabled."]
    EN = 1,
}
impl From<ACENSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: ACENSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACENSELECT_A {
        match self.bits {
            false => ACENSELECT_A::DIS,
            true => ACENSELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ACENSELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ACENSELECT_A::EN
    }
}
#[doc = "Field `ACEN` writer - Analog Comparator Enable"]
pub type ACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, ACENSELECT_A, O>;
impl<'a, const O: u8> ACEN_W<'a, O> {
    #[doc = "Analog comparator disabled."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(ACENSELECT_A::DIS)
    }
    #[doc = "Analog comparator enabled."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(ACENSELECT_A::EN)
    }
}
#[doc = "Field `EDGETYP` reader - Edge Type"]
pub type EDGETYP_R = crate::FieldReader<u8, EDGETYPSELECT_A>;
#[doc = "Edge Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGETYPSELECT_A {
    #[doc = "0: Only rising edge of comparator output"]
    RISING = 0,
    #[doc = "1: Falling edge of comparator output"]
    FALLING = 1,
    #[doc = "2: Any edge of comparator output"]
    ANY = 2,
}
impl From<EDGETYPSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGETYPSELECT_A) -> Self {
        variant as _
    }
}
impl EDGETYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EDGETYPSELECT_A> {
        match self.bits {
            0 => Some(EDGETYPSELECT_A::RISING),
            1 => Some(EDGETYPSELECT_A::FALLING),
            2 => Some(EDGETYPSELECT_A::ANY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == EDGETYPSELECT_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == EDGETYPSELECT_A::FALLING
    }
    #[doc = "Checks if the value of the field is `ANY`"]
    #[inline(always)]
    pub fn is_any(&self) -> bool {
        *self == EDGETYPSELECT_A::ANY
    }
}
#[doc = "Field `EDGETYP` writer - Edge Type"]
pub type EDGETYP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MR_SPEC, u8, EDGETYPSELECT_A, 2, O>;
impl<'a, const O: u8> EDGETYP_W<'a, O> {
    #[doc = "Only rising edge of comparator output"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(EDGETYPSELECT_A::RISING)
    }
    #[doc = "Falling edge of comparator output"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(EDGETYPSELECT_A::FALLING)
    }
    #[doc = "Any edge of comparator output"]
    #[inline(always)]
    pub fn any(self) -> &'a mut W {
        self.variant(EDGETYPSELECT_A::ANY)
    }
}
#[doc = "Field `INV` reader - Invert Comparator Output"]
pub type INV_R = crate::BitReader<INVSELECT_A>;
#[doc = "Invert Comparator Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INVSELECT_A {
    #[doc = "0: Analog comparator output is directly processed."]
    DIS = 0,
    #[doc = "1: Analog comparator output is inverted prior to being processed."]
    EN = 1,
}
impl From<INVSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: INVSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INVSELECT_A {
        match self.bits {
            false => INVSELECT_A::DIS,
            true => INVSELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == INVSELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == INVSELECT_A::EN
    }
}
#[doc = "Field `INV` writer - Invert Comparator Output"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, INVSELECT_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Analog comparator output is directly processed."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(INVSELECT_A::DIS)
    }
    #[doc = "Analog comparator output is inverted prior to being processed."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(INVSELECT_A::EN)
    }
}
#[doc = "Field `SELFS` reader - Selection Of Fault Source"]
pub type SELFS_R = crate::BitReader<SELFSSELECT_A>;
#[doc = "Selection Of Fault Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELFSSELECT_A {
    #[doc = "0: The CE flag is used to drive the FAULT output."]
    CE = 0,
    #[doc = "1: The output of the analog comparator flag is used to drive the FAULT output."]
    OUTPUT = 1,
}
impl From<SELFSSELECT_A> for bool {
    #[inline(always)]
    fn from(variant: SELFSSELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl SELFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELFSSELECT_A {
        match self.bits {
            false => SELFSSELECT_A::CE,
            true => SELFSSELECT_A::OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `CE`"]
    #[inline(always)]
    pub fn is_ce(&self) -> bool {
        *self == SELFSSELECT_A::CE
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == SELFSSELECT_A::OUTPUT
    }
}
#[doc = "Field `SELFS` writer - Selection Of Fault Source"]
pub type SELFS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, SELFSSELECT_A, O>;
impl<'a, const O: u8> SELFS_W<'a, O> {
    #[doc = "The CE flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn ce(self) -> &'a mut W {
        self.variant(SELFSSELECT_A::CE)
    }
    #[doc = "The output of the analog comparator flag is used to drive the FAULT output."]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(SELFSSELECT_A::OUTPUT)
    }
}
#[doc = "Field `FE` reader - Fault Enable"]
pub type FE_R = crate::BitReader<FESELECT_A>;
#[doc = "Fault Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FESELECT_A {
    #[doc = "0: The FAULT output is tied to 0."]
    DIS = 0,
    #[doc = "1: The FAULT output is driven by the signal defined by SELFS."]
    EN = 1,
}
impl From<FESELECT_A> for bool {
    #[inline(always)]
    fn from(variant: FESELECT_A) -> Self {
        variant as u8 != 0
    }
}
impl FE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FESELECT_A {
        match self.bits {
            false => FESELECT_A::DIS,
            true => FESELECT_A::EN,
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == FESELECT_A::DIS
    }
    #[doc = "Checks if the value of the field is `EN`"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == FESELECT_A::EN
    }
}
#[doc = "Field `FE` writer - Fault Enable"]
pub type FE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, FESELECT_A, O>;
impl<'a, const O: u8> FE_W<'a, O> {
    #[doc = "The FAULT output is tied to 0."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(FESELECT_A::DIS)
    }
    #[doc = "The FAULT output is driven by the signal defined by SELFS."]
    #[inline(always)]
    pub fn en(self) -> &'a mut W {
        self.variant(FESELECT_A::EN)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&self) -> SELMINUS_R {
        SELMINUS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&self) -> SELPLUS_R {
        SELPLUS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&self) -> ACEN_R {
        ACEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&self) -> EDGETYP_R {
        EDGETYP_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&self) -> SELFS_R {
        SELFS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&self) -> FE_R {
        FE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selection for Minus Comparator Input"]
    #[inline(always)]
    pub fn selminus(&mut self) -> SELMINUS_W<0> {
        SELMINUS_W::new(self)
    }
    #[doc = "Bits 4:6 - Selection For Plus Comparator Input"]
    #[inline(always)]
    pub fn selplus(&mut self) -> SELPLUS_W<4> {
        SELPLUS_W::new(self)
    }
    #[doc = "Bit 8 - Analog Comparator Enable"]
    #[inline(always)]
    pub fn acen(&mut self) -> ACEN_W<8> {
        ACEN_W::new(self)
    }
    #[doc = "Bits 9:10 - Edge Type"]
    #[inline(always)]
    pub fn edgetyp(&mut self) -> EDGETYP_W<9> {
        EDGETYP_W::new(self)
    }
    #[doc = "Bit 12 - Invert Comparator Output"]
    #[inline(always)]
    pub fn inv(&mut self) -> INV_W<12> {
        INV_W::new(self)
    }
    #[doc = "Bit 13 - Selection Of Fault Source"]
    #[inline(always)]
    pub fn selfs(&mut self) -> SELFS_W<13> {
        SELFS_W::new(self)
    }
    #[doc = "Bit 14 - Fault Enable"]
    #[inline(always)]
    pub fn fe(&mut self) -> FE_W<14> {
        FE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
