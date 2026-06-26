KERNEL_NAME := kernel

TARGET := i686-dogos.json
TARGET_NAME := i686-dogos

PROFILE := debug

CARGO := cargo +nightly
ASM := nasm
LD := ld.lld

BUILD := build
ISO := $(BUILD)/iso

BOOT_OBJ := $(BUILD)/boot.o
KERNEL_ELF := $(BUILD)/kernel.elf
ISO_FILE := $(BUILD)/DogOS.iso

KERNEL_BIN := kernel/target/$(TARGET_NAME)/$(PROFILE)/$(KERNEL_NAME)

.PHONY: all iso run clean

all: iso

$(BUILD):
	mkdir -p $(BUILD)

$(BOOT_OBJ): kernel/boot.asm | $(BUILD)
	$(ASM) -f elf32 $< -o $@

$(KERNEL_BIN):
	$(CARGO) build \
		-Zbuild-std=core,compiler_builtins \
		-Zbuild-std-features=compiler-builtins-mem \
		-Zjson-target-spec \
		--manifest-path kernel/Cargo.toml \
		--target $(TARGET)

$(KERNEL_ELF): $(BOOT_OBJ) $(KERNEL_BIN)
	$(LD) \
		-T kernel/linker.ld \
		$(BOOT_OBJ) \
		$(KERNEL_BIN) \
		-o $(KERNEL_ELF)

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
	$(CARGO) clean --manifest-path kernel/Cargo.toml
	rm -rf $(BUILD)