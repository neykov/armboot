# Binaries will be generated with this name (.elf, .bin, .hex, etc)
PROJ_NAME=blinky

# Put your stlink folder here so make burn will work.
STFLASH=st-flash
RUSTC=rustc
LLC=llc-3.6

CC=arm-none-eabi-gcc
AR=arm-none-eabi-ar
OBJCOPY=arm-none-eabi-objcopy

# Put your source files here (or *.c, etc)
SRCS=sys/system_stm32f4xx.c

# add startup file to build
SRCS += sys/startup_stm32f4xx.s

OBJS = $(SRCS:.c=.o)
OBJS := $(OBJS:.s=.o)

AFLAGS  = -mthumb -mcpu=cortex-m4 -mfloat-abi=hard -mfpu=fpv4-sp-d16

CFLAGS  = -g -O0 -Wall
CFLAGS += -mlittle-endian $(AFLAGS)
CFLAGS += -Isys/inc -Isys/inc/core

LDFLAGS = -Tsys/stm32_flash.ld $(AFLAGS)

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@

.s.o:
	$(CC) $(CFLAGS) -c $< -o $@

.PHONY: proj

all: proj

proj: $(PROJ_NAME).hex $(PROJ_NAME).bin

libcompiler-rt.a: $(OBJS)
	$(AR) rcs $@ $(OBJS)

blinky.elf: main.rs libcompiler-rt.a
	$(RUSTC) --target target.json -A non_camel_case_types -A dead_code -A non_snake_case main.rs -C link-args="$(LDFLAGS)" -L. -Z no-landing-pads -o $@ -Z print-link-args

$(PROJ_NAME).hex: blinky.elf
	$(OBJCOPY) -O ihex $(PROJ_NAME).elf $(PROJ_NAME).hex

$(PROJ_NAME).bin: blinky.elf
	$(OBJCOPY) -O binary $(PROJ_NAME).elf $(PROJ_NAME).bin

clean:
	rm -f *.o *.a sys/*.o $(PROJ_NAME).elf $(PROJ_NAME).hex $(PROJ_NAME).bin

burn: $(PROJ_NAME).bin
	$(STFLASH) write $(PROJ_NAME).bin 0x8000000

