#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AES_MR {
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
pub type CIPHER_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _CIPHERW<'a> {
    w: &'a mut W,
}
impl<'a> _CIPHERW<'a> {
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
pub type GTAGEN_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _GTAGENW<'a> {
    w: &'a mut W,
}
impl<'a> _GTAGENW<'a> {
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
#[doc = "Possible values of the field `DUALBUFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALBUFFR {
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    INACTIVE,
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    ACTIVE,
}
impl crate::ToBits<bool> for DUALBUFFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DUALBUFFR::INACTIVE => false,
            DUALBUFFR::ACTIVE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DUALBUFF_R = crate::FR<bool, DUALBUFFR>;
impl DUALBUFF_R {
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DUALBUFFR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DUALBUFFR::ACTIVE
    }
}
#[doc = "Values that can be written to the field `DUALBUFF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DUALBUFFW {
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    INACTIVE,
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    ACTIVE,
}
impl DUALBUFFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DUALBUFFW::INACTIVE => false,
            DUALBUFFW::ACTIVE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DUALBUFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DUALBUFFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DUALBUFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DUALBUFFW::INACTIVE)
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline(always)]
    pub fn active(self) -> &'a mut W {
        self.variant(DUALBUFFW::ACTIVE)
    }
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
pub type PROCDLY_R = crate::FR<u8, u8>;
#[doc = r"Proxy"]
pub struct _PROCDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _PROCDLYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMODR {
    #[doc = "Manual Mode"]
    MANUAL_START,
    #[doc = "Auto Mode"]
    AUTO_START,
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    IDATAR0_START,
}
impl crate::ToBits<u8> for SMODR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SMODR::MANUAL_START => 0,
            SMODR::AUTO_START => 1,
            SMODR::IDATAR0_START => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SMOD_R = crate::FR<u8, SMODR>;
