# Rust Variables
DEFAULT_RUST_PROFILE	=	release
RUST_PROFILE	=	$(DEFAULT_RUST_PROFILE)

CARGO	=	cargo
RUSTUP	=	rustup

# Directories

DIST_DIR	=	$(CURDIR)/dist
MNT_DIR	=	$(CURDIR)/mnt

BOOTLOADER_DIR	=	$(CURDIR)/bootloader
KERNEL_DIR	=	$(CURDIR)/kernel
STDLIB_DIR	=	$(CURDIR)/stdlib

# Utilities

RUN_SH	=	sh $(CURDIR)/run.sh

# Artifacts

BOOTLOADER	=	$(DIST_DIR)/bootloader.efi
KERNEL	=	$(DIST_DIR)/kernel

all:	build

.PHONY: run
run:	$(BOOTLOADER) $(KERNEL)
	mkdir -p $(MNT_DIR)/EFI/BOOT
	cp $(BOOTLOADER) $(MNT_DIR)/EFI/BOOT/BOOTX64.EFI
	cp $(KERNEL) $(MNT_DIR)
	$(RUN_SH)

.PHONY: clean
clean:
	$(CARGO) clean
	rm -rf $(DIST_DIR) $(MNT_DIR)

.PHONY: clippy
clippy:	prepare
	cd $(BOOTLOADER_DIR); \
	$(CARGO) clippy
	cd $(KERNEL_DIR); \
	$(CARGO) clippy
	cd $(STDLIB_DIR); \
	$(CARGO) clippy

build:	$(BOOTLOADER_DIR) $(KERNEL_DIR)

$(BOOTLOADER_DIR):	prepare
	mkdir -p $(DIST_DIR)
	cd $(BOOTLOADER_DIR); \
	$(CARGO) build -Z unstable-options --profile $(RUST_PROFILE) --out-dir $(DIST_DIR)

$(KERNEL_DIR):	prepare
	mkdir -p $(DIST_DIR)
	cd $(KERNEL_DIR); \
	$(CARGO) build -Z unstable-options --profile $(RUST_PROFILE) --out-dir $(DIST_DIR)

.PHONY: prepare
prepare:
	$(RUSTUP) component add rust-src
	$(RUSTUP) component add llvm-tools-preview
	$(RUSTUP) target add x86_64-unknown-linux-gnu
	$(RUSTUP) target add x86_64-unknown-uefi

