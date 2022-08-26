FROM rust:1.63.0-slim

RUN apt-get update && apt-get install -y \
  binutils-arm-none-eabi \
  git \
  && rm -rf /var/lib/apt/lists/*

RUN rustup target add thumbv7em-none-eabihf && \
    rustup component add rustfmt && \
    rustup component add clippy
