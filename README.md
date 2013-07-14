armboot
=======
A template project for using the Rust language on STM32F4xx ARM microcontrollers.

Requirements:
-------------
  * arm-none-eabi toolchain
  * llvm-3.4 toolchain
  * rustc with the patch at https://raw.github.com/neykov/armboot/master/rustc.patch applied (works even when compiled only for x86 target)

Compiling:
----------

Edit "Makefile" with the path to the patched rustc compiler and type "make".

Upload the resulting binary (one of blinky.{elf, bin, hex}) on the target.

