AS=nasm -f elf64
build:
	mkdir -p isofiles/boot
	$(AS) multiboot_header.s
	$(AS) boot.s
	ld -n -o isofiles/boot/kernel.bin -T linker.ld multiboot_header.o boot.o

iso:
	mkdir -p isofiles/boot
	grub-mkrescue -o alchemy.iso isofiles
