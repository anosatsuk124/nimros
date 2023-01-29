bootloader_dir = bootloader/
kernel_dir = kernel/
mikanlib_dir = mikanlib/
dist_dir = dist/
mnt_dir = mnt/

all:	kernel-build bootloader-build

run:	all
	./run.sh $(dist_dir)/bootloader.efi $(dist_dir)/kernel

clean:
	cargo clean
	rm -rf $(dist_dir) $(mnt_dir)

clippy:
	cd $(bootloader_dir); \
	cargo clippy
	cd $(kernel_dir); \
	cargo clippy
	cd $(mikanlib_dir); \
	cargo clippy

bootloader-build:
	mkdir -p $(dist_dir)
	docker compose run bootloader

kernel-build:
	mkdir -p $(dist_dir)
	docker compose run kernel
