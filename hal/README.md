# ATSAMx7x Hardware Abstraction Layer

This is crate with implementation of HAL traits from [embedded-hal](https://crates.io/crates/embedded-hal) for following families of Atmel/MicroChip 32-bit MCUs:

* [ATSAMS70](https://www.microchip.com/ParamChartSearch/chart.aspx?branchID=2116)
* [ATSAMV7x](https://www.microchip.com/ParamChartSearch/Chart.aspx?branchID=2117) (_to be done_)
* [ATSAME70](https://www.microchip.com/ParamChartSearch/Chart.aspx?branchID=2113) (_in progress_)

## Usage
For example, to use ATSAMS70Q21 To your `Cargo.toml` add:
```toml
[dependencies]
atsamx7x-hal = { version = "0.1.0", features = ["sams70q21b"] }
```
or to use with [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) startup additions:
```toml
[dependencies]
atsamx7x-hal = { version = "0.1.0", features = ["sams70q21b-rt"] }
```

## MCU Revisions

At the moment this HAL crate supports only revision **B** of SVD files and future removal of PAC crates for _pre-B_ revisions are now considered. Discrepancies between both type in terms of register structure support is too significant to maintain completely different code for them.

## License

[BSD Zero Clause License](https://choosealicense.com/licenses/0bsd/)

