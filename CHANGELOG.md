# Changelog

## [Unreleased]

## [v0.2.0] 22-08-08

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

[Unreleased]: https://github.com/atsams-rs/atsamx7x-hal/compare/v0.2.0...HEAD
[v0.2.0]: https://github.com/atsams-rs/atsamx7x-hal/compare/v0.1.0...v0.2.0
[v0.1.0]: https://github.com/atsams-rs/atsamx7x-hal/compare/compare/v0.2.0...v0.1.0
[v0.0.2]: https://github.com/atsams-rs/atsamx7x-hal/compare/v0.0.1...v0.0.2
[v0.0.1]: https://github.com/atsams-rs/atsamx7x-hal/compare/tree/v0.0.1
[svd2rust]: https://github.com/rust-embedded/svd2rust
