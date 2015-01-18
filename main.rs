#![allow(improper_ctypes)]
#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(int_uint)] // update fail_bounds_check

//extern crate libc

#![crate_name="blinky"]

use zero::std_types::*;
use libarm::stm32f4xx::*;

mod zero {
	pub mod std_types;
	pub mod zero;
}

#[macro_use]
mod libarm {
	#[macro_use] pub mod stm32f4xx;
	#[macro_use] pub mod stm32f4xx_gpio;
	#[macro_use] pub mod stm32f4xx_rcc;
}

const LED_GREEN :uint = 12;
const LED_ORANGE :uint = 13;
const LED_RED :uint = 14;
const LED_BLUE :uint = 15;

static LED :uint = LED_RED;

#[no_mangle]
pub extern "C" fn TIM2_IRQHandler() {
	let TIM2 = TIM2();
	let GPIOD = GPIOD();
	let toggle_led = 1 << LED;

	// flash on update event
	if TIM2.SR & TIM_SR_UIF!() > 0 {
		GPIOD.ODR ^= toggle_led;
	}
   
	TIM2.SR = 0x0; // reset the status register
}


#[no_mangle]
pub extern "C" fn main()
{
	let pin = LED;
	let RCC = RCC();
	let GPIOD = GPIOD();
	let TIM2 = TIM2();
	let NVIC = NVIC();

	let mode = GPIO_Mode_OUT!() << (pin * 2);
	let speed = GPIO_Speed_100MHz!() << (pin * 2);
	let otype = GPIO_OType_PP!() << pin;
	let pullup = GPIO_PuPd_NOPULL!() << (pin * 2);
	let irq_en = 1 << (TIM2_IRQn!() as uint);

	RCC.AHB1ENR |= RCC_AHB1ENR_GPIODEN!(); // enable the clock to GPIOD
	RCC.APB1ENR |= RCC_APB1ENR_TIM2EN!(); // enable TIM2 clock
    
	//
	//  Initilaise x`the GPIO port.
	//
	GPIOD.MODER |= mode;
	GPIOD.OSPEEDR |= speed;
	GPIOD.OTYPER |= otype;
	GPIOD.PUPDR |= pullup;
    
	NVIC.ISER[0] |= irq_en; // enable the TIM2 IRQ
    
	TIM2.PSC = 0xFFFF; // max prescaler
	TIM2.DIER |= TIM_DIER_UIE!(); // enable update interrupt
	TIM2.ARR = 0xFF; // count to 255 (autoreload value 255)
	TIM2.CR1 |= TIM_CR1_ARPE!() | TIM_CR1_CEN!(); // autoreload on, counter enabled
	TIM2.EGR = 1; // trigger update event to reload timer registers
     
	loop {}
}

