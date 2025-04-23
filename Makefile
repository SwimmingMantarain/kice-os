.PHONY: default target run clean

default: target
target: target/kernel.bin

default: run
run: target/kice-os.iso
	qemu-system-x86_64 -cdrom target/kice-os.iso

clean:
	cargo clean

cargo:
	cargo build --release -Z build-std=core,alloc

target/multiboot2.o: src/asm/multiboot2.asm
	mkdir -p target
	nasm -f elf64 src/asm/multiboot2.asm -o target/multiboot2.o

target/boot.o: src/asm/boot.asm
	mkdir -p target
	nasm -f elf64 src/asm/boot.asm -o target/boot.o

target/kernel.bin: target/multiboot2.o target/boot.o  src/asm/linker.ld cargo
	ld -n -o target/kernel.bin -T src/asm/linker.ld target/multiboot2.o target/boot.o  target/x86_64-unknown-kiceos-gnu/release/libkice_os.a

target/kice-os.iso: target/kernel.bin src/asm/grub.cfg
	mkdir -p target/isofiles/boot/grub
	cp src/asm/grub.cfg target/isofiles/boot/grub
	cp target/kernel.bin target/isofiles/boot/
	grub-mkrescue -o target/kice-os.iso target/isofiles
