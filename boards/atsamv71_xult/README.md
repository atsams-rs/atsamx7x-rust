# SAM V71 Xplained Ultra Evaluation Kit examples

This crate provides HAL usage examples for working with the [SAM V71 Xplained Ultra Evaulation Kit](https://www.microchipdeveloper.com/boards:sam-v71-xult).
The examples are written in [RTIC](https://rtic.rs).

## Prerequisites
* Install the cross-compilation toolchain: `rustup target add thumbv7em-none-eabihf`.
* Install [probe-rs](https://probe.rs/) (formerly cargo-embed):
  
  On Windows `irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex`
  
  On Linux `curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh`


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

The examples can now be flashed and run. For example:
```shell
$ cargo embed --example blinky
```

## Erasing bad firmware
In case the board has been flashed with software that sets it in a bad state before the debugger can attach, bridge the "ERASE" header (J400; north of the MCU) between power cycles, and remove the bridge.
The flash area has now been zeroed.
