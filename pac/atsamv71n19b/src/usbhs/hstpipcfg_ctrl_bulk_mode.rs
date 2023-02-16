#[doc = "Register `HSTPIPCFG_CTRL_BULK_MODE[%s]` reader"]
pub struct R(crate::R<HSTPIPCFG_CTRL_BULK_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSTPIPCFG_CTRL_BULK_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSTPIPCFG_CTRL_BULK_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSTPIPCFG_CTRL_BULK_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSTPIPCFG_CTRL_BULK_MODE[%s]` writer"]
pub struct W(crate::W<HSTPIPCFG_CTRL_BULK_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSTPIPCFG_CTRL_BULK_MODE_SPEC>;
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
impl From<crate::W<HSTPIPCFG_CTRL_BULK_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSTPIPCFG_CTRL_BULK_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALLOC` reader - Pipe Memory Allocate"]
pub type ALLOC_R = crate::BitReader<bool>;
#[doc = "Field `ALLOC` writer - Pipe Memory Allocate"]
pub type ALLOC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, bool, O>;
#[doc = "Field `PBK` reader - Pipe Banks"]
pub type PBK_R = crate::FieldReader<u8, PBKSELECT_A>;
#[doc = "Pipe Banks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBKSELECT_A {
    #[doc = "0: Single-bank pipe"]
    _1_BANK = 0,
    #[doc = "1: Double-bank pipe"]
    _2_BANK = 1,
    #[doc = "2: Triple-bank pipe"]
    _3_BANK = 2,
}
impl From<PBKSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PBKSELECT_A) -> Self {
        variant as _
    }
}
impl PBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PBKSELECT_A> {
        match self.bits {
            0 => Some(PBKSELECT_A::_1_BANK),
            1 => Some(PBKSELECT_A::_2_BANK),
            2 => Some(PBKSELECT_A::_3_BANK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1_BANK`"]
    #[inline(always)]
    pub fn is_1_bank(&self) -> bool {
        *self == PBKSELECT_A::_1_BANK
    }
    #[doc = "Checks if the value of the field is `_2_BANK`"]
    #[inline(always)]
    pub fn is_2_bank(&self) -> bool {
        *self == PBKSELECT_A::_2_BANK
    }
    #[doc = "Checks if the value of the field is `_3_BANK`"]
    #[inline(always)]
    pub fn is_3_bank(&self) -> bool {
        *self == PBKSELECT_A::_3_BANK
    }
}
#[doc = "Field `PBK` writer - Pipe Banks"]
pub type PBK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, u8, PBKSELECT_A, 2, O>;
impl<'a, const O: u8> PBK_W<'a, O> {
    #[doc = "Single-bank pipe"]
    #[inline(always)]
    pub fn _1_bank(self) -> &'a mut W {
        self.variant(PBKSELECT_A::_1_BANK)
    }
    #[doc = "Double-bank pipe"]
    #[inline(always)]
    pub fn _2_bank(self) -> &'a mut W {
        self.variant(PBKSELECT_A::_2_BANK)
    }
    #[doc = "Triple-bank pipe"]
    #[inline(always)]
    pub fn _3_bank(self) -> &'a mut W {
        self.variant(PBKSELECT_A::_3_BANK)
    }
}
#[doc = "Field `PSIZE` reader - Pipe Size"]
pub type PSIZE_R = crate::FieldReader<u8, PSIZESELECT_A>;
#[doc = "Pipe Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZESELECT_A {
    #[doc = "0: 8 bytes"]
    _8_BYTE = 0,
    #[doc = "1: 16 bytes"]
    _16_BYTE = 1,
    #[doc = "2: 32 bytes"]
    _32_BYTE = 2,
    #[doc = "3: 64 bytes"]
    _64_BYTE = 3,
    #[doc = "4: 128 bytes"]
    _128_BYTE = 4,
    #[doc = "5: 256 bytes"]
    _256_BYTE = 5,
    #[doc = "6: 512 bytes"]
    _512_BYTE = 6,
    #[doc = "7: 1024 bytes"]
    _1024_BYTE = 7,
}
impl From<PSIZESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZESELECT_A) -> Self {
        variant as _
    }
}
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSIZESELECT_A {
        match self.bits {
            0 => PSIZESELECT_A::_8_BYTE,
            1 => PSIZESELECT_A::_16_BYTE,
            2 => PSIZESELECT_A::_32_BYTE,
            3 => PSIZESELECT_A::_64_BYTE,
            4 => PSIZESELECT_A::_128_BYTE,
            5 => PSIZESELECT_A::_256_BYTE,
            6 => PSIZESELECT_A::_512_BYTE,
            7 => PSIZESELECT_A::_1024_BYTE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_8_BYTE`"]
    #[inline(always)]
    pub fn is_8_byte(&self) -> bool {
        *self == PSIZESELECT_A::_8_BYTE
    }
    #[doc = "Checks if the value of the field is `_16_BYTE`"]
    #[inline(always)]
    pub fn is_16_byte(&self) -> bool {
        *self == PSIZESELECT_A::_16_BYTE
    }
    #[doc = "Checks if the value of the field is `_32_BYTE`"]
    #[inline(always)]
    pub fn is_32_byte(&self) -> bool {
        *self == PSIZESELECT_A::_32_BYTE
    }
    #[doc = "Checks if the value of the field is `_64_BYTE`"]
    #[inline(always)]
    pub fn is_64_byte(&self) -> bool {
        *self == PSIZESELECT_A::_64_BYTE
    }
    #[doc = "Checks if the value of the field is `_128_BYTE`"]
    #[inline(always)]
    pub fn is_128_byte(&self) -> bool {
        *self == PSIZESELECT_A::_128_BYTE
    }
    #[doc = "Checks if the value of the field is `_256_BYTE`"]
    #[inline(always)]
    pub fn is_256_byte(&self) -> bool {
        *self == PSIZESELECT_A::_256_BYTE
    }
    #[doc = "Checks if the value of the field is `_512_BYTE`"]
    #[inline(always)]
    pub fn is_512_byte(&self) -> bool {
        *self == PSIZESELECT_A::_512_BYTE
    }
    #[doc = "Checks if the value of the field is `_1024_BYTE`"]
    #[inline(always)]
    pub fn is_1024_byte(&self) -> bool {
        *self == PSIZESELECT_A::_1024_BYTE
    }
}
#[doc = "Field `PSIZE` writer - Pipe Size"]
pub type PSIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, u8, PSIZESELECT_A, 3, O>;
impl<'a, const O: u8> PSIZE_W<'a, O> {
    #[doc = "8 bytes"]
    #[inline(always)]
    pub fn _8_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_8_BYTE)
    }
    #[doc = "16 bytes"]
    #[inline(always)]
    pub fn _16_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_16_BYTE)
    }
    #[doc = "32 bytes"]
    #[inline(always)]
    pub fn _32_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_32_BYTE)
    }
    #[doc = "64 bytes"]
    #[inline(always)]
    pub fn _64_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_64_BYTE)
    }
    #[doc = "128 bytes"]
    #[inline(always)]
    pub fn _128_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_128_BYTE)
    }
    #[doc = "256 bytes"]
    #[inline(always)]
    pub fn _256_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_256_BYTE)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn _512_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_512_BYTE)
    }
    #[doc = "1024 bytes"]
    #[inline(always)]
    pub fn _1024_byte(self) -> &'a mut W {
        self.variant(PSIZESELECT_A::_1024_BYTE)
    }
}
#[doc = "Field `PTOKEN` reader - Pipe Token"]
pub type PTOKEN_R = crate::FieldReader<u8, PTOKENSELECT_A>;
#[doc = "Pipe Token\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTOKENSELECT_A {
    #[doc = "0: SETUP"]
    SETUP = 0,
    #[doc = "1: IN"]
    IN = 1,
    #[doc = "2: OUT"]
    OUT = 2,
}
impl From<PTOKENSELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PTOKENSELECT_A) -> Self {
        variant as _
    }
}
impl PTOKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PTOKENSELECT_A> {
        match self.bits {
            0 => Some(PTOKENSELECT_A::SETUP),
            1 => Some(PTOKENSELECT_A::IN),
            2 => Some(PTOKENSELECT_A::OUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SETUP`"]
    #[inline(always)]
    pub fn is_setup(&self) -> bool {
        *self == PTOKENSELECT_A::SETUP
    }
    #[doc = "Checks if the value of the field is `IN`"]
    #[inline(always)]
    pub fn is_in(&self) -> bool {
        *self == PTOKENSELECT_A::IN
    }
    #[doc = "Checks if the value of the field is `OUT`"]
    #[inline(always)]
    pub fn is_out(&self) -> bool {
        *self == PTOKENSELECT_A::OUT
    }
}
#[doc = "Field `PTOKEN` writer - Pipe Token"]
pub type PTOKEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, u8, PTOKENSELECT_A, 2, O>;
impl<'a, const O: u8> PTOKEN_W<'a, O> {
    #[doc = "SETUP"]
    #[inline(always)]
    pub fn setup(self) -> &'a mut W {
        self.variant(PTOKENSELECT_A::SETUP)
    }
    #[doc = "IN"]
    #[inline(always)]
    pub fn in_(self) -> &'a mut W {
        self.variant(PTOKENSELECT_A::IN)
    }
    #[doc = "OUT"]
    #[inline(always)]
    pub fn out(self) -> &'a mut W {
        self.variant(PTOKENSELECT_A::OUT)
    }
}
#[doc = "Field `AUTOSW` reader - Automatic Switch"]
pub type AUTOSW_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSW` writer - Automatic Switch"]
pub type AUTOSW_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, bool, O>;
#[doc = "Field `PTYPE` reader - Pipe Type"]
pub type PTYPE_R = crate::FieldReader<u8, PTYPESELECT_A>;
#[doc = "Pipe Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PTYPESELECT_A {
    #[doc = "0: Control"]
    CTRL = 0,
    #[doc = "1: Isochronous"]
    ISO = 1,
    #[doc = "2: Bulk"]
    BLK = 2,
    #[doc = "3: Interrupt"]
    INTRPT = 3,
}
impl From<PTYPESELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PTYPESELECT_A) -> Self {
        variant as _
    }
}
impl PTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTYPESELECT_A {
        match self.bits {
            0 => PTYPESELECT_A::CTRL,
            1 => PTYPESELECT_A::ISO,
            2 => PTYPESELECT_A::BLK,
            3 => PTYPESELECT_A::INTRPT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTRL`"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == PTYPESELECT_A::CTRL
    }
    #[doc = "Checks if the value of the field is `ISO`"]
    #[inline(always)]
    pub fn is_iso(&self) -> bool {
        *self == PTYPESELECT_A::ISO
    }
    #[doc = "Checks if the value of the field is `BLK`"]
    #[inline(always)]
    pub fn is_blk(&self) -> bool {
        *self == PTYPESELECT_A::BLK
    }
    #[doc = "Checks if the value of the field is `INTRPT`"]
    #[inline(always)]
    pub fn is_intrpt(&self) -> bool {
        *self == PTYPESELECT_A::INTRPT
    }
}
#[doc = "Field `PTYPE` writer - Pipe Type"]
pub type PTYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, u8, PTYPESELECT_A, 2, O>;
impl<'a, const O: u8> PTYPE_W<'a, O> {
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::CTRL)
    }
    #[doc = "Isochronous"]
    #[inline(always)]
    pub fn iso(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::ISO)
    }
    #[doc = "Bulk"]
    #[inline(always)]
    pub fn blk(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::BLK)
    }
    #[doc = "Interrupt"]
    #[inline(always)]
    pub fn intrpt(self) -> &'a mut W {
        self.variant(PTYPESELECT_A::INTRPT)
    }
}
#[doc = "Field `PEPNUM` reader - Pipe Endpoint Number"]
pub type PEPNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PEPNUM` writer - Pipe Endpoint Number"]
pub type PEPNUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, u8, u8, 4, O>;
#[doc = "Field `PINGEN` reader - Ping Enable"]
pub type PINGEN_R = crate::BitReader<bool>;
#[doc = "Field `PINGEN` writer - Ping Enable"]
pub type PINGEN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, bool, O>;
#[doc = "Field `BINTERVAL` reader - bInterval Parameter for the Bulk-Out/Ping Transaction"]
pub type BINTERVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BINTERVAL` writer - bInterval Parameter for the Bulk-Out/Ping Transaction"]
pub type BINTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HSTPIPCFG_CTRL_BULK_MODE_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&self) -> ALLOC_R {
        ALLOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&self) -> PBK_R {
        PBK_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&self) -> PTOKEN_R {
        PTOKEN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&self) -> AUTOSW_R {
        AUTOSW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&self) -> PTYPE_R {
        PTYPE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&self) -> PEPNUM_R {
        PEPNUM_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&self) -> PINGEN_R {
        PINGEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:31 - bInterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&self) -> BINTERVAL_R {
        BINTERVAL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Pipe Memory Allocate"]
    #[inline(always)]
    pub fn alloc(&mut self) -> ALLOC_W<1> {
        ALLOC_W::new(self)
    }
    #[doc = "Bits 2:3 - Pipe Banks"]
    #[inline(always)]
    pub fn pbk(&mut self) -> PBK_W<2> {
        PBK_W::new(self)
    }
    #[doc = "Bits 4:6 - Pipe Size"]
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<4> {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 8:9 - Pipe Token"]
    #[inline(always)]
    pub fn ptoken(&mut self) -> PTOKEN_W<8> {
        PTOKEN_W::new(self)
    }
    #[doc = "Bit 10 - Automatic Switch"]
    #[inline(always)]
    pub fn autosw(&mut self) -> AUTOSW_W<10> {
        AUTOSW_W::new(self)
    }
    #[doc = "Bits 12:13 - Pipe Type"]
    #[inline(always)]
    pub fn ptype(&mut self) -> PTYPE_W<12> {
        PTYPE_W::new(self)
    }
    #[doc = "Bits 16:19 - Pipe Endpoint Number"]
    #[inline(always)]
    pub fn pepnum(&mut self) -> PEPNUM_W<16> {
        PEPNUM_W::new(self)
    }
    #[doc = "Bit 20 - Ping Enable"]
    #[inline(always)]
    pub fn pingen(&mut self) -> PINGEN_W<20> {
        PINGEN_W::new(self)
    }
    #[doc = "Bits 24:31 - bInterval Parameter for the Bulk-Out/Ping Transaction"]
    #[inline(always)]
    pub fn binterval(&mut self) -> BINTERVAL_W<24> {
        BINTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Host Pipe Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hstpipcfg_ctrl_bulk_mode](index.html) module"]
pub struct HSTPIPCFG_CTRL_BULK_MODE_SPEC;
impl crate::RegisterSpec for HSTPIPCFG_CTRL_BULK_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hstpipcfg_ctrl_bulk_mode::R](R) reader structure"]
impl crate::Readable for HSTPIPCFG_CTRL_BULK_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hstpipcfg_ctrl_bulk_mode::W](W) writer structure"]
impl crate::Writable for HSTPIPCFG_CTRL_BULK_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSTPIPCFG_CTRL_BULK_MODE[%s]
to value 0"]
impl crate::Resettable for HSTPIPCFG_CTRL_BULK_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
