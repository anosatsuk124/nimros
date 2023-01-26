bootloader_dir = bootloader/
kernel_dir = kernel/
dist_dir = dist/
mnt_dir = mnt/

all:	kernel-build bootloader-build

run:	all
	./run.sh $(dist_dir)/bootloader.efi $(dist_dir)/kernel

clean:
	cargo clean
	rm -rf $(dist_dir) $(mnt_dir)

bootloader-build:
	mkdir -p $(dist_dir)
	docker compose run bootloader
#	cd $(bootloader_dir); \
#	cargo build -Z unstable-options --out-dir ../$(dist_dir)

kernel-build:
	mkdir -p $(dist_dir)
	docker compose run kernel
#	cd $(kernel_dir); \
#	cargo build -Z unstable-options --out-dir ../$(dist_dir)
