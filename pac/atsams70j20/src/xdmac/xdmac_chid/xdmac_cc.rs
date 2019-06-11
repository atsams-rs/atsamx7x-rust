#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::XDMAC_CC {
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
#[doc = "Possible values of the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPER {
    #[doc = "Self triggered mode (Memory to Memory Transfer)."]
    MEM_TRAN,
    #[doc = "Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    PER_TRAN,
}
impl TYPER {
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
            TYPER::MEM_TRAN => false,
            TYPER::PER_TRAN => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TYPER {
        match value {
            false => TYPER::MEM_TRAN,
            true => TYPER::PER_TRAN,
        }
    }
    #[doc = "Checks if the value of the field is `MEM_TRAN`"]
    #[inline]
    pub fn is_mem_tran(&self) -> bool {
        *self == TYPER::MEM_TRAN
    }
    #[doc = "Checks if the value of the field is `PER_TRAN`"]
    #[inline]
    pub fn is_per_tran(&self) -> bool {
        *self == TYPER::PER_TRAN
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
impl MBSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            MBSIZER::SINGLE => 0,
            MBSIZER::FOUR => 1,
            MBSIZER::EIGHT => 2,
            MBSIZER::SIXTEEN => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> MBSIZER {
        match value {
            0 => MBSIZER::SINGLE,
            1 => MBSIZER::FOUR,
            2 => MBSIZER::EIGHT,
            3 => MBSIZER::SIXTEEN,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline]
    pub fn is_single(&self) -> bool {
        *self == MBSIZER::SINGLE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline]
    pub fn is_four(&self) -> bool {
        *self == MBSIZER::FOUR
    }
    #[doc = "Checks if the value of the field is `EIGHT`"]
    #[inline]
    pub fn is_eight(&self) -> bool {
        *self == MBSIZER::EIGHT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN`"]
    #[inline]
    pub fn is_sixteen(&self) -> bool {
        *self == MBSIZER::SIXTEEN
    }
}
#[doc = "Possible values of the field `DSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSYNCR {
    #[doc = "Peripheral to Memory transfer."]
    PER2MEM,
    #[doc = "Memory to Peripheral transfer."]
    MEM2PER,
}
impl DSYNCR {
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
            DSYNCR::PER2MEM => false,
            DSYNCR::MEM2PER => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DSYNCR {
        match value {
            false => DSYNCR::PER2MEM,
            true => DSYNCR::MEM2PER,
        }
    }
    #[doc = "Checks if the value of the field is `PER2MEM`"]
    #[inline]
    pub fn is_per2mem(&self) -> bool {
        *self == DSYNCR::PER2MEM
    }
    #[doc = "Checks if the value of the field is `MEM2PER`"]
    #[inline]
    pub fn is_mem2per(&self) -> bool {
        *self == DSYNCR::MEM2PER
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
impl SWREQR {
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
            SWREQR::HWR_CONNECTED => false,
            SWREQR::SWR_CONNECTED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SWREQR {
        match value {
            false => SWREQR::HWR_CONNECTED,
            true => SWREQR::SWR_CONNECTED,
        }
    }
    #[doc = "Checks if the value of the field is `HWR_CONNECTED`"]
    #[inline]
    pub fn is_hwr_connected(&self) -> bool {
        *self == SWREQR::HWR_CONNECTED
    }
    #[doc = "Checks if the value of the field is `SWR_CONNECTED`"]
    #[inline]
    pub fn is_swr_connected(&self) -> bool {
        *self == SWREQR::SWR_CONNECTED
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
impl MEMSETR {
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
            MEMSETR::NORMAL_MODE => false,
            MEMSETR::HW_MODE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> MEMSETR {
        match value {
            false => MEMSETR::NORMAL_MODE,
            true => MEMSETR::HW_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE`"]
    #[inline]
    pub fn is_normal_mode(&self) -> bool {
        *self == MEMSETR::NORMAL_MODE
    }
    #[doc = "Checks if the value of the field is `HW_MODE`"]
    #[inline]
    pub fn is_hw_mode(&self) -> bool {
        *self == MEMSETR::HW_MODE
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl CSIZER {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            CSIZER::CHK_1 => 0,
            CSIZER::CHK_2 => 1,
            CSIZER::CHK_4 => 2,
            CSIZER::CHK_8 => 3,
            CSIZER::CHK_16 => 4,
            CSIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> CSIZER {
        match value {
            0 => CSIZER::CHK_1,
            1 => CSIZER::CHK_2,
            2 => CSIZER::CHK_4,
            3 => CSIZER::CHK_8,
            4 => CSIZER::CHK_16,
            i => CSIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `CHK_1`"]
    #[inline]
    pub fn is_chk_1(&self) -> bool {
        *self == CSIZER::CHK_1
    }
    #[doc = "Checks if the value of the field is `CHK_2`"]
    #[inline]
    pub fn is_chk_2(&self) -> bool {
        *self == CSIZER::CHK_2
    }
    #[doc = "Checks if the value of the field is `CHK_4`"]
    #[inline]
    pub fn is_chk_4(&self) -> bool {
        *self == CSIZER::CHK_4
    }
    #[doc = "Checks if the value of the field is `CHK_8`"]
    #[inline]
    pub fn is_chk_8(&self) -> bool {
        *self == CSIZER::CHK_8
    }
    #[doc = "Checks if the value of the field is `CHK_16`"]
    #[inline]
    pub fn is_chk_16(&self) -> bool {
        *self == CSIZER::CHK_16
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
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl DWIDTHR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DWIDTHR::BYTE => 0,
            DWIDTHR::HALFWORD => 1,
            DWIDTHR::WORD => 2,
            DWIDTHR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DWIDTHR {
        match value {
            0 => DWIDTHR::BYTE,
            1 => DWIDTHR::HALFWORD,
            2 => DWIDTHR::WORD,
            i => DWIDTHR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline]
    pub fn is_byte(&self) -> bool {
        *self == DWIDTHR::BYTE
    }
    #[doc = "Checks if the value of the field is `HALFWORD`"]
    #[inline]
    pub fn is_halfword(&self) -> bool {
        *self == DWIDTHR::HALFWORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline]
    pub fn is_word(&self) -> bool {
        *self == DWIDTHR::WORD
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
impl SIFR {
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
            SIFR::AHB_IF0 => false,
            SIFR::AHB_IF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> SIFR {
        match value {
            false => SIFR::AHB_IF0,
            true => SIFR::AHB_IF1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline]
    pub fn is_ahb_if0(&self) -> bool {
        *self == SIFR::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline]
    pub fn is_ahb_if1(&self) -> bool {
        *self == SIFR::AHB_IF1
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
impl DIFR {
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
            DIFR::AHB_IF0 => false,
            DIFR::AHB_IF1 => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> DIFR {
        match value {
            false => DIFR::AHB_IF0,
            true => DIFR::AHB_IF1,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline]
    pub fn is_ahb_if0(&self) -> bool {
        *self == DIFR::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline]
    pub fn is_ahb_if1(&self) -> bool {
        *self == DIFR::AHB_IF1
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
impl SAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            SAMR::FIXED_AM => 0,
            SAMR::INCREMENTED_AM => 1,
            SAMR::UBS_AM => 2,
            SAMR::UBS_DS_AM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> SAMR {
        match value {
            0 => SAMR::FIXED_AM,
            1 => SAMR::INCREMENTED_AM,
            2 => SAMR::UBS_AM,
            3 => SAMR::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline]
    pub fn is_fixed_am(&self) -> bool {
        *self == SAMR::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline]
    pub fn is_incremented_am(&self) -> bool {
        *self == SAMR::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline]
    pub fn is_ubs_am(&self) -> bool {
        *self == SAMR::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == SAMR::UBS_DS_AM
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
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl DAMR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            DAMR::FIXED_AM => 0,
            DAMR::INCREMENTED_AM => 1,
            DAMR::UBS_AM => 2,
            DAMR::UBS_DS_AM => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> DAMR {
        match value {
            0 => DAMR::FIXED_AM,
            1 => DAMR::INCREMENTED_AM,
            2 => DAMR::UBS_AM,
            3 => DAMR::UBS_DS_AM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FIXED_AM`"]
    #[inline]
    pub fn is_fixed_am(&self) -> bool {
        *self == DAMR::FIXED_AM
    }
    #[doc = "Checks if the value of the field is `INCREMENTED_AM`"]
    #[inline]
    pub fn is_incremented_am(&self) -> bool {
        *self == DAMR::INCREMENTED_AM
    }
    #[doc = "Checks if the value of the field is `UBS_AM`"]
    #[inline]
    pub fn is_ubs_am(&self) -> bool {
        *self == DAMR::UBS_AM
    }
    #[doc = "Checks if the value of the field is `UBS_DS_AM`"]
    #[inline]
    pub fn is_ubs_ds_am(&self) -> bool {
        *self == DAMR::UBS_DS_AM
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
impl INITDR {
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
            INITDR::IN_PROGRESS => false,
            INITDR::TERMINATED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INITDR {
        match value {
            false => INITDR::IN_PROGRESS,
            true => INITDR::TERMINATED,
        }
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline]
    pub fn is_in_progress(&self) -> bool {
        *self == INITDR::IN_PROGRESS
    }
    #[doc = "Checks if the value of the field is `TERMINATED`"]
    #[inline]
    pub fn is_terminated(&self) -> bool {
        *self == INITDR::TERMINATED
    }
}
#[doc = "Possible values of the field `RDIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIPR {
    #[doc = "No Active read transaction on the bus."]
    DONE,
    #[doc = "A read transaction is in progress."]
    IN_PROGRESS,
}
impl RDIPR {
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
            RDIPR::DONE => false,
            RDIPR::IN_PROGRESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> RDIPR {
        match value {
            false => RDIPR::DONE,
            true => RDIPR::IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline]
    pub fn is_done(&self) -> bool {
        *self == RDIPR::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline]
    pub fn is_in_progress(&self) -> bool {
        *self == RDIPR::IN_PROGRESS
    }
}
#[doc = "Possible values of the field `WRIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRIPR {
    #[doc = "No Active write transaction on the bus."]
    DONE,
    #[doc = "A Write transaction is in progress."]
    IN_PROGRESS,
}
impl WRIPR {
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
            WRIPR::DONE => false,
            WRIPR::IN_PROGRESS => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> WRIPR {
        match value {
            false => WRIPR::DONE,
            true => WRIPR::IN_PROGRESS,
        }
    }
    #[doc = "Checks if the value of the field is `DONE`"]
    #[inline]
    pub fn is_done(&self) -> bool {
        *self == WRIPR::DONE
    }
    #[doc = "Checks if the value of the field is `IN_PROGRESS`"]
    #[inline]
    pub fn is_in_progress(&self) -> bool {
        *self == WRIPR::IN_PROGRESS
    }
}
#[doc = r" Value of the field"]
pub struct PERIDR {
    bits: u8,
}
impl PERIDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = "Values that can be written to the field `TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TYPEW {
    #[doc = "Self triggered mode (Memory to Memory Transfer)."]
    MEM_TRAN,
    #[doc = "Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    PER_TRAN,
}
impl TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TYPEW::MEM_TRAN => false,
            TYPEW::PER_TRAN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _TYPEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TYPEW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Self triggered mode (Memory to Memory Transfer)."]
    #[inline]
    pub fn mem_tran(self) -> &'a mut W {
        self.variant(TYPEW::MEM_TRAN)
    }
    #[doc = "Synchronized mode (Peripheral to Memory or Memory to Peripheral Transfer)."]
    #[inline]
    pub fn per_tran(self) -> &'a mut W {
        self.variant(TYPEW::PER_TRAN)
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
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            MBSIZEW::SINGLE => 0,
            MBSIZEW::FOUR => 1,
            MBSIZEW::EIGHT => 2,
            MBSIZEW::SIXTEEN => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MBSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _MBSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MBSIZEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The memory burst size is set to one."]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(MBSIZEW::SINGLE)
    }
    #[doc = "The memory burst size is set to four."]
    #[inline]
    pub fn four(self) -> &'a mut W {
        self.variant(MBSIZEW::FOUR)
    }
    #[doc = "The memory burst size is set to eight."]
    #[inline]
    pub fn eight(self) -> &'a mut W {
        self.variant(MBSIZEW::EIGHT)
    }
    #[doc = "The memory burst size is set to sixteen."]
    #[inline]
    pub fn sixteen(self) -> &'a mut W {
        self.variant(MBSIZEW::SIXTEEN)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `DSYNC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSYNCW {
    #[doc = "Peripheral to Memory transfer."]
    PER2MEM,
    #[doc = "Memory to Peripheral transfer."]
    MEM2PER,
}
impl DSYNCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DSYNCW::PER2MEM => false,
            DSYNCW::MEM2PER => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DSYNCW<'a> {
    w: &'a mut W,
}
impl<'a> _DSYNCW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DSYNCW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Peripheral to Memory transfer."]
    #[inline]
    pub fn per2mem(self) -> &'a mut W {
        self.variant(DSYNCW::PER2MEM)
    }
    #[doc = "Memory to Peripheral transfer."]
    #[inline]
    pub fn mem2per(self) -> &'a mut W {
        self.variant(DSYNCW::MEM2PER)
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
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SWREQW::HWR_CONNECTED => false,
            SWREQW::SWR_CONNECTED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SWREQW<'a> {
    w: &'a mut W,
}
impl<'a> _SWREQW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SWREQW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Hardware request line is connected to the peripheral request line."]
    #[inline]
    pub fn hwr_connected(self) -> &'a mut W {
        self.variant(SWREQW::HWR_CONNECTED)
    }
    #[doc = "Software request is connected to the peripheral request line."]
    #[inline]
    pub fn swr_connected(self) -> &'a mut W {
        self.variant(SWREQW::SWR_CONNECTED)
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
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MEMSETW::NORMAL_MODE => false,
            MEMSETW::HW_MODE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MEMSETW<'a> {
    w: &'a mut W,
}
impl<'a> _MEMSETW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MEMSETW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Memset is not activated."]
    #[inline]
    pub fn normal_mode(self) -> &'a mut W {
        self.variant(MEMSETW::NORMAL_MODE)
    }
    #[doc = "Sets the block of memory pointed by DA field to the specified value. This operation is performed on 8-, 16- or 32-bit basis."]
    #[inline]
    pub fn hw_mode(self) -> &'a mut W {
        self.variant(MEMSETW::HW_MODE)
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
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
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
#[doc = r" Proxy"]
pub struct _CSIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _CSIZEW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: CSIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "1 data transferred"]
    #[inline]
    pub fn chk_1(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_1)
    }
    #[doc = "2 data transferred"]
    #[inline]
    pub fn chk_2(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_2)
    }
    #[doc = "4 data transferred"]
    #[inline]
    pub fn chk_4(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_4)
    }
    #[doc = "8 data transferred"]
    #[inline]
    pub fn chk_8(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_8)
    }
    #[doc = "16 data transferred"]
    #[inline]
    pub fn chk_16(self) -> &'a mut W {
        self.variant(CSIZEW::CHK_16)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DWIDTHW::BYTE => 0,
            DWIDTHW::HALFWORD => 1,
            DWIDTHW::WORD => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _DWIDTHW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DWIDTHW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "The data size is set to 8 bits"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(DWIDTHW::BYTE)
    }
    #[doc = "The data size is set to 16 bits"]
    #[inline]
    pub fn halfword(self) -> &'a mut W {
        self.variant(DWIDTHW::HALFWORD)
    }
    #[doc = "The data size is set to 32 bits"]
    #[inline]
    pub fn word(self) -> &'a mut W {
        self.variant(DWIDTHW::WORD)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            SIFW::AHB_IF0 => false,
            SIFW::AHB_IF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SIFW<'a> {
    w: &'a mut W,
}
impl<'a> _SIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data is read through the system bus interface 0."]
    #[inline]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(SIFW::AHB_IF0)
    }
    #[doc = "The data is read through the system bus interface 1."]
    #[inline]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(SIFW::AHB_IF1)
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
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            DIFW::AHB_IF0 => false,
            DIFW::AHB_IF1 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DIFW<'a> {
    w: &'a mut W,
}
impl<'a> _DIFW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DIFW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "The data is written through the system bus interface 0."]
    #[inline]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(DIFW::AHB_IF0)
    }
    #[doc = "The data is written though the system bus interface 1."]
    #[inline]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(DIFW::AHB_IF1)
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
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SAMW::FIXED_AM => 0,
            SAMW::INCREMENTED_AM => 1,
            SAMW::UBS_AM => 2,
            SAMW::UBS_DS_AM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SAMW<'a> {
    w: &'a mut W,
}
impl<'a> _SAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(SAMW::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(SAMW::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(SAMW::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(SAMW::UBS_DS_AM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    UBS_DS_AM,
}
impl DAMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            DAMW::FIXED_AM => 0,
            DAMW::INCREMENTED_AM => 1,
            DAMW::UBS_AM => 2,
            DAMW::UBS_DS_AM => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _DAMW<'a> {
    w: &'a mut W,
}
impl<'a> _DAMW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: DAMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The address remains unchanged."]
    #[inline]
    pub fn fixed_am(self) -> &'a mut W {
        self.variant(DAMW::FIXED_AM)
    }
    #[doc = "The addressing mode is incremented (the increment size is set to the data size)."]
    #[inline]
    pub fn incremented_am(self) -> &'a mut W {
        self.variant(DAMW::INCREMENTED_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary."]
    #[inline]
    pub fn ubs_am(self) -> &'a mut W {
        self.variant(DAMW::UBS_AM)
    }
    #[doc = "The microblock stride is added at the microblock boundary, the data stride is added at the data boundary."]
    #[inline]
    pub fn ubs_ds_am(self) -> &'a mut W {
        self.variant(DAMW::UBS_DS_AM)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INITDW::IN_PROGRESS => false,
            INITDW::TERMINATED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INITDW<'a> {
    w: &'a mut W,
}
impl<'a> _INITDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INITDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Channel initialization is in progress."]
    #[inline]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(INITDW::IN_PROGRESS)
    }
    #[doc = "Channel initialization is completed."]
    #[inline]
    pub fn terminated(self) -> &'a mut W {
        self.variant(INITDW::TERMINATED)
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
        const OFFSET: u8 = 21;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RDIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RDIPW {
    #[doc = "No Active read transaction on the bus."]
    DONE,
    #[doc = "A read transaction is in progress."]
    IN_PROGRESS,
}
impl RDIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            RDIPW::DONE => false,
            RDIPW::IN_PROGRESS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RDIPW<'a> {
    w: &'a mut W,
}
impl<'a> _RDIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RDIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Active read transaction on the bus."]
    #[inline]
    pub fn done(self) -> &'a mut W {
        self.variant(RDIPW::DONE)
    }
    #[doc = "A read transaction is in progress."]
    #[inline]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(RDIPW::IN_PROGRESS)
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
        const OFFSET: u8 = 22;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `WRIP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WRIPW {
    #[doc = "No Active write transaction on the bus."]
    DONE,
    #[doc = "A Write transaction is in progress."]
    IN_PROGRESS,
}
impl WRIPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            WRIPW::DONE => false,
            WRIPW::IN_PROGRESS => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _WRIPW<'a> {
    w: &'a mut W,
}
impl<'a> _WRIPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: WRIPW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No Active write transaction on the bus."]
    #[inline]
    pub fn done(self) -> &'a mut W {
        self.variant(WRIPW::DONE)
    }
    #[doc = "A Write transaction is in progress."]
    #[inline]
    pub fn in_progress(self) -> &'a mut W {
        self.variant(WRIPW::IN_PROGRESS)
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
        const OFFSET: u8 = 23;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _PERIDW<'a> {
    w: &'a mut W,
}
impl<'a> _PERIDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 127;
        const OFFSET: u8 = 24;
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
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline]
    pub fn type_(&self) -> TYPER {
        TYPER::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline]
    pub fn mbsize(&self) -> MBSIZER {
        MBSIZER::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline]
    pub fn dsync(&self) -> DSYNCR {
        DSYNCR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline]
    pub fn swreq(&self) -> SWREQR {
        SWREQR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 6;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline]
    pub fn memset(&self) -> MEMSETR {
        MEMSETR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 7;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline]
    pub fn csize(&self) -> CSIZER {
        CSIZER::_from({
            const MASK: u8 = 7;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline]
    pub fn dwidth(&self) -> DWIDTHR {
        DWIDTHR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 11;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline]
    pub fn sif(&self) -> SIFR {
        SIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 13;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline]
    pub fn dif(&self) -> DIFR {
        DIFR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 14;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline]
    pub fn sam(&self) -> SAMR {
        SAMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline]
    pub fn dam(&self) -> DAMR {
        DAMR::_from({
            const MASK: u8 = 3;
            const OFFSET: u8 = 18;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline]
    pub fn initd(&self) -> INITDR {
        INITDR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 21;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline]
    pub fn rdip(&self) -> RDIPR {
        RDIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 22;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline]
    pub fn wrip(&self) -> WRIPR {
        WRIPR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 23;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline]
    pub fn perid(&self) -> PERIDR {
        let bits = {
            const MASK: u8 = 127;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        PERIDR { bits }
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
    #[doc = "Bit 0 - Channel x Transfer Type"]
    #[inline]
    pub fn type_(&mut self) -> _TYPEW {
        _TYPEW { w: self }
    }
    #[doc = "Bits 1:2 - Channel x Memory Burst Size"]
    #[inline]
    pub fn mbsize(&mut self) -> _MBSIZEW {
        _MBSIZEW { w: self }
    }
    #[doc = "Bit 4 - Channel x Synchronization"]
    #[inline]
    pub fn dsync(&mut self) -> _DSYNCW {
        _DSYNCW { w: self }
    }
    #[doc = "Bit 6 - Channel x Software Request Trigger"]
    #[inline]
    pub fn swreq(&mut self) -> _SWREQW {
        _SWREQW { w: self }
    }
    #[doc = "Bit 7 - Channel x Fill Block of memory"]
    #[inline]
    pub fn memset(&mut self) -> _MEMSETW {
        _MEMSETW { w: self }
    }
    #[doc = "Bits 8:10 - Channel x Chunk Size"]
    #[inline]
    pub fn csize(&mut self) -> _CSIZEW {
        _CSIZEW { w: self }
    }
    #[doc = "Bits 11:12 - Channel x Data Width"]
    #[inline]
    pub fn dwidth(&mut self) -> _DWIDTHW {
        _DWIDTHW { w: self }
    }
    #[doc = "Bit 13 - Channel x Source Interface Identifier"]
    #[inline]
    pub fn sif(&mut self) -> _SIFW {
        _SIFW { w: self }
    }
    #[doc = "Bit 14 - Channel x Destination Interface Identifier"]
    #[inline]
    pub fn dif(&mut self) -> _DIFW {
        _DIFW { w: self }
    }
    #[doc = "Bits 16:17 - Channel x Source Addressing Mode"]
    #[inline]
    pub fn sam(&mut self) -> _SAMW {
        _SAMW { w: self }
    }
    #[doc = "Bits 18:19 - Channel x Destination Addressing Mode"]
    #[inline]
    pub fn dam(&mut self) -> _DAMW {
        _DAMW { w: self }
    }
    #[doc = "Bit 21 - Channel Initialization Terminated (this bit is read-only)"]
    #[inline]
    pub fn initd(&mut self) -> _INITDW {
        _INITDW { w: self }
    }
    #[doc = "Bit 22 - Read in Progress (this bit is read-only)"]
    #[inline]
    pub fn rdip(&mut self) -> _RDIPW {
        _RDIPW { w: self }
    }
    #[doc = "Bit 23 - Write in Progress (this bit is read-only)"]
    #[inline]
    pub fn wrip(&mut self) -> _WRIPW {
        _WRIPW { w: self }
    }
    #[doc = "Bits 24:30 - Channel x Peripheral Hardware Request Line Identifier"]
    #[inline]
    pub fn perid(&mut self) -> _PERIDW {
        _PERIDW { w: self }
    }
}
