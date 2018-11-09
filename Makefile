BUILD_DIR := ./dist

asm:	$(BUILD_DIR_i686)/ipl.bin \
 	$(BUILD_DIR_i686)/secondboot.bin


$(BUILD_DIR_i686)/ipl.bin: ./kernel/asm/ipl.asm
	nasm -f bin -o $(BUILD_DIR)/ipl.bin ./kernel/asm/ipl.asm -l $(BUILD_DIR)/ipl.lst

$(BUILD_DIR_i686)/secondboot.bin: ./kernel/asm/secondboot.asm
	nasm -f bin -o $(BUILD_DIR)/secondboot.bin ./kernel/asm/secondboot.asm -l $(BUILD_DIR)/secondboot.lst

rustcomp:	rustc --target=i686-unknown-linux-gnu --crate-type=staticlib --emit=obj -C lto -C opt-level=2 -C no-prepopulate-passes -C no-stack-check -Z verbose -Z no-landing-pads -o ./dist/lib.o ./kernel/src/lib.rs --extern core=libcore.rlib
