# ATSAMx7x Hardware Abstraction Layer

This is crate with implementation of HAL traits from [embedded-hal](https://crates.io/crates/embedded-hal) for following families of Atmel/MicroChip 32-bit MCUs:

* [ATSAMS70](https://www.microchip.com/ParamChartSearch/chart.aspx?branchID=2116)
* [ATSAMV7x](https://www.microchip.com/ParamChartSearch/Chart.aspx?branchID=2117) (_to be done_)
* [ATSAME70](https://www.microchip.com/ParamChartSearch/Chart.aspx?branchID=2113) (_in progress_)

## MCU Revisions

At the moment this HAL crate supports only revision **B** of SVD files and future removal of PAC crates for _pre-B_ revisions are now considered. Discrepancies between both type in terms of register structue support is too significant to maintain completely different code for them.

## License

[BSD Zero Clause License](https://choosealicense.com/licenses/0bsd/)

