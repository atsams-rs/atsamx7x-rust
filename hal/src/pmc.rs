//! Clock hierarchy configuration
//!
//! This module allows the user to fully control the various clock sources available on ATSAMx7x
//! chips.
//!

use crate::target_device::PMC;
use crate::target_device::SUPC;
use fugit::Rate;

pub use crate::target_device::pmc::ckgr_mor::MOSCRCF_A as MainRcFreq;
pub use crate::target_device::pmc::pmc_mckr::CSS_A as HCC_CSS;
pub use crate::target_device::pmc::pmc_mckr::MDIV_A as MckDivider;
pub use crate::target_device::pmc::pmc_mckr::PRES_A as MckPrescaler;
pub use crate::target_device::pmc::pmc_pck::CSS_A as PCK_CSS;

pub struct Pmc {
    periph: PMC,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PmcError {
    ClockingError(PeripheralIdentifier),
    InvalidConfiguration,
    UnimplementedError,
    InternalError,
}

/// The selected "Main Clock Oscillator" source
#[derive(Debug, PartialEq, Clone)]
pub enum MainClockOscillatorSource {
    MainRcOsc(MainRcFreq),
    MainCrystalOsc(Rate<u32, 1, 1>),
    MainExternalOsc(Rate<u32, 1, 1>),
}

/// MAINCK Token
pub struct MainClock {
    source: MainClockOscillatorSource,
}

// Slow Clock Oscillator Source is set in SUPC
pub enum SlowClockOscillatorSource {
    SlowRcOsc,
    SlowCrystalOsc,
    SlowExternalOsc,
}

/// SCLK Token
pub struct SlowClock {
    source: SlowClockOscillatorSource,
}

pub struct PllaConfig {
    pub div: u8,
    pub mult: u8,
}

/// PLLA Token
pub struct PllaClock {}

pub struct UpllClock;

/// HCLK/MCK Config
pub struct HostClockConfig {
    pub pres: MckPrescaler,
    pub div: MckDivider,
}

/// MCK Token
pub struct HostClock {}

/// HCLK Token
pub struct ProcessorClock {}

/// The selected "Master Clock" source
///
/// TODO/NOTE: At the moment, we only support the PLLA Clock.
/// Some driver behavior is hardcoded on this assumption for
/// the sake of simplicity.
///
/// This corresponds to PMC_MCKR.CSS
pub enum MasterClockSource {
    PllaClock,
}

/// PCK token
pub struct Pck {
    id: PckId,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PckId {
    Pck0,
    Pck1,
    Pck2,
    Pck3,
    Pck4,
    Pck5,
    Pck6,
    Pck7,
}

pub trait PllaSource {}

impl PllaSource for MainClock {}

pub trait HostClockSource {
    const HCC_CSS: HCC_CSS;
}

impl HostClockSource for SlowClock {
    const HCC_CSS: HCC_CSS = HCC_CSS::SLOW_CLK;
}
impl HostClockSource for MainClock {
    const HCC_CSS: HCC_CSS = HCC_CSS::MAIN_CLK;
}
impl HostClockSource for PllaClock {
    const HCC_CSS: HCC_CSS = HCC_CSS::PLLA_CLK;
}
impl HostClockSource for UpllClock {
    const HCC_CSS: HCC_CSS = HCC_CSS::UPLL_CLK;
}

pub trait PckSource {
    const PCK_CSS: PCK_CSS;
}

impl PckSource for SlowClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::SLOW_CLK;
}
impl PckSource for MainClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::MAIN_CLK;
}
impl PckSource for UpllClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::UPLL_CLK;
}
impl PckSource for PllaClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::PLLA_CLK;
}
impl PckSource for HostClock {
    const PCK_CSS: PCK_CSS = PCK_CSS::MCK;
}

impl Pmc {
    pub fn new(periph: PMC) -> Self {
        periph.pmc_wpmr.modify(|_r, w| {
            w.wpkey().passwd();
            w.wpen().clear_bit();
            w
        });

        Self {
            periph,
            // TODO: I could probably figure out the default settings...
            // this is fine for now.
            // TODO: If the get_mainck() etc. API is to be used pmc needs to have ownership over
            // the clock structs at startup to prevent multiple instances of MainClock to be
            // constructed. Watchucallit, Singletons.
            // settings: None,
        }
    }

