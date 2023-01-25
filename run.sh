#!/bin/sh

mkdir -p mnt/EFI/BOOT

cp "$@" mnt/EFI/BOOT/BOOTX64.EFI

qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:mnt
