//! Clock hierarchy configuration

use crate::efc::Efc;
use crate::efc::FlashWaitStates;
use crate::target_device::PMC;

pub use crate::target_device::pmc::pmc_mckr::MDIV_A as MckDivider;
pub use crate::target_device::pmc::pmc_mckr::PRES_A as MckPrescaler;

pub struct Pmc {
    periph: PMC,
    settings: Option<ClockSettings>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PmcError {
    ClockingError(PeripheralIdentifier),
    InvalidConfiguration,
    UnimplementedError,
    InternalError,
}

/// The selected "Main Clock Oscillator" source
///
/// TODO/NOTE: At the moment, we only support the internal trimmed 12MHz
/// oscillator. Some driver behavior is hardcoded on this assumption for
/// the sake of simplicity.
///
/// This corresponds to CKGR_MOR.MOSCSEL
pub enum MainClockOscillatorSource {
    MainCk12MHz,
}

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

pub struct ClockSettings {
    // "Main Clock" is abbreviated "MAINCK"
    /// Main Clock Oscillator Source
    pub main_clk_osc_src: MainClockOscillatorSource,

    // "Master Clock" is abbreviated "MCK"
    /// Master Clock Prescaler
    pub mck_pres: MckPrescaler,
    /// Master Clock Source
    pub mck_src: MasterClockSource,
    /// Master Clock Divider
    pub mck_div: MckDivider,

