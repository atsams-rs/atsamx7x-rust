#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MATRIX_MCFG {
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
#[doc = "Possible values of the field `ULBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULBTR {
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    UNLTD_LENGTH,
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    SINGLE_ACCESS,
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    _4BEAT_BURST,
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    _8BEAT_BURST,
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    _16BEAT_BURST,
    #[doc = "32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    _32BEAT_BURST,
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    _64BEAT_BURST,
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    _128BEAT_BURST,
}
impl crate::ToBits<u8> for ULBTR {
    #[inline(always)]
    fn _bits(&self) -> u8 {
        match *self {
            ULBTR::UNLTD_LENGTH => 0,
            ULBTR::SINGLE_ACCESS => 1,
            ULBTR::_4BEAT_BURST => 2,
            ULBTR::_8BEAT_BURST => 3,
            ULBTR::_16BEAT_BURST => 4,
            ULBTR::_32BEAT_BURST => 5,
            ULBTR::_64BEAT_BURST => 6,
            ULBTR::_128BEAT_BURST => 7,
        }
    }
}
#[doc = r"Reader of the field"]
pub type ULBT_R = crate::FR<u8, ULBTR>;
impl ULBT_R {
    #[doc = "Checks if the value of the field is `UNLTD_LENGTH`"]
    #[inline(always)]
    pub fn is_unltd_length(&self) -> bool {
        *self == ULBTR::UNLTD_LENGTH
    }
    #[doc = "Checks if the value of the field is `SINGLE_ACCESS`"]
    #[inline(always)]
    pub fn is_single_access(&self) -> bool {
        *self == ULBTR::SINGLE_ACCESS
    }
    #[doc = "Checks if the value of the field is `_4BEAT_BURST`"]
    #[inline(always)]
    pub fn is_4beat_burst(&self) -> bool {
        *self == ULBTR::_4BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_8BEAT_BURST`"]
    #[inline(always)]
    pub fn is_8beat_burst(&self) -> bool {
        *self == ULBTR::_8BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_16BEAT_BURST`"]
    #[inline(always)]
    pub fn is_16beat_burst(&self) -> bool {
        *self == ULBTR::_16BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_32BEAT_BURST`"]
    #[inline(always)]
    pub fn is_32beat_burst(&self) -> bool {
        *self == ULBTR::_32BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_64BEAT_BURST`"]
    #[inline(always)]
    pub fn is_64beat_burst(&self) -> bool {
        *self == ULBTR::_64BEAT_BURST
    }
    #[doc = "Checks if the value of the field is `_128BEAT_BURST`"]
    #[inline(always)]
    pub fn is_128beat_burst(&self) -> bool {
        *self == ULBTR::_128BEAT_BURST
    }
}
#[doc = "Values that can be written to the field `ULBT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ULBTW {
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    UNLTD_LENGTH,
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    SINGLE_ACCESS,
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    _4BEAT_BURST,
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    _8BEAT_BURST,
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    _16BEAT_BURST,
    #[doc = "32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    _32BEAT_BURST,
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    _64BEAT_BURST,
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    _128BEAT_BURST,
}
impl ULBTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ULBTW::UNLTD_LENGTH => 0,
            ULBTW::SINGLE_ACCESS => 1,
            ULBTW::_4BEAT_BURST => 2,
            ULBTW::_8BEAT_BURST => 3,
            ULBTW::_16BEAT_BURST => 4,
            ULBTW::_32BEAT_BURST => 5,
            ULBTW::_64BEAT_BURST => 6,
            ULBTW::_128BEAT_BURST => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ULBTW<'a> {
    w: &'a mut W,
}
impl<'a> _ULBTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ULBTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Unlimited Length Burst-No predicted end of burst is generated, therefore INCR bursts coming from this master can only be broken if the Slave Slot Cycle Limit is reached. If the Slot Cycle Limit is not reached, the burst is normally completed by the master, at the latest, on the next AHB 1-Kbyte address boundary, allowing up to 256-beat word bursts or 128-beat double-word bursts.This value should not be used in the very particular case of a master capable of performing back-to-back undefined length bursts on a single slave, since this could indefinitely freeze the slave arbitration and thus prevent another master from accessing this slave."]
    #[inline(always)]
    pub fn unltd_length(self) -> &'a mut W {
        self.variant(ULBTW::UNLTD_LENGTH)
    }
    #[doc = "Single Access-The undefined length burst is treated as a succession of single accesses, allowing re-arbitration at each beat of the INCR burst or bursts sequence."]
    #[inline(always)]
    pub fn single_access(self) -> &'a mut W {
        self.variant(ULBTW::SINGLE_ACCESS)
    }
    #[doc = "4-beat Burst-The undefined length burst or bursts sequence is split into 4-beat bursts or less, allowing re-arbitration every 4 beats."]
    #[inline(always)]
    pub fn _4beat_burst(self) -> &'a mut W {
        self.variant(ULBTW::_4BEAT_BURST)
    }
    #[doc = "8-beat Burst-The undefined length burst or bursts sequence is split into 8-beat bursts or less, allowing re-arbitration every 8 beats."]
    #[inline(always)]
    pub fn _8beat_burst(self) -> &'a mut W {
        self.variant(ULBTW::_8BEAT_BURST)
    }
    #[doc = "16-beat Burst-The undefined length burst or bursts sequence is split into 16-beat bursts or less, allowing re-arbitration every 16 beats."]
    #[inline(always)]
    pub fn _16beat_burst(self) -> &'a mut W {
        self.variant(ULBTW::_16BEAT_BURST)
    }
    #[doc = "32-beat Burst -The undefined length burst or bursts sequence is split into 32-beat bursts or less, allowing re-arbitration every 32 beats."]
    #[inline(always)]
    pub fn _32beat_burst(self) -> &'a mut W {
        self.variant(ULBTW::_32BEAT_BURST)
    }
    #[doc = "64-beat Burst-The undefined length burst or bursts sequence is split into 64-beat bursts or less, allowing re-arbitration every 64 beats."]
    #[inline(always)]
    pub fn _64beat_burst(self) -> &'a mut W {
        self.variant(ULBTW::_64BEAT_BURST)
    }
    #[doc = "128-beat Burst-The undefined length burst or bursts sequence is split into 128-beat bursts or less, allowing re-arbitration every 128 beats."]
    #[inline(always)]
    pub fn _128beat_burst(self) -> &'a mut W {
        self.variant(ULBTW::_128BEAT_BURST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&self) -> ULBT_R {
        ULBT_R::new((self.bits() & 0x07) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Undefined Length Burst Type"]
    #[inline(always)]
    pub fn ulbt(&mut self) -> _ULBTW {
        _ULBTW { w: self }
    }
}
