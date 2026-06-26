KERNEL=kernel
BUILD=build
ISO=iso
ELF=$(BUILD)/kernel.elf
ISOFILE=$(BUILD)/DogOS.iso

all: iso

$(BUILD):
	mkdir -p $(BUILD)

boot.o: boot.asm | $(BUILD)
	nasm -f elf32 boot.asm -o $(BUILD)/boot.o

kernel:
	cargo build --manifest-path kernel/Cargo.toml

$(ELF): boot.o kernel
	ld.lld -T linker.ld \
		$(BUILD)/boot.o \
		target/release/libkernel.a \
		-o $(ELF)

iso: $(ELF)
	rm -rf $(ISO)
	mkdir -p $(ISO)/boot/grub

	cp $(ELF) $(ISO)/boot/kernel.elf
	cp grub.cfg $(ISO)/boot/grub/

	grub-mkrescue -o $(ISOFILE) $(ISO)

run: iso
	qemu-system-i386 -cdrom $(ISOFILE)

clean:
	cargo clean
	rm -rf $(BUILD) $(ISO)