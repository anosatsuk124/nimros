FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

RUN apt-get update && apt-get install make lld qemu-system -y

WORKDIR /cache

COPY ./rust-toolchain /cache
COPY ./Makefile /cache

RUN cargo init --bin && cargo build --release && \
  cd .. && rm -rf cache
