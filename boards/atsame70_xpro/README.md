# SAM E70 Xplained Pro Evaluation Kit examples

This crate provides HAL usage examples for working with the [SAM E70 Xplained Pro Evaulation Kit](https://www.microchip.com/en-us/development-tool/DM320113).
The examples are written in [RTIC](https://rtic.rs).

## Prerequisites
* Install the cross-compilation toolchain: `rustup target add thumbv7em-none-eabihf`.
* Install [openocd `v0.11.0`](https://openocd.org/openocd-0-11-0-released.html) (or above).

## Flashing an example
First, the General-Purpose Non-Volatile-Memory (GPNVM) boot bit must be set in order to map the flashed firmware to address `0x0`;
required for the interrupts and software resets to work as expected:
1. Connect the board via the "Debug USB" port and run
   ```shell
   $ openocd -f openocd.cfg -c "atsamv gpnvm set 1" -c exit
   ```
2. Power-cycle the board, and the verify with
   ```shell
   $ openocd -f openocd.cfg -c "atsamv gpnvm show 1" -c exit
   [...]
   samv-gpnvm1: 1
   ```

The example can now be flashed after first, in a seperate shell, `openocd -f openocd.cfg`
```shell
$ cargo run
```
