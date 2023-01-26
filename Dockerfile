FROM rust:1.66.1-slim-buster

COPY ./rust-toolchain .

RUN rustup target add x86_64-unknown-uefi

RUN rustup target add x86_64-unknown-linux-gnu

RUN rustup component add rust-src

RUN apt-get update && apt-get install make lld -y

COPY . /cache

RUN cd /cache/bootloader/ && cargo build && cd /cache/kernel && cargo build

