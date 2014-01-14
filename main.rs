#[allow(ctypes)];
#[no_std];
#[feature(macro_rules)];
#[feature(globs)];

#[crate_id="blinky#0.3"];

use zero::std_types::*;
use libarm::stm32f4xx::*;

mod zero {
	pub mod std_types;
	pub mod zero;
}

mod libarm {
	#[macro_escape];
	pub mod stm32f4xx;
	pub mod stm32f4xx_gpio;
	pub mod stm32f4xx_rcc;
}

static LED_GREEN :u32 = 12;
static LED_ORANGE :u32 = 13;
static LED_RED :u32 = 14;
static LED_BLUE :u32 = 15;

static LED :u32 = LED_RED;

#[no_mangle]
pub extern "C" fn TIM2_IRQHandler() {
	let TIM2 = TIM2();
	let GPIOD = GPIOD();
	let toggle_led = (1 << LED);

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
	let irq_en = 1 << (TIM2_IRQn!());

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

