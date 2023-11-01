#!/bin/sh

qemu-system-x86_64 \
    -m 1G \
    -vga std \
    -drive if=pflash,format=raw,readonly=on,file=OVMF.fd \
    -drive format=raw,file=fat:rw:mnt \
    -device nec-usb-xhci,id=xhci \
    -device usb-mouse -device usb-kbd \
    -monitor stdio \
    -display default
    # -vnc :0