    /// Configures SLCK and returns a corresponding Clock Token.
    ///
    /// Note: Clearing xtalset or oscbypass has no effect.
    /// Setting oscbypass after setting xtalset has no effect.
    /// Changes to The SLCK source cannot be unmade in software.
    pub fn get_slck(
        &mut self,
        supc: &mut SUPC,
        source: SlowClockOscillatorSource,
    ) -> Result<SlowClock, PmcError> {
        match source {
            SlowClockOscillatorSource::SlowRcOsc => (),
            SlowClockOscillatorSource::SlowCrystalOsc => {
                supc.supc_cr.write(|w| {
                    w.xtalsel().set_bit();
                    w.key().passwd()
                });
            }
            SlowClockOscillatorSource::SlowExternalOsc => {
                supc.supc_mr.modify(|_, w| {
                    w.oscbypass().set_bit();
                    w.key().passwd()
                });
                supc.supc_cr.write(|w| {
                    w.xtalsel().set_bit();
                    w.key().passwd()
                });
            }
        }
        Ok(SlowClock { source })
    }

    /// Configures MAINCK and returns a corresponding Clock Token.
    /// This Method corresponds to Steps 2-4 in 31.17 Recommended Programming Sequence.
    pub fn get_mainck(&mut self, source: MainClockOscillatorSource) -> Result<MainClock, PmcError> {
        match source {
            MainClockOscillatorSource::MainRcOsc(ref freq) => {
                let freq_bits = *freq as u8;
                self.periph.ckgr_mor.modify(|_, w| {
                    w.key().passwd();
                    w.moscsel().clear_bit();
                    w.moscrcen().set_bit();
                    unsafe {
                        w.moscrcf().bits(freq_bits);
                    }
                    w
                });
            }
            MainClockOscillatorSource::MainCrystalOsc(ref freq) => {
                // Crystal Frequency needs to be between 3 and 20MHz (30.2)
                if freq.to_MHz() < 3 || freq.to_MHz() > 20 {
                    return Err(PmcError::InvalidConfiguration);
                }
                self.periph.ckgr_mor.modify(|_, w| {
                    w.key().passwd();
                    w.moscxten().set_bit();
                    unsafe {
                        w.moscxtst().bits(255);
                    }
                    w
                });
                // loop until main crystal oscillator has stabilised
                while self.periph.pmc_sr.read().moscxts().bit_is_clear() {}
                self.periph.ckgr_mor.modify(|_, w| {
                    w.key().passwd();
                    w.moscsel().set_bit();
                    unsafe {
                        w.moscxtst().bits(255);
                    }
                    w
                });
                // loop until source switch has completed
                while self.periph.pmc_sr.read().moscsels().bit_is_clear() {}
            }
            MainClockOscillatorSource::MainExternalOsc(ref freq) => {
                // Oscillator Frequency needs to be between 3 and 20MHz (30.2)
                if freq.to_MHz() < 3 || freq.to_MHz() > 20 {
                    return Err(PmcError::InvalidConfiguration);
                }
                self.periph.ckgr_mor.modify(|_, w| {
                    w.key().passwd();
                    w.moscxtby().set_bit();
                    w
                });
                // loop until source switch has completed
                while self.periph.pmc_sr.read().moscsels().bit_is_clear() {}
                self.periph.ckgr_mor.modify(|_, w| {
                    w.key().passwd();
                    w.moscsel().set_bit();
                    w
                });
                while self.periph.pmc_sr.read().moscsels().bit_is_clear() {}
                self.periph.ckgr_mor.modify(|_, w| {
                    w.key().passwd();
                    w.moscrcen().clear_bit();
                    w
                });
            }
        }
        Ok(MainClock { source })
    }

    /// Configures PLLACK and returns a corresponding clock token.
    /// This method corresponds to Step 6 of 31.17 Recommended Programming Sequence.
    pub fn get_pllack<SRC: PllaSource>(
        &mut self,
        config: PllaConfig,
        source: &SRC,
    ) -> Result<PllaClock, PmcError> {
        if config.mult > 63 || config.mult < 2 {
            return Err(PmcError::InvalidConfiguration);
        }
        if config.div == 0 || config.div > 127 {
            return Err(PmcError::InvalidConfiguration);
        }
        // NOTE: Maximum frequency is not checked her

        self.periph.ckgr_pllar.modify(|_, w| {
            w.one().set_bit();
            unsafe {
                w.mula().bits(config.mult as u16 - 1);
                w.diva().bits(config.div);
            }
            w
        });
        // loop until PLLA Lock Status
        while self.periph.pmc_sr.read().locka().bit_is_clear() {}
        Ok(PllaClock {})
    }

    /// Configures UPLLCK
    /// TODO: There's the UPLLDIV2 that is not touched right now. Should be toggleable.
    pub fn get_upllck(&mut self) -> Result<UpllClock, PmcError> {
        self.periph.ckgr_uckr.modify(|_, w| w.upllen().set_bit());
        // loop until UPLL Lock Status
        while self.periph.pmc_sr.read().locku().bit_is_clear() {}

        Ok(UpllClock)
    }

