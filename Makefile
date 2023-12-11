# Directories

CURDIR	:=	$(dir $(abspath $(firstword $(MAKEFILE_LIST))))

DIST_DIR	=	$(CURDIR)/dist
MNT_DIR	=	$(CURDIR)/mnt

BOOTLOADER_DIR	=	$(CURDIR)/bootloader
KERNEL_DIR	=	$(CURDIR)/kernel
STDLIB_DIR	=	$(CURDIR)/stdlib

ASSETS_DIR	=	$(CURDIR)/assets

EDK2_DIR	=	$(ASSETS_DIR)/edk2

HANKAKU_FONTS_DIR	=	$(ASSETS_DIR)/hankaku-fonts

# Artifacts

BOOTLOADER	=	$(DIST_DIR)/bootloader.efi
KERNEL	=	$(DIST_DIR)/kernel

# Rust Variables
DEFAULT_RUST_PROFILE	=	release
RUST_PROFILE	=	$(DEFAULT_RUST_PROFILE)

CARGO	=	cargo
RUSTUP	=	rustup

# QEMU Variables

OVMF_FD	=	$(EDK2_DIR)/OVMF.fd

QEMU_SYSTEM_X86_64	=	qemu-system-x86_64

QEMU_MEMORY	=	-m 1G
QEMU_VGA	=	-vga std
QEMU_DRIVES	=	-drive if=pflash,format=raw,readonly=on,file=$(OVMF_FD) $\
							-drive format=raw,file=fat:rw:$(MNT_DIR)
QEMU_DEVICES	=	-device nec-usb-xhci,id=xhci $\
								-device usb-mouse $\
								-device usb-kbd
QEMU_MONITOR	=	-monitor stdio

QEMU_VNC	=	-vnc :0
QEMU_SPICE	=	-spice port=5930,disable-ticketing=on
QEMU_DISPLAY	?=	-display default

QEMU_ARGS	=	$(QEMU_MEMORY) $\
							$(QEMU_VGA) $\
							$(QEMU_DRIVES) $\
							$(QEMU_DEVICES) $\
							$(QEMU_MONITOR) $\
							$(QEMU_EXTRA_ARGS)

# Environment Variables

export HANKAKU_BIN	:=	$(HANKAKU_FONTS_DIR)/hankaku.bin

define ENV_VARS
HANKAKU_BIN=$(HANKAKU_BIN)
endef

.DEFAULT_GOAL	:=	all
.PHONY:	all
all:	build run

.PHONY:	run
run:	qemu-spice

.PHONY:	build
build:	$(BOOTLOADER_DIR) $(KERNEL_DIR)
	mkdir -p $(MNT_DIR)/EFI/BOOT
	cp $(BOOTLOADER) $(MNT_DIR)/EFI/BOOT/BOOTX64.EFI
	cp $(KERNEL) $(MNT_DIR)

.PHONY:	clean
clean:
	$(CARGO) clean
	rm -rf $(DIST_DIR) $(MNT_DIR)

.PHONY:	clippy
clippy:	prepare
	cd $(BOOTLOADER_DIR); \
	$(CARGO) clippy
	cd $(KERNEL_DIR); \
	$(CARGO) clippy
	cd $(STDLIB_DIR); \
	$(CARGO) clippy

.PHONY:	prepare
prepare:
	$(RUSTUP) component add rust-src
	$(RUSTUP) component add llvm-tools-preview
	$(RUSTUP) target add x86_64-unknown-linux-gnu
	$(RUSTUP) target add x86_64-unknown-linux-musl
	$(RUSTUP) target add x86_64-unknown-uefi

$(BOOTLOADER_DIR):	prepare
	mkdir -p $(DIST_DIR)
	cd $(BOOTLOADER_DIR); \
	$(CARGO) build -Z unstable-options --profile $(RUST_PROFILE) --out-dir $(DIST_DIR)

$(KERNEL_DIR):	prepare
	mkdir -p $(DIST_DIR)
	cd $(KERNEL_DIR); \
	$(CARGO) build -Z unstable-options --profile $(RUST_PROFILE) --out-dir $(DIST_DIR)

.PHONY:	qemu-spice
qemu-spice:
	$(QEMU_SYSTEM_X86_64) $(QEMU_SPICE) $(QEMU_ARGS)

.PHONY:	qemu-vnc
qemu-vnc:
	$(QEMU_SYSTEM_X86_64) $(QEMU_VNC) $(QEMU_ARGS)

.PHONY:	qemu-native
qemu-native:
	$(QEMU_SYSTEM_X86_64) $(QEMU_DISPLAY) $(QEMU_ARGS)

.PHONY: env
env:	export ENV_VARS:=$(ENV_VARS)
env:
	@echo "$$ENV_VARS"

