FROM rust:1.66.1-buster

USER root

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
  gcc clang cmake make automake autogen patch cpio gzip bzip2 wget git sed tar

RUN apt-get update && apt-get upgrade -y && apt-get install -y \ 
  make lld qemu-system \
  gcc-aarch64-linux-gnu gcc-x86-64-linux-gnu mingw-w64 binutils-mingw-w64 gcc-mingw-w64 g++-mingw-w64

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
  python xz-utils uuid-dev llvm-dev lzma-dev libxml2-dev libssl-dev libbz2-dev libtool

## Utilities

WORKDIR /root/work

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
  direnv zsh

COPY ./Makefile .
COPY ./.envrc .

RUN echo 'eval "$(direnv hook bash)"' >> ~/.bashrc
RUN echo 'eval "$(direnv hook zsh)"' >> ~/.zshrc

RUN bash -c "direnv allow"

##

## Rust

WORKDIR /cache

COPY ./rust-toolchain /cache
COPY ./Makefile /cache

RUN make prepare

RUN cargo init --bin && cargo build --release

##

## NIM

WORKDIR /Nim

RUN git clone https://github.com/nim-lang/Nim /Nim && \
  git checkout v2.0.0 && \
  echo 'export PATH=$PATH:/Nim/bin' >> ~/.bashrc && \
  sh build_all.sh

##

