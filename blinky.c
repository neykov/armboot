//source: http://jeremyherbert.net/get/stm32f4_getting_started
//Only for comparison, not used in the project build.

#include "stm32f4xx_conf.h"
 
void TIM2_IRQHandler(void) {
  // flash on update event
  if (TIM2->SR & TIM_SR_UIF) GPIOD->ODR ^= (1 << 13);
   
  TIM2->SR = 0x0; // reset the status register
}
 
int main(void)
{
    RCC->AHB1ENR |= RCC_AHB1ENR_GPIODEN; // enable the clock to GPIOD
    RCC->APB1ENR |= RCC_APB1ENR_TIM2EN; // enable TIM2 clock
     
    GPIOD->MODER = (1 << 26); // set pin 13 to be general purpose output
     
    NVIC->ISER[0] |= 1<< (TIM2_IRQn); // enable the TIM2 IRQ
     
    TIM2->PSC = 0x0; // no prescaler, timer counts up in sync with the peripheral clock
    TIM2->DIER |= TIM_DIER_UIE; // enable update interrupt
    TIM2->ARR = 0x01; // count to 1 (autoreload value 1)
    TIM2->CR1 |= TIM_CR1_ARPE | TIM_CR1_CEN; // autoreload on, counter enabled
    TIM2->EGR = 1; // trigger update event to reload timer registers
     
    while (1);
}

