AS=nasm -f elf64
DIR=platforms
x86_64:
	mkdir -p libs/boot
	$(AS) $(DIR)/x86_64/multiboot_header.s -o libs/boot/multiboot_header.o
	$(AS) $(DIR)/x86_64/boot.s -o libs/boot/boot.o
	$(AS) $(DIR)/x86_64/long_mode_start.s -o libs/boot/long_mode_start.o
	$(AS) $(DIR)/x86_64/port.s -o libs/port.o
	ar rcs libs/libport.a libs/port.o
	cargo build --release
	ld -L libs/boot --gc-sections -n -o isofiles/boot/kernel.bin -T linker.ld\
		libs/boot/long_mode_start.o\
		libs/boot/multiboot_header.o\
		libs/boot/boot.o\
		target/release/libalchemy.a

iso:
	grub-mkrescue -o alchemy.iso isofiles

doc:
	cargo doc