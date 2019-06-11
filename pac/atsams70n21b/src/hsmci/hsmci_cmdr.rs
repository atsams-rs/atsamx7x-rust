#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSMCI_CMDR {
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
}
#[doc = r" Proxy"]
pub struct _CMDNBW<'a> {
    w: &'a mut W,
}
impl<'a> _CMDNBW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 63;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RSPTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSPTYPW {
    #[doc = "No response"]
    NORESP,
    #[doc = "48-bit response"]
    _48_BIT,
    #[doc = "136-bit response"]
    _136_BIT,
    #[doc = "R1b response type"]
    R1B,
}
impl RSPTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RSPTYPW::NORESP => 0,
            RSPTYPW::_48_BIT => 1,
            RSPTYPW::_136_BIT => 2,
            RSPTYPW::R1B => 3,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RSPTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _RSPTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RSPTYPW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No response"]
    #[inline]
    pub fn noresp(self) -> &'a mut W {
        self.variant(RSPTYPW::NORESP)
    }
    #[doc = "48-bit response"]
    #[inline]
    pub fn _48_bit(self) -> &'a mut W {
        self.variant(RSPTYPW::_48_BIT)
    }
    #[doc = "136-bit response"]
    #[inline]
    pub fn _136_bit(self) -> &'a mut W {
        self.variant(RSPTYPW::_136_BIT)
    }
    #[doc = "R1b response type"]
    #[inline]
    pub fn r1b(self) -> &'a mut W {
        self.variant(RSPTYPW::R1B)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `SPCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPCMDW {
    #[doc = "Not a special CMD."]
    STD,
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    INIT,
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    SYNC,
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    CE_ATA,
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    IT_CMD,
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    IT_RESP,
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    BOR,
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    EBO,
}
impl SPCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            SPCMDW::STD => 0,
            SPCMDW::INIT => 1,
            SPCMDW::SYNC => 2,
            SPCMDW::CE_ATA => 3,
            SPCMDW::IT_CMD => 4,
            SPCMDW::IT_RESP => 5,
            SPCMDW::BOR => 6,
            SPCMDW::EBO => 7,
        }
    }
}
#[doc = r" Proxy"]
pub struct _SPCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _SPCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: SPCMDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not a special CMD."]
    #[inline]
    pub fn std(self) -> &'a mut W {
        self.variant(SPCMDW::STD)
    }
    #[doc = "Initialization CMD: 74 clock cycles for initialization sequence."]
    #[inline]
    pub fn init(self) -> &'a mut W {
        self.variant(SPCMDW::INIT)
    }
    #[doc = "Synchronized CMD: Wait for the end of the current data block transfer before sending the pending command."]
    #[inline]
    pub fn sync(self) -> &'a mut W {
        self.variant(SPCMDW::SYNC)
    }
    #[doc = "CE-ATA Completion Signal disable Command. The host cancels the ability for the device to return a command completion signal on the command line."]
    #[inline]
    pub fn ce_ata(self) -> &'a mut W {
        self.variant(SPCMDW::CE_ATA)
    }
    #[doc = "Interrupt command: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline]
    pub fn it_cmd(self) -> &'a mut W {
        self.variant(SPCMDW::IT_CMD)
    }
    #[doc = "Interrupt response: Corresponds to the Interrupt Mode (CMD40)."]
    #[inline]
    pub fn it_resp(self) -> &'a mut W {
        self.variant(SPCMDW::IT_RESP)
    }
    #[doc = "Boot Operation Request. Start a boot operation mode, the host processor can read boot data from the MMC device directly."]
    #[inline]
    pub fn bor(self) -> &'a mut W {
        self.variant(SPCMDW::BOR)
    }
    #[doc = "End Boot Operation. This command allows the host processor to terminate the boot operation mode."]
    #[inline]
    pub fn ebo(self) -> &'a mut W {
        self.variant(SPCMDW::EBO)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `OPDCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OPDCMDW {
    #[doc = "Push pull command."]
    PUSHPULL,
    #[doc = "Open drain command."]
    OPENDRAIN,
}
impl OPDCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            OPDCMDW::PUSHPULL => false,
            OPDCMDW::OPENDRAIN => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _OPDCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _OPDCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: OPDCMDW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Push pull command."]
    #[inline]
    pub fn pushpull(self) -> &'a mut W {
        self.variant(OPDCMDW::PUSHPULL)
    }
    #[doc = "Open drain command."]
    #[inline]
    pub fn opendrain(self) -> &'a mut W {
        self.variant(OPDCMDW::OPENDRAIN)
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
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `MAXLAT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MAXLATW {
    #[doc = "5-cycle max latency."]
    _5,
    #[doc = "64-cycle max latency."]
    _64,
}
impl MAXLATW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            MAXLATW::_5 => false,
            MAXLATW::_64 => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _MAXLATW<'a> {
    w: &'a mut W,
}
impl<'a> _MAXLATW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: MAXLATW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "5-cycle max latency."]
    #[inline]
    pub fn _5(self) -> &'a mut W {
        self.variant(MAXLATW::_5)
    }
    #[doc = "64-cycle max latency."]
    #[inline]
    pub fn _64(self) -> &'a mut W {
        self.variant(MAXLATW::_64)
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
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRCMDW {
    #[doc = "No data transfer"]
    NO_DATA,
    #[doc = "Start data transfer"]
    START_DATA,
    #[doc = "Stop data transfer"]
    STOP_DATA,
}
impl TRCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRCMDW::NO_DATA => 0,
            TRCMDW::START_DATA => 1,
            TRCMDW::STOP_DATA => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _TRCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No data transfer"]
    #[inline]
    pub fn no_data(self) -> &'a mut W {
        self.variant(TRCMDW::NO_DATA)
    }
    #[doc = "Start data transfer"]
    #[inline]
    pub fn start_data(self) -> &'a mut W {
        self.variant(TRCMDW::START_DATA)
    }
    #[doc = "Stop data transfer"]
    #[inline]
    pub fn stop_data(self) -> &'a mut W {
        self.variant(TRCMDW::STOP_DATA)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRDIR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRDIRW {
    #[doc = "Write."]
    WRITE,
    #[doc = "Read."]
    READ,
}
impl TRDIRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRDIRW::WRITE => false,
            TRDIRW::READ => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRDIRW<'a> {
    w: &'a mut W,
}
impl<'a> _TRDIRW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRDIRW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Write."]
    #[inline]
    pub fn write(self) -> &'a mut W {
        self.variant(TRDIRW::WRITE)
    }
    #[doc = "Read."]
    #[inline]
    pub fn read(self) -> &'a mut W {
        self.variant(TRDIRW::READ)
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
        const OFFSET: u8 = 18;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRTYP`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRTYPW {
    #[doc = "MMC/SD Card Single Block"]
    SINGLE,
    #[doc = "MMC/SD Card Multiple Block"]
    MULTIPLE,
    #[doc = "MMC Stream"]
    STREAM,
    #[doc = "SDIO Byte"]
    BYTE,
    #[doc = "SDIO Block"]
    BLOCK,
}
impl TRTYPW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            TRTYPW::SINGLE => 0,
            TRTYPW::MULTIPLE => 1,
            TRTYPW::STREAM => 2,
            TRTYPW::BYTE => 4,
            TRTYPW::BLOCK => 5,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRTYPW<'a> {
    w: &'a mut W,
}
impl<'a> _TRTYPW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRTYPW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "MMC/SD Card Single Block"]
    #[inline]
    pub fn single(self) -> &'a mut W {
        self.variant(TRTYPW::SINGLE)
    }
    #[doc = "MMC/SD Card Multiple Block"]
    #[inline]
    pub fn multiple(self) -> &'a mut W {
        self.variant(TRTYPW::MULTIPLE)
    }
    #[doc = "MMC Stream"]
    #[inline]
    pub fn stream(self) -> &'a mut W {
        self.variant(TRTYPW::STREAM)
    }
    #[doc = "SDIO Byte"]
    #[inline]
    pub fn byte(self) -> &'a mut W {
        self.variant(TRTYPW::BYTE)
    }
    #[doc = "SDIO Block"]
    #[inline]
    pub fn block(self) -> &'a mut W {
        self.variant(TRTYPW::BLOCK)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 7;
        const OFFSET: u8 = 19;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `IOSPCMD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IOSPCMDW {
    #[doc = "Not an SDIO Special Command"]
    STD,
    #[doc = "SDIO Suspend Command"]
    SUSPEND,
    #[doc = "SDIO Resume Command"]
    RESUME,
}
impl IOSPCMDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            IOSPCMDW::STD => 0,
            IOSPCMDW::SUSPEND => 1,
            IOSPCMDW::RESUME => 2,
        }
    }
}
#[doc = r" Proxy"]
pub struct _IOSPCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _IOSPCMDW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: IOSPCMDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not an SDIO Special Command"]
    #[inline]
    pub fn std(self) -> &'a mut W {
        self.variant(IOSPCMDW::STD)
    }
    #[doc = "SDIO Suspend Command"]
    #[inline]
    pub fn suspend(self) -> &'a mut W {
        self.variant(IOSPCMDW::SUSPEND)
    }
    #[doc = "SDIO Resume Command"]
    #[inline]
    pub fn resume(self) -> &'a mut W {
        self.variant(IOSPCMDW::RESUME)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 3;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `ATACS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ATACSW {
    #[doc = "Normal operation mode."]
    NORMAL,
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    COMPLETION,
}
impl ATACSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            ATACSW::NORMAL => false,
            ATACSW::COMPLETION => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _ATACSW<'a> {
    w: &'a mut W,
}
impl<'a> _ATACSW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: ATACSW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Normal operation mode."]
    #[inline]
    pub fn normal(self) -> &'a mut W {
        self.variant(ATACSW::NORMAL)
    }
    #[doc = "This bit indicates that a completion signal is expected within a programmed amount of time (HSMCI_CSTOR)."]
    #[inline]
    pub fn completion(self) -> &'a mut W {
        self.variant(ATACSW::COMPLETION)
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
        const OFFSET: u8 = 26;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _BOOT_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _BOOT_ACKW<'a> {
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
        const OFFSET: u8 = 27;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
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
    #[doc = "Bits 0:5 - Command Number"]
    #[inline]
    pub fn cmdnb(&mut self) -> _CMDNBW {
        _CMDNBW { w: self }
    }
    #[doc = "Bits 6:7 - Response Type"]
    #[inline]
    pub fn rsptyp(&mut self) -> _RSPTYPW {
        _RSPTYPW { w: self }
    }
    #[doc = "Bits 8:10 - Special Command"]
    #[inline]
    pub fn spcmd(&mut self) -> _SPCMDW {
        _SPCMDW { w: self }
    }
    #[doc = "Bit 11 - Open Drain Command"]
    #[inline]
    pub fn opdcmd(&mut self) -> _OPDCMDW {
        _OPDCMDW { w: self }
    }
    #[doc = "Bit 12 - Max Latency for Command to Response"]
    #[inline]
    pub fn maxlat(&mut self) -> _MAXLATW {
        _MAXLATW { w: self }
    }
    #[doc = "Bits 16:17 - Transfer Command"]
    #[inline]
    pub fn trcmd(&mut self) -> _TRCMDW {
        _TRCMDW { w: self }
    }
    #[doc = "Bit 18 - Transfer Direction"]
    #[inline]
    pub fn trdir(&mut self) -> _TRDIRW {
        _TRDIRW { w: self }
    }
    #[doc = "Bits 19:21 - Transfer Type"]
    #[inline]
    pub fn trtyp(&mut self) -> _TRTYPW {
        _TRTYPW { w: self }
    }
    #[doc = "Bits 24:25 - SDIO Special Command"]
    #[inline]
    pub fn iospcmd(&mut self) -> _IOSPCMDW {
        _IOSPCMDW { w: self }
    }
    #[doc = "Bit 26 - ATA with Command Completion Signal"]
    #[inline]
    pub fn atacs(&mut self) -> _ATACSW {
        _ATACSW { w: self }
    }
    #[doc = "Bit 27 - Boot Operation Acknowledge"]
    #[inline]
    pub fn boot_ack(&mut self) -> _BOOT_ACKW {
        _BOOT_ACKW { w: self }
    }
}
