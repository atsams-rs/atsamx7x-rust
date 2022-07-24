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
#[doc = "Field `CIPHER` reader - Processing Mode"]
pub type CIPHER_R = crate::BitReader<bool>;
#[doc = "Field `CIPHER` writer - Processing Mode"]
pub type CIPHER_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `GTAGEN` reader - GCM Automatic Tag Generation Enable"]
pub type GTAGEN_R = crate::BitReader<bool>;
#[doc = "Field `GTAGEN` writer - GCM Automatic Tag Generation Enable"]
pub type GTAGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Dual Input Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALBUFF_A {
    #[doc = "0: AES_IDATARx cannot be written during processing of previous block."]
    INACTIVE = 0,
    #[doc = "1: AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    ACTIVE = 1,
}
impl From<DUALBUFF_A> for bool {
    #[inline(always)]
    fn from(variant: DUALBUFF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUALBUFF` reader - Dual Input Buffer"]
pub type DUALBUFF_R = crate::BitReader<DUALBUFF_A>;
impl DUALBUFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUALBUFF_A {
        match self.bits {
            false => DUALBUFF_A::INACTIVE,
            true => DUALBUFF_A::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DUALBUFF_A::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DUALBUFF_A::ACTIVE
    }
}
#[doc = "Field `DUALBUFF` writer - Dual Input Buffer"]
pub type DUALBUFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, DUALBUFF_A, O>;
impl<'a, const O: u8> DUALBUFF_W<'a, O> {
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DUALBUFF_A::INACTIVE)
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(DUALBUFF_A::ACTIVE)
    }
}
#[doc = "Field `PROCDLY` reader - Processing Delay"]
pub type PROCDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROCDLY` writer - Processing Delay"]
pub type PROCDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 4, O>;
#[doc = "Start Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Manual Mode"]
    MANUAL_START = 0,
    #[doc = "1: Auto Mode"]
    AUTO_START = 1,
    #[doc = "2: AES_IDATAR0 access only Auto Mode (DMA)"]
    IDATAR0_START = 2,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SMOD` reader - Start Mode"]
pub type SMOD_R = crate::FieldReader<u8, SMOD_A>;
impl SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD_A> {
        match self.bits {
            0 => Some(SMOD_A::MANUAL_START),
            1 => Some(SMOD_A::AUTO_START),
            2 => Some(SMOD_A::IDATAR0_START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL_START`"]
    #[inline(always)]
    pub fn is_manual_start(&self) -> bool {
        *self == SMOD_A::MANUAL_START
    }
    #[doc = "Checks if the value of the field is `AUTO_START`"]
    #[inline(always)]
    pub fn is_auto_start(&self) -> bool {
        *self == SMOD_A::AUTO_START
    }
    #[doc = "Checks if the value of the field is `IDATAR0_START`"]
    #[inline(always)]
    pub fn is_idatar0_start(&self) -> bool {
        *self == SMOD_A::IDATAR0_START
    }
}
#[doc = "Field `SMOD` writer - Start Mode"]
pub type SMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, SMOD_A, 2, O>;
impl<'a, const O: u8> SMOD_W<'a, O> {
    #[doc = "Manual Mode"]
    #[inline(always)]
    pub fn manual_start(self) -> &'a mut W {
        self.variant(SMOD_A::MANUAL_START)
    }
    #[doc = "Auto Mode"]
    #[inline(always)]
    pub fn auto_start(self) -> &'a mut W {
        self.variant(SMOD_A::AUTO_START)
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline(always)]
    pub fn idatar0_start(self) -> &'a mut W {
        self.variant(SMOD_A::IDATAR0_START)
    }
}
#[doc = "Key Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum KEYSIZE_A {
    #[doc = "0: AES Key Size is 128 bits"]
    AES128 = 0,
    #[doc = "1: AES Key Size is 192 bits"]
    AES192 = 1,
    #[doc = "2: AES Key Size is 256 bits"]
    AES256 = 2,
}
impl From<KEYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYSIZE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `KEYSIZE` reader - Key Size"]
pub type KEYSIZE_R = crate::FieldReader<u8, KEYSIZE_A>;
impl KEYSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYSIZE_A> {
        match self.bits {
            0 => Some(KEYSIZE_A::AES128),
            1 => Some(KEYSIZE_A::AES192),
            2 => Some(KEYSIZE_A::AES256),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEYSIZE_A::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KEYSIZE_A::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEYSIZE_A::AES256
    }
}
#[doc = "Field `KEYSIZE` writer - Key Size"]
pub type KEYSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, KEYSIZE_A, 2, O>;
impl<'a, const O: u8> KEYSIZE_W<'a, O> {
    #[doc = "AES Key Size is 128 bits"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEYSIZE_A::AES128)
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEYSIZE_A::AES192)
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEYSIZE_A::AES256)
    }
}
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OPMOD_A {
    #[doc = "0: ECB: Electronic Code Book mode"]
    ECB = 0,
    #[doc = "1: CBC: Cipher Block Chaining mode"]
    CBC = 1,
    #[doc = "2: OFB: Output Feedback mode"]
    OFB = 2,
    #[doc = "3: CFB: Cipher Feedback mode"]
    CFB = 3,
    #[doc = "4: CTR: Counter mode (16-bit internal counter)"]
    CTR = 4,
    #[doc = "5: GCM: Galois/Counter mode"]
    GCM = 5,
}
impl From<OPMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: OPMOD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `OPMOD` reader - Operating Mode"]
pub type OPMOD_R = crate::FieldReader<u8, OPMOD_A>;
impl OPMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPMOD_A> {
        match self.bits {
            0 => Some(OPMOD_A::ECB),
            1 => Some(OPMOD_A::CBC),
            2 => Some(OPMOD_A::OFB),
            3 => Some(OPMOD_A::CFB),
            4 => Some(OPMOD_A::CTR),
            5 => Some(OPMOD_A::GCM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == OPMOD_A::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == OPMOD_A::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == OPMOD_A::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == OPMOD_A::CFB
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == OPMOD_A::CTR
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == OPMOD_A::GCM
    }
}
#[doc = "Field `OPMOD` writer - Operating Mode"]
pub type OPMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, OPMOD_A, 3, O>;
impl<'a, const O: u8> OPMOD_W<'a, O> {
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(OPMOD_A::ECB)
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(OPMOD_A::CBC)
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut W {
        self.variant(OPMOD_A::OFB)
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut W {
        self.variant(OPMOD_A::CFB)
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(OPMOD_A::CTR)
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut W {
        self.variant(OPMOD_A::GCM)
    }
}
#[doc = "Field `LOD` reader - Last Output Data Mode"]
pub type LOD_R = crate::BitReader<bool>;
#[doc = "Field `LOD` writer - Last Output Data Mode"]
pub type LOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Cipher Feedback Data Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CFBS_A {
    #[doc = "0: 128-bit"]
    SIZE_128BIT = 0,
    #[doc = "1: 64-bit"]
    SIZE_64BIT = 1,
    #[doc = "2: 32-bit"]
    SIZE_32BIT = 2,
    #[doc = "3: 16-bit"]
    SIZE_16BIT = 3,
    #[doc = "4: 8-bit"]
    SIZE_8BIT = 4,
}
impl From<CFBS_A> for u8 {
    #[inline(always)]
    fn from(variant: CFBS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CFBS` reader - Cipher Feedback Data Size"]
pub type CFBS_R = crate::FieldReader<u8, CFBS_A>;
impl CFBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CFBS_A> {
        match self.bits {
            0 => Some(CFBS_A::SIZE_128BIT),
            1 => Some(CFBS_A::SIZE_64BIT),
            2 => Some(CFBS_A::SIZE_32BIT),
            3 => Some(CFBS_A::SIZE_16BIT),
            4 => Some(CFBS_A::SIZE_8BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_128BIT`"]
    #[inline(always)]
    pub fn is_size_128bit(&self) -> bool {
        *self == CFBS_A::SIZE_128BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_64BIT`"]
    #[inline(always)]
    pub fn is_size_64bit(&self) -> bool {
        *self == CFBS_A::SIZE_64BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_32BIT`"]
    #[inline(always)]
    pub fn is_size_32bit(&self) -> bool {
        *self == CFBS_A::SIZE_32BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_16BIT`"]
    #[inline(always)]
    pub fn is_size_16bit(&self) -> bool {
        *self == CFBS_A::SIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_8BIT`"]
    #[inline(always)]
    pub fn is_size_8bit(&self) -> bool {
        *self == CFBS_A::SIZE_8BIT
    }
}
#[doc = "Field `CFBS` writer - Cipher Feedback Data Size"]
pub type CFBS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, CFBS_A, 3, O>;
impl<'a, const O: u8> CFBS_W<'a, O> {
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn size_128bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_128BIT)
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn size_64bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_64BIT)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn size_32bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_32BIT)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn size_16bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_16BIT)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn size_8bit(self) -> &'a mut W {
        self.variant(CFBS_A::SIZE_8BIT)
    }
}
#[doc = "Countermeasure Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKEY_A {
    #[doc = "14: This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    PASSWD = 14,
}
impl From<CKEY_A> for u8 {
    #[inline(always)]
    fn from(variant: CKEY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CKEY` reader - Countermeasure Key"]
pub type CKEY_R = crate::FieldReader<u8, CKEY_A>;
impl CKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKEY_A> {
        match self.bits {
            14 => Some(CKEY_A::PASSWD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == CKEY_A::PASSWD
    }
}
#[doc = "Field `CKEY` writer - Countermeasure Key"]
pub type CKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, CKEY_A, 4, O>;
impl<'a, const O: u8> CKEY_W<'a, O> {
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(CKEY_A::PASSWD)
    }
}
#[doc = "Countermeasure Type 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTYP1_A {
    #[doc = "0: Countermeasure type 1 is disabled."]
    NOPROT_EXTKEY = 0,
    #[doc = "1: Countermeasure type 1 is enabled."]
    PROT_EXTKEY = 1,
}
impl From<CMTYP1_A> for bool {
    #[inline(always)]
    fn from(variant: CMTYP1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTYP1` reader - Countermeasure Type 1"]
pub type CMTYP1_R = crate::BitReader<CMTYP1_A>;
impl CMTYP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTYP1_A {
        match self.bits {
            false => CMTYP1_A::NOPROT_EXTKEY,
            true => CMTYP1_A::PROT_EXTKEY,
        }
    }
    #[doc = "Checks if the value of the field is `NOPROT_EXTKEY`"]
    #[inline(always)]
    pub fn is_noprot_extkey(&self) -> bool {
        *self == CMTYP1_A::NOPROT_EXTKEY
    }
    #[doc = "Checks if the value of the field is `PROT_EXTKEY`"]
    #[inline(always)]
    pub fn is_prot_extkey(&self) -> bool {
        *self == CMTYP1_A::PROT_EXTKEY
    }
}
#[doc = "Field `CMTYP1` writer - Countermeasure Type 1"]
pub type CMTYP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, CMTYP1_A, O>;
impl<'a, const O: u8> CMTYP1_W<'a, O> {
    #[doc = "Countermeasure type 1 is disabled."]
    #[inline(always)]
    pub fn noprot_extkey(self) -> &'a mut W {
        self.variant(CMTYP1_A::NOPROT_EXTKEY)
    }
    #[doc = "Countermeasure type 1 is enabled."]
    #[inline(always)]
    pub fn prot_extkey(self) -> &'a mut W {
        self.variant(CMTYP1_A::PROT_EXTKEY)
    }
}
#[doc = "Countermeasure Type 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTYP2_A {
    #[doc = "0: Countermeasure type 2 is disabled."]
    NO_PAUSE = 0,
    #[doc = "1: Countermeasure type 2 is enabled."]
    PAUSE = 1,
}
impl From<CMTYP2_A> for bool {
    #[inline(always)]
    fn from(variant: CMTYP2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTYP2` reader - Countermeasure Type 2"]
pub type CMTYP2_R = crate::BitReader<CMTYP2_A>;
impl CMTYP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTYP2_A {
        match self.bits {
            false => CMTYP2_A::NO_PAUSE,
            true => CMTYP2_A::PAUSE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PAUSE`"]
    #[inline(always)]
    pub fn is_no_pause(&self) -> bool {
        *self == CMTYP2_A::NO_PAUSE
    }
    #[doc = "Checks if the value of the field is `PAUSE`"]
    #[inline(always)]
    pub fn is_pause(&self) -> bool {
        *self == CMTYP2_A::PAUSE
    }
}
#[doc = "Field `CMTYP2` writer - Countermeasure Type 2"]
pub type CMTYP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, CMTYP2_A, O>;
impl<'a, const O: u8> CMTYP2_W<'a, O> {
    #[doc = "Countermeasure type 2 is disabled."]
    #[inline(always)]
    pub fn no_pause(self) -> &'a mut W {
        self.variant(CMTYP2_A::NO_PAUSE)
    }
    #[doc = "Countermeasure type 2 is enabled."]
    #[inline(always)]
    pub fn pause(self) -> &'a mut W {
        self.variant(CMTYP2_A::PAUSE)
    }
}
#[doc = "Countermeasure Type 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTYP3_A {
    #[doc = "0: Countermeasure type 3 is disabled."]
    NO_DUMMY = 0,
    #[doc = "1: Countermeasure type 3 is enabled."]
    DUMMY = 1,
}
impl From<CMTYP3_A> for bool {
    #[inline(always)]
    fn from(variant: CMTYP3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTYP3` reader - Countermeasure Type 3"]
pub type CMTYP3_R = crate::BitReader<CMTYP3_A>;
impl CMTYP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTYP3_A {
        match self.bits {
            false => CMTYP3_A::NO_DUMMY,
            true => CMTYP3_A::DUMMY,
        }
    }
    #[doc = "Checks if the value of the field is `NO_DUMMY`"]
    #[inline(always)]
    pub fn is_no_dummy(&self) -> bool {
        *self == CMTYP3_A::NO_DUMMY
    }
    #[doc = "Checks if the value of the field is `DUMMY`"]
    #[inline(always)]
    pub fn is_dummy(&self) -> bool {
        *self == CMTYP3_A::DUMMY
    }
}
#[doc = "Field `CMTYP3` writer - Countermeasure Type 3"]
pub type CMTYP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, CMTYP3_A, O>;
impl<'a, const O: u8> CMTYP3_W<'a, O> {
    #[doc = "Countermeasure type 3 is disabled."]
    #[inline(always)]
    pub fn no_dummy(self) -> &'a mut W {
        self.variant(CMTYP3_A::NO_DUMMY)
    }
    #[doc = "Countermeasure type 3 is enabled."]
    #[inline(always)]
    pub fn dummy(self) -> &'a mut W {
        self.variant(CMTYP3_A::DUMMY)
    }
}
#[doc = "Countermeasure Type 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTYP4_A {
    #[doc = "0: Countermeasure type 4 is disabled."]
    NO_RESTART = 0,
    #[doc = "1: Countermeasure type 4 is enabled."]
    RESTART = 1,
}
impl From<CMTYP4_A> for bool {
    #[inline(always)]
    fn from(variant: CMTYP4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTYP4` reader - Countermeasure Type 4"]
pub type CMTYP4_R = crate::BitReader<CMTYP4_A>;
impl CMTYP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTYP4_A {
        match self.bits {
            false => CMTYP4_A::NO_RESTART,
            true => CMTYP4_A::RESTART,
        }
    }
    #[doc = "Checks if the value of the field is `NO_RESTART`"]
    #[inline(always)]
    pub fn is_no_restart(&self) -> bool {
        *self == CMTYP4_A::NO_RESTART
    }
    #[doc = "Checks if the value of the field is `RESTART`"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == CMTYP4_A::RESTART
    }
}
#[doc = "Field `CMTYP4` writer - Countermeasure Type 4"]
pub type CMTYP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, CMTYP4_A, O>;
impl<'a, const O: u8> CMTYP4_W<'a, O> {
    #[doc = "Countermeasure type 4 is disabled."]
    #[inline(always)]
    pub fn no_restart(self) -> &'a mut W {
        self.variant(CMTYP4_A::NO_RESTART)
    }
    #[doc = "Countermeasure type 4 is enabled."]
    #[inline(always)]
    pub fn restart(self) -> &'a mut W {
        self.variant(CMTYP4_A::RESTART)
    }
}
#[doc = "Countermeasure Type 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTYP5_A {
    #[doc = "0: Countermeasure type 5 is disabled."]
    NO_ADDACCESS = 0,
    #[doc = "1: Countermeasure type 5 is enabled."]
    ADDACCESS = 1,
}
impl From<CMTYP5_A> for bool {
    #[inline(always)]
    fn from(variant: CMTYP5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTYP5` reader - Countermeasure Type 5"]
pub type CMTYP5_R = crate::BitReader<CMTYP5_A>;
impl CMTYP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTYP5_A {
        match self.bits {
            false => CMTYP5_A::NO_ADDACCESS,
            true => CMTYP5_A::ADDACCESS,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ADDACCESS`"]
    #[inline(always)]
    pub fn is_no_addaccess(&self) -> bool {
        *self == CMTYP5_A::NO_ADDACCESS
    }
    #[doc = "Checks if the value of the field is `ADDACCESS`"]
    #[inline(always)]
    pub fn is_addaccess(&self) -> bool {
        *self == CMTYP5_A::ADDACCESS
    }
}
#[doc = "Field `CMTYP5` writer - Countermeasure Type 5"]
pub type CMTYP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, CMTYP5_A, O>;
impl<'a, const O: u8> CMTYP5_W<'a, O> {
    #[doc = "Countermeasure type 5 is disabled."]
    #[inline(always)]
    pub fn no_addaccess(self) -> &'a mut W {
        self.variant(CMTYP5_A::NO_ADDACCESS)
    }
    #[doc = "Countermeasure type 5 is enabled."]
    #[inline(always)]
    pub fn addaccess(self) -> &'a mut W {
        self.variant(CMTYP5_A::ADDACCESS)
    }
}
#[doc = "Countermeasure Type 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CMTYP6_A {
    #[doc = "0: Countermeasure type 6 is disabled."]
    NO_IDLECURRENT = 0,
    #[doc = "1: Countermeasure type 6 is enabled."]
    IDLECURRENT = 1,
}
impl From<CMTYP6_A> for bool {
    #[inline(always)]
    fn from(variant: CMTYP6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMTYP6` reader - Countermeasure Type 6"]
pub type CMTYP6_R = crate::BitReader<CMTYP6_A>;
impl CMTYP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTYP6_A {
        match self.bits {
            false => CMTYP6_A::NO_IDLECURRENT,
            true => CMTYP6_A::IDLECURRENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_IDLECURRENT`"]
    #[inline(always)]
    pub fn is_no_idlecurrent(&self) -> bool {
        *self == CMTYP6_A::NO_IDLECURRENT
    }
    #[doc = "Checks if the value of the field is `IDLECURRENT`"]
    #[inline(always)]
    pub fn is_idlecurrent(&self) -> bool {
        *self == CMTYP6_A::IDLECURRENT
    }
}
#[doc = "Field `CMTYP6` writer - Countermeasure Type 6"]
pub type CMTYP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, CMTYP6_A, O>;
impl<'a, const O: u8> CMTYP6_W<'a, O> {
    #[doc = "Countermeasure type 6 is disabled."]
    #[inline(always)]
    pub fn no_idlecurrent(self) -> &'a mut W {
        self.variant(CMTYP6_A::NO_IDLECURRENT)
    }
    #[doc = "Countermeasure type 6 is enabled."]
    #[inline(always)]
    pub fn idlecurrent(self) -> &'a mut W {
        self.variant(CMTYP6_A::IDLECURRENT)
    }
}
impl R {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&self) -> GTAGEN_R {
        GTAGEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&self) -> PROCDLY_R {
        PROCDLY_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&self) -> OPMOD_R {
        OPMOD_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LOD_R {
        LOD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&self) -> CKEY_R {
        CKEY_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Countermeasure Type 1"]
    #[inline(always)]
    pub fn cmtyp1(&self) -> CMTYP1_R {
        CMTYP1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Countermeasure Type 2"]
    #[inline(always)]
    pub fn cmtyp2(&self) -> CMTYP2_R {
        CMTYP2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Countermeasure Type 3"]
    #[inline(always)]
    pub fn cmtyp3(&self) -> CMTYP3_R {
        CMTYP3_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Countermeasure Type 4"]
    #[inline(always)]
    pub fn cmtyp4(&self) -> CMTYP4_R {
        CMTYP4_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Countermeasure Type 5"]
    #[inline(always)]
    pub fn cmtyp5(&self) -> CMTYP5_R {
        CMTYP5_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Countermeasure Type 6"]
    #[inline(always)]
    pub fn cmtyp6(&self) -> CMTYP6_R {
        CMTYP6_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&mut self) -> CIPHER_W<0> {
        CIPHER_W::new(self)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&mut self) -> GTAGEN_W<1> {
        GTAGEN_W::new(self)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&mut self) -> DUALBUFF_W<3> {
        DUALBUFF_W::new(self)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&mut self) -> PROCDLY_W<4> {
        PROCDLY_W::new(self)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&mut self) -> SMOD_W<8> {
        SMOD_W::new(self)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&mut self) -> KEYSIZE_W<10> {
        KEYSIZE_W::new(self)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&mut self) -> OPMOD_W<12> {
        OPMOD_W::new(self)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&mut self) -> LOD_W<15> {
        LOD_W::new(self)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&mut self) -> CFBS_W<16> {
        CFBS_W::new(self)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&mut self) -> CKEY_W<20> {
        CKEY_W::new(self)
    }
    #[doc = "Bit 24 - Countermeasure Type 1"]
    #[inline(always)]
    pub fn cmtyp1(&mut self) -> CMTYP1_W<24> {
        CMTYP1_W::new(self)
    }
    #[doc = "Bit 25 - Countermeasure Type 2"]
    #[inline(always)]
    pub fn cmtyp2(&mut self) -> CMTYP2_W<25> {
        CMTYP2_W::new(self)
    }
    #[doc = "Bit 26 - Countermeasure Type 3"]
    #[inline(always)]
    pub fn cmtyp3(&mut self) -> CMTYP3_W<26> {
        CMTYP3_W::new(self)
    }
    #[doc = "Bit 27 - Countermeasure Type 4"]
    #[inline(always)]
    pub fn cmtyp4(&mut self) -> CMTYP4_W<27> {
        CMTYP4_W::new(self)
    }
    #[doc = "Bit 28 - Countermeasure Type 5"]
    #[inline(always)]
    pub fn cmtyp5(&mut self) -> CMTYP5_W<28> {
        CMTYP5_W::new(self)
    }
    #[doc = "Bit 29 - Countermeasure Type 6"]
    #[inline(always)]
    pub fn cmtyp6(&mut self) -> CMTYP6_W<29> {
        CMTYP6_W::new(self)
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
