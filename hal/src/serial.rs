// Smaller part have 3x UART & 2x USART
use crate::target_device::{UART0, UART1, UART2, USART0, USART1};
#[cfg(any(
    feature = "sams70n19b",
    feature = "sams70n20b",
    feature = "sams70n21b",
    feature = "sams70q19b",
    feature = "sams70q20b",
    feature = "sams70q21b",
    feature = "same70n19b",
    feature = "same70n20b",
    feature = "same70n21b",
    feature = "same70q19b",
    feature = "same70q20b",
    feature = "same70q21b",
))]
use crate::target_device::{UART3, USART2};

#[cfg(any(
    feature = "sams70n19b",
    feature = "sams70n20b",
    feature = "sams70n21b",
    feature = "sams70q19b",
    feature = "sams70q20b",
    feature = "sams70q21b",
    feature = "same70n19b",
    feature = "same70n20b",
    feature = "same70n21b",
    feature = "same70q19b",
    feature = "same70q20b",
    feature = "same70q21b",
))]
use crate::target_device::UART4;

use crate::target_device::{
    uart0::RegisterBlock as UARTRegisterBlock, usart0::RegisterBlock as USARTRegisterBlock,
};

pub struct Serial<P> {
    peripheral: P,
}

pub type Serial0 = Serial<UART0>;
pub type Serial1 = Serial<UART1>;
pub type Serial2 = Serial<UART2>;

#[cfg(any(
    feature = "sams70n19b",
    feature = "sams70n20b",
    feature = "sams70n21b",
    feature = "sams70q19b",
    feature = "sams70q20b",
    feature = "sams70q21b",
    feature = "same70n19b",
    feature = "same70n20b",
    feature = "same70n21b",
    feature = "same70q19b",
    feature = "same70q20b",
    feature = "same70q21b",
))]
pub type Serial3 = Serial<UART3>;

#[cfg(any(
    feature = "sams70n19b",
    feature = "sams70n20b",
    feature = "sams70n21b",
    feature = "sams70q19b",
    feature = "sams70q20b",
    feature = "sams70q21b",
    feature = "same70n19b",
    feature = "same70n20b",
    feature = "same70n21b",
    feature = "same70q19b",
    feature = "same70q20b",
    feature = "same70q21b",
))]
pub type Serial4 = Serial<UART4>;

pub type Serial5 = Serial<USART0>;
pub type Serial6 = Serial<USART1>;

#[cfg(any(
    feature = "sams70n19b",
    feature = "sams70n20b",
    feature = "sams70n21b",
    feature = "sams70q19b",
    feature = "sams70q20b",
    feature = "sams70q21b",
    feature = "same70n19b",
    feature = "same70n20b",
    feature = "same70n21b",
    feature = "same70q19b",
    feature = "same70q20b",
    feature = "same70q21b",
))]
pub type Serial7 = Serial<USART2>;

//#[cfg(any(
//    feature = "?"))]
//pub type Serial8 = Serial<USART3>;

#[derive(Debug)]
pub enum Error {
    /// Buffer overrun
    Overrun,
    // omitted: other error variants
}

impl hal::serial::Write<u8> for Serial<UART0> {
    type Error = Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Error> {
        write_uart(&*self.peripheral, word)
    }

    fn flush(&mut self) -> nb::Result<(), Error> {
        flush_uart(&*self.peripheral)
    }
}

impl hal::serial::Write<u8> for Serial<USART1> {
    type Error = Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Error> {
        write_usart(&*self.peripheral, word)
    }

    fn flush(&mut self) -> nb::Result<(), Error> {
        flush_usart(&*self.peripheral)
    }
}

#[cfg(any(
    feature = "sams70q20b",
    feature = "sams70q21b",
    feature = "same70q20b",
    feature = "same70q21b",
))]
impl hal::serial::Write<u8> for Serial<UART3> {
    type Error = Error;

    fn write(&mut self, word: u8) -> nb::Result<(), Error> {
        write_uart(&*self.peripheral, word)
    }

    fn flush(&mut self) -> nb::Result<(), Error> {
        flush_uart(&*self.peripheral)
    }
}

fn write_uart(regs: &UARTRegisterBlock, word: u8) -> nb::Result<(), Error> {
    if regs.uart_sr.read().txempty().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        regs.uart_thr.write(|w| unsafe { w.txchr().bits(word) });
        Ok(())
    }
}

fn write_usart(regs: &USARTRegisterBlock, word: u8) -> nb::Result<(), Error> {
    if regs.us_csr.read().txempty().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        regs.us_thr
            .write(|w| unsafe { w.txchr().bits(word as u16) });
        Ok(())
    }
}

fn flush_uart(regs: &UARTRegisterBlock) -> nb::Result<(), Error> {
    if regs.uart_sr.read().txempty().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        Ok(())
    }
}

fn flush_usart(regs: &USARTRegisterBlock) -> nb::Result<(), Error> {
    if regs.us_csr.read().txempty().bit_is_clear() {
        Err(nb::Error::WouldBlock)
    } else {
        Ok(())
    }
}
