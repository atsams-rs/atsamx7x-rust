#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_CC {
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
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    MEM_TRAN,
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    PER_TRAN,
}
impl crate::ToBits<bool> for TYPER {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            TYPER::MEM_TRAN => false,
            TYPER::PER_TRAN => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type TYPE_R = crate::FR<bool, TYPER>;
impl TYPE_R {
    #[doc = "Checks if the value of the field is `MEM_TRAN`"]
    #[inline(always)]
    pub fn is_mem_tran(&self) -> bool {
        *self == TYPER::MEM_TRAN
    }
    #[doc = "Checks if the value of the field is `PER_TRAN`"]
    #[inline(always)]
    pub fn is_per_tran(&self) -> bool {
        *self == TYPER::PER_TRAN
    }
}
#[doc = "Values that can be written to the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPEW {
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    MEM_TRAN,
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    PER_TRAN,
}
impl TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            TYPEW::MEM_TRAN => false,
            TYPEW::PER_TRAN => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TYPEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Self-triggered mode (memory-to-memory transfer)."]
    #[inline(always)]
    pub fn mem_tran(self) -> &'a mut W {
        self.variant(TYPEW::MEM_TRAN)
    }
    #[doc = "Synchronized mode (peripheral-to-memory or memory-to-peripheral transfer)."]
    #[inline(always)]
    pub fn per_tran(self) -> &'a mut W {
        self.variant(TYPEW::PER_TRAN)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Possible values of the field `MBSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBSIZER {
    #[doc = "The memory burst size is set to one."]
    SINGLE,
    #[doc = "The memory burst size is set to four."]
    FOUR,
    #[doc = "The memory burst size is set to eight."]
    EIGHT,
    #[doc = "The memory burst size is set to sixteen."]
    SIXTEEN,
}
impl crate::ToBits<u8> for MBSIZER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            MBSIZER::SINGLE => 0,
            MBSIZER::FOUR => 1,
            MBSIZER::EIGHT => 2,
            MBSIZER::SIXTEEN => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MBSIZE_R = crate::FR<u8, MBSIZER>;
impl MBSIZE_R {
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == MBSIZER::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == MBSIZER::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline(always)]
    pub fn is_eight(&self) -> bool {
        *self == MBSIZER::EIGHT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline(always)]
    pub fn is_sixteen(&self) -> bool {
        *self == MBSIZER::SIXTEEN
    }
}
#[doc = "Values that can be written to the field `MBSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MBSIZEW {
    #[doc = "The memory burst size is set to one."]
    SINGLE,
    #[doc = "The memory burst size is set to four."]
    FOUR,
    #[doc = "The memory burst size is set to eight."]
    EIGHT,
    #[doc = "The memory burst size is set to sixteen."]
    SIXTEEN,
}
impl MBSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            MBSIZEW::SINGLE => 0,
            MBSIZEW::FOUR => 1,
            MBSIZEW::EIGHT => 2,
            MBSIZEW::SIXTEEN => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MBSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _MBSIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MBSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The memory burst size is set to one."]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(MBSIZEW::SINGLE)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(MBSIZEW::FOUR)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline(always)]
    pub fn eight(self) -> &'a mut W {
        self.variant(MBSIZEW::EIGHT)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline(always)]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(MBSIZEW::SIXTEEN)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Possible values of the field `DSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSYNCR {
    #[doc = "Peripheral-to-memory transfer."]
    PER2MEM,
    #[doc = "Memory-to-peripheral transfer."]
    MEM2PER,
}
impl crate::ToBits<bool> for DSYNCR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DSYNCR::PER2MEM => false,
            DSYNCR::MEM2PER => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DSYNC_R = crate::FR<bool, DSYNCR>;
impl DSYNC_R {
    #[doc = "Checks if the value of the field is `PER2MEM`"]
    #[inline(always)]
    pub fn is_per2mem(&self) -> bool {
        *self == DSYNCR::PER2MEM
    }
    #[doc = "Checks if the value of the field is `MEM2PER`"]
    #[inline(always)]
    pub fn is_mem2per(&self) -> bool {
        *self == DSYNCR::MEM2PER
    }
}
#[doc = "Values that can be written to the field `DSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSYNCW {
    #[doc = "Peripheral-to-memory transfer."]
    PER2MEM,
    #[doc = "Memory-to-peripheral transfer."]
    MEM2PER,
}
impl DSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DSYNCW::PER2MEM => false,
            DSYNCW::MEM2PER => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSYNCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral-to-memory transfer."]
    #[inline(always)]
    pub fn per2mem(self) -> &'a mut W {
        self.variant(DSYNCW::PER2MEM)
    }
    #[doc = "Memory-to-peripheral transfer."]
    #[inline(always)]
    pub fn mem2per(self) -> &'a mut W {
        self.variant(DSYNCW::MEM2PER)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Possible values of the field `SWREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQR {
    #[doc = "Hardware request line is connected to the peripheral request line."]
    HWR_CONNECTED,
    #[doc = "Software request is connected to the peripheral request line."]
    SWR_CONNECTED,
}
impl crate::ToBits<bool> for SWREQR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SWREQR::HWR_CONNECTED => false,
            SWREQR::SWR_CONNECTED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SWREQ_R = crate::FR<bool, SWREQR>;
impl SWREQ_R {
    #[doc = "Checks if the value of the field is `HWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_hwr_connected(&self) -> bool {
        *self == SWREQR::HWR_CONNECTED
    }
    #[doc = "Checks if the value of the field is `SWR_CONNECTED`"]
    #[inline(always)]
    pub fn is_swr_connected(&self) -> bool {
        *self == SWREQR::SWR_CONNECTED
    }
}
#[doc = "Values that can be written to the field `SWREQ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWREQW {
    #[doc = "Hardware request line is connected to the peripheral request line."]
    HWR_CONNECTED,
    #[doc = "Software request is connected to the peripheral request line."]
    SWR_CONNECTED,
}
impl SWREQW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SWREQW::HWR_CONNECTED => false,
            SWREQW::SWR_CONNECTED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SWREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline(always)]
    pub fn hwr_connected(self) -> &'a mut W {
        self.variant(SWREQW::HWR_CONNECTED)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline(always)]
    pub fn swr_connected(self) -> &'a mut W {
        self.variant(SWREQW::SWR_CONNECTED)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Possible values of the field `MEMSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMSETR {
    #[doc = "Memset is not activated."]
    NORMAL_MODE,
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HW_MODE,
}
impl crate::ToBits<bool> for MEMSETR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            MEMSETR::NORMAL_MODE => false,
            MEMSETR::HW_MODE => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type MEMSET_R = crate::FR<bool, MEMSETR>;
impl MEMSET_R {
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == MEMSETR::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `HW_MODE`"]
    #[inline(always)]
    pub fn is_hw_mode(&self) -> bool {
        *self == MEMSETR::HW_MODE
    }
}
#[doc = "Values that can be written to the field `MEMSET`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MEMSETW {
    #[doc = "Memset is not activated."]
    NORMAL_MODE,
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    HW_MODE,
}
impl MEMSETW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMSETW::NORMAL_MODE => false,
            MEMSETW::HW_MODE => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _MEMSETW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMSETW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MEMSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memset is not activated."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(MEMSETW::NORMAL_MODE)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline(always)]
    pub fn hw_mode(self) -> &'a mut W {
        self.variant(MEMSETW::HW_MODE)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Possible values of the field `CSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIZER {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "2 data transferred"]
    CHK_2,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
}
impl crate::ToBits<u8> for CSIZER {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            CSIZER::CHK_1 => 0,
            CSIZER::CHK_2 => 1,
            CSIZER::CHK_4 => 2,
            CSIZER::CHK_8 => 3,
            CSIZER::CHK_16 => 4,
        }
    }
}
#[doc = r"Reader of the field"]
pub type CSIZE_R = crate::FR<u8, CSIZER>;
impl CSIZE_R {
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline(always)]
    pub fn is_chk_1(&self) -> bool {
        *self == CSIZER::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_2`"]
    #[inline(always)]
    pub fn is_chk_2(&self) -> bool {
        *self == CSIZER::CHK_2
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline(always)]
    pub fn is_chk_4(&self) -> bool {
        *self == CSIZER::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline(always)]
    pub fn is_chk_8(&self) -> bool {
        *self == CSIZER::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline(always)]
    pub fn is_chk_16(&self) -> bool {
        *self == CSIZER::CHK_16
    }
}
#[doc = "Values that can be written to the field `CSIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CSIZEW {
    #[doc = "1 data transferred"]
    CHK_1,
    #[doc = "2 data transferred"]
    CHK_2,
    #[doc = "4 data transferred"]
    CHK_4,
    #[doc = "8 data transferred"]
    CHK_8,
    #[doc = "16 data transferred"]
    CHK_16,
}
impl CSIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CSIZEW::CHK_1 => 0,
            CSIZEW::CHK_2 => 1,
            CSIZEW::CHK_4 => 2,
            CSIZEW::CHK_8 => 3,
            CSIZEW::CHK_16 => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data transferred"]
    #[inline(always)]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_1)
    }
    #[doc = "2 data transferred"]
    #[inline(always)]
    pub fn chk_2(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_2)
    }
    #[doc = "4 data transferred"]
    #[inline(always)]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline(always)]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline(always)]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Possible values of the field `DWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWIDTHR {
    #[doc = "The data size is set to 8 bits"]
    BYTE,
    #[doc = "The data size is set to 16 bits"]
    HALFWORD,
    #[doc = "The data size is set to 32 bits"]
    WORD,
}
impl crate::ToBits<u8> for DWIDTHR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DWIDTHR::BYTE => 0,
            DWIDTHR::HALFWORD => 1,
            DWIDTHR::WORD => 2,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DWIDTH_R = crate::FR<u8, DWIDTHR>;
