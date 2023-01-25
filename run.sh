#!/bin/sh

mkdir -p mnt/EFI/BOOT

cp "$1" mnt/EFI/BOOT/BOOTX64.EFI
cp "$2" mnt/

qemu-system-x86_64 -bios OVMF.fd -drive format=raw,file=fat:rw:mnt -monitor stdio
