AS=nasm -f elf64
build:
	cargo build --release
	$(AS) multiboot_header.s
	$(AS) boot.s
	ld -n -o isofiles/boot/kernel.bin -T linker.ld multiboot_header.o boot.o target/release/libalchemy.a

iso:
	grub-mkrescue -o alchemy.iso isofiles
