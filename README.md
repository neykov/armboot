armboot
=======
A template project for using the Rust language on bare metal STM32F4xx ARM microcontrollers.
Tested on STM32F4DISCOVERY.

The current version supports only static memory allocation and no stack pointer safety checks.

Requirements:
-------------
  * arm-none-eabi toolchain
  * llvm-3.4 toolchain
  * rustc (tested with 0.7) with the patch at https://raw.github.com/neykov/armboot/master/rustc.patch applied (works even when compiled only for x86 target)

Compiling:
----------

Edit "Makefile" with the path to the patched rustc compiler and type "make".

Upload the resulting binary (one of blinky.{elf, bin, hex}) on the target.

Structure
---------
  * main.rs - sample program (blinks the red led of the STM32F4DISCOVERY board)
  * libarm/ - microcontroller related definitions
  * sys/ - bootstrap code (boot loader and system initialization)
  * zero/ - zero.rs and additional C stubs

Credits
-------
  * The C source for the rust code: http://jeremyherbert.net/get/stm32f4_getting_started
  * The project structure and build script: https://github.com/rowol/stm32_discovery_arm_gcc
  * Rust zero.rs: https://github.com/pcwalton/zero.rs

