BUILD_DIR := ./dist
TARGET_ARCH_i686 := i686-unknown-linux-gnu
BUILD_NAME := shos-$(TARGET_ARCH_i686)
QEMU_ARCH_i686 := i386

BUILD_MODE=debug

asm:	$(BUILD_DIR)/ipl.bin \
 	$(BUILD_DIR)/secondboot.bin

# make image file
$(BUILD_DIR)/$(BUILD_NAME).img: $(BUILD_DIR)/ipl.bin $(BUILD_DIR)/$(BUILD_NAME).sys Makefile
	mformat -f 1440 -C -B $(BUILD_DIR)/ipl.bin -i $(BUILD_DIR)/$(BUILD_NAME).img ::
	mcopy -i $(BUILD_DIR)/$(BUILD_NAME).img $(BUILD_DIR)/$(BUILD_NAME).sys ::

$(BUILD_DIR)/$(BUILD_NAME).sys: $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/secondboot.bin
	cat $(BUILD_DIR)/secondboot.bin $(BUILD_DIR)/kernel.bin > $(BUILD_DIR)/$(BUILD_NAME).sys

$(BUILD_DIR)/kernel.bin: ./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/libshos.a ./kernel/asm/kernel.ld
	$(TARGET_ARCH_i686)-ld --gc-sections -t -nostdlib -Tdata=0x00310000 -T ./kernel/asm/kernel.ld -o $(BUILD_DIR)/kernel.bin --library-path=./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE) -lshos -Map $(BUILD_DIR)/kernel.map

$(BUILD_DIR)/ipl.bin: ./kernel/asm/ipl.asm
	nasm -f bin -o $(BUILD_DIR)/ipl.bin ./kernel/asm/ipl.asm -l $(BUILD_DIR)/ipl.lst

$(BUILD_DIR)/secondboot.bin: ./kernel/asm/secondboot.asm
	nasm -f bin -o $(BUILD_DIR)/secondboot.bin ./kernel/asm/secondboot.asm -l $(BUILD_DIR)/secondboot.lst

#kernel
./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/libshos.a:	$(TARGET_ARCH_i686)-rust.json ./kernel/Cargo.toml ./kernel/src/*.rs
	RUST_TARGET_PATH=$(pwd)	rustup run nightly `which cargo` build -v --target=$(TARGET_ARCH_i686) --manifest-path ./kernel/Cargo.toml

qemu:
	qemu-system-$(QEMU_ARCH_i686) -m 32 -localtime -vga std -fda $(BUILD_DIR)/$(BUILD_NAME).img -monitor stdio

clean:
	rm -rf ./dist/*
	rm -rf ./kenel/target

od:
	od $(BUILD_DIR)/$(BUILD_NAME).img -t x1z -A x