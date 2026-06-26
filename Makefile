KERNEL_NAME := kernel
TARGET := target.json
PROFILE := debug

ASM := nasm
RUST := cargo
LD := ld.lld

BUILD := build
ISO := $(BUILD)/iso

BOOT_OBJ := $(BUILD)/boot.o
KERNEL_BIN := target/$(TARGET)/$(PROFILE)/$(KERNEL_NAME)
KERNEL_ELF := $(BUILD)/kernel.elf
ISO_FILE := $(BUILD)/myos.iso

.PHONY: all run iso clean

all: iso

$(BUILD):
	mkdir -p $(BUILD)

$(BOOT_OBJ): kernel/boot.asm | $(BUILD)
	$(ASM) -f elf32 $< -o $@

$(KERNEL_BIN):
	$(RUST) build --target $(TARGET)

$(KERNEL_ELF): $(BOOT_OBJ) $(KERNEL_BIN)
	$(LD) \
		-T kernel/linker.ld \
		$(BOOT_OBJ) \
		$(KERNEL_BIN) \
		-o $@

iso: $(KERNEL_ELF)
	rm -rf $(ISO)
	mkdir -p $(ISO)/boot/grub

	cp $(KERNEL_ELF) $(ISO)/boot/kernel.elf
	cp iso/boot/grub/grub.cfg $(ISO)/boot/grub/

	grub-mkrescue -o $(ISO_FILE) $(ISO)

run: iso
	qemu-system-i386 \
		-cdrom $(ISO_FILE)

clean:
	cargo clean
	rm -rf $(BUILD)