impl SMOD_R {
    #[doc = "Checks if the value of the field is `MANUAL_START`"]
    #[inline(always)]
    pub fn is_manual_start(&self) -> bool {
        *self == SMODR::MANUAL_START
    }
    #[doc = "Checks if the value of the field is `AUTO_START`"]
    #[inline(always)]
    pub fn is_auto_start(&self) -> bool {
        *self == SMODR::AUTO_START
    }
    #[doc = "Checks if the value of the field is `IDATAR0_START`"]
    #[inline(always)]
    pub fn is_idatar0_start(&self) -> bool {
        *self == SMODR::IDATAR0_START
    }
}
#[doc = "Values that can be written to the field `SMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SMODW {
    #[doc = "Manual Mode"]
    MANUAL_START,
    #[doc = "Auto Mode"]
    AUTO_START,
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    IDATAR0_START,
}
impl SMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMODW::MANUAL_START => 0,
            SMODW::AUTO_START => 1,
            SMODW::IDATAR0_START => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SMODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Manual Mode"]
    #[inline(always)]
    pub fn manual_start(self) -> &'a mut W {
        self.variant(SMODW::MANUAL_START)
    }
    #[doc = "Auto Mode"]
    #[inline(always)]
    pub fn auto_start(self) -> &'a mut W {
        self.variant(SMODW::AUTO_START)
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline(always)]
    pub fn idatar0_start(self) -> &'a mut W {
        self.variant(SMODW::IDATAR0_START)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `KEYSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSIZER {
    #[doc = "AES Key Size is 128 bits"]
    AES128,
    #[doc = "AES Key Size is 192 bits"]
    AES192,
    #[doc = "AES Key Size is 256 bits"]
    AES256,
}
impl crate::ToBits<u8> for KEYSIZER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            KEYSIZER::AES128 => 0,
            KEYSIZER::AES192 => 1,
            KEYSIZER::AES256 => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type KEYSIZE_R = crate::FR<u8, KEYSIZER>;
impl KEYSIZE_R {
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline(always)]
    pub fn is_aes128(&self) -> bool {
        *self == KEYSIZER::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline(always)]
    pub fn is_aes192(&self) -> bool {
        *self == KEYSIZER::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline(always)]
    pub fn is_aes256(&self) -> bool {
        *self == KEYSIZER::AES256
    }
}
#[doc = "Values that can be written to the field `KEYSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KEYSIZEW {
    #[doc = "AES Key Size is 128 bits"]
    AES128,
    #[doc = "AES Key Size is 192 bits"]
    AES192,
    #[doc = "AES Key Size is 256 bits"]
    AES256,
}
impl KEYSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYSIZEW::AES128 => 0,
            KEYSIZEW::AES192 => 1,
            KEYSIZEW::AES256 => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _KEYSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYSIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: KEYSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AES Key Size is 128 bits"]
    #[inline(always)]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEYSIZEW::AES128)
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline(always)]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEYSIZEW::AES192)
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline(always)]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEYSIZEW::AES256)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | (((value as u32) & 0x03) << 10);
        self.w
    }
}
#[doc = "Possible values of the field `OPMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPMODR {
    #[doc = "ECB: Electronic Code Book mode"]
    ECB,
    #[doc = "CBC: Cipher Block Chaining mode"]
    CBC,
    #[doc = "OFB: Output Feedback mode"]
    OFB,
    #[doc = "CFB: Cipher Feedback mode"]
    CFB,
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    CTR,
    #[doc = "GCM: Galois/Counter mode"]
    GCM,
}
impl crate::ToBits<u8> for OPMODR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            OPMODR::ECB => 0,
            OPMODR::CBC => 1,
            OPMODR::OFB => 2,
            OPMODR::CFB => 3,
            OPMODR::CTR => 4,
            OPMODR::GCM => 5,
        }
    }
}
#[doc = r"Reader of the field"]
pub type OPMOD_R = crate::FR<u8, OPMODR>;
impl OPMOD_R {
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline(always)]
    pub fn is_ecb(&self) -> bool {
        *self == OPMODR::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline(always)]
    pub fn is_cbc(&self) -> bool {
        *self == OPMODR::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline(always)]
    pub fn is_ofb(&self) -> bool {
        *self == OPMODR::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline(always)]
    pub fn is_cfb(&self) -> bool {
        *self == OPMODR::CFB
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline(always)]
    pub fn is_ctr(&self) -> bool {
        *self == OPMODR::CTR
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline(always)]
    pub fn is_gcm(&self) -> bool {
        *self == OPMODR::GCM
    }
}
#[doc = "Values that can be written to the field `OPMOD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPMODW {
    #[doc = "ECB: Electronic Code Book mode"]
    ECB,
    #[doc = "CBC: Cipher Block Chaining mode"]
    CBC,
    #[doc = "OFB: Output Feedback mode"]
    OFB,
    #[doc = "CFB: Cipher Feedback mode"]
    CFB,
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    CTR,
    #[doc = "GCM: Galois/Counter mode"]
    GCM,
}
impl OPMODW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            OPMODW::ECB => 0,
            OPMODW::CBC => 1,
            OPMODW::OFB => 2,
            OPMODW::CFB => 3,
            OPMODW::CTR => 4,
            OPMODW::GCM => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _OPMODW<'a> {
    w: &'a mut W,
}
impl<'a> _OPMODW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OPMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline(always)]
    pub fn ecb(self) -> &'a mut W {
        self.variant(OPMODW::ECB)
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline(always)]
    pub fn cbc(self) -> &'a mut W {
        self.variant(OPMODW::CBC)
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline(always)]
    pub fn ofb(self) -> &'a mut W {
        self.variant(OPMODW::OFB)
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline(always)]
    pub fn cfb(self) -> &'a mut W {
        self.variant(OPMODW::CFB)
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline(always)]
    pub fn ctr(self) -> &'a mut W {
        self.variant(OPMODW::CTR)
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline(always)]
    pub fn gcm(self) -> &'a mut W {
        self.variant(OPMODW::GCM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = r"Reader of the field"]
pub type LOD_R = crate::FR<bool, bool>;
#[doc = r"Proxy"]
pub struct _LODW<'a> {
    w: &'a mut W,
}
impl<'a> _LODW<'a> {
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
#[doc = "Possible values of the field `CFBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFBSR {
    #[doc = "128-bit"]
    SIZE_128BIT,
    #[doc = "64-bit"]
    SIZE_64BIT,
    #[doc = "32-bit"]
    SIZE_32BIT,
    #[doc = "16-bit"]
    SIZE_16BIT,
    #[doc = "8-bit"]
    SIZE_8BIT,
}
impl crate::ToBits<u8> for CFBSR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CFBSR::SIZE_128BIT => 0,
            CFBSR::SIZE_64BIT => 1,
            CFBSR::SIZE_32BIT => 2,
            CFBSR::SIZE_16BIT => 3,
            CFBSR::SIZE_8BIT => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CFBS_R = crate::FR<u8, CFBSR>;
