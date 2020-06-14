armboot
=======

A testbed project used for porting the Rust language to bare metal STM32F4xx ARM microcontrollers.
Tested on STM32F4DISCOVERY.

The project resulted in patches to the [Rust compiler](https://github.com/rust-lang/llvm/pull/4
) and eventually to [LLVM](https://github.com/llvm-mirror/llvm/commit/af48fc4136b1fca9b0542bc858a3bddbf87dcc02
) to add support for segmented stacks to the platform.

The current version supports only static memory allocation and no stack pointer safety checks.

Requirements:
-------------
  * arm-none-eabi toolchain
  * llvm-3.6 (nightly) toolchain
  * rustc (requires Rust 1.0.0-alpha2)

Compiling:
----------

Edit "Makefile" with the path to the rustc compiler and type "make".

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

