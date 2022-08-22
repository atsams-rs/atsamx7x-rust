# Changelog

## [Unreleased]

### Added
- [`Monotonic`](https://docs.rs/rtic-monotonic/1.0.0/rtic_monotonic/trait.Monotonic.html), [`ehal::timer::{Countdown,Cancel}`](https://docs.rs/embedded-hal/0.2.7/embedded_hal/timer/index.html), and [`ehal::blocking::delay::Delay{Ms,Us}`](https://docs.rs/embedded-hal/0.2.7/embedded_hal/blocking/delay/index.html) implementations based upon chained Timer Counter channels.
- `boards/atsamv71_xult/examples`:
  - `blinky_tc`: blinks a LED by use of a `tc::Monotonic`; and
  - `blinky_tc_irq`: blinks a LED by use of a `tc::Timer`.
- `serial::Uart::UartError::PrescalerUnderflow` error.
- `serial::Uart::UartError::InvalidPck` error.

### Removed
- `serial::Uart::UartError::BaudRateNotInRange` error.
- `serial::Uart::reconfigure`, which did not exhaustively error check input parameters.

### Fixed
- `Pck::configure` now records its output frequency correctly.
- `Uart::{new,reconfigure}`: now rounds the calculated prescaler to the closest value instead of rounding down.
- `Uart::new`: now returns `Err(UartError::InvalidPck)` if the baud-rate generating `Pck` is not at least three times slow than the peripheral clock (`HostClock`).

### Changed
- `Pck::configure` now takes a `u16` prescaler instead of a `u8`, and returns `Err(PckError)` if the prescaler cannot be applied.
- Renamed `serial::ExtU32` to `serial::ExtBpsU32`, in order to not clash with `fugit::ExtU32`.

## [v0.2.1] 2022-08-08

### Fixed
- `boards/`: Incorrect HAL crate version.

## [v0.2.0] 2022-08-08

### Added
- Nonexhaustive APIs for the following peripherals:
  - PMC;
  - EFC (to configure flash wait states only);
  - GPIO;
  - Serial peripherals: SPI, TWI (I²C), UART and USART (SPI/UART only);
  - AFEC (ADC);
  - RTT (for RTIC scheduling, `ehal::blocking::delay::Delay*` impl); and
  - USB support (via `usb-device`).
- SAM V71/E70 Xplained Ultra/Pro board examples for most above peripherals.
- GitLab CI configuration.

## [v0.1.0] 2022-03-16

* [cmsis-svd](./cmsis-svd/README.md) submodule as source of SVD files instead of collection in LFS (#2)
* [svd2rust] v0.21.0 used for PAC generation (#2)
* watchdog disabling facility (#4)

## [v0.0.2] 2020-06-03

* same70*: added foundations for ATSAME70 series support
* Commitment to 0BSD license following practice from other PAC/HAL crates

## [v0.0.1] 2019-07-31

* pac: Generated for ATSAMS70 family
* hal: Just very simple stuff added
* automation script in Python

[Unreleased]: https://github.com/atsams-rs/atsamx7x-hal/compare/v0.2.1...HEAD
[v0.2.1]: https://github.com/atsams-rs/atsamx7x-hal/compare/v0.2.0...v0.2.1
[v0.2.0]: https://github.com/atsams-rs/atsamx7x-hal/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/atsams-rs/atsamx7x-hal/compare/compare/v0.2.0...v0.1.0
[v0.0.2]: https://github.com/atsams-rs/atsamx7x-hal/compare/v0.0.1...v0.0.2
[v0.0.1]: https://github.com/atsams-rs/atsamx7x-hal/compare/tree/v0.0.1
[svd2rust]: https://github.com/rust-embedded/svd2rust
