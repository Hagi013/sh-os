BUILD_DIR := ./dist
KERNEL_DIR := ./kernel
TARGET_ARCH_i686 := i686-unknown-linux-gnu
BUILD_NAME := shos-$(TARGET_ARCH_i686)
QEMU_ARCH_i686 := i386

BUILD_MODE=debug
#BUILD_MODE=release

DEBUG := -S -gdb tcp::9000

asm:	$(BUILD_DIR)/ipl.bin \
 	$(BUILD_DIR)/secondboot.bin

# make image file
$(BUILD_DIR)/$(BUILD_NAME).img: $(BUILD_DIR)/ipl.bin $(BUILD_DIR)/$(BUILD_NAME).sys Makefile
	mformat -f 1440 -C -B $(BUILD_DIR)/ipl.bin -i $(BUILD_DIR)/$(BUILD_NAME).img ::
	mcopy -i $(BUILD_DIR)/$(BUILD_NAME).img $(BUILD_DIR)/$(BUILD_NAME).sys ::

$(BUILD_DIR)/$(BUILD_NAME).sys: $(BUILD_DIR)/kernel.bin $(BUILD_DIR)/secondboot.bin
	cat $(BUILD_DIR)/secondboot.bin $(BUILD_DIR)/kernel.bin > $(BUILD_DIR)/$(BUILD_NAME).sys

$(BUILD_DIR)/kernel.bin: ./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/libshos.a ./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/asmfunc.o kernel/boot
#	$(TARGET_ARCH_i686)-ld --gc-sections -t -nostdlib -Tdata=0x00310000 -T ./kernel/boot/kernel.ld -o $(BUILD_DIR)/kernel.bin --library-path=./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE) -lshos -Map $(BUILD_DIR)/kernel.map -noinhibit-exec
	$(TARGET_ARCH_i686)-ld --print-gc-sections --gc-sections -t -nostdlib -Tdata=0x00310000 -T ./kernel/boot/kernel.ld -o $(BUILD_DIR)/kernel.bin ./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/asmfunc.o --library-path=./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE) -lshos -Map $(BUILD_DIR)/kernel.map --verbose

$(BUILD_DIR)/ipl.bin: kernel/boot
	nasm -f bin -o $(BUILD_DIR)/ipl.bin ./kernel/boot/ipl.asm -l $(BUILD_DIR)/ipl.lst

$(BUILD_DIR)/secondboot.bin: kernel/boot
	nasm -f bin -o $(BUILD_DIR)/secondboot.bin ./kernel/boot/secondboot.asm -l $(BUILD_DIR)/secondboot.lst

#kernel
./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/libshos.a: ./kernel/$(TARGET_ARCH_i686).json ./kernel/Cargo.toml ./kernel/src/*.rs
	# cd ${KERNEL_DIR}; RUSTFLAGS="-Z pre-link-arg=-fno-PIC" rustup run nightly `which cargo` build --target $(TARGET_ARCH_i686) --release -v
	# cd ${KERNEL_DIR}; RUST_TARGET_PATH=$(PWD); RUST_BACKTRACE=1; rustup run nightly `which cargo` xbuild --target $(TARGET_ARCH_i686).json --release -v
	cd ${KERNEL_DIR}; RUST_TARGET_PATH=$(PWD); set RUST_BACKTRACE=1; rustup run nightly `which cargo` xbuild --target $(TARGET_ARCH_i686).json -v

./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/%.o: ./kernel/boot/asmfunc.asm
	nasm -f elf32 ./kernel/boot/asmfunc.asm -o ./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/$*.o -l ./kernel/target/$(TARGET_ARCH_i686)/$(BUILD_MODE)/$*.lst

qemu:
	qemu-system-$(QEMU_ARCH_i686) -m 32 -localtime -vga std -fda $(BUILD_DIR)/$(BUILD_NAME).img -monitor stdio $(DEBUG)

clean:
	rm -rf ./dist/*
	rm -rf ./target
	cd ./kernel && cargo clean
	cd ./kernel && xargo clean

od:
	od $(BUILD_DIR)/$(BUILD_NAME).img -t x1z -A x