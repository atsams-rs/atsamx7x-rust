#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::AES_MR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct CIPHERR {
    bits: bool,
}
impl CIPHERR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r" Value of the field"]
pub struct GTAGENR {
    bits: bool,
}
impl GTAGENR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
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
impl DUALBUFFR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            DUALBUFFR::INACTIVE => false,
            DUALBUFFR::ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DUALBUFFR {
        match value {
            false => DUALBUFFR::INACTIVE,
            true => DUALBUFFR::ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `INACTIVE`"]
    #[inline]
    pub fn is_inactive(&self) -> bool {
        *self == DUALBUFFR::INACTIVE
    }
    #[doc = "Checks if the value of the field is `ACTIVE`"]
    #[inline]
    pub fn is_active(&self) -> bool {
        *self == DUALBUFFR::ACTIVE
    }
}
#[doc = r" Value of the field"]
pub struct PROCDLYR {
    bits: u8,
}
impl PROCDLYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl SMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SMODR::MANUAL_START => 0,
            SMODR::AUTO_START => 1,
            SMODR::IDATAR0_START => 2,
            SMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SMODR {
        match value {
            0 => SMODR::MANUAL_START,
            1 => SMODR::AUTO_START,
            2 => SMODR::IDATAR0_START,
            i => SMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `MANUAL_START`"]
    #[inline]
    pub fn is_manual_start(&self) -> bool {
        *self == SMODR::MANUAL_START
    }
    #[doc = "Checks if the value of the field is `AUTO_START`"]
    #[inline]
    pub fn is_auto_start(&self) -> bool {
        *self == SMODR::AUTO_START
    }
    #[doc = "Checks if the value of the field is `IDATAR0_START`"]
    #[inline]
    pub fn is_idatar0_start(&self) -> bool {
        *self == SMODR::IDATAR0_START
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl KEYSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            KEYSIZER::AES128 => 0,
            KEYSIZER::AES192 => 1,
            KEYSIZER::AES256 => 2,
            KEYSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> KEYSIZER {
        match value {
            0 => KEYSIZER::AES128,
            1 => KEYSIZER::AES192,
            2 => KEYSIZER::AES256,
            i => KEYSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `AES128`"]
    #[inline]
    pub fn is_aes128(&self) -> bool {
        *self == KEYSIZER::AES128
    }
    #[doc = "Checks if the value of the field is `AES192`"]
    #[inline]
    pub fn is_aes192(&self) -> bool {
        *self == KEYSIZER::AES192
    }
    #[doc = "Checks if the value of the field is `AES256`"]
    #[inline]
    pub fn is_aes256(&self) -> bool {
        *self == KEYSIZER::AES256
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl OPMODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            OPMODR::ECB => 0,
            OPMODR::CBC => 1,
            OPMODR::OFB => 2,
            OPMODR::CFB => 3,
            OPMODR::CTR => 4,
            OPMODR::GCM => 5,
            OPMODR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> OPMODR {
        match value {
            0 => OPMODR::ECB,
            1 => OPMODR::CBC,
            2 => OPMODR::OFB,
            3 => OPMODR::CFB,
            4 => OPMODR::CTR,
            5 => OPMODR::GCM,
            i => OPMODR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ECB`"]
    #[inline]
    pub fn is_ecb(&self) -> bool {
        *self == OPMODR::ECB
    }
    #[doc = "Checks if the value of the field is `CBC`"]
    #[inline]
    pub fn is_cbc(&self) -> bool {
        *self == OPMODR::CBC
    }
    #[doc = "Checks if the value of the field is `OFB`"]
    #[inline]
    pub fn is_ofb(&self) -> bool {
        *self == OPMODR::OFB
    }
    #[doc = "Checks if the value of the field is `CFB`"]
    #[inline]
    pub fn is_cfb(&self) -> bool {
        *self == OPMODR::CFB
    }
    #[doc = "Checks if the value of the field is `CTR`"]
    #[inline]
    pub fn is_ctr(&self) -> bool {
        *self == OPMODR::CTR
    }
    #[doc = "Checks if the value of the field is `GCM`"]
    #[inline]
    pub fn is_gcm(&self) -> bool {
        *self == OPMODR::GCM
    }
}
#[doc = r" Value of the field"]
pub struct LODR {
    bits: bool,
}
impl LODR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CFBSR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CFBSR::SIZE_128BIT => 0,
            CFBSR::SIZE_64BIT => 1,
            CFBSR::SIZE_32BIT => 2,
            CFBSR::SIZE_16BIT => 3,
            CFBSR::SIZE_8BIT => 4,
            CFBSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CFBSR {
        match value {
            0 => CFBSR::SIZE_128BIT,
            1 => CFBSR::SIZE_64BIT,
            2 => CFBSR::SIZE_32BIT,
            3 => CFBSR::SIZE_16BIT,
            4 => CFBSR::SIZE_8BIT,
            i => CFBSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SIZE_128BIT`"]
    #[inline]
    pub fn is_size_128bit(&self) -> bool {
        *self == CFBSR::SIZE_128BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_64BIT`"]
    #[inline]
    pub fn is_size_64bit(&self) -> bool {
        *self == CFBSR::SIZE_64BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_32BIT`"]
    #[inline]
    pub fn is_size_32bit(&self) -> bool {
        *self == CFBSR::SIZE_32BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_16BIT`"]
    #[inline]
    pub fn is_size_16bit(&self) -> bool {
        *self == CFBSR::SIZE_16BIT
    }
    #[doc = "Checks if the value of the field is `SIZE_8BIT`"]
    #[inline]
    pub fn is_size_8bit(&self) -> bool {
        *self == CFBSR::SIZE_8BIT
    }
}
#[doc = "Possible values of the field `CKEY`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CKEYR {
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    PASSWD,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CKEYR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CKEYR::PASSWD => 14,
            CKEYR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CKEYR {
        match value {
            14 => CKEYR::PASSWD,
            i => CKEYR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PASSWD`"]
    #[inline]
    pub fn is_passwd(&self) -> bool {
        *self == CKEYR::PASSWD
    }
}
#[doc = r" Proxy"]
pub struct _CIPHERW<'a> {
    w: &'a mut W,
}
impl<'a> _CIPHERW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _GTAGENW<'a> {
    w: &'a mut W,
}
impl<'a> _GTAGENW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DUALBUFFW::INACTIVE => false,
            DUALBUFFW::ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DUALBUFFW<'a> {
    w: &'a mut W,
}
impl<'a> _DUALBUFFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DUALBUFFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "AES_IDATARx cannot be written during processing of previous block."]
    #[inline]
    pub fn inactive(self) -> &'a mut W {
        self.variant(DUALBUFFW::INACTIVE)
    }
    #[doc = "AES_IDATARx can be written during processing of previous block when SMOD = 2. It speeds up the overall runtime of large files."]
    #[inline]
    pub fn active(self) -> &'a mut W {
        self.variant(DUALBUFFW::ACTIVE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PROCDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _PROCDLYW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SMODW::MANUAL_START => 0,
            SMODW::AUTO_START => 1,
            SMODW::IDATAR0_START => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SMODW<'a> {
    w: &'a mut W,
}
impl<'a> _SMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Manual Mode"]
    #[inline]
    pub fn manual_start(self) -> &'a mut W {
        self.variant(SMODW::MANUAL_START)
    }
    #[doc = "Auto Mode"]
    #[inline]
    pub fn auto_start(self) -> &'a mut W {
        self.variant(SMODW::AUTO_START)
    }
    #[doc = "AES_IDATAR0 access only Auto Mode (DMA)"]
    #[inline]
    pub fn idatar0_start(self) -> &'a mut W {
        self.variant(SMODW::IDATAR0_START)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            KEYSIZEW::AES128 => 0,
            KEYSIZEW::AES192 => 1,
            KEYSIZEW::AES256 => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _KEYSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _KEYSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: KEYSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "AES Key Size is 128 bits"]
    #[inline]
    pub fn aes128(self) -> &'a mut W {
        self.variant(KEYSIZEW::AES128)
    }
    #[doc = "AES Key Size is 192 bits"]
    #[inline]
    pub fn aes192(self) -> &'a mut W {
        self.variant(KEYSIZEW::AES192)
    }
    #[doc = "AES Key Size is 256 bits"]
    #[inline]
    pub fn aes256(self) -> &'a mut W {
        self.variant(KEYSIZEW::AES256)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _OPMODW<'a> {
    w: &'a mut W,
}
impl<'a> _OPMODW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPMODW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "ECB: Electronic Code Book mode"]
    #[inline]
    pub fn ecb(self) -> &'a mut W {
        self.variant(OPMODW::ECB)
    }
    #[doc = "CBC: Cipher Block Chaining mode"]
    #[inline]
    pub fn cbc(self) -> &'a mut W {
        self.variant(OPMODW::CBC)
    }
    #[doc = "OFB: Output Feedback mode"]
    #[inline]
    pub fn ofb(self) -> &'a mut W {
        self.variant(OPMODW::OFB)
    }
    #[doc = "CFB: Cipher Feedback mode"]
    #[inline]
    pub fn cfb(self) -> &'a mut W {
        self.variant(OPMODW::CFB)
    }
    #[doc = "CTR: Counter mode (16-bit internal counter)"]
    #[inline]
    pub fn ctr(self) -> &'a mut W {
        self.variant(OPMODW::CTR)
    }
    #[doc = "GCM: Galois/Counter mode"]
    #[inline]
    pub fn gcm(self) -> &'a mut W {
        self.variant(OPMODW::GCM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _LODW<'a> {
    w: &'a mut W,
}
impl<'a> _LODW<'a> {
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _CFBSW<'a> {
    w: &'a mut W,
}
impl<'a> _CFBSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CFBSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "128-bit"]
    #[inline]
    pub fn size_128bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_128BIT)
    }
    #[doc = "64-bit"]
    #[inline]
    pub fn size_64bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_64BIT)
    }
    #[doc = "32-bit"]
    #[inline]
    pub fn size_32bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_32BIT)
    }
    #[doc = "16-bit"]
    #[inline]
    pub fn size_16bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_16BIT)
    }
    #[doc = "8-bit"]
    #[inline]
    pub fn size_8bit(self) -> &'a mut W {
        self.variant(CFBSW::SIZE_8BIT)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            CKEYW::PASSWD => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _CKEYW<'a> {
    w: &'a mut W,
}
impl<'a> _CKEYW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CKEYW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "This field must be written with 0xE to allow CMTYPx bit configuration changes. Any other values will abort the write operation in CMTYPx bits.Always reads as 0."]
    #[inline]
    pub fn passwd(self) -> &'a mut W {
        self.variant(CKEYW::PASSWD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Processing Mode"]
    #[inline]
    pub fn cipher(&self) -> CIPHERR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        CIPHERR { bits }
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline]
    pub fn gtagen(&self) -> GTAGENR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        GTAGENR { bits }
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline]
    pub fn dualbuff(&self) -> DUALBUFFR {
        DUALBUFFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 3;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline]
    pub fn procdly(&self) -> PROCDLYR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PROCDLYR { bits }
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline]
    pub fn smod(&self) -> SMODR {
        SMODR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline]
    pub fn keysize(&self) -> KEYSIZER {
        KEYSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 10;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline]
    pub fn opmod(&self) -> OPMODR {
        OPMODR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline]
    pub fn lod(&self) -> LODR {
        let bits = {
            const MASK: bool = true;
            const OFFSET: u8 = 15;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        };
        LODR { bits }
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline]
    pub fn cfbs(&self) -> CFBSR {
        CFBSR::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline]
    pub fn ckey(&self) -> CKEYR {
        CKEYR::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Processing Mode"]
    #[inline]
    pub fn cipher(&mut self) -> _CIPHERW {
        _CIPHERW { w: self }
    }
    #[doc = "Bit 1 - GCM Automatic Tag Generation Enable"]
    #[inline]
    pub fn gtagen(&mut self) -> _GTAGENW {
        _GTAGENW { w: self }
    }
    #[doc = "Bit 3 - Dual Input Buffer"]
    #[inline]
    pub fn dualbuff(&mut self) -> _DUALBUFFW {
        _DUALBUFFW { w: self }
    }
    #[doc = "Bits 4:7 - Processing Delay"]
    #[inline]
    pub fn procdly(&mut self) -> _PROCDLYW {
        _PROCDLYW { w: self }
    }
    #[doc = "Bits 8:9 - Start Mode"]
    #[inline]
    pub fn smod(&mut self) -> _SMODW {
        _SMODW { w: self }
    }
    #[doc = "Bits 10:11 - Key Size"]
    #[inline]
    pub fn keysize(&mut self) -> _KEYSIZEW {
        _KEYSIZEW { w: self }
    }
    #[doc = "Bits 12:14 - Operating Mode"]
    #[inline]
    pub fn opmod(&mut self) -> _OPMODW {
        _OPMODW { w: self }
    }
    #[doc = "Bit 15 - Last Output Data Mode"]
    #[inline]
    pub fn lod(&mut self) -> _LODW {
        _LODW { w: self }
    }
    #[doc = "Bits 16:18 - Cipher Feedback Data Size"]
    #[inline]
    pub fn cfbs(&mut self) -> _CFBSW {
        _CFBSW { w: self }
    }
    #[doc = "Bits 20:23 - Countermeasure Key"]
    #[inline]
    pub fn ckey(&mut self) -> _CKEYW {
        _CKEYW { w: self }
    }
}