    /// PLLA Multiplier - 10 bits
    ///
    /// Resulting change is (1 + MULA) * INPUT
    ///
    /// A value of zero disables the PLLA.
    pub multiplier_a: u16,
    /// PLLA Divider - 8 bits
    ///
    /// Result change is INPUT / DIVA
    ///
    /// A value of zero disables the PLLA.
    pub divider_a: u8,
}

impl ClockSettings {
    pub fn calc_master_clk_mhz(&self) -> Result<u8, PmcError> {
        // NOTE: This is based on Figure 31-1 - "General Clock Distribution Block Diagram"

        // PLLACK is currently the (only) choice for driving the Master Clock Controller
        let mck_input = match self.mck_src {
            MasterClockSource::PllaClock => {
                // The internal 12MHz osc is currently the (only) choice for driving the PLLACK
                let main_ck: f32 = match self.main_clk_osc_src {
                    MainClockOscillatorSource::MainCk12MHz => 12.0,
                };

                // These values disable the PLLA
                if (self.multiplier_a == 0) || (self.divider_a == 0) {
                    return Err(PmcError::InvalidConfiguration);
                }

                let plla_ck = main_ck * ((self.multiplier_a + 1) as f32);
                let plla_ck = plla_ck / (self.divider_a as f32);

                plla_ck
            }
        };

        // MCK input first passes through a prescaler...
        let presc: f32 = match self.mck_pres {
            MckPrescaler::CLK_1 => 1.0,
            MckPrescaler::CLK_2 => 2.0,
            MckPrescaler::CLK_3 => 3.0,
            MckPrescaler::CLK_4 => 4.0,
            MckPrescaler::CLK_8 => 8.0,
            MckPrescaler::CLK_16 => 16.0,
            MckPrescaler::CLK_32 => 32.0,
            MckPrescaler::CLK_64 => 64.0,
        };

        let post_pres = mck_input / presc;

        // ... then the output of the prescaler is passed through a divider
        let div: f32 = match self.mck_div {
            MckDivider::EQ_PCK => 1.0,
            MckDivider::PCK_DIV2 => 2.0,
            MckDivider::PCK_DIV3 => 3.0,
            MckDivider::PCK_DIV4 => 4.0,
        };
        let mck = post_pres / div;

        // Make sure we're still in a reasonable range
        //
        // TODO: The datasheet says:
        //
        // > Master Clock (MCK), programmable from a few hundred Hz to the maximum operating frequency of
        // > the device. It is available to the modules running permanently, such as the Enhanced Embedded
        // > Flash Controller
        //
        // However I currently assume that it never exceeds 150MHz. This could probably be changed.
        // This is mostly because the datasheet only lists calculations for flash wait states up to
        // a master clock of 150MHz.
        if mck > 255.0 {
            return Err(PmcError::InvalidConfiguration);
        } else {
            // Note, this is a "floor" operation, while we should probably
            // be using "ceil", though that requires libm (or a similar float library).
            //
            // For now, this is probably close enough.
            let mck_u8 = mck as u8;
            Ok(mck_u8)
        }
    }
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
            settings: None,
        }
    }

    pub fn settings(&self) -> Option<&ClockSettings> {
        self.settings.as_ref()
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

    pub fn set_clocks(&mut self, efc: &mut Efc, cfg: ClockSettings) -> Result<(), PmcError> {
        // Calculate the master clock to determine the number of flash wait states.
        // This must be done BEFORE increasing the clock speed, in case the current number
        // of wait states is insufficient for the new speed.
        //
        // The flash controller (EEFC) is driven from the master clock (stated in section 31.2)
        let mck_new = cfg.calc_master_clk_mhz()?;
        let fws = FlashWaitStates::from_mck_mhz(mck_new)?;

        efc.periph.eefc_wpmr.modify(|_r, w| {
            w.wpkey().passwd();
            w.wpen().clear_bit();
            w
        });
        efc.periph
            .eefc_fmr
            .modify(|_r, w| unsafe { w.fws().bits(fws as u8) });

        // Note: This follows Datasheet 31.17 "Recommendeded Programming Sequence"
        //
        // Steps 1-5 skipped, using the internal osc

        // The Main RC oscillator. Three output frequencies can be selected: 4/8/12 MHz. By default 12 MHz is
        // selected. 8 MHz and 12 MHz are factory-trimmed. The Main RC Oscillator is the default choice, and
        // requires no further configuration.
        //
        // TODO: This is the only supported variant. This code will fail if we add another option.
        // You should implement the logic of steps 1-5 here if you are adding more MCO source
        // options to the public interface!
        let MainClockOscillatorSource::MainCk12MHz = cfg.main_clk_osc_src;

        // # Step 6
        //
        // All parameters needed to configure PLLA and the divider are located in CKGR_PLLAR.
        // CKGR_PLLAR.DIVA is used to control the divider. This parameter can be programmed between 0
        // and 127. Divider output is divider input divided by DIVA parameter. By default, DIVA field is cleared
        // which means that the divider and PLLA are turned off.
        //
        // CKGR_PLLAR.MULA is the PLLA multiplier factor. This parameter can be programmed between 0
        // and 62. If MULA is cleared, PLLA will be turned off, otherwise the PLLA output frequency is PLLA
        // input frequency multiplied by (MULA + 1).
        //
        // CKGR_PLLAR.PLLACOUNT specifies the number of SLCK cycles before PMC_SR.LOCKA is set
        // after CKGR_PLLAR has been written.

        // TODO: Since we have a fixed input clock of 12MHz, mul_a cannot exceed 24 (25x12 = 300MHz)
        if cfg.multiplier_a > 24 {
            return Err(PmcError::InvalidConfiguration);
        }

        if cfg.divider_a == 0 || cfg.divider_a > 127 {
            return Err(PmcError::InvalidConfiguration);
        }

        self.periph.ckgr_pllar.modify(|_r, w| {
            w.one().set_bit();
            unsafe {
                w.mula().bits(cfg.multiplier_a.into());

                // This is the reset value?
                w.pllacount().bits(0b111111);
                w.diva().bits(cfg.divider_a);
            }
            w
        });

        // Once CKGR_PLLAR has been written, the user must wait for PMC_SR.LOCKA to be set. This can
        // be done either by polling PMC_SR.LOCKA or by waiting for the interrupt line to be raised if the
        // associated interrupt source (LOCKA) has been enabled in PMC_IER. All fields in CKGR_PLLAR
        // can be programmed in a single write operation. If MULA or DIVA is modified, the LOCKA bit goes
        // low to indicate that PLLA is not yet ready. When PLLA is locked, LOCKA is set again. The user
        // must wait for the LOCKA bit to be set before using the PLLA output clock.
        while self.periph.pmc_sr.read().locka().bit_is_clear() {}

        // # Step 7
        // Select MCK and HCLK:
        // MCK and HCLK are configurable via PMC_MCKR.
        //
        // CSS is used to select the clock source of MCK and HCLK. By default, the selected clock source is
        // MAINCK.
        //
        // PRES is used to define the HCLK and MCK prescalers The user can choose between different
        // values (1, 2, 3, 4, 8, 16, 32, 64). Prescaler output is the selected clock source frequency divided by
        // the PRES value.
        //
        // MDIV is used to define the MCK divider. It is possible to choose between different values (0, 1, 2,
        // 3). MCK output is the HCLK frequency divided by 1, 2, 3 or 4, depending on the value programmed
        // in MDIV.
        //
        // By default, MDIV is cleared, which indicates that the HCLK is equal to MCK.
        // Once the PMC_MCKR has been written, the user must wait for PMC_SR.MCKRDY to be set. This
        // can be done either by polling PMC_SR.MCKRDY or by waiting for the interrupt line to be raised if
        // the associated interrupt source (MCKRDY) has been enabled in PMC_IER. PMC_MCKR must not
        // be programmed in a single write operation. The programming sequence for PMC_MCKR is as
        // follows:
        //
        // If a new value for PMC_MCKR.CSS corresponds to any of the available PLL clocks:
        // a. Program PMC_MCKR.PRES.
        // b. Wait for PMC_SR.MCKRDY to be set.
        self.periph
            .pmc_mckr
            .modify(|_r, w| w.pres().variant(cfg.mck_pres));
        while self.periph.pmc_sr.read().mckrdy().bit_is_clear() {}

        // c. Program PMC_MCKR.MDIV.
        // d. Wait for PMC_SR.MCKRDY to be set.
        self.periph
            .pmc_mckr
            .modify(|_r, w| w.mdiv().variant(cfg.mck_div));
        while self.periph.pmc_sr.read().mckrdy().bit_is_clear() {}

        // TODO: Again: a hardcoded compile time check
        let MasterClockSource::PllaClock = cfg.mck_src;

        // e. Program PMC_MCKR.CSS.
        // f. Wait for PMC_SR.MCKRDY to be set.
        self.periph.pmc_mckr.modify(|_r, w| w.css().plla_clk());
        while self.periph.pmc_sr.read().mckrdy().bit_is_clear() {}

        // If a new value for PMC_MCKR.CSS corresponds to MAINCK or SLCK:
        // a. Program PMC_MCKR.CSS.
        // b. Wait for PMC_SR.MCKRDY to be set.
        // c. Program PMC_MCKR.PRES.
        // d. Wait for PMC_SR.MCKRDY to be set.
        //
        // If CSS, MDIV or PRES are modified at any stage, the MCKRDY bit goes low to indicate that MCK
        // and HCLK are not yet ready. The user must wait for MCKRDY bit to be set again before using MCK
        // and HCLK.
        //
        // Note: If PLLA clock was selected as MCK and the user decides to modify it by writing a new value
        // into CKGR_PLLAR, the MCKRDY flag will go low while PLLA is unlocked. Once PLLA is locked
        // again, LOCKA goes high and MCKRDY is set.
        //
        // While PLLA is unlocked, MCK selection is automatically changed to SLCK for PLLA. For further
        // information, see "Clock Switching Waveforms".
        //
        // MCK is MAINCK divided by 2.
        self.settings = Some(cfg);
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
