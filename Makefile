AS=nasm -f elf64
build:
	mkdir -p isofiles/boot
	cargo build --release
	$(AS) multiboot_header.s
	$(AS) boot.s
	$(AS) long_mode_start.s
	ld --gc-sections -n -o isofiles/boot/kernel.bin -T linker.ld long_mode_start.o multiboot_header.o boot.o target/release/libalchemy.a

iso:
	mkdir -p isofiles/boot
	grub-mkrescue -o alchemy.iso isofiles
