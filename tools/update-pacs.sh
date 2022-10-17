#!/usr/bin/env bash
set -xeu

# The version of the generated PACs.
# MINOR tracks MINOR of svd2rust.
# PATCH tracks PATCH or svd2rust and SVD fixes (either patched in this repo or updated SVDs from Microchip): bump if svd2rust or SVDs are patched.
version="0.25.0"

# Install dependencies
#cargo install --force --version 0.25.1 svd2rust
#cargo install --force --version 0.10.0 form

top=$(dirname $(realpath $0))

expand() {
    local template="$(cat $1)"
    eval "echo \"${template}\""
}

for svd in svd/*.svd; do
    # Find chipname, and create an empty PAC directory
    chip=$(basename "${svd}" .svd)
    crate=$(echo ${chip} | tr '[:upper:]' '[:lower:]')
    pac="${top}/pac/${crate}"
    rm -rf $pac
    mkdir -p $pac
    pushd $pac

    sed -e "s|\${crate}|${crate}|g" -e "s|\${chip}|${chip}|g" -e "s|\${pac}|${pac}|g" -e "s|\${version}|${version}|g" "${top}/pac/templates/Cargo.toml.template" > Cargo.toml
    sed -e "s|\${crate}|${crate}|g" -e "s|\${chip}|${chip}|g" -e "s|\${pac}|${pac}|g" -e "s|\${version}|${version}|g" "${top}/pac/templates/README.md.template" > README.md

    svd2rust -i "${top}/${svd}"
    rm -rf src/
    form -i lib.rs -o src/ && rm lib.rs

    popd

    # Set hal/Cargo.toml for crate to VERSION
    # TODO #?: figure out if we need this
    # cargo add --package atsamx7x-hal --path pac/${crate}
done

# Format workspace, PACs included
cargo fmt