impl DWIDTH_R {
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == DWIDTHR::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == DWIDTHR::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == DWIDTHR::WORD
    }
}
#[doc = "Values that can be written to the field `DWIDTH`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DWIDTHW {
    #[doc = "The data size is set to 8 bits"]
    BYTE,
    #[doc = "The data size is set to 16 bits"]
    HALFWORD,
    #[doc = "The data size is set to 32 bits"]
    WORD,
}
impl DWIDTHW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DWIDTHW::BYTE => 0,
            DWIDTHW::HALFWORD => 1,
            DWIDTHW::WORD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DWIDTHW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The data size is set to 8 bits"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(DWIDTHW::BYTE)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DWIDTHW::HALFWORD)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(DWIDTHW::WORD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Possible values of the field `SIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIFR {
    #[doc = "The data is read through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is read through the system bus interface 1."]
    AHB_IF1,
}
impl crate::ToBits<bool> for SIFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            SIFR::AHB_IF0 => false,
            SIFR::AHB_IF1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SIF_R = crate::FR<bool, SIFR>;
impl SIF_R {
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == SIFR::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == SIFR::AHB_IF1
    }
}
#[doc = "Values that can be written to the field `SIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SIFW {
    #[doc = "The data is read through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is read through the system bus interface 1."]
    AHB_IF1,
}
impl SIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            SIFW::AHB_IF0 => false,
            SIFW::AHB_IF1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SIFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data is read through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(SIFW::AHB_IF0)
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(SIFW::AHB_IF1)
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Possible values of the field `DIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFR {
    #[doc = "The data is written through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is written though the system bus interface 1."]
    AHB_IF1,
}
impl crate::ToBits<bool> for DIFR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            DIFR::AHB_IF0 => false,
            DIFR::AHB_IF1 => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DIF_R = crate::FR<bool, DIFR>;
impl DIF_R {
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == DIFR::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == DIFR::AHB_IF1
    }
}
#[doc = "Values that can be written to the field `DIF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DIFW {
    #[doc = "The data is written through the system bus interface 0."]
    AHB_IF0,
    #[doc = "The data is written though the system bus interface 1."]
    AHB_IF1,
}
impl DIFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFW::AHB_IF0 => false,
            DIFW::AHB_IF1 => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data is written through the system bus interface 0."]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(DIFW::AHB_IF0)
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(DIFW::AHB_IF1)
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Possible values of the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMR {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl crate::ToBits<u8> for SAMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            SAMR::FIXED_AM => 0,
            SAMR::INCREMENTED_AM => 1,
            SAMR::UBS_AM => 2,
            SAMR::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type SAM_R = crate::FR<u8, SAMR>;
impl SAM_R {
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == SAMR::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == SAMR::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == SAMR::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == SAMR::UBS_DS_AM
    }
}
#[doc = "Values that can be written to the field `SAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SAMW {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl SAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMW::FIXED_AM => 0,
            SAMW::INCREMENTED_AM => 1,
            SAMW::UBS_AM => 2,
            SAMW::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(SAMW::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(SAMW::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(SAMW::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(SAMW::UBS_DS_AM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Possible values of the field `DAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAMR {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl crate::ToBits<u8> for DAMR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            DAMR::FIXED_AM => 0,
            DAMR::INCREMENTED_AM => 1,
            DAMR::UBS_AM => 2,
            DAMR::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Reader of the field"]
pub type DAM_R = crate::FR<u8, DAMR>;
impl DAM_R {
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline(always)]
    pub fn is_fixed_am(&self) -> bool {
        *self == DAMR::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline(always)]
    pub fn is_incremented_am(&self) -> bool {
        *self == DAMR::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline(always)]
    pub fn is_ubs_am(&self) -> bool {
        *self == DAMR::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline(always)]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == DAMR::UBS_DS_AM
    }
}
#[doc = "Values that can be written to the field `DAM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DAMW {
    #[doc = "The address remains unchanged."]
    FIXED_AM,
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    INCREMENTED_AM,
    #[doc = "The microblock stride is added at the microblock boundary."]
    UBS_AM,
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl DAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAMW::FIXED_AM => 0,
            DAMW::INCREMENTED_AM => 1,
            DAMW::UBS_AM => 2,
            DAMW::UBS_DS_AM => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _DAMW<'a> {
    w: &'a mut W,
}
impl<'a> _DAMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline(always)]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(DAMW::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline(always)]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(DAMW::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline(always)]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(DAMW::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary; the data stride is added at the data boundary."]
    #[inline(always)]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(DAMW::UBS_DS_AM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | (((value as u32) & 0x03) << 18);
        self.w
    }
}
#[doc = "Possible values of the field `INITD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDR {
    #[doc = "Channel initialization is in progress."]
    IN_PROGRESS,
    #[doc = "Channel initialization is completed."]
    TERMINATED,
}
impl crate::ToBits<bool> for INITDR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            INITDR::IN_PROGRESS => false,
            INITDR::TERMINATED => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type INITD_R = crate::FR<bool, INITDR>;
impl INITD_R {
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == INITDR::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `TERMINATED`"]
    #[inline(always)]
    pub fn is_terminated(&self) -> bool {
        *self == INITDR::TERMINATED
    }
}
#[doc = "Values that can be written to the field `INITD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INITDW {
    #[doc = "Channel initialization is in progress."]
    IN_PROGRESS,
    #[doc = "Channel initialization is completed."]
    TERMINATED,
}
impl INITDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            INITDW::IN_PROGRESS => false,
            INITDW::TERMINATED => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _INITDW<'a> {
    w: &'a mut W,
}
impl<'a> _INITDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INITDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel initialization is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(INITDW::IN_PROGRESS)
    }
    #[doc = "Channel initialization is completed."]
    #[inline(always)]
    pub fn terminated(self) -> &'a mut W {
        self.variant(INITDW::TERMINATED)
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Possible values of the field `RDIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIPR {
    #[doc = "No active read transaction on the bus."]
    DONE,
    #[doc = "A read transaction is in progress."]
    IN_PROGRESS,
}
impl crate::ToBits<bool> for RDIPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            RDIPR::DONE => false,
            RDIPR::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type RDIP_R = crate::FR<bool, RDIPR>;
impl RDIP_R {
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == RDIPR::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == RDIPR::IN_PROGRESS
    }
}
#[doc = "Values that can be written to the field `RDIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIPW {
    #[doc = "No active read transaction on the bus."]
    DONE,
    #[doc = "A read transaction is in progress."]
    IN_PROGRESS,
}
impl RDIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            RDIPW::DONE => false,
            RDIPW::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _RDIPW<'a> {
    w: &'a mut W,
}
impl<'a> _RDIPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RDIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No active read transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(RDIPW::DONE)
    }
    #[doc = "A read transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(RDIPW::IN_PROGRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Possible values of the field `WRIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRIPR {
    #[doc = "No active write transaction on the bus."]
    DONE,
    #[doc = "A write transaction is in progress."]
    IN_PROGRESS,
}
impl crate::ToBits<bool> for WRIPR {
    #[inline(always)]
    fn _bits(&self) -> bool {
        match *self {
            WRIPR::DONE => false,
            WRIPR::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Reader of the field"]
pub type WRIP_R = crate::FR<bool, WRIPR>;
impl WRIP_R {
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline(always)]
    pub fn is_done(&self) -> bool {
        *self == WRIPR::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline(always)]
    pub fn is_in_progress(&self) -> bool {
        *self == WRIPR::IN_PROGRESS
    }
}
#[doc = "Values that can be written to the field `WRIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRIPW {
    #[doc = "No active write transaction on the bus."]
    DONE,
    #[doc = "A write transaction is in progress."]
    IN_PROGRESS,
}
impl WRIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> bool {
        match *self {
            WRIPW::DONE => false,
            WRIPW::IN_PROGRESS => true,
        }
    }
}
#[doc = r"Proxy"]
pub struct _WRIPW<'a> {
    w: &'a mut W,
}
impl<'a> _WRIPW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WRIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No active write transaction on the bus."]
    #[inline(always)]
    pub fn done(self) -> &'a mut W {
        self.variant(WRIPW::DONE)
    }
    #[doc = "A write transaction is in progress."]
    #[inline(always)]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(WRIPW::IN_PROGRESS)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Possible values of the field `PERID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIDR {
    #[doc = "HSMCI"]
    HSMCI,
    #[doc = "SPI0_TX"]
    SPI0_TX,
    #[doc = "SPI0_RX"]
    SPI0_RX,
    #[doc = "QSPI_TX"]
    QSPI_TX,
    #[doc = "QSPI_RX"]
    QSPI_RX,
    #[doc = "USART0_TX"]
    USART0_TX,
    #[doc = "USART0_RX"]
    USART0_RX,
    #[doc = "USART1_TX"]
    USART1_TX,
    #[doc = "USART1_RX"]
    USART1_RX,
    #[doc = "USART2_TX"]
    USART2_TX,
    #[doc = "USART2_RX"]
    USART2_RX,
    #[doc = "PWM0"]
    PWM0,
    #[doc = "TWIHS0_TX"]
    TWIHS0_TX,
    #[doc = "TWIHS0_RX"]
    TWIHS0_RX,
    #[doc = "TWIHS1_TX"]
    TWIHS1_TX,
    #[doc = "TWIHS1_RX"]
    TWIHS1_RX,
    #[doc = "TWIHS2_TX"]
    TWIHS2_TX,
    #[doc = "TWIHS2_RX"]
    TWIHS2_RX,
    #[doc = "UART0_TX"]
    UART0_TX,
    #[doc = "UART0_RX"]
    UART0_RX,
    #[doc = "UART1_TX"]
    UART1_TX,
    #[doc = "UART1_RX"]
    UART1_RX,
    #[doc = "UART2_TX"]
    UART2_TX,
    #[doc = "UART2_RX"]
    UART2_RX,
    #[doc = "UART3_TX"]
    UART3_TX,
    #[doc = "UART3_RX"]
    UART3_RX,
    #[doc = "UART4_TX"]
    UART4_TX,
    #[doc = "UART4_RX"]
    UART4_RX,
    #[doc = "DACC0"]
    DACC0,
    #[doc = "DACC1"]
    DACC1,
    #[doc = "SSC_TX"]
    SSC_TX,
    #[doc = "SSC_RX"]
    SSC_RX,
    #[doc = "PIOA"]
    PIOA,
    #[doc = "AFEC0"]
    AFEC0,
    #[doc = "AFEC1"]
    AFEC1,
    #[doc = "AES_TX"]
    AES_TX,
    #[doc = "AES_RX"]
    AES_RX,
    #[doc = "PWM1"]
    PWM1,
    #[doc = "TC0"]
    TC0,
    #[doc = "TC3"]
    TC3,
    #[doc = "TC6"]
    TC6,
    #[doc = "TC9"]
    TC9,
    #[doc = "I2SC0_TX_LEFT"]
    I2SC0_TX_LEFT,
    #[doc = "I2SC0_RX_LEFT"]
    I2SC0_RX_LEFT,
    #[doc = "I2SC0_TX_RIGHT"]
    I2SC0_TX_RIGHT,
    #[doc = "I2SC0_RX_RIGHT"]
    I2SC0_RX_RIGHT,
}
impl crate::ToBits<u8> for PERIDR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            PERIDR::HSMCI => 0,
            PERIDR::SPI0_TX => 1,
            PERIDR::SPI0_RX => 2,
            PERIDR::QSPI_TX => 5,
            PERIDR::QSPI_RX => 6,
            PERIDR::USART0_TX => 7,
            PERIDR::USART0_RX => 8,
            PERIDR::USART1_TX => 9,
            PERIDR::USART1_RX => 10,
            PERIDR::USART2_TX => 11,
            PERIDR::USART2_RX => 12,
            PERIDR::PWM0 => 13,
            PERIDR::TWIHS0_TX => 14,
            PERIDR::TWIHS0_RX => 15,
            PERIDR::TWIHS1_TX => 16,
            PERIDR::TWIHS1_RX => 17,
            PERIDR::TWIHS2_TX => 18,
            PERIDR::TWIHS2_RX => 19,
            PERIDR::UART0_TX => 20,
            PERIDR::UART0_RX => 21,
            PERIDR::UART1_TX => 22,
            PERIDR::UART1_RX => 23,
            PERIDR::UART2_TX => 24,
            PERIDR::UART2_RX => 25,
            PERIDR::UART3_TX => 26,
            PERIDR::UART3_RX => 27,
            PERIDR::UART4_TX => 28,
            PERIDR::UART4_RX => 29,
            PERIDR::DACC0 => 30,
            PERIDR::DACC1 => 31,
            PERIDR::SSC_TX => 32,
            PERIDR::SSC_RX => 33,
            PERIDR::PIOA => 34,
            PERIDR::AFEC0 => 35,
            PERIDR::AFEC1 => 36,
            PERIDR::AES_TX => 37,
            PERIDR::AES_RX => 38,
            PERIDR::PWM1 => 39,
            PERIDR::TC0 => 40,
            PERIDR::TC3 => 41,
            PERIDR::TC6 => 42,
            PERIDR::TC9 => 43,
            PERIDR::I2SC0_TX_LEFT => 44,
            PERIDR::I2SC0_RX_LEFT => 45,
            PERIDR::I2SC0_TX_RIGHT => 48,
            PERIDR::I2SC0_RX_RIGHT => 49,
        }
    }
}
#[doc = r"Reader of the field"]
pub type PERID_R = crate::FR<u8, PERIDR>;
impl PERID_R {
    #[doc = "Checks if the value of the field is `HSMCI`"]
    #[inline(always)]
    pub fn is_hsmci(&self) -> bool {
        *self == PERIDR::HSMCI
    }
    #[doc = "Checks if the value of the field is `SPI0_TX`"]
    #[inline(always)]
    pub fn is_spi0_tx(&self) -> bool {
        *self == PERIDR::SPI0_TX
    }
    #[doc = "Checks if the value of the field is `SPI0_RX`"]
    #[inline(always)]
    pub fn is_spi0_rx(&self) -> bool {
        *self == PERIDR::SPI0_RX
    }
    #[doc = "Checks if the value of the field is `QSPI_TX`"]
    #[inline(always)]
    pub fn is_qspi_tx(&self) -> bool {
        *self == PERIDR::QSPI_TX
    }
    #[doc = "Checks if the value of the field is `QSPI_RX`"]
    #[inline(always)]
    pub fn is_qspi_rx(&self) -> bool {
        *self == PERIDR::QSPI_RX
    }
    #[doc = "Checks if the value of the field is `USART0_TX`"]
    #[inline(always)]
    pub fn is_usart0_tx(&self) -> bool {
        *self == PERIDR::USART0_TX
    }
    #[doc = "Checks if the value of the field is `USART0_RX`"]
    #[inline(always)]
    pub fn is_usart0_rx(&self) -> bool {
        *self == PERIDR::USART0_RX
    }
    #[doc = "Checks if the value of the field is `USART1_TX`"]
    #[inline(always)]
    pub fn is_usart1_tx(&self) -> bool {
        *self == PERIDR::USART1_TX
    }
    #[doc = "Checks if the value of the field is `USART1_RX`"]
    #[inline(always)]
    pub fn is_usart1_rx(&self) -> bool {
        *self == PERIDR::USART1_RX
    }
    #[doc = "Checks if the value of the field is `USART2_TX`"]
    #[inline(always)]
    pub fn is_usart2_tx(&self) -> bool {
        *self == PERIDR::USART2_TX
    }
    #[doc = "Checks if the value of the field is `USART2_RX`"]
    #[inline(always)]
    pub fn is_usart2_rx(&self) -> bool {
        *self == PERIDR::USART2_RX
    }
    #[doc = "Checks if the value of the field is `PWM0`"]
    #[inline(always)]
    pub fn is_pwm0(&self) -> bool {
        *self == PERIDR::PWM0
    }
    #[doc = "Checks if the value of the field is `TWIHS0_TX`"]
    #[inline(always)]
    pub fn is_twihs0_tx(&self) -> bool {
        *self == PERIDR::TWIHS0_TX
    }
    #[doc = "Checks if the value of the field is `TWIHS0_RX`"]
    #[inline(always)]
    pub fn is_twihs0_rx(&self) -> bool {
        *self == PERIDR::TWIHS0_RX
    }
    #[doc = "Checks if the value of the field is `TWIHS1_TX`"]
    #[inline(always)]
    pub fn is_twihs1_tx(&self) -> bool {
        *self == PERIDR::TWIHS1_TX
    }
    #[doc = "Checks if the value of the field is `TWIHS1_RX`"]
    #[inline(always)]
    pub fn is_twihs1_rx(&self) -> bool {
        *self == PERIDR::TWIHS1_RX
    }
    #[doc = "Checks if the value of the field is `TWIHS2_TX`"]
    #[inline(always)]
    pub fn is_twihs2_tx(&self) -> bool {
        *self == PERIDR::TWIHS2_TX
    }
    #[doc = "Checks if the value of the field is `TWIHS2_RX`"]
    #[inline(always)]
    pub fn is_twihs2_rx(&self) -> bool {
        *self == PERIDR::TWIHS2_RX
    }
    #[doc = "Checks if the value of the field is `UART0_TX`"]
    #[inline(always)]
    pub fn is_uart0_tx(&self) -> bool {
        *self == PERIDR::UART0_TX
    }
    #[doc = "Checks if the value of the field is `UART0_RX`"]
    #[inline(always)]
    pub fn is_uart0_rx(&self) -> bool {
        *self == PERIDR::UART0_RX
    }
    #[doc = "Checks if the value of the field is `UART1_TX`"]
    #[inline(always)]
    pub fn is_uart1_tx(&self) -> bool {
        *self == PERIDR::UART1_TX
    }
    #[doc = "Checks if the value of the field is `UART1_RX`"]
    #[inline(always)]
    pub fn is_uart1_rx(&self) -> bool {
        *self == PERIDR::UART1_RX
    }
    #[doc = "Checks if the value of the field is `UART2_TX`"]
    #[inline(always)]
    pub fn is_uart2_tx(&self) -> bool {
        *self == PERIDR::UART2_TX
    }
    #[doc = "Checks if the value of the field is `UART2_RX`"]
    #[inline(always)]
    pub fn is_uart2_rx(&self) -> bool {
        *self == PERIDR::UART2_RX
    }
    #[doc = "Checks if the value of the field is `UART3_TX`"]
    #[inline(always)]
    pub fn is_uart3_tx(&self) -> bool {
        *self == PERIDR::UART3_TX
    }
    #[doc = "Checks if the value of the field is `UART3_RX`"]
    #[inline(always)]
    pub fn is_uart3_rx(&self) -> bool {
        *self == PERIDR::UART3_RX
    }
    #[doc = "Checks if the value of the field is `UART4_TX`"]
    #[inline(always)]
    pub fn is_uart4_tx(&self) -> bool {
        *self == PERIDR::UART4_TX
    }
    #[doc = "Checks if the value of the field is `UART4_RX`"]
    #[inline(always)]
    pub fn is_uart4_rx(&self) -> bool {
        *self == PERIDR::UART4_RX
    }
    #[doc = "Checks if the value of the field is `DACC0`"]
    #[inline(always)]
    pub fn is_dacc0(&self) -> bool {
        *self == PERIDR::DACC0
    }
    #[doc = "Checks if the value of the field is `DACC1`"]
    #[inline(always)]
    pub fn is_dacc1(&self) -> bool {
        *self == PERIDR::DACC1
    }
    #[doc = "Checks if the value of the field is `SSC_TX`"]
    #[inline(always)]
    pub fn is_ssc_tx(&self) -> bool {
        *self == PERIDR::SSC_TX
    }
    #[doc = "Checks if the value of the field is `SSC_RX`"]
    #[inline(always)]
    pub fn is_ssc_rx(&self) -> bool {
        *self == PERIDR::SSC_RX
    }
    #[doc = "Checks if the value of the field is `PIOA`"]
    #[inline(always)]
    pub fn is_pioa(&self) -> bool {
        *self == PERIDR::PIOA
    }
    #[doc = "Checks if the value of the field is `AFEC0`"]
    #[inline(always)]
    pub fn is_afec0(&self) -> bool {
        *self == PERIDR::AFEC0
    }
    #[doc = "Checks if the value of the field is `AFEC1`"]
    #[inline(always)]
    pub fn is_afec1(&self) -> bool {
        *self == PERIDR::AFEC1
    }
    #[doc = "Checks if the value of the field is `AES_TX`"]
    #[inline(always)]
    pub fn is_aes_tx(&self) -> bool {
        *self == PERIDR::AES_TX
    }
    #[doc = "Checks if the value of the field is `AES_RX`"]
    #[inline(always)]
    pub fn is_aes_rx(&self) -> bool {
        *self == PERIDR::AES_RX
    }
    #[doc = "Checks if the value of the field is `PWM1`"]
    #[inline(always)]
    pub fn is_pwm1(&self) -> bool {
        *self == PERIDR::PWM1
    }
    #[doc = "Checks if the value of the field is `TC0`"]
    #[inline(always)]
    pub fn is_tc0(&self) -> bool {
        *self == PERIDR::TC0
    }
    #[doc = "Checks if the value of the field is `TC3`"]
    #[inline(always)]
    pub fn is_tc3(&self) -> bool {
        *self == PERIDR::TC3
    }
    #[doc = "Checks if the value of the field is `TC6`"]
    #[inline(always)]
    pub fn is_tc6(&self) -> bool {
        *self == PERIDR::TC6
    }
    #[doc = "Checks if the value of the field is `TC9`"]
    #[inline(always)]
    pub fn is_tc9(&self) -> bool {
        *self == PERIDR::TC9
    }
    #[doc = "Checks if the value of the field is `I2SC0_TX_LEFT`"]
    #[inline(always)]
    pub fn is_i2sc0_tx_left(&self) -> bool {
        *self == PERIDR::I2SC0_TX_LEFT
    }
    #[doc = "Checks if the value of the field is `I2SC0_RX_LEFT`"]
    #[inline(always)]
    pub fn is_i2sc0_rx_left(&self) -> bool {
        *self == PERIDR::I2SC0_RX_LEFT
    }
    #[doc = "Checks if the value of the field is `I2SC0_TX_RIGHT`"]
    #[inline(always)]
    pub fn is_i2sc0_tx_right(&self) -> bool {
        *self == PERIDR::I2SC0_TX_RIGHT
    }
    #[doc = "Checks if the value of the field is `I2SC0_RX_RIGHT`"]
    #[inline(always)]
    pub fn is_i2sc0_rx_right(&self) -> bool {
        *self == PERIDR::I2SC0_RX_RIGHT
    }
}
#[doc = "Values that can be written to the field `PERID`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PERIDW {
    #[doc = "HSMCI"]
    HSMCI,
    #[doc = "SPI0_TX"]
    SPI0_TX,
    #[doc = "SPI0_RX"]
    SPI0_RX,
    #[doc = "QSPI_TX"]
    QSPI_TX,
    #[doc = "QSPI_RX"]
    QSPI_RX,
    #[doc = "USART0_TX"]
    USART0_TX,
    #[doc = "USART0_RX"]
    USART0_RX,
    #[doc = "USART1_TX"]
    USART1_TX,
    #[doc = "USART1_RX"]
    USART1_RX,
    #[doc = "USART2_TX"]
    USART2_TX,
    #[doc = "USART2_RX"]
    USART2_RX,
    #[doc = "PWM0"]
    PWM0,
    #[doc = "TWIHS0_TX"]
    TWIHS0_TX,
    #[doc = "TWIHS0_RX"]
    TWIHS0_RX,
    #[doc = "TWIHS1_TX"]
    TWIHS1_TX,
    #[doc = "TWIHS1_RX"]
    TWIHS1_RX,
    #[doc = "TWIHS2_TX"]
    TWIHS2_TX,
    #[doc = "TWIHS2_RX"]
    TWIHS2_RX,
    #[doc = "UART0_TX"]
    UART0_TX,
    #[doc = "UART0_RX"]
    UART0_RX,
    #[doc = "UART1_TX"]
    UART1_TX,
    #[doc = "UART1_RX"]
    UART1_RX,
    #[doc = "UART2_TX"]
    UART2_TX,
    #[doc = "UART2_RX"]
    UART2_RX,
    #[doc = "UART3_TX"]
    UART3_TX,
    #[doc = "UART3_RX"]
    UART3_RX,
    #[doc = "UART4_TX"]
    UART4_TX,
    #[doc = "UART4_RX"]
    UART4_RX,
    #[doc = "DACC0"]
    DACC0,
    #[doc = "DACC1"]
    DACC1,
    #[doc = "SSC_TX"]
    SSC_TX,
    #[doc = "SSC_RX"]
    SSC_RX,
    #[doc = "PIOA"]
    PIOA,
    #[doc = "AFEC0"]
    AFEC0,
    #[doc = "AFEC1"]
    AFEC1,
    #[doc = "AES_TX"]
    AES_TX,
    #[doc = "AES_RX"]
    AES_RX,
    #[doc = "PWM1"]
    PWM1,
    #[doc = "TC0"]
    TC0,
    #[doc = "TC3"]
    TC3,
    #[doc = "TC6"]
    TC6,
    #[doc = "TC9"]
    TC9,
    #[doc = "I2SC0_TX_LEFT"]
    I2SC0_TX_LEFT,
    #[doc = "I2SC0_RX_LEFT"]
    I2SC0_RX_LEFT,
    #[doc = "I2SC0_TX_RIGHT"]
    I2SC0_TX_RIGHT,
    #[doc = "I2SC0_RX_RIGHT"]
    I2SC0_RX_RIGHT,
}
impl PERIDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PERIDW::HSMCI => 0,
            PERIDW::SPI0_TX => 1,
            PERIDW::SPI0_RX => 2,
            PERIDW::QSPI_TX => 5,
            PERIDW::QSPI_RX => 6,
            PERIDW::USART0_TX => 7,
            PERIDW::USART0_RX => 8,
            PERIDW::USART1_TX => 9,
            PERIDW::USART1_RX => 10,
            PERIDW::USART2_TX => 11,
            PERIDW::USART2_RX => 12,
            PERIDW::PWM0 => 13,
            PERIDW::TWIHS0_TX => 14,
            PERIDW::TWIHS0_RX => 15,
            PERIDW::TWIHS1_TX => 16,
            PERIDW::TWIHS1_RX => 17,
            PERIDW::TWIHS2_TX => 18,
            PERIDW::TWIHS2_RX => 19,
            PERIDW::UART0_TX => 20,
            PERIDW::UART0_RX => 21,
            PERIDW::UART1_TX => 22,
            PERIDW::UART1_RX => 23,
            PERIDW::UART2_TX => 24,
            PERIDW::UART2_RX => 25,
            PERIDW::UART3_TX => 26,
            PERIDW::UART3_RX => 27,
            PERIDW::UART4_TX => 28,
            PERIDW::UART4_RX => 29,
            PERIDW::DACC0 => 30,
            PERIDW::DACC1 => 31,
            PERIDW::SSC_TX => 32,
            PERIDW::SSC_RX => 33,
            PERIDW::PIOA => 34,
            PERIDW::AFEC0 => 35,
            PERIDW::AFEC1 => 36,
            PERIDW::AES_TX => 37,
            PERIDW::AES_RX => 38,
            PERIDW::PWM1 => 39,
            PERIDW::TC0 => 40,
            PERIDW::TC3 => 41,
            PERIDW::TC6 => 42,
            PERIDW::TC9 => 43,
            PERIDW::I2SC0_TX_LEFT => 44,
            PERIDW::I2SC0_RX_LEFT => 45,
            PERIDW::I2SC0_TX_RIGHT => 48,
            PERIDW::I2SC0_RX_RIGHT => 49,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PERIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERIDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "HSMCI"]
    #[inline(always)]
    pub fn hsmci(self) -> &'a mut W {
        self.variant(PERIDW::HSMCI)
    }
    #[doc = "SPI0_TX"]
    #[inline(always)]
    pub fn spi0_tx(self) -> &'a mut W {
        self.variant(PERIDW::SPI0_TX)
    }
    #[doc = "SPI0_RX"]
    #[inline(always)]
    pub fn spi0_rx(self) -> &'a mut W {
        self.variant(PERIDW::SPI0_RX)
    }
    #[doc = "QSPI_TX"]
    #[inline(always)]
    pub fn qspi_tx(self) -> &'a mut W {
        self.variant(PERIDW::QSPI_TX)
    }
    #[doc = "QSPI_RX"]
    #[inline(always)]
    pub fn qspi_rx(self) -> &'a mut W {
        self.variant(PERIDW::QSPI_RX)
    }
    #[doc = "USART0_TX"]
    #[inline(always)]
    pub fn usart0_tx(self) -> &'a mut W {
        self.variant(PERIDW::USART0_TX)
    }
    #[doc = "USART0_RX"]
    #[inline(always)]
    pub fn usart0_rx(self) -> &'a mut W {
        self.variant(PERIDW::USART0_RX)
    }
    #[doc = "USART1_TX"]
    #[inline(always)]
    pub fn usart1_tx(self) -> &'a mut W {
        self.variant(PERIDW::USART1_TX)
    }
    #[doc = "USART1_RX"]
    #[inline(always)]
    pub fn usart1_rx(self) -> &'a mut W {
        self.variant(PERIDW::USART1_RX)
    }
    #[doc = "USART2_TX"]
    #[inline(always)]
    pub fn usart2_tx(self) -> &'a mut W {
        self.variant(PERIDW::USART2_TX)
    }
    #[doc = "USART2_RX"]
    #[inline(always)]
    pub fn usart2_rx(self) -> &'a mut W {
        self.variant(PERIDW::USART2_RX)
    }
    #[doc = "PWM0"]
    #[inline(always)]
    pub fn pwm0(self) -> &'a mut W {
        self.variant(PERIDW::PWM0)
    }
    #[doc = "TWIHS0_TX"]
    #[inline(always)]
    pub fn twihs0_tx(self) -> &'a mut W {
        self.variant(PERIDW::TWIHS0_TX)
    }
    #[doc = "TWIHS0_RX"]
    #[inline(always)]
    pub fn twihs0_rx(self) -> &'a mut W {
        self.variant(PERIDW::TWIHS0_RX)
    }
    #[doc = "TWIHS1_TX"]
    #[inline(always)]
    pub fn twihs1_tx(self) -> &'a mut W {
        self.variant(PERIDW::TWIHS1_TX)
    }
    #[doc = "TWIHS1_RX"]
    #[inline(always)]
    pub fn twihs1_rx(self) -> &'a mut W {
        self.variant(PERIDW::TWIHS1_RX)
    }
    #[doc = "TWIHS2_TX"]
    #[inline(always)]
    pub fn twihs2_tx(self) -> &'a mut W {
        self.variant(PERIDW::TWIHS2_TX)
    }
    #[doc = "TWIHS2_RX"]
    #[inline(always)]
    pub fn twihs2_rx(self) -> &'a mut W {
        self.variant(PERIDW::TWIHS2_RX)
    }
    #[doc = "UART0_TX"]
    #[inline(always)]
    pub fn uart0_tx(self) -> &'a mut W {
        self.variant(PERIDW::UART0_TX)
    }
    #[doc = "UART0_RX"]
    #[inline(always)]
    pub fn uart0_rx(self) -> &'a mut W {
        self.variant(PERIDW::UART0_RX)
    }
    #[doc = "UART1_TX"]
    #[inline(always)]
    pub fn uart1_tx(self) -> &'a mut W {
        self.variant(PERIDW::UART1_TX)
    }
    #[doc = "UART1_RX"]
    #[inline(always)]
    pub fn uart1_rx(self) -> &'a mut W {
        self.variant(PERIDW::UART1_RX)
    }
    #[doc = "UART2_TX"]
    #[inline(always)]
    pub fn uart2_tx(self) -> &'a mut W {
        self.variant(PERIDW::UART2_TX)
    }
    #[doc = "UART2_RX"]
    #[inline(always)]
    pub fn uart2_rx(self) -> &'a mut W {
        self.variant(PERIDW::UART2_RX)
    }
    #[doc = "UART3_TX"]
    #[inline(always)]
    pub fn uart3_tx(self) -> &'a mut W {
        self.variant(PERIDW::UART3_TX)
    }
    #[doc = "UART3_RX"]
    #[inline(always)]
    pub fn uart3_rx(self) -> &'a mut W {
        self.variant(PERIDW::UART3_RX)
    }
    #[doc = "UART4_TX"]
    #[inline(always)]
    pub fn uart4_tx(self) -> &'a mut W {
        self.variant(PERIDW::UART4_TX)
    }
    #[doc = "UART4_RX"]
    #[inline(always)]
    pub fn uart4_rx(self) -> &'a mut W {
        self.variant(PERIDW::UART4_RX)
    }
    #[doc = "DACC0"]
    #[inline(always)]
    pub fn dacc0(self) -> &'a mut W {
        self.variant(PERIDW::DACC0)
    }
    #[doc = "DACC1"]
    #[inline(always)]
    pub fn dacc1(self) -> &'a mut W {
        self.variant(PERIDW::DACC1)
    }
    #[doc = "SSC_TX"]
    #[inline(always)]
    pub fn ssc_tx(self) -> &'a mut W {
        self.variant(PERIDW::SSC_TX)
    }
    #[doc = "SSC_RX"]
    #[inline(always)]
    pub fn ssc_rx(self) -> &'a mut W {
        self.variant(PERIDW::SSC_RX)
    }
    #[doc = "PIOA"]
    #[inline(always)]
    pub fn pioa(self) -> &'a mut W {
        self.variant(PERIDW::PIOA)
    }
    #[doc = "AFEC0"]
    #[inline(always)]
    pub fn afec0(self) -> &'a mut W {
        self.variant(PERIDW::AFEC0)
    }
    #[doc = "AFEC1"]
    #[inline(always)]
    pub fn afec1(self) -> &'a mut W {
        self.variant(PERIDW::AFEC1)
    }
    #[doc = "AES_TX"]
    #[inline(always)]
    pub fn aes_tx(self) -> &'a mut W {
        self.variant(PERIDW::AES_TX)
    }
    #[doc = "AES_RX"]
    #[inline(always)]
    pub fn aes_rx(self) -> &'a mut W {
        self.variant(PERIDW::AES_RX)
    }
    #[doc = "PWM1"]
    #[inline(always)]
    pub fn pwm1(self) -> &'a mut W {
        self.variant(PERIDW::PWM1)
    }
    #[doc = "TC0"]
    #[inline(always)]
    pub fn tc0(self) -> &'a mut W {
        self.variant(PERIDW::TC0)
    }
    #[doc = "TC3"]
    #[inline(always)]
    pub fn tc3(self) -> &'a mut W {
        self.variant(PERIDW::TC3)
    }
    #[doc = "TC6"]
    #[inline(always)]
    pub fn tc6(self) -> &'a mut W {
        self.variant(PERIDW::TC6)
    }
    #[doc = "TC9"]
    #[inline(always)]
    pub fn tc9(self) -> &'a mut W {
        self.variant(PERIDW::TC9)
    }
    #[doc = "I2SC0_TX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_tx_left(self) -> &'a mut W {
        self.variant(PERIDW::I2SC0_TX_LEFT)
    }
    #[doc = "I2SC0_RX_LEFT"]
    #[inline(always)]
    pub fn i2sc0_rx_left(self) -> &'a mut W {
        self.variant(PERIDW::I2SC0_RX_LEFT)
    }
    #[doc = "I2SC0_TX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_tx_right(self) -> &'a mut W {
        self.variant(PERIDW::I2SC0_TX_RIGHT)
    }
    #[doc = "I2SC0_RX_RIGHT"]
    #[inline(always)]
    pub fn i2sc0_rx_right(self) -> &'a mut W {
        self.variant(PERIDW::I2SC0_RX_RIGHT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&self) -> TYPE_R {
        TYPE_R::new((self.bits() & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&self) -> MBSIZE_R {
        MBSIZE_R::new(((self.bits() >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&self) -> DSYNC_R {
        DSYNC_R::new(((self.bits() >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&self) -> SWREQ_R {
        SWREQ_R::new(((self.bits() >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&self) -> MEMSET_R {
        MEMSET_R::new(((self.bits() >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&self) -> CSIZE_R {
        CSIZE_R::new(((self.bits() >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&self) -> DWIDTH_R {
        DWIDTH_R::new(((self.bits() >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&self) -> SIF_R {
        SIF_R::new(((self.bits() >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits() >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&self) -> SAM_R {
        SAM_R::new(((self.bits() >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&self) -> DAM_R {
        DAM_R::new(((self.bits() >> 18) & 0x03) as u8)
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&self) -> INITD_R {
        INITD_R::new(((self.bits() >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&self) -> RDIP_R {
        RDIP_R::new(((self.bits() >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&self) -> WRIP_R {
        WRIP_R::new(((self.bits() >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&self) -> PERID_R {
        PERID_R::new(((self.bits() >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline(always)]
    pub fn type_(&mut self) -> _TYPEW {
        _TYPEW { w: self }
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline(always)]
    pub fn mbsize(&mut self) -> _MBSIZEW {
        _MBSIZEW { w: self }
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline(always)]
    pub fn dsync(&mut self) -> _DSYNCW {
        _DSYNCW { w: self }
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline(always)]
    pub fn swreq(&mut self) -> _SWREQW {
        _SWREQW { w: self }
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline(always)]
    pub fn memset(&mut self) -> _MEMSETW {
        _MEMSETW { w: self }
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline(always)]
    pub fn csize(&mut self) -> _CSIZEW {
        _CSIZEW { w: self }
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline(always)]
    pub fn dwidth(&mut self) -> _DWIDTHW {
        _DWIDTHW { w: self }
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline(always)]
    pub fn sif(&mut self) -> _SIFW {
        _SIFW { w: self }
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline(always)]
    pub fn dif(&mut self) -> _DIFW {
        _DIFW { w: self }
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline(always)]
    pub fn sam(&mut self) -> _SAMW {
        _SAMW { w: self }
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline(always)]
    pub fn dam(&mut self) -> _DAMW {
        _DAMW { w: self }
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline(always)]
    pub fn initd(&mut self) -> _INITDW {
        _INITDW { w: self }
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn rdip(&mut self) -> _RDIPW {
        _RDIPW { w: self }
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline(always)]
    pub fn wrip(&mut self) -> _WRIPW {
        _WRIPW { w: self }
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline(always)]
    pub fn perid(&mut self) -> _PERIDW {
        _PERIDW { w: self }
    }
}
