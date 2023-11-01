bootloader_dir = bootloader/
kernel_dir = kernel/
mikanlib_dir = mikanlib/
dist_dir = dist/
mnt_dir = mnt/

all:	build

run:	all
	./run.sh $(dist_dir)/bootloader.efi $(dist_dir)/kernel

clean:
	cargo clean
	rm -rf $(dist_dir) $(mnt_dir)

clippy:	prepare
	cd $(bootloader_dir); \
	cargo clippy
	cd $(kernel_dir); \
	cargo clippy
	cd $(mikanlib_dir); \
	cargo clippy

prepare:
	rustup target add x86_64-unknown-linux-gnu
	rustup target add x86_64-unknown-uefi
	rustup component add rust-src

build:	kernel-build bootloader-build

bootloader-build:	prepare
	mkdir -p $(dist_dir)
	cd $(bootloader_dir); \
	cargo build -Z unstable-options --out-dir /dist --release

kernel-build:	prepare
	mkdir -p $(dist_dir)
	cd $(kernel_dir); \
	cargo build -Z unstable-options --out-dir /dist --release
