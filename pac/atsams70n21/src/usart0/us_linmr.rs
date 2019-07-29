#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::US_LINMR {
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
#[doc = "Possible values of the field `NACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACTR {
    #[doc = "The USART transmits the response."]
    PUBLISH,
    #[doc = "The USART receives the response."]
    SUBSCRIBE,
    #[doc = "The USART does not transmit and does not receive the response."]
    IGNORE,
}
impl crate::ToBits<u8> for NACTR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            NACTR::PUBLISH => 0,
            NACTR::SUBSCRIBE => 1,
            NACTR::IGNORE => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type NACT_R = crate::FR<u8, NACTR>;
impl NACT_R {
    #[doc = "Checks if the value of the field is `PUBLISH`"]
    #[inline(always)]
    pub fn is_publish(&self) -> bool {
        *self == NACTR::PUBLISH
    }
    #[doc = "Checks if the value of the field is `SUBSCRIBE`"]
    #[inline(always)]
    pub fn is_subscribe(&self) -> bool {
        *self == NACTR::SUBSCRIBE
    }
    #[doc = "Checks if the value of the field is `IGNORE`"]
    #[inline(always)]
    pub fn is_ignore(&self) -> bool {
        *self == NACTR::IGNORE
    }
}
#[doc = "Values that can be written to the field `NACT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NACTW {
    #[doc = "The USART transmits the response."]
    PUBLISH,
    #[doc = "The USART receives the response."]
    SUBSCRIBE,
    #[doc = "The USART does not transmit and does not receive the response."]
    IGNORE,
}
impl NACTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            NACTW::PUBLISH => 0,
            NACTW::SUBSCRIBE => 1,
            NACTW::IGNORE => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _NACTW<'a> {
    w: &'a mut W,
}
impl<'a> _NACTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NACTW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The USART transmits the response."]
    #[inline(always)]
    pub fn publish(self) -> &'a mut W {
        self.variant(NACTW::PUBLISH)
    }
    #[doc = "The USART receives the response."]
    #[inline(always)]
    pub fn subscribe(self) -> &'a mut W {
        self.variant(NACTW::SUBSCRIBE)
    }
    #[doc = "The USART does not transmit and does not receive the response."]
    #[inline(always)]
    pub fn ignore(self) -> &'a mut W {
        self.variant(NACTW::IGNORE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PARDIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PARDISW<'a> {
    w: &'a mut W,
}
impl<'a> _PARDISW<'a> {
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
pub type CHKDIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CHKDISW<'a> {
    w: &'a mut W,
}
impl<'a> _CHKDISW<'a> {
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
pub type CHKTYP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CHKTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _CHKTYPW<'a> {
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
pub type DLM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _DLMW<'a> {
    w: &'a mut W,
}
impl<'a> _DLMW<'a> {
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
pub type FSDIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _FSDISW<'a> {
    w: &'a mut W,
}
impl<'a> _FSDISW<'a> {
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
pub type WKUPTYP_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _WKUPTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _WKUPTYPW<'a> {
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
pub type DLC_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _DLCW<'a> {
    w: &'a mut W,
}
impl<'a> _DLCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type PDCM_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _PDCMW<'a> {
    w: &'a mut W,
}
impl<'a> _PDCMW<'a> {
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
pub type SYNCDIS_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _SYNCDISW<'a> {
    w: &'a mut W,
}
impl<'a> _SYNCDISW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    pub fn nact(&self) -> NACT_R {
        NACT_R::new((self.bits() & 0x03) as u8)
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    pub fn pardis(&self) -> PARDIS_R {
        PARDIS_R::new(((self.bits() >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    pub fn chkdis(&self) -> CHKDIS_R {
        CHKDIS_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    pub fn chktyp(&self) -> CHKTYP_R {
        CHKTYP_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    pub fn dlm(&self) -> DLM_R {
        DLM_R::new(((self.bits() >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    pub fn fsdis(&self) -> FSDIS_R {
        FSDIS_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    pub fn wkuptyp(&self) -> WKUPTYP_R {
        WKUPTYP_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits() >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline(always)]
    pub fn pdcm(&self) -> PDCM_R {
        PDCM_R::new(((self.bits() >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline(always)]
    pub fn syncdis(&self) -> SYNCDIS_R {
        SYNCDIS_R::new(((self.bits() >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - LIN Node Action"]
    #[inline(always)]
    pub fn nact(&mut self) -> _NACTW {
        _NACTW { w: self }
    }
    #[doc = "Bit 2 - Parity Disable"]
    #[inline(always)]
    pub fn pardis(&mut self) -> _PARDISW {
        _PARDISW { w: self }
    }
    #[doc = "Bit 3 - Checksum Disable"]
    #[inline(always)]
    pub fn chkdis(&mut self) -> _CHKDISW {
        _CHKDISW { w: self }
    }
    #[doc = "Bit 4 - Checksum Type"]
    #[inline(always)]
    pub fn chktyp(&mut self) -> _CHKTYPW {
        _CHKTYPW { w: self }
    }
    #[doc = "Bit 5 - Data Length Mode"]
    #[inline(always)]
    pub fn dlm(&mut self) -> _DLMW {
        _DLMW { w: self }
    }
    #[doc = "Bit 6 - Frame Slot Mode Disable"]
    #[inline(always)]
    pub fn fsdis(&mut self) -> _FSDISW {
        _FSDISW { w: self }
    }
    #[doc = "Bit 7 - Wakeup Signal Type"]
    #[inline(always)]
    pub fn wkuptyp(&mut self) -> _WKUPTYPW {
        _WKUPTYPW { w: self }
    }
    #[doc = "Bits 8:15 - Data Length Control"]
    #[inline(always)]
    pub fn dlc(&mut self) -> _DLCW {
        _DLCW { w: self }
    }
    #[doc = "Bit 16 - DMAC Mode"]
    #[inline(always)]
    pub fn pdcm(&mut self) -> _PDCMW {
        _PDCMW { w: self }
    }
    #[doc = "Bit 17 - Synchronization Disable"]
    #[inline(always)]
    pub fn syncdis(&mut self) -> _SYNCDISW {
        _SYNCDISW { w: self }
    }
}