impl CFBS_R {
    #[doc = "Checks if the value of the field is `SIZE_128BIT`"]
    #[inline(always)]
    pub fn is_size_128bit(&self) -> bool {
        *self == CFBSR::SIZE_128BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_64BIT`"]
    #[inline(always)]
    pub fn is_size_64bit(&self) -> bool {
        *self == CFBSR::SIZE_64BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_32BIT`"]
    #[inline(always)]
    pub fn is_size_32bit(&self) -> bool {
        *self == CFBSR::SIZE_32BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_16BIT`"]
    #[inline(always)]
    pub fn is_size_16bit(&self) -> bool {
        *self == CFBSR::SIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_8BIT`"]
    #[inline(always)]
    pub fn is_size_8bit(&self) -> bool {
        *self == CFBSR::SIZE_8BIT
    }
}
#[doc = "Values that can be written to the field `CFBS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CFBSW {
    #[doc = "128-bit"]
    SIZE_128BIT,
    #[doc = "64-bit"]
    SIZE_64BIT,
    #[doc = "32-bit"]
    SIZE_32BIT,
    #[doc = "16-bit"]
    SIZE_16BIT,
    #[doc = "8-bit"]
    SIZE_8BIT,
}
impl CFBSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CFBSW::SIZE_128BIT => 0,
            CFBSW::SIZE_64BIT => 1,
            CFBSW::SIZE_32BIT => 2,
            CFBSW::SIZE_16BIT => 3,
            CFBSW::SIZE_8BIT => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CFBSW<'a> {
    w: &'a mut W,
}
impl<'a> _CFBSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CFBSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "128-bit"]
    #[inline(always)]
    pub fn size_128bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_128BIT)
    }
    #[doc = "64-bit"]
    #[inline(always)]
    pub fn size_64bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_64BIT)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn size_32bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_32BIT)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn size_16bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_16BIT)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn size_8bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_8BIT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `CKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKEYR {
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    PASSWD,
}
impl crate::ToBits<u8> for CKEYR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CKEYR::PASSWD => 14,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CKEY_R = crate::FR<u8, CKEYR>;
impl CKEY_R {
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline(always)]
    pub fn is_passwd(&self) -> bool {
        *self == CKEYR::PASSWD
    }
}
#[doc = "Values that can be written to the field `CKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKEYW {
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    PASSWD,
}
impl CKEYW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKEYW::PASSWD => 14,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _CKEYW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline(always)]
    pub fn passwd(self) -> &'a mut W {
        self.variant(CKEYW::PASSWD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&self) -> CIPHER_R {
        CIPHER_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&self) -> GTAGEN_R {
        GTAGEN_R::new(((self.bits() >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&self) -> DUALBUFF_R {
        DUALBUFF_R::new(((self.bits() >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&self) -> PROCDLY_R {
        PROCDLY_R::new(((self.bits() >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits() >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&self) -> KEYSIZE_R {
        KEYSIZE_R::new(((self.bits() >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&self) -> OPMOD_R {
        OPMOD_R::new(((self.bits() >> 12) & 0x07) as u8)
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&self) -> LOD_R {
        LOD_R::new(((self.bits() >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&self) -> CFBS_R {
        CFBS_R::new(((self.bits() >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&self) -> CKEY_R {
        CKEY_R::new(((self.bits() >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Processing Mode"]
    #[inline(always)]
    pub fn cipher(&mut self) -> _CIPHERW {
        _CIPHERW { w: self }
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline(always)]
    pub fn gtagen(&mut self) -> _GTAGENW {
        _GTAGENW { w: self }
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline(always)]
    pub fn dualbuff(&mut self) -> _DUALBUFFW {
        _DUALBUFFW { w: self }
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline(always)]
    pub fn procdly(&mut self) -> _PROCDLYW {
        _PROCDLYW { w: self }
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline(always)]
    pub fn smod(&mut self) -> _SMODW {
        _SMODW { w: self }
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline(always)]
    pub fn keysize(&mut self) -> _KEYSIZEW {
        _KEYSIZEW { w: self }
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline(always)]
    pub fn opmod(&mut self) -> _OPMODW {
        _OPMODW { w: self }
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline(always)]
    pub fn lod(&mut self) -> _LODW {
        _LODW { w: self }
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline(always)]
    pub fn cfbs(&mut self) -> _CFBSW {
        _CFBSW { w: self }
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline(always)]
    pub fn ckey(&mut self) -> _CKEYW {
        _CKEYW { w: self }
    }
}
