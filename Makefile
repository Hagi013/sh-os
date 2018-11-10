BUILD_DIR := ./dist
TARGET_ARCH_i686 := i686-unknown-linux-gnu

asm:	$(BUILD_DIR_i686)/ipl.bin \
 	$(BUILD_DIR_i686)/secondboot.bin


$(BUILD_DIR_i686)/ipl.bin: ./kernel/asm/ipl.asm
	nasm -f bin -o $(BUILD_DIR)/ipl.bin ./kernel/asm/ipl.asm -l $(BUILD_DIR)/ipl.lst

$(BUILD_DIR_i686)/secondboot.bin: ./kernel/asm/secondboot.asm
	nasm -f bin -o $(BUILD_DIR)/secondboot.bin ./kernel/asm/secondboot.asm -l $(BUILD_DIR)/secondboot.lst

#kernel
$(BUILD_DIR_i686)/libshos.a:	$(TARGET_ARCH_i686)-rust.json ./kernel/Cargo.toml ./kernel/src/*.rs
	RUST_TARGET_PATH=$(pwd)	rustup run nightly `which cargo` build -v --target=$(TARGET_ARCH_i686)-rust --manifest-path ./kernel/Cargo.toml