    /// Configures HCLK and MCK and returns corresponding Clock Tokens.
    /// This method corresponds to Step 7 in 31.17.
    pub fn get_hclk<SRC: HostClockSource>(
        &mut self,
        config: &HostClockConfig,
        source: &SRC,
    ) -> Result<(ProcessorClock, HostClock), PmcError> {
        let pres_bits = config.pres as u8;
        let div_bits = config.div as u8;

        self.periph
            .pmc_mckr
            .modify(|_, w| w.css().bits(SRC::HCC_CSS as u8));
        while self.periph.pmc_sr.read().mckrdy().bit_is_clear() {}
        self.periph.pmc_mckr.modify(|_, w| w.pres().bits(pres_bits));
        while self.periph.pmc_sr.read().mckrdy().bit_is_clear() {}
        self.periph.pmc_mckr.modify(|_, w| w.mdiv().bits(div_bits));
        while self.periph.pmc_sr.read().mckrdy().bit_is_clear() {}

        Ok((ProcessorClock {}, HostClock {}))
    }

    /// Configures PCKx and returns a token.
    /// Corresponds to Step 8 in 31.17
    pub fn get_pck<SRC: PckSource>(
        &mut self,
        source: &SRC,
        pres: u8,
        id: PckId,
    ) -> Result<Pck, PmcError> {
        self.periph.pmc_pck[id as usize].write(|w| unsafe {
            w.pres().bits(pres);
            w.css().bits(SRC::PCK_CSS as u8)
        });
        self.periph
            .pmc_scer
            .write(|w| unsafe { w.bits(1 << (id as u8 + 8)) });
        while (self.periph.pmc_scsr.read().bits() & (1 << (id as u8 + 8))) == 0 {}
        Ok(Pck { id })
    }

