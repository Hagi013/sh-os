BUILD_DIR := ./dist
TARGET_ARCH_i686 := i686-unknown-linux-gnu
BUILD_NAME := shos-$(TARGET_ARCH_i686)

BUILD_MODE=debug

asm:	$(BUILD_DIR_i686)/ipl.bin \
 	$(BUILD_DIR_i686)/secondboot.bin

# make image file
$(BUILD_DIR_i686)/$(BUILD_NAME).img: $(BUILD_DIR)/ipl.bin $(BUILD_DIR_i686)/$(BUILD_NAME).sys Makefile
	mformat -f 1440 -C -B $(BUILD_DIR_i686)/ipl.bin -i $(BUILD_DIR_i686)/$(BUILD_NAME).img ::
	mcopy -i $(BUILD_DIR_i686)/$(BUILD_NAME).img $(BUILD_DIR_i686)/$(BUILD_NAME).sys ::

$(BUILD_DIR_i686)/$(BUILD_NAME).sys: $(BUILD_DIR_i686)/kernel.bin $(BUILD_DIR_i686)/secondboot.bin
	cat $(BUILD_DIR_i686)/secondboot.bin $(BUILD_DIR_i686)/kernel.bin > $(BUILD_DIR_i686)/$(BUILD_NAME).sys

$(BUILD_DIR_i686)/kernel.bin: ./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/libshos.a ./kernel/asm/kernel.ld
	$(TARGET_ARCH_i686)-ld --gc-sections -t -nostdlib -Tdata=0x00310000 -T ./kernel/asm/kernel.ld -o $(BUILD_DIR)/kernel.bin --library-path=./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE) -lshos -Map $(BUILD_DIR)/kernel.map

$(BUILD_DIR_i686)/ipl.bin: ./kernel/asm/ipl.asm
	nasm -f bin -o $(BUILD_DIR)/ipl.bin ./kernel/asm/ipl.asm -l $(BUILD_DIR)/ipl.lst

$(BUILD_DIR_i686)/secondboot.bin: ./kernel/asm/secondboot.asm
	nasm -f bin -o $(BUILD_DIR)/secondboot.bin ./kernel/asm/secondboot.asm -l $(BUILD_DIR)/secondboot.lst

#kernel
$(BUILD_DIR_i686)/libshos.a:	$(TARGET_ARCH_i686)-rust.json ./kernel/Cargo.toml ./kernel/src/*.rs
	RUST_TARGET_PATH=$(pwd)	rustup run nightly `which cargo` build -v --target=$(TARGET_ARCH_i686) --manifest-path ./kernel/Cargo.toml
