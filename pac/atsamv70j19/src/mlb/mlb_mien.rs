#[doc = "Register `MLB_MIEN` reader"]
pub struct R(crate::R<MLB_MIEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MLB_MIEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MLB_MIEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MLB_MIEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MLB_MIEN` writer"]
pub struct W(crate::W<MLB_MIEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MLB_MIEN_SPEC>;
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
impl From<crate::W<MLB_MIEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MLB_MIEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISOC_PE` reader - Isochronous Rx Protocol Error Enable"]
pub struct ISOC_PE_R(crate::FieldReader<bool, bool>);
impl ISOC_PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISOC_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOC_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOC_PE` writer - Isochronous Rx Protocol Error Enable"]
pub struct ISOC_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOC_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ISOC_BUFO` reader - Isochronous Rx Buffer Overflow Enable"]
pub struct ISOC_BUFO_R(crate::FieldReader<bool, bool>);
impl ISOC_BUFO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ISOC_BUFO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOC_BUFO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISOC_BUFO` writer - Isochronous Rx Buffer Overflow Enable"]
pub struct ISOC_BUFO_W<'a> {
    w: &'a mut W,
}
impl<'a> ISOC_BUFO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SYNC_PE` reader - Synchronous Protocol Error Enable"]
pub struct SYNC_PE_R(crate::FieldReader<bool, bool>);
impl SYNC_PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYNC_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYNC_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYNC_PE` writer - Synchronous Protocol Error Enable"]
pub struct SYNC_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYNC_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ARX_DONE` reader - Asynchronous Rx Done Enable"]
pub struct ARX_DONE_R(crate::FieldReader<bool, bool>);
impl ARX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARX_DONE` writer - Asynchronous Rx Done Enable"]
pub struct ARX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARX_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ARX_PE` reader - Asynchronous Rx Protocol Error Enable"]
pub struct ARX_PE_R(crate::FieldReader<bool, bool>);
impl ARX_PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARX_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARX_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARX_PE` writer - Asynchronous Rx Protocol Error Enable"]
pub struct ARX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> ARX_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `ARX_BREAK` reader - Asynchronous Rx Break Enable"]
pub struct ARX_BREAK_R(crate::FieldReader<bool, bool>);
impl ARX_BREAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ARX_BREAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARX_BREAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARX_BREAK` writer - Asynchronous Rx Break Enable"]
pub struct ARX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> ARX_BREAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `ATX_DONE` reader - Asynchronous Tx Packet Done Enable"]
pub struct ATX_DONE_R(crate::FieldReader<bool, bool>);
impl ATX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATX_DONE` writer - Asynchronous Tx Packet Done Enable"]
pub struct ATX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATX_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ATX_PE` reader - Asynchronous Tx Protocol Error Enable"]
pub struct ATX_PE_R(crate::FieldReader<bool, bool>);
impl ATX_PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATX_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATX_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATX_PE` writer - Asynchronous Tx Protocol Error Enable"]
pub struct ATX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> ATX_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `ATX_BREAK` reader - Asynchronous Tx Break Enable"]
pub struct ATX_BREAK_R(crate::FieldReader<bool, bool>);
impl ATX_BREAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ATX_BREAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ATX_BREAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ATX_BREAK` writer - Asynchronous Tx Break Enable"]
pub struct ATX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> ATX_BREAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `CRX_DONE` reader - Control Rx Packet Done Enable"]
pub struct CRX_DONE_R(crate::FieldReader<bool, bool>);
impl CRX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRX_DONE` writer - Control Rx Packet Done Enable"]
pub struct CRX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRX_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `CRX_PE` reader - Control Rx Protocol Error Enable"]
pub struct CRX_PE_R(crate::FieldReader<bool, bool>);
impl CRX_PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRX_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRX_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRX_PE` writer - Control Rx Protocol Error Enable"]
pub struct CRX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRX_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `CRX_BREAK` reader - Control Rx Break Enable"]
pub struct CRX_BREAK_R(crate::FieldReader<bool, bool>);
impl CRX_BREAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRX_BREAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRX_BREAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CRX_BREAK` writer - Control Rx Break Enable"]
pub struct CRX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CRX_BREAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
#[doc = "Field `CTX_DONE` reader - Control Tx Packet Done Enable"]
pub struct CTX_DONE_R(crate::FieldReader<bool, bool>);
impl CTX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTX_DONE` writer - Control Tx Packet Done Enable"]
pub struct CTX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_DONE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `CTX_PE` reader - Control Tx Protocol Error Enable"]
pub struct CTX_PE_R(crate::FieldReader<bool, bool>);
impl CTX_PE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTX_PE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTX_PE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTX_PE` writer - Control Tx Protocol Error Enable"]
pub struct CTX_PE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_PE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `CTX_BREAK` reader - Control Tx Break Enable"]
pub struct CTX_BREAK_R(crate::FieldReader<bool, bool>);
impl CTX_BREAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTX_BREAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTX_BREAK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTX_BREAK` writer - Control Tx Break Enable"]
pub struct CTX_BREAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CTX_BREAK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn isoc_pe(&self) -> ISOC_PE_R {
        ISOC_PE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    pub fn isoc_bufo(&self) -> ISOC_BUFO_R {
        ISOC_BUFO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    pub fn sync_pe(&self) -> SYNC_PE_R {
        SYNC_PE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    pub fn arx_done(&self) -> ARX_DONE_R {
        ARX_DONE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn arx_pe(&self) -> ARX_PE_R {
        ARX_PE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    pub fn arx_break(&self) -> ARX_BREAK_R {
        ARX_BREAK_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    pub fn atx_done(&self) -> ATX_DONE_R {
        ATX_DONE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn atx_pe(&self) -> ATX_PE_R {
        ATX_PE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    pub fn atx_break(&self) -> ATX_BREAK_R {
        ATX_BREAK_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    pub fn crx_done(&self) -> CRX_DONE_R {
        CRX_DONE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn crx_pe(&self) -> CRX_PE_R {
        CRX_PE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    pub fn crx_break(&self) -> CRX_BREAK_R {
        CRX_BREAK_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    pub fn ctx_done(&self) -> CTX_DONE_R {
        CTX_DONE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn ctx_pe(&self) -> CTX_PE_R {
        CTX_PE_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    pub fn ctx_break(&self) -> CTX_BREAK_R {
        CTX_BREAK_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Isochronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn isoc_pe(&mut self) -> ISOC_PE_W {
        ISOC_PE_W { w: self }
    }
    #[doc = "Bit 1 - Isochronous Rx Buffer Overflow Enable"]
    #[inline(always)]
    pub fn isoc_bufo(&mut self) -> ISOC_BUFO_W {
        ISOC_BUFO_W { w: self }
    }
    #[doc = "Bit 16 - Synchronous Protocol Error Enable"]
    #[inline(always)]
    pub fn sync_pe(&mut self) -> SYNC_PE_W {
        SYNC_PE_W { w: self }
    }
    #[doc = "Bit 17 - Asynchronous Rx Done Enable"]
    #[inline(always)]
    pub fn arx_done(&mut self) -> ARX_DONE_W {
        ARX_DONE_W { w: self }
    }
    #[doc = "Bit 18 - Asynchronous Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn arx_pe(&mut self) -> ARX_PE_W {
        ARX_PE_W { w: self }
    }
    #[doc = "Bit 19 - Asynchronous Rx Break Enable"]
    #[inline(always)]
    pub fn arx_break(&mut self) -> ARX_BREAK_W {
        ARX_BREAK_W { w: self }
    }
    #[doc = "Bit 20 - Asynchronous Tx Packet Done Enable"]
    #[inline(always)]
    pub fn atx_done(&mut self) -> ATX_DONE_W {
        ATX_DONE_W { w: self }
    }
    #[doc = "Bit 21 - Asynchronous Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn atx_pe(&mut self) -> ATX_PE_W {
        ATX_PE_W { w: self }
    }
    #[doc = "Bit 22 - Asynchronous Tx Break Enable"]
    #[inline(always)]
    pub fn atx_break(&mut self) -> ATX_BREAK_W {
        ATX_BREAK_W { w: self }
    }
    #[doc = "Bit 24 - Control Rx Packet Done Enable"]
    #[inline(always)]
    pub fn crx_done(&mut self) -> CRX_DONE_W {
        CRX_DONE_W { w: self }
    }
    #[doc = "Bit 25 - Control Rx Protocol Error Enable"]
    #[inline(always)]
    pub fn crx_pe(&mut self) -> CRX_PE_W {
        CRX_PE_W { w: self }
    }
    #[doc = "Bit 26 - Control Rx Break Enable"]
    #[inline(always)]
    pub fn crx_break(&mut self) -> CRX_BREAK_W {
        CRX_BREAK_W { w: self }
    }
    #[doc = "Bit 27 - Control Tx Packet Done Enable"]
    #[inline(always)]
    pub fn ctx_done(&mut self) -> CTX_DONE_W {
        CTX_DONE_W { w: self }
    }
    #[doc = "Bit 28 - Control Tx Protocol Error Enable"]
    #[inline(always)]
    pub fn ctx_pe(&mut self) -> CTX_PE_W {
        CTX_PE_W { w: self }
    }
    #[doc = "Bit 29 - Control Tx Break Enable"]
    #[inline(always)]
    pub fn ctx_break(&mut self) -> CTX_BREAK_W {
        CTX_BREAK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MediaLB Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mlb_mien](index.html) module"]
pub struct MLB_MIEN_SPEC;
impl crate::RegisterSpec for MLB_MIEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mlb_mien::R](R) reader structure"]
impl crate::Readable for MLB_MIEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mlb_mien::W](W) writer structure"]
impl crate::Writable for MLB_MIEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MLB_MIEN to value 0"]
impl crate::Resettable for MLB_MIEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