    pub fn enable_peripherals(&mut self, pids: &[PeripheralIdentifier]) -> Result<(), PmcError> {
        if pids.is_empty() {
            return Ok(());
        }

        let pcsr0 = self.periph.pmc_pcsr0.read().bits();
        let pcsr1 = self.periph.pmc_pcsr1.read().bits();

        let mut pcr0 = 0;
        let mut pcr1 = 0;

        for pid in pids {
            // Check if this supports PMC clocking
            pid.supports_pmc_clocking()
                .map_err(|_| PmcError::ClockingError(*pid))?;

            let pid_val: u32 = (*pid) as u32;

            match pid_val {
                7..=31 => {
                    let mask = 1 << pid_val;
                    let pre_set = (pcsr0 & mask) != 0;
                    let dup_set = (pcr0 & mask) != 0;

                    if pre_set || dup_set {
                        // defmt::warn!("[PMC] Duplicate Clock Enable: {}", pid);
                    }

                    pcr0 |= mask;
                }
                32..=63 => {
                    let mask = 1 << (pid_val - 32);

                    let pre_set = (pcsr1 & mask) != 0;
                    let dup_set = (pcr1 & mask) != 0;

                    if pre_set || dup_set {
                        // defmt::warn!("[PMC] Duplicate Clock Enable: {}", pid);
                    }

                    pcr1 |= mask;
                }
                69 | 70 => {
                    // I don't know how to enable these peripherals yet
                    return Err(PmcError::UnimplementedError);
                }
                _ => {
                    // This should be impossible, and probably means there is an
                    // error in the `supports_pmc_clocking()` function
                    return Err(PmcError::InternalError);
                }
            }
        }

        // Enable the newly set peripherals
        self.periph.pmc_pcer0.write(|w| unsafe { w.bits(pcr0) });
        self.periph.pmc_pcer1.write(|w| unsafe { w.bits(pcr1) });

        Ok(())
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(u32)]
pub enum PeripheralIdentifier {
    /// 0   (NVIC + !PMC CC) Supply Controller
    SUPC = 0,
    /// 1   (NVIC + !PMC CC) Reset Controller
    RSTC = 1,
    /// 2   (NVIC + !PMC CC) Real Time Clock
    RTC = 2,
    /// 3   (NVIC + !PMC CC) Real Time Timer
    RTT = 3,
    /// 4   (NVIC + !PMC CC) Watchdog Timer
    WDT = 4,
    /// 5   (NVIC + !PMC CC) Power Management Controller
    PMC = 5,
    /// 6   (NVIC + !PMC CC) Enhanced Embedded Flash Controller
    EFC = 6,
    /// 7   (NVIC + PMC CC) Universal Asynchronous Receiver/Transmitter
    UART0 = 7,
    /// 8   (NVIC + PMC CC) Universal Asynchronous Receiver/Transmitter
    UART1 = 8,
    /// 9   (!NVIC + PMC CC) Static Memory Controller
    SMC = 9,
    /// 10  (NVIC + PMC CC) Parallel I/O Controller A
    PIOA = 10,
    /// 11  (NVIC + PMC CC) Parallel I/O Controller B
    PIOB = 11,
    /// 12  (NVIC + PMC CC) Parallel I/O Controller C
    PIOC = 12,
    /// 13  (NVIC + PMC CC) Universal Synchronous/Asynchronous Receiver/Transmitter
    USART0 = 13,
    /// 14  (NVIC + PMC CC) Universal Synchronous/Asynchronous Receiver/Transmitter
    USART1 = 14,
    /// 15  (NVIC + PMC CC) Universal Synchronous/Asynchronous Receiver/Transmitter
    USART2 = 15,
    /// 16  (NVIC + PMC CC) Parallel I/O Controller D
    PIOD = 16,
    /// 17  (NVIC + PMC CC) Parallel I/O Controller E
    PIOE = 17,
    /// 18  (NVIC + PMC CC) Multimedia Card Interface
    HSMCI = 18,
    /// 19  (NVIC + PMC CC) Two-wire Interface (I2C-compatible)
    TWIHS0 = 19,
    /// 20  (NVIC + PMC CC) Two-wire Interface (I2C-compatible)
    TWIHS1 = 20,
    /// 21  (NVIC + PMC CC) Serial Peripheral Interface
    SPI0 = 21,
    /// 22  (NVIC + PMC CC) Synchronous Serial Controller
    SSC = 22,
    /// 23  (NVIC + PMC CC) 16-bit Timer Counter 0, Channel 0
    TC0_CHANNEL0 = 23,
    /// 24  (NVIC + PMC CC) 16-bit Timer Counter 0, Channel 1
    TC0_CHANNEL1 = 24,
    /// 25  (NVIC + PMC CC) 16-bit Timer Counter 0, Channel 2
    TC0_CHANNEL2 = 25,
    /// 26  (NVIC + PMC CC) 16-bit Timer Counter 1, Channel 0
    TC1_CHANNEL0 = 26,
    /// 27  (NVIC + PMC CC) 16-bit Timer Counter 1, Channel 1
    TC1_CHANNEL1 = 27,
    /// 28  (NVIC + PMC CC) 16-bit Timer Counter 1, Channel 2
    TC1_CHANNEL2 = 28,
    /// 29  (NVIC + PMC CC) Analog Front-End Controller
    AFEC0 = 29,
    /// 30  (NVIC + PMC CC) Digital-to-Analog Converter
    DACC = 30,
    /// 31  (NVIC + PMC CC) Pulse Width Modulation Controller
    PWM0 = 31,
    /// 32  (NVIC + PMC CC) Integrity Check Monitor
    ICM = 32,
    /// 33  (NVIC + PMC CC) Analog Comparator Controller
    ACC = 33,
    /// 34  (NVIC + PMC CC) USB Host / Device Controller
    USBHS = 34,
    /// 35  (NVIC + PMC CC) CAN IRQ Line 0
    MCAN0 = 35,
    /// 36  (NVIC + !PMC CC) CAN IRQ Line 1
    MCAN0INT1 = 36,
    /// 37  (NVIC + PMC CC) CAN IRQ Line 0
    MCAN1 = 37,
    /// 38  (NVIC + !PMC CC) CAN IRQ Line 1
    MCAN1INT1 = 38,
    /// 39  (NVIC + PMC CC) Ethernet MAC
    GMAC = 39,
    /// 40  (NVIC + PMC CC) Analog Front End Controller
    AFEC1 = 40,
    /// 41  (NVIC + PMC CC) Two-wire Interface
    TWIHS2 = 41,
    /// 42  (NVIC + PMC CC) Serial Peripheral Interface
    SPI1 = 42,
    /// 43  (NVIC + PMC CC) Quad I/O Serial Peripheral Interface
    QSPI = 43,
    /// 44  (NVIC + PMC CC) Universal Asynchronous Receiver/ Transmitter
    UART2 = 44,
    /// 45  (NVIC + PMC CC) Universal Asynchronous Receiver/ Transmitter
    UART3 = 45,
    /// 46  (NVIC + PMC CC) Universal Asynchronous Receiver/ Transmitter
    UART4 = 46,
    /// 47  (NVIC + PMC CC) 16-bit Timer Counter 2, Channel 0
    TC2_CHANNEL0 = 47,
    /// 48  (NVIC + PMC CC) 16-bit Timer Counter 2, Channel 1
    TC2_CHANNEL1 = 48,
    /// 49  (NVIC + PMC CC) 16-bit Timer Counter 2, Channel 2
    TC2_CHANNEL2 = 49,
    /// 50  (NVIC + PMC CC) 16-bit Timer Counter 3, Channel 0
    TC3_CHANNEL0 = 50,
    /// 51  (NVIC + PMC CC) 16-bit Timer Counter 3, Channel 1
    TC3_CHANNEL1 = 51,
    /// 52  (NVIC + PMC CC) 16-bit Timer Counter 3, Channel 2
    TC3_CHANNEL2 = 52,
    /// 53  (NVIC + PMC CC) MediaLB IRQ 0
    MLB_IRQ0 = 53,
    /// 54  (NVIC + !PMC CC) MediaLB IRQ 1
    MLB_IRQ1 = 54,
    /// 55  (NVIC + !PMC CC) Reserved
    _RESERVED = 55,
    /// 56  (NVIC + PMC CC) Advanced Encryption Standard
    AES = 56,
    /// 57  (NVIC + PMC CC) True Random Number Generator
    TRNG = 57,
    /// 58  (NVIC + PMC CC) DMA Controller
    XDMAC = 58,
    /// 59  (NVIC + PMC CC) Image Sensor Interface
    ISI = 59,
    /// 60  (NVIC + PMC CC) Pulse Width Modulation Controller
    PWM1 = 60,
    /// 61 (NVIC:FPU + !PMC CC) ARM Floating Point Unit interrupt associated with OFC, UFC, IOC, DZC and IDC bits
    ARM = 61,
    /// 62  (NVIC + !PMC CC) SDRAM Controller
    SDRAMC = 62,
    /// 63  (NVIC + !PMC CC) Reinforced Safety Watchdog Timer
    RSWDT = 63,
    /// 64  (NVIC:CCW + !PMC CC) ARM Cache ECC Warning
    ARM_CACHE_ECC_WARNING = 64,
    /// 65  (NVIC:CCF + !PMC CC) ARM Cache ECC Fault
    ARM_CACHE_ECC_FAULT = 65,
    /// 66  (NVIC:Q1 + !PMC CC) GMAC Queue 1 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 1
    GMAC_Q1 = 66,
    /// 67  (NVIC:Q2 + !PMC CC) GMAC Queue 2 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 2
    GMAC_Q2 = 67,
    /// 68  (NVIC:IX + !PMC CC) –Floating Point Unit Interrupt IXC associated with FPU cumulative exception bit
    ARM_FPU_IXC_FPU = 68,
    /// 69  (NVIC + PMC CC) Inter-IC Sound Controller
    // TODO: How does this get enabled with PMC? The PMC bits only go up to 63...
    I2SC0 = 69,
    /// 70  (NVIC + PMC CC) Inter-IC Sound Controller
    // TODO: How does this get enabled with PMC? The PMC bits only go up to 63...
    I2SC1 = 70,
    /// 71  (NVIC:Q3 + !PMC CC) GMAC Queue 3 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 3
    GMAC_Q3 = 71,
    /// 72  (NVIC:Q4 + !PMC CC) GMAC Queue 4 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 4
    GMAC_Q4 = 72,
    /// 73  (NVIC:Q5 + !PMC CC) GMAC Queue 5 Interrupt signal toggled on a DMA write to the first word of each DMA data buffer associated with queue 5
    GMAC_Q5 = 73,
}

impl PeripheralIdentifier {
    pub fn supports_pmc_clocking(&self) -> Result<(), ()> {
        use PeripheralIdentifier::*;
        // These pids don't support PMC clocking
        match self {
            SUPC => Err(()),
            RSTC => Err(()),
            RTC => Err(()),
            RTT => Err(()),
            WDT => Err(()),
            PMC => Err(()),
            EFC => Err(()),
            MCAN0INT1 => Err(()),
            MCAN1INT1 => Err(()),
            MLB_IRQ1 => Err(()),
            _RESERVED => Err(()),
            ARM => Err(()),
            SDRAMC => Err(()),
            RSWDT => Err(()),
            ARM_CACHE_ECC_WARNING => Err(()),
            ARM_CACHE_ECC_FAULT => Err(()),
            GMAC_Q1 => Err(()),
            GMAC_Q2 => Err(()),
            ARM_FPU_IXC_FPU => Err(()),
            GMAC_Q3 => Err(()),
            GMAC_Q4 => Err(()),
            GMAC_Q5 => Err(()),
            _ => Ok(()),
        }
    }
}
