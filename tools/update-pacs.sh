#!/usr/bin/env bash
set -xeu

# The version of the generated PACs.
# MINOR tracks MINOR of svd2rust.
# PATCH tracks PATCH or svd2rust and SVD fixes (either patched in this repo or updated SVDs from Microchip): bump if svd2rust or SVDs are patched.
version="0.26.0"

valid_args=$(getopt -n '$0' -o "t" --long update-tools -- "$@")
if [[ $? -ne 0 ]]; then
    exit 1;
fi

eval set -- "${valid_args}"
while [ : ]; do
  case "$1" in
    --update-tools)
        echo "Installing tools required to generate PACs..."
        # Install dependencies
        cargo install --force --version 0.26.0 svd2rust
        cargo install --force --version 0.10.0 form
        ;;
    --)
        break
        ;;
  esac
done

rust2svd_version=`expr match "$(svd2rust --version)" 'svd2rust \([0-9]*.[0-9]*.[0-9]*\)'`

top=$(git rev-parse --show-toplevel)

expand() {
    template="${1}.template"
    result="${1}"
    sed -e "s|\${crate}|${crate}|g" \
        -e "s|\${chip}|${chip}|g" \
        -e "s|\${pac}|${pac}|g" \
        -e "s|\${version}|${version}|g" \
        -e "s|\${rust2svd_version}|${rust2svd_version}|g" \
        "${top}/pac/templates/${template}" > ${result}
}

for svd in svd/*.svd; do
    # Find chipname, and create an empty PAC directory
    chip=$(basename "${svd}" .svd)
    crate=$(echo ${chip} | tr '[:upper:]' '[:lower:]')
    pac="${top}/pac/${crate}"
    rm -rf $pac
    mkdir -p $pac
    pushd $pac

    expand Cargo.toml
    expand README.md

    svd2rust -i "${top}/${svd}"
    rm -rf src/
    form -i lib.rs -o src/ && rm lib.rs || true

    popd

    # Set hal/Cargo.toml for crate to VERSION
    cargo add --package atsamx7x-hal --path pac/${crate}
done

# Format workspace, PACs included
cargo fmt
