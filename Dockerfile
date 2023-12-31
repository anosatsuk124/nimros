FROM rust:1.66.1-buster

USER root

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update && apt-get upgrade -y && apt-get install -y \
    gcc clang cmake make automake autogen patch cpio gzip bzip2 wget git sed tar \
    build-essential

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update && apt-get upgrade -y && apt-get install -y \
    make lld \
    gcc-aarch64-linux-gnu gcc-x86-64-linux-gnu mingw-w64 binutils-mingw-w64 gcc-mingw-w64 g++-mingw-w64

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update && apt-get upgrade -y && apt-get install -y --no-install-recommends \
    qemu-system

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update && apt-get upgrade -y && apt-get install -y \
    python xz-utils uuid-dev llvm-dev lzma-dev libxml2-dev libssl-dev libbz2-dev libtool

## Utilities

WORKDIR /work

RUN --mount=target=/var/lib/apt/lists,type=cache,sharing=locked \
    --mount=target=/var/cache/apt,type=cache,sharing=locked \
    apt-get update && apt-get upgrade -y && apt-get install -y \
    direnv zsh

COPY ./Makefile .
COPY ./.envrc .

RUN echo 'source ~/.bashrc' > ~/.zshrc
RUN echo 'eval "$(direnv hook bash)"' >> ~/.bashrc

RUN bash -c "direnv allow"

##

## Rust

WORKDIR /work

COPY rust-toolchain .

RUN make prepare

##

## NIM

WORKDIR /Nim

RUN git clone https://github.com/nim-lang/Nim /Nim && \
    git checkout v2.0.0 && \
    sh build_all.sh

RUN echo 'export PATH=$PATH:/Nim/bin' >> ~/.bashrc

##

## Build for caching

WORKDIR /work

COPY . .

RUN make build

##
