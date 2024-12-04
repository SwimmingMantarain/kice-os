#!/bin/bash

cargo build
mkdir -p iso/boot/grub
cp target/x86_64-none-bare_metal/debug/Kice-OS iso/boot/kernel.elf
grub-mkrescue -o kice-os.iso iso