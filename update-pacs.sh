#!/usr/bin/env bash
set -xeu

# The version of the generated PACs.
# MINOR tracks MINOR of svd2rust.
# PATCH tracks PATCH or svd2rust and SVD fixes (either patched in this repo or updated SVDs from Microchip): bump if svd2rust or SVDs are patched.
VERSION="0.25.0"

# Install dependencies
cargo install --force --version 0.25.1 svd2rust
cargo install --force --version 0.10.0 form

TOP=$(dirname $(realpath $0))

for svd in svd/*.svd; do
    # Find chipname, and create an empty PAC directory
    CHIP=$(basename "${svd}" .svd)
    chip=$(echo ${CHIP} | tr '[:upper:]' '[:lower:]')
    pac="${TOP}/pac/${chip}"
    rm -rf $pac
    mkdir -p $pac
    pushd $pac

    cat >Cargo.toml <<EOF
[package]
name = "${chip}"
description = "Peripheral access API for the ${CHIP} MCU from Microchip (generated using svd2rust)"
version = "${VERSION}"
authors = ["Michal Fita <michal.fita@gmail.com>", "Viktor Sonesten <viktor.sonesten@grepit.se>"]
categories = ["no-std", "embedded", "hardware-support"]
keywords = ["no-std", "arm", "cortex-m", "atsam", "pac"]
repository = "https://github.com/atsams-rs/atsamx7x-rust"
readme = "README.md"
edition = "2021"
rust-version = "1.63.0"

[dependencies]
cortex-m = "0.7"
vcell = "0.1.2"

[dependencies.cortex-m-rt]
optional = true
version = "0.7"

[features]
rt = ["cortex-m-rt/device"]

[lib]
name = "${chip}"
path = "src/lib.rs"
EOF

    svd2rust -i "${TOP}/${svd}"
    rm -rf src/
    form -i lib.rs -o src/ && rm lib.rs

    popd

    # Set hal/Cargo.toml for crate to VERSION
    cargo add --package atsamx7x-hal --path pac/${chip}
done

# Format workspace, PACs included
cargo fmt