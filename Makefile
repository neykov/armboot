# Put your stlink folder here so make burn will work.
STLINK=~/stlink.git
RUSTC=/opt/rust-0.9/bin/rustc
LLC=llc-3.5

# Put your source files here (or *.c, etc)
SRCS=sys/system_stm32f4xx.c

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

blinky.s: main.rs
	$(RUSTC) --target arm-linux-eabi --lib -c main.rs -S --emit-llvm -A non-uppercase-statics -A unused-imports -A dead_code
	$(LLC) -mtriple arm-none-eabi -march=thumb -mattr=+thumb2 -mcpu=cortex-m4 --float-abi=hard --asm-verbose=false blinky.ll -o=blinky.s
	sed -i 's/.note.rustc,"aw"/.note.rustc,"a"/g' blinky.s

$(PROJ_NAME).elf: $(SRCS) blinky.s
	$(CC) $(CFLAGS) $^ -o $@ 
	$(OBJCOPY) -O ihex $(PROJ_NAME).elf $(PROJ_NAME).hex
	$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin

clean:
	rm -f *.o $(PROJ_NAME).elf $(PROJ_NAME).hex $(PROJ_NAME).bin $(PROJ_NAME).s $(PROJ_NAME).ll

# Flash the STM32F4
burn: proj
	$(STLINK)/st-flash write $(PROJ_NAME).bin 0x8000000
