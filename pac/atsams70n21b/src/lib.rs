#![doc = "Peripheral access API for ATSAMS70N21B microcontrollers (generated using svd2rust v0.15.2)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.15.2/svd2rust/#peripheral-api"]
#![deny(missing_docs)]
#![deny(warnings)]
#![allow(non_camel_case_types)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[cfg(feature = "rt")]
extern "C" {
    fn SUPC();
    fn RSTC();
    fn RTC();
    fn RTT();
    fn WDT();
    fn PMC();
    fn EFC();
    fn UART0();
    fn UART1();
    fn PIOA();
    fn PIOB();
    fn USART0();
    fn USART1();
    fn USART2();
    fn PIOD();
    fn HSMCI();
    fn TWIHS0();
    fn TWIHS1();
    fn SPI0();
    fn SSC();
    fn TC0();
    fn TC1();
    fn TC2();
    fn TC3();
    fn TC4();
    fn TC5();
    fn AFEC0();
    fn DACC();
    fn PWM0();
    fn ICM();
    fn ACC();
    fn USBHS();
    fn AFEC1();
    fn TWIHS2();
    fn QSPI();
    fn UART2();
    fn UART3();
    fn UART4();
    fn TC6();
    fn TC7();
    fn TC8();
    fn TC9();
    fn TC10();
    fn TC11();
    fn AES();
    fn TRNG();
    fn XDMAC();
    fn ISI();
    fn PWM1();
    fn FPU();
    fn RSWDT();
    fn CCW();
    fn CCF();
    fn IXC();
    fn I2SC0();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 70] = [
    Vector { _handler: SUPC },
    Vector { _handler: RSTC },
    Vector { _handler: RTC },
    Vector { _handler: RTT },
    Vector { _handler: WDT },
    Vector { _handler: PMC },
    Vector { _handler: EFC },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _reserved: 0 },
    Vector { _handler: PIOA },
    Vector { _handler: PIOB },
    Vector { _reserved: 0 },
    Vector { _handler: USART0 },
    Vector { _handler: USART1 },
    Vector { _handler: USART2 },
    Vector { _handler: PIOD },
    Vector { _reserved: 0 },
    Vector { _handler: HSMCI },
    Vector { _handler: TWIHS0 },
    Vector { _handler: TWIHS1 },
    Vector { _handler: SPI0 },
    Vector { _handler: SSC },
    Vector { _handler: TC0 },
    Vector { _handler: TC1 },
    Vector { _handler: TC2 },
    Vector { _handler: TC3 },
    Vector { _handler: TC4 },
    Vector { _handler: TC5 },
    Vector { _handler: AFEC0 },
    Vector { _handler: DACC },
    Vector { _handler: PWM0 },
    Vector { _handler: ICM },
    Vector { _handler: ACC },
    Vector { _handler: USBHS },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: AFEC1 },
    Vector { _handler: TWIHS2 },
    Vector { _reserved: 0 },
    Vector { _handler: QSPI },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: UART4 },
    Vector { _handler: TC6 },
    Vector { _handler: TC7 },
    Vector { _handler: TC8 },
    Vector { _handler: TC9 },
    Vector { _handler: TC10 },
    Vector { _handler: TC11 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: AES },
    Vector { _handler: TRNG },
    Vector { _handler: XDMAC },
    Vector { _handler: ISI },
    Vector { _handler: PWM1 },
    Vector { _handler: FPU },
    Vector { _reserved: 0 },
    Vector { _handler: RSWDT },
    Vector { _handler: CCW },
    Vector { _handler: CCF },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: IXC },
    Vector { _handler: I2SC0 },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - SUPC"]
    SUPC,
    #[doc = "1 - RSTC"]
    RSTC,
    #[doc = "2 - RTC"]
    RTC,
    #[doc = "3 - RTT"]
    RTT,
    #[doc = "4 - WDT"]
    WDT,
    #[doc = "5 - PMC"]
    PMC,
    #[doc = "6 - EFC"]
    EFC,
    #[doc = "7 - UART0"]
    UART0,
    #[doc = "8 - UART1"]
    UART1,
    #[doc = "10 - PIOA"]
    PIOA,
    #[doc = "11 - PIOB"]
    PIOB,
    #[doc = "13 - USART0"]
    USART0,
    #[doc = "14 - USART1"]
    USART1,
    #[doc = "15 - USART2"]
    USART2,
    #[doc = "16 - PIOD"]
    PIOD,
    #[doc = "18 - HSMCI"]
    HSMCI,
    #[doc = "19 - TWIHS0"]
    TWIHS0,
    #[doc = "20 - TWIHS1"]
    TWIHS1,
    #[doc = "21 - SPI0"]
    SPI0,
    #[doc = "22 - SSC"]
    SSC,
    #[doc = "23 - TC0"]
    TC0,
    #[doc = "24 - TC1"]
    TC1,
    #[doc = "25 - TC2"]
    TC2,
    #[doc = "26 - TC3"]
    TC3,
    #[doc = "27 - TC4"]
    TC4,
    #[doc = "28 - TC5"]
    TC5,
    #[doc = "29 - AFEC0"]
    AFEC0,
    #[doc = "30 - DACC"]
    DACC,
    #[doc = "31 - PWM0"]
    PWM0,
    #[doc = "32 - ICM"]
    ICM,
    #[doc = "33 - ACC"]
    ACC,
    #[doc = "34 - USBHS"]
    USBHS,
    #[doc = "40 - AFEC1"]
    AFEC1,
    #[doc = "41 - TWIHS2"]
    TWIHS2,
    #[doc = "43 - QSPI"]
    QSPI,
    #[doc = "44 - UART2"]
    UART2,
    #[doc = "45 - UART3"]
    UART3,
    #[doc = "46 - UART4"]
    UART4,
    #[doc = "47 - TC6"]
    TC6,
    #[doc = "48 - TC7"]
    TC7,
    #[doc = "49 - TC8"]
    TC8,
    #[doc = "50 - TC9"]
    TC9,
    #[doc = "51 - TC10"]
    TC10,
    #[doc = "52 - TC11"]
    TC11,
    #[doc = "56 - AES"]
    AES,
    #[doc = "57 - TRNG"]
    TRNG,
    #[doc = "58 - XDMAC"]
    XDMAC,
    #[doc = "59 - ISI"]
    ISI,
    #[doc = "60 - PWM1"]
    PWM1,
    #[doc = "61 - FPU"]
    FPU,
    #[doc = "63 - RSWDT"]
    RSWDT,
    #[doc = "64 - CCW"]
    CCW,
    #[doc = "65 - CCF"]
    CCF,
    #[doc = "68 - IXC"]
    IXC,
    #[doc = "69 - I2SC0"]
    I2SC0,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::SUPC => 0,
            Interrupt::RSTC => 1,
            Interrupt::RTC => 2,
            Interrupt::RTT => 3,
            Interrupt::WDT => 4,
            Interrupt::PMC => 5,
            Interrupt::EFC => 6,
            Interrupt::UART0 => 7,
            Interrupt::UART1 => 8,
            Interrupt::PIOA => 10,
            Interrupt::PIOB => 11,
            Interrupt::USART0 => 13,
            Interrupt::USART1 => 14,
            Interrupt::USART2 => 15,
            Interrupt::PIOD => 16,
            Interrupt::HSMCI => 18,
            Interrupt::TWIHS0 => 19,
            Interrupt::TWIHS1 => 20,
            Interrupt::SPI0 => 21,
            Interrupt::SSC => 22,
            Interrupt::TC0 => 23,
            Interrupt::TC1 => 24,
            Interrupt::TC2 => 25,
            Interrupt::TC3 => 26,
            Interrupt::TC4 => 27,
            Interrupt::TC5 => 28,
            Interrupt::AFEC0 => 29,
            Interrupt::DACC => 30,
            Interrupt::PWM0 => 31,
            Interrupt::ICM => 32,
            Interrupt::ACC => 33,
            Interrupt::USBHS => 34,
            Interrupt::AFEC1 => 40,
            Interrupt::TWIHS2 => 41,
            Interrupt::QSPI => 43,
            Interrupt::UART2 => 44,
            Interrupt::UART3 => 45,
            Interrupt::UART4 => 46,
            Interrupt::TC6 => 47,
            Interrupt::TC7 => 48,
            Interrupt::TC8 => 49,
            Interrupt::TC9 => 50,
            Interrupt::TC10 => 51,
            Interrupt::TC11 => 52,
            Interrupt::AES => 56,
            Interrupt::TRNG => 57,
            Interrupt::XDMAC => 58,
            Interrupt::ISI => 59,
            Interrupt::PWM1 => 60,
            Interrupt::FPU => 61,
            Interrupt::RSWDT => 63,
            Interrupt::CCW => 64,
            Interrupt::CCF => 65,
            Interrupt::IXC => 68,
            Interrupt::I2SC0 => 69,
        }
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, FPU, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r" Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Analog Comparator Controller"]
pub struct ACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ACC {}
impl ACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const acc::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for ACC {
    type Target = acc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ACC::ptr() }
    }
}
#[doc = "Analog Comparator Controller"]
pub mod acc;
#[doc = "Advanced Encryption Standard"]
pub struct AES {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AES {}
impl AES {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const aes::RegisterBlock {
        0x4006_c000 as *const _
    }
}
impl Deref for AES {
    type Target = aes::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AES::ptr() }
    }
}
#[doc = "Advanced Encryption Standard"]
pub mod aes;
#[doc = "Analog Front-End Controller"]
pub struct AFEC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFEC0 {}
impl AFEC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afec0::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for AFEC0 {
    type Target = afec0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFEC0::ptr() }
    }
}
#[doc = "Analog Front-End Controller"]
pub mod afec0;
#[doc = "Analog Front-End Controller"]
pub struct AFEC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for AFEC1 {}
impl AFEC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const afec0::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for AFEC1 {
    type Target = afec0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*AFEC1::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub struct CHIPID {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CHIPID {}
impl CHIPID {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const chipid::RegisterBlock {
        0x400e_0940 as *const _
    }
}
impl Deref for CHIPID {
    type Target = chipid::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*CHIPID::ptr() }
    }
}
#[doc = "Chip Identifier"]
pub mod chipid;
#[doc = "Digital-to-Analog Converter Controller"]
pub struct DACC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DACC {}
impl DACC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dacc::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for DACC {
    type Target = dacc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*DACC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter Controller"]
pub mod dacc;
#[doc = "Embedded Flash Controller"]
pub struct EFC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFC {}
impl EFC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efc::RegisterBlock {
        0x400e_0c00 as *const _
    }
}
impl Deref for EFC {
    type Target = efc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*EFC::ptr() }
    }
}
#[doc = "Embedded Flash Controller"]
pub mod efc;
#[doc = "General Purpose Backup Registers"]
pub struct GPBR {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPBR {}
impl GPBR {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpbr::RegisterBlock {
        0x400e_1890 as *const _
    }
}
impl Deref for GPBR {
    type Target = gpbr::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPBR::ptr() }
    }
}
#[doc = "General Purpose Backup Registers"]
pub mod gpbr;
#[doc = "High Speed MultiMedia Card Interface"]
pub struct HSMCI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for HSMCI {}
impl HSMCI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const hsmci::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for HSMCI {
    type Target = hsmci::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*HSMCI::ptr() }
    }
}
#[doc = "High Speed MultiMedia Card Interface"]
pub mod hsmci;
#[doc = "Inter-IC Sound Controller"]
pub struct I2SC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2SC0 {}
impl I2SC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2sc0::RegisterBlock {
        0x4008_c000 as *const _
    }
}
impl Deref for I2SC0 {
    type Target = i2sc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2SC0::ptr() }
    }
}
#[doc = "Inter-IC Sound Controller"]
pub mod i2sc0;
#[doc = "Integrity Check Monitor"]
pub struct ICM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ICM {}
impl ICM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const icm::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for ICM {
    type Target = icm::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ICM::ptr() }
    }
}
#[doc = "Integrity Check Monitor"]
pub mod icm;
#[doc = "Image Sensor Interface"]
pub struct ISI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ISI {}
impl ISI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const isi::RegisterBlock {
        0x4004_c000 as *const _
    }
}
impl Deref for ISI {
    type Target = isi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*ISI::ptr() }
    }
}
#[doc = "Image Sensor Interface"]
pub mod isi;
#[doc = "AHB Bus Matrix"]
pub struct MATRIX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MATRIX {}
impl MATRIX {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const matrix::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for MATRIX {
    type Target = matrix::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*MATRIX::ptr() }
    }
}
#[doc = "AHB Bus Matrix"]
pub mod matrix;
#[doc = "Parallel Input/Output Controller"]
pub struct PIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOA {}
impl PIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        0x400e_0e00 as *const _
    }
}
impl Deref for PIOA {
    type Target = pioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOA::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller"]
pub mod pioa;
#[doc = "Parallel Input/Output Controller"]
pub struct PIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOB {}
impl PIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        0x400e_1000 as *const _
    }
}
impl Deref for PIOB {
    type Target = pioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOB::ptr() }
    }
}
#[doc = "Parallel Input/Output Controller"]
pub struct PIOD {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PIOD {}
impl PIOD {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pioa::RegisterBlock {
        0x400e_1400 as *const _
    }
}
impl Deref for PIOD {
    type Target = pioa::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PIOD::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x400e_0600 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "Pulse Width Modulation Controller"]
pub struct PWM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM0 {}
impl PWM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for PWM0 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM0::ptr() }
    }
}
#[doc = "Pulse Width Modulation Controller"]
pub mod pwm0;
#[doc = "Pulse Width Modulation Controller"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm0::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub struct QSPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QSPI {}
impl QSPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qspi::RegisterBlock {
        0x4007_c000 as *const _
    }
}
impl Deref for QSPI {
    type Target = qspi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*QSPI::ptr() }
    }
}
#[doc = "Quad Serial Peripheral Interface"]
pub mod qspi;
#[doc = "Reset Controller"]
pub struct RSTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSTC {}
impl RSTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rstc::RegisterBlock {
        0x400e_1800 as *const _
    }
}
impl Deref for RSTC {
    type Target = rstc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSTC::ptr() }
    }
}
#[doc = "Reset Controller"]
pub mod rstc;
#[doc = "Reinforced Safety Watchdog Timer"]
pub struct RSWDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RSWDT {}
impl RSWDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rswdt::RegisterBlock {
        0x400e_1900 as *const _
    }
}
impl Deref for RSWDT {
    type Target = rswdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RSWDT::ptr() }
    }
}
#[doc = "Reinforced Safety Watchdog Timer"]
pub mod rswdt;
#[doc = "Real-time Clock"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x400e_1860 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real-time Clock"]
pub mod rtc;
#[doc = "Real-time Timer"]
pub struct RTT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTT {}
impl RTT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtt::RegisterBlock {
        0x400e_1830 as *const _
    }
}
impl Deref for RTT {
    type Target = rtt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTT::ptr() }
    }
}
#[doc = "Real-time Timer"]
pub mod rtt;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Synchronous Serial Controller"]
pub struct SSC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSC {}
impl SSC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssc::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for SSC {
    type Target = ssc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSC::ptr() }
    }
}
#[doc = "Synchronous Serial Controller"]
pub mod ssc;
#[doc = "Supply Controller"]
pub struct SUPC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SUPC {}
impl SUPC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const supc::RegisterBlock {
        0x400e_1810 as *const _
    }
}
impl Deref for SUPC {
    type Target = supc::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SUPC::ptr() }
    }
}
#[doc = "Supply Controller"]
pub mod supc;
#[doc = "Timer Counter"]
pub struct TC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC0 {}
impl TC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for TC0 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC0::ptr() }
    }
}
#[doc = "Timer Counter"]
pub mod tc0;
#[doc = "Timer Counter"]
pub struct TC1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC1 {}
impl TC1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for TC1 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC1::ptr() }
    }
}
#[doc = "Timer Counter"]
pub struct TC2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC2 {}
impl TC2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4001_4000 as *const _
    }
}
impl Deref for TC2 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC2::ptr() }
    }
}
#[doc = "Timer Counter"]
pub struct TC3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TC3 {}
impl TC3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tc0::RegisterBlock {
        0x4005_4000 as *const _
    }
}
impl Deref for TC3 {
    type Target = tc0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TC3::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub struct TRNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TRNG {}
impl TRNG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const trng::RegisterBlock {
        0x4007_0000 as *const _
    }
}
impl Deref for TRNG {
    type Target = trng::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TRNG::ptr() }
    }
}
#[doc = "True Random Number Generator"]
pub mod trng;
#[doc = "Two-wire Interface High Speed"]
pub struct TWIHS0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIHS0 {}
impl TWIHS0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twihs0::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for TWIHS0 {
    type Target = twihs0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIHS0::ptr() }
    }
}
#[doc = "Two-wire Interface High Speed"]
pub mod twihs0;
#[doc = "Two-wire Interface High Speed"]
pub struct TWIHS1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIHS1 {}
impl TWIHS1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twihs0::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for TWIHS1 {
    type Target = twihs0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIHS1::ptr() }
    }
}
#[doc = "Two-wire Interface High Speed"]
pub struct TWIHS2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TWIHS2 {}
impl TWIHS2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const twihs0::RegisterBlock {
        0x4006_0000 as *const _
    }
}
impl Deref for TWIHS2 {
    type Target = twihs0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*TWIHS2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_0800 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub mod uart0;
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_0a00 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_1a00 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_1c00 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver Transmitter"]
pub struct UART4 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART4 {}
impl UART4 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x400e_1e00 as *const _
    }
}
impl Deref for UART4 {
    type Target = uart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART4::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter"]
pub struct USART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART0 {}
impl USART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for USART0 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART0::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter"]
pub mod usart0;
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter"]
pub struct USART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART1 {}
impl USART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_8000 as *const _
    }
}
impl Deref for USART1 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART1::ptr() }
    }
}
#[doc = "Universal Synchronous Asynchronous Receiver Transmitter"]
pub struct USART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USART2 {}
impl USART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usart0::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for USART2 {
    type Target = usart0::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USART2::ptr() }
    }
}
#[doc = "USB High-Speed Interface"]
pub struct USBHS {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USBHS {}
impl USBHS {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usbhs::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for USBHS {
    type Target = usbhs::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*USBHS::ptr() }
    }
}
#[doc = "USB High-Speed Interface"]
pub mod usbhs;
#[doc = "USB Transmitter Interface Macrocell"]
pub struct UTMI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UTMI {}
impl UTMI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const utmi::RegisterBlock {
        0x400e_0400 as *const _
    }
}
impl Deref for UTMI {
    type Target = utmi::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*UTMI::ptr() }
    }
}
#[doc = "USB Transmitter Interface Macrocell"]
pub mod utmi;
#[doc = "Watchdog Timer"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x400e_1850 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer"]
pub mod wdt;
#[doc = "Extensible DMA Controller"]
pub struct XDMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for XDMAC {}
impl XDMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const xdmac::RegisterBlock {
        0x4007_8000 as *const _
    }
}
impl Deref for XDMAC {
    type Target = xdmac::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*XDMAC::ptr() }
    }
}
#[doc = "Extensible DMA Controller"]
pub mod xdmac;
#[doc = "LOCKBIT"]
pub struct LOCKBIT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LOCKBIT {}
impl LOCKBIT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lockbit::RegisterBlock {
        0 as *const _
    }
}
impl Deref for LOCKBIT {
    type Target = lockbit::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*LOCKBIT::ptr() }
    }
}
#[doc = "LOCKBIT"]
pub mod lockbit;
#[doc = "System control not in SCB"]
pub struct SCNSCB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SCNSCB {}
impl SCNSCB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const scn_scb::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SCNSCB {
    type Target = scn_scb::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SCNSCB::ptr() }
    }
}
#[doc = "System control not in SCB"]
pub mod scn_scb;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "ACC"]
    pub ACC: ACC,
    #[doc = "AES"]
    pub AES: AES,
    #[doc = "AFEC0"]
    pub AFEC0: AFEC0,
    #[doc = "AFEC1"]
    pub AFEC1: AFEC1,
    #[doc = "CHIPID"]
    pub CHIPID: CHIPID,
    #[doc = "DACC"]
    pub DACC: DACC,
    #[doc = "EFC"]
    pub EFC: EFC,
    #[doc = "GPBR"]
    pub GPBR: GPBR,
    #[doc = "HSMCI"]
    pub HSMCI: HSMCI,
    #[doc = "I2SC0"]
    pub I2SC0: I2SC0,
    #[doc = "ICM"]
    pub ICM: ICM,
    #[doc = "ISI"]
    pub ISI: ISI,
    #[doc = "MATRIX"]
    pub MATRIX: MATRIX,
    #[doc = "PIOA"]
    pub PIOA: PIOA,
    #[doc = "PIOB"]
    pub PIOB: PIOB,
    #[doc = "PIOD"]
    pub PIOD: PIOD,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "PWM0"]
    pub PWM0: PWM0,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "QSPI"]
    pub QSPI: QSPI,
    #[doc = "RSTC"]
    pub RSTC: RSTC,
    #[doc = "RSWDT"]
    pub RSWDT: RSWDT,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "RTT"]
    pub RTT: RTT,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SSC"]
    pub SSC: SSC,
    #[doc = "SUPC"]
    pub SUPC: SUPC,
    #[doc = "TC0"]
    pub TC0: TC0,
    #[doc = "TC1"]
    pub TC1: TC1,
    #[doc = "TC2"]
    pub TC2: TC2,
    #[doc = "TC3"]
    pub TC3: TC3,
    #[doc = "TRNG"]
    pub TRNG: TRNG,
    #[doc = "TWIHS0"]
    pub TWIHS0: TWIHS0,
    #[doc = "TWIHS1"]
    pub TWIHS1: TWIHS1,
    #[doc = "TWIHS2"]
    pub TWIHS2: TWIHS2,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "UART4"]
    pub UART4: UART4,
    #[doc = "USART0"]
    pub USART0: USART0,
    #[doc = "USART1"]
    pub USART1: USART1,
    #[doc = "USART2"]
    pub USART2: USART2,
    #[doc = "USBHS"]
    pub USBHS: USBHS,
    #[doc = "UTMI"]
    pub UTMI: UTMI,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "XDMAC"]
    pub XDMAC: XDMAC,
    #[doc = "LOCKBIT"]
    pub LOCKBIT: LOCKBIT,
    #[doc = "SCNSCB"]
    pub SCNSCB: SCNSCB,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            ACC: ACC {
                _marker: PhantomData,
            },
            AES: AES {
                _marker: PhantomData,
            },
            AFEC0: AFEC0 {
                _marker: PhantomData,
            },
            AFEC1: AFEC1 {
                _marker: PhantomData,
            },
            CHIPID: CHIPID {
                _marker: PhantomData,
            },
            DACC: DACC {
                _marker: PhantomData,
            },
            EFC: EFC {
                _marker: PhantomData,
            },
            GPBR: GPBR {
                _marker: PhantomData,
            },
            HSMCI: HSMCI {
                _marker: PhantomData,
            },
            I2SC0: I2SC0 {
                _marker: PhantomData,
            },
            ICM: ICM {
                _marker: PhantomData,
            },
            ISI: ISI {
                _marker: PhantomData,
            },
            MATRIX: MATRIX {
                _marker: PhantomData,
            },
            PIOA: PIOA {
                _marker: PhantomData,
            },
            PIOB: PIOB {
                _marker: PhantomData,
            },
            PIOD: PIOD {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            PWM0: PWM0 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            QSPI: QSPI {
                _marker: PhantomData,
            },
            RSTC: RSTC {
                _marker: PhantomData,
            },
            RSWDT: RSWDT {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            RTT: RTT {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SSC: SSC {
                _marker: PhantomData,
            },
            SUPC: SUPC {
                _marker: PhantomData,
            },
            TC0: TC0 {
                _marker: PhantomData,
            },
            TC1: TC1 {
                _marker: PhantomData,
            },
            TC2: TC2 {
                _marker: PhantomData,
            },
            TC3: TC3 {
                _marker: PhantomData,
            },
            TRNG: TRNG {
                _marker: PhantomData,
            },
            TWIHS0: TWIHS0 {
                _marker: PhantomData,
            },
            TWIHS1: TWIHS1 {
                _marker: PhantomData,
            },
            TWIHS2: TWIHS2 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            UART4: UART4 {
                _marker: PhantomData,
            },
            USART0: USART0 {
                _marker: PhantomData,
            },
            USART1: USART1 {
                _marker: PhantomData,
            },
            USART2: USART2 {
                _marker: PhantomData,
            },
            USBHS: USBHS {
                _marker: PhantomData,
            },
            UTMI: UTMI {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            XDMAC: XDMAC {
                _marker: PhantomData,
            },
            LOCKBIT: LOCKBIT {
                _marker: PhantomData,
            },
            SCNSCB: SCNSCB {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
        }
    }
}
