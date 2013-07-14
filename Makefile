# Put your stlink folder here so make burn will work.
STLINK=~/stlink.git
RUSTC=/opt/armrust/bin/rustc

# Put your source files here (or *.c, etc)
SRCS=zero/zero.c sys/system_stm32f4xx.c

# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=blinky

# Normally you shouldn't need to change anything below this line!
#######################################################################################

CC=arm-none-eabi-gcc
OBJCOPY=arm-none-eabi-objcopy

CFLAGS  = -g -O0 -Wall -Tsys/stm32_flash.ld 
CFLAGS += -mlittle-endian -mthumb -mcpu=cortex-m4
CFLAGS += -mfloat-abi=hard -mfpu=fpv4-sp-d16
CFLAGS += -Isys/inc -Isys/inc/core

# add startup file to build
SRCS += sys/startup_stm32f4xx.s 
OBJS = $(SRCS:.c=.o)

.PHONY: proj

all: clean proj

proj: $(PROJ_NAME).elf

main.s: main.rs
	$(RUSTC) --target arm-linux-noeabi --lib -c main.rs -S -o main.ll --emit-llvm -A non-uppercase-statics -A unused-imports
	sed -i 's/fixedstacksegment //g' main.ll
	sed -i 's/arm-unknown-linux-gnueabihf/arm-none-eabi/g' main.ll
	llc-3.4 -march=thumb -mattr=+thumb2 -mcpu=cortex-m4 --float-abi=hard -asm-verbose main.ll -o=main.s
	sed -i 's/.note.rustc,"aw"/.note.rustc,"a"/g' main.s

$(PROJ_NAME).elf: $(SRCS) main.s
	$(CC) $(CFLAGS) $^ -o $@ 
	$(OBJCOPY) -O ihex $(PROJ_NAME).elf $(PROJ_NAME).hex
	$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin

clean:
	rm -f *.o $(PROJ_NAME).elf $(PROJ_NAME).hex $(PROJ_NAME).bin main.s main.ll

# Flash the STM32F4
burn: proj
	$(STLINK)/st-flash write $(PROJ_NAME).bin 0x8000000
