use zero::std_types::*;

/* STM32F4xx_StdPeriph_Driver
  */

/* GPIO
  */ 

/* Exported types ------------------------------------------------------------*/
macro_rules! IS_GPIO_ALL_PERIPH {($PERIPH:ident) => ((($PERIPH) == GPIOA) ||
                                                     (($PERIPH) == GPIOB) || 
                                                     (($PERIPH) == GPIOC) ||
                                                     (($PERIPH) == GPIOD) ||
                                                     (($PERIPH) == GPIOE) ||
                                                     (($PERIPH) == GPIOF) ||
                                                     (($PERIPH) == GPIOG) ||
                                                     (($PERIPH) == GPIOH) ||
                                                     (($PERIPH) == GPIOI));}
                                                                
/* 
  * GPIO Configuration Mode enumeration 
  */   
pub type GPIOMode_TypeDef = c_uint;
macro_rules! GPIO_Mode_IN {() => (0x00u32 as c_uint);} /*< GPIO Input Mode */
macro_rules! GPIO_Mode_OUT {() => (0x01u32 as c_uint);} /*< GPIO Output Mode */
macro_rules! GPIO_Mode_AF {() => (0x02u32 as c_uint);} /*< GPIO Alternate function Mode */
macro_rules! GPIO_Mode_AN {() => (0x03u32 as c_uint);}  /*< GPIO Analog Mode */

macro_rules! IS_GPIO_MODE {($MODE:ident) => ((($MODE) == GPIO_Mode_IN!())  || (($MODE) == GPIO_Mode_OUT!()) ||
                                             (($MODE) == GPIO_Mode_AF!())|| (($MODE) == GPIO_Mode_AN!()));}

/* 
  * GPIO Output type enumeration 
  */  
pub type GPIOOType_TypeDef = c_uint;
macro_rules! GPIO_OType_PP {() => (0x00u32 as c_uint);}
macro_rules! GPIO_OType_OD {() => (0x01u32 as c_uint);}
macro_rules! IS_GPIO_OTYPE {($OTYPE:ident) => ((($OTYPE) == GPIO_OType_PP!()) || (($OTYPE) == GPIO_OType_OD!()));}


/* 
  * GPIO Output Maximum frequency enumeration 
  */  
pub type GPIOSpeed_TypeDef = c_uint;
macro_rules! GPIO_Speed_2MHz {() => (0x00u32 as c_uint);} /*< Low speed */
macro_rules! GPIO_Speed_25MHz {() => (0x01u32 as c_uint);} /*< Medium speed */
macro_rules! GPIO_Speed_50MHz {() => (0x02u32 as c_uint);} /*< Fast speed */
macro_rules! GPIO_Speed_100MHz {() => (0x03u32 as c_uint);}  /*< High speed on 30 pF (80 MHz Output max speed on 15 pF) */
macro_rules! IS_GPIO_SPEED {($SPEED:ident) => ((($SPEED) == GPIO_Speed_2MHz!()) || (($SPEED) == GPIO_Speed_25MHz!()) ||
                                               (($SPEED) == GPIO_Speed_50MHz!())||  (($SPEED) == GPIO_Speed_100MHz!()));}

/* 
  * GPIO Configuration PullUp PullDown enumeration 
  */ 
pub type GPIOPuPd_TypeDef = c_uint;
macro_rules! GPIO_PuPd_NOPULL {() => (0x00u32 as c_uint);}
macro_rules! GPIO_PuPd_UP {() => (0x01u32 as c_uint);}
macro_rules! GPIO_PuPd_DOWN {() => (0x02u32 as c_uint);}
macro_rules! IS_GPIO_PUPD {($PUPD:ident) => ((($PUPD) == GPIO_PuPd_NOPULL!()) || (($PUPD) == GPIO_PuPd_UP!()) ||
                                             (($PUPD) == GPIO_PuPd_DOWN!()));}

/* 
  * GPIO Bit SET and Bit RESET enumeration 
  */ 
pub type BitAction = c_uint;
macro_rules! Bit_RESET {() => (0u32 as c_uint);}
macro_rules! Bit_SET {() => (1u32 as c_uint);}
macro_rules! IS_GPIO_BIT_ACTION {($ACTION:ident) => ((($ACTION) == Bit_RESET!()) || (($ACTION) == Bit_SET!()));}


/* 
  *  GPIO Init structure definition  
  */ 
pub struct GPIO_InitTypeDef {
  pub GPIO_Pin :uint32_t,              /*< Specifies the GPIO pins to be configured.
                                       This parameter can be any value of GPIO_pins_define */

  pub GPIO_Mode :GPIOMode_TypeDef,     /*< Specifies the operating mode for the selected pins.
                                       This parameter can be a value of GPIOMode_TypeDef */

  pub GPIO_Speed :GPIOSpeed_TypeDef,   /*< Specifies the speed for the selected pins.
                                       This parameter can be a value of GPIOSpeed_TypeDef */

  pub GPIO_OType :GPIOOType_TypeDef,   /*< Specifies the operating output type for the selected pins.
                                       This parameter can be a value of GPIOOType_TypeDef */

  pub GPIO_PuPd :GPIOPuPd_TypeDef     /*< Specifies the operating Pull-up/Pull down for the selected pins.
                                       This parameter can be a value of GPIOPuPd_TypeDef */
}

/* Exported constants --------------------------------------------------------*/

/* GPIO_Exported_Constants
  */ 

/* GPIO_pins_define 
  */ 
macro_rules! GPIO_Pin_0 {() =>                 (0x0001u16 as uint16_t);}  /* Pin 0 selected */
macro_rules! GPIO_Pin_1 {() =>                 (0x0002u16 as uint16_t);}  /* Pin 1 selected */
macro_rules! GPIO_Pin_2 {() =>                 (0x0004u16 as uint16_t);}  /* Pin 2 selected */
macro_rules! GPIO_Pin_3 {() =>                 (0x0008u16 as uint16_t);}  /* Pin 3 selected */
macro_rules! GPIO_Pin_4 {() =>                 (0x0010u16 as uint16_t);}  /* Pin 4 selected */
macro_rules! GPIO_Pin_5 {() =>                 (0x0020u16 as uint16_t);}  /* Pin 5 selected */
macro_rules! GPIO_Pin_6 {() =>                 (0x0040u16 as uint16_t);}  /* Pin 6 selected */
macro_rules! GPIO_Pin_7 {() =>                 (0x0080u16 as uint16_t);}  /* Pin 7 selected */
macro_rules! GPIO_Pin_8 {() =>                 (0x0100u16 as uint16_t);}  /* Pin 8 selected */
macro_rules! GPIO_Pin_9 {() =>                 (0x0200u16 as uint16_t);}  /* Pin 9 selected */
macro_rules! GPIO_Pin_10 {() =>                (0x0400u16 as uint16_t);}  /* Pin 10 selected */
macro_rules! GPIO_Pin_11 {() =>                (0x0800u16 as uint16_t);}  /* Pin 11 selected */
macro_rules! GPIO_Pin_12 {() =>                (0x1000u16 as uint16_t);}  /* Pin 12 selected */
macro_rules! GPIO_Pin_13 {() =>                (0x2000u16 as uint16_t);}  /* Pin 13 selected */
macro_rules! GPIO_Pin_14 {() =>                (0x4000u16 as uint16_t);}  /* Pin 14 selected */
macro_rules! GPIO_Pin_15 {() =>                (0x8000u16 as uint16_t);}  /* Pin 15 selected */
macro_rules! GPIO_Pin_All {() =>               (0xFFFFu16 as uint16_t);}  /* All pins selected */

macro_rules! IS_GPIO_PIN {($PIN:ident) => (((($PIN) & (0x00u16 as uint16_t)) == 0x00) && ((PIN) != (0x00u16 as uint16_t)));}
macro_rules! IS_GET_GPIO_PIN {($PIN:ident) => ((($PIN) == GPIO_Pin_0!()) ||
                                               (($PIN) == GPIO_Pin_1!()) ||
                                               (($PIN) == GPIO_Pin_2!()) || 
                                               (($PIN) == GPIO_Pin_3!()) ||
                                               (($PIN) == GPIO_Pin_4!()) ||
                                               (($PIN) == GPIO_Pin_5!()) ||
                                               (($PIN) == GPIO_Pin_6!()) ||
                                               (($PIN) == GPIO_Pin_7!()) ||
                                               (($PIN) == GPIO_Pin_8!()) ||
                                               (($PIN) == GPIO_Pin_9!()) ||
                                               (($PIN) == GPIO_Pin_10!()) ||
                                               (($PIN) == GPIO_Pin_11!()) ||
                                               (($PIN) == GPIO_Pin_12!()) ||
                                               (($PIN) == GPIO_Pin_13!()) ||
                                               (($PIN) == GPIO_Pin_14!()) ||
                                               (($PIN) == GPIO_Pin_15!()));}
/* GPIO_Pin_sources 
  */ 
macro_rules! GPIO_PinSource0 {() =>            (0x00u8 as uint8_t);}
macro_rules! GPIO_PinSource1 {() =>            (0x01u8 as uint8_t);}
macro_rules! GPIO_PinSource2 {() =>            (0x02u8 as uint8_t);}
macro_rules! GPIO_PinSource3 {() =>            (0x03u8 as uint8_t);}
macro_rules! GPIO_PinSource4 {() =>            (0x04u8 as uint8_t);}
macro_rules! GPIO_PinSource5 {() =>            (0x05u8 as uint8_t);}
macro_rules! GPIO_PinSource6 {() =>            (0x06u8 as uint8_t);}
macro_rules! GPIO_PinSource7 {() =>            (0x07u8 as uint8_t);}
macro_rules! GPIO_PinSource8 {() =>            (0x08u8 as uint8_t);}
macro_rules! GPIO_PinSource9 {() =>            (0x09u8 as uint8_t);}
macro_rules! GPIO_PinSource10 {() =>           (0x0Au8 as uint8_t);}
macro_rules! GPIO_PinSource11 {() =>           (0x0Bu8 as uint8_t);}
macro_rules! GPIO_PinSource12 {() =>           (0x0Cu8 as uint8_t);}
macro_rules! GPIO_PinSource13 {() =>           (0x0Du8 as uint8_t);}
macro_rules! GPIO_PinSource14 {() =>           (0x0Eu8 as uint8_t);}
macro_rules! GPIO_PinSource15 {() =>           (0x0Fu8 as uint8_t);}

macro_rules! IS_GPIO_PIN_SOURCE {($PINSOURCE:ident) => ((($PINSOURCE) == GPIO_PinSource0!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource1!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource2!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource3!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource4!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource5!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource6!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource7!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource8!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource9!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource10!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource11!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource12!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource13!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource14!()) ||
                                                        (($PINSOURCE) == GPIO_PinSource15!()));}
/* GPIO_Alternat_function_selection_define 
  */ 
/* 
  *  AF 0 selection  
  */ 
macro_rules! GPIO_AF_RTC_50Hz {() =>      (0x00u8 as uint8_t);}  /* RTC_50Hz Alternate Function mapping */
macro_rules! GPIO_AF_MCO {() =>           (0x00u8 as uint8_t);}  /* MCO (MCO1 and MCO2) Alternate Function mapping */
macro_rules! GPIO_AF_TAMPER {() =>        (0x00u8 as uint8_t);}  /* TAMPER (TAMPER_1 and TAMPER_2) Alternate Function mapping */
macro_rules! GPIO_AF_SWJ {() =>           (0x00u8 as uint8_t);}  /* SWJ (SWD and JTAG) Alternate Function mapping */
macro_rules! GPIO_AF_TRACE {() =>         (0x00u8 as uint8_t);}  /* TRACE Alternate Function mapping */

/* 
  *  AF 1 selection  
  */ 
macro_rules! GPIO_AF_TIM1 {() =>          (0x01u8 as uint8_t);}  /* TIM1 Alternate Function mapping */
macro_rules! GPIO_AF_TIM2 {() =>          (0x01u8 as uint8_t);}  /* TIM2 Alternate Function mapping */

/* 
  *  AF 2 selection  
  */ 
macro_rules! GPIO_AF_TIM3 {() =>          (0x02u8 as uint8_t);}  /* TIM3 Alternate Function mapping */
macro_rules! GPIO_AF_TIM4 {() =>          (0x02u8 as uint8_t);}  /* TIM4 Alternate Function mapping */
macro_rules! GPIO_AF_TIM5 {() =>          (0x02u8 as uint8_t);}  /* TIM5 Alternate Function mapping */

/* 
  *  AF 3 selection  
  */ 
macro_rules! GPIO_AF_TIM8 {() =>          (0x03u8 as uint8_t);}  /* TIM8 Alternate Function mapping */
macro_rules! GPIO_AF_TIM9 {() =>          (0x03u8 as uint8_t);}  /* TIM9 Alternate Function mapping */
macro_rules! GPIO_AF_TIM10 {() =>         (0x03u8 as uint8_t);}  /* TIM10 Alternate Function mapping */
macro_rules! GPIO_AF_TIM11 {() =>         (0x03u8 as uint8_t);}  /* TIM11 Alternate Function mapping */

/* 
  *  AF 4 selection  
  */ 
macro_rules! GPIO_AF_I2C1 {() =>          (0x04u8 as uint8_t);}  /* I2C1 Alternate Function mapping */
macro_rules! GPIO_AF_I2C2 {() =>          (0x04u8 as uint8_t);}  /* I2C2 Alternate Function mapping */
macro_rules! GPIO_AF_I2C3 {() =>          (0x04u8 as uint8_t);}  /* I2C3 Alternate Function mapping */

/* 
  *  AF 5 selection  
  */ 
macro_rules! GPIO_AF_SPI1 {() =>          (0x05u8 as uint8_t);}  /* SPI1 Alternate Function mapping */
macro_rules! GPIO_AF_SPI2 {() =>          (0x05u8 as uint8_t);}  /* SPI2/I2S2 Alternate Function mapping */

/* 
  *  AF 6 selection  
  */ 
macro_rules! GPIO_AF_SPI3 {() =>          (0x06u8 as uint8_t);}  /* SPI3/I2S3 Alternate Function mapping */

/* 
  *  AF 7 selection  
  */ 
macro_rules! GPIO_AF_USART1 {() =>        (0x07u8 as uint8_t);}  /* USART1 Alternate Function mapping */
macro_rules! GPIO_AF_USART2 {() =>        (0x07u8 as uint8_t);}  /* USART2 Alternate Function mapping */
macro_rules! GPIO_AF_USART3 {() =>        (0x07u8 as uint8_t);}  /* USART3 Alternate Function mapping */
macro_rules! GPIO_AF_I2S3ext {() =>       (0x07u8 as uint8_t);}  /* I2S3ext Alternate Function mapping */

/* 
  *  AF 8 selection  
  */ 
macro_rules! GPIO_AF_UART4 {() =>         (0x08u8 as uint8_t);}  /* UART4 Alternate Function mapping */
macro_rules! GPIO_AF_UART5 {() =>         (0x08u8 as uint8_t);}  /* UART5 Alternate Function mapping */
macro_rules! GPIO_AF_USART6 {() =>        (0x08u8 as uint8_t);}  /* USART6 Alternate Function mapping */

/* 
  *  AF 9 selection 
  */ 
macro_rules! GPIO_AF_CAN1 {() =>          (0x09u8 as uint8_t);}  /* CAN1 Alternate Function mapping */
macro_rules! GPIO_AF_CAN2 {() =>          (0x09u8 as uint8_t);}  /* CAN2 Alternate Function mapping */
macro_rules! GPIO_AF_TIM12 {() =>         (0x09u8 as uint8_t);}  /* TIM12 Alternate Function mapping */
macro_rules! GPIO_AF_TIM13 {() =>         (0x09u8 as uint8_t);}  /* TIM13 Alternate Function mapping */
macro_rules! GPIO_AF_TIM14 {() =>         (0x09u8 as uint8_t);}  /* TIM14 Alternate Function mapping */

/* 
  *  AF 10 selection  
  */ 
macro_rules! GPIO_AF_OTG_FS {() =>        (0xAu8 as uint8_t);}  /* OTG_FS Alternate Function mapping */
macro_rules! GPIO_AF_OTG_HS {() =>        (0xAu8 as uint8_t);}  /* OTG_HS Alternate Function mapping */

/* 
  *  AF 11 selection  
  */ 
macro_rules! GPIO_AF_ETH {() =>             (0x0Bu8 as uint8_t);}  /* ETHERNET Alternate Function mapping */

/* 
  *  AF 12 selection  
  */ 
macro_rules! GPIO_AF_FSMC {() =>            (0xCu8 as uint8_t);}  /* FSMC Alternate Function mapping */
macro_rules! GPIO_AF_OTG_HS_FS {() =>       (0xCu8 as uint8_t);}  /* OTG HS configured in FS, Alternate Function mapping */
macro_rules! GPIO_AF_SDIO {() =>            (0xCu8 as uint8_t);}  /* SDIO Alternate Function mapping */

/* 
  *  AF 13 selection  
  */ 
macro_rules! GPIO_AF_DCMI {() =>          (0x0Du8 as uint8_t);}  /* DCMI Alternate Function mapping */

/* 
  *  AF 15 selection  
  */ 
macro_rules! GPIO_AF_EVENTOUT {() =>      (0x0Fu8 as uint8_t);}  /* EVENTOUT Alternate Function mapping */

macro_rules! IS_GPIO_AF {($AF:ident) =>  ((($AF) == GPIO_AF_RTC_50Hz!())  || (($AF) == GPIO_AF_TIM14!())  ||
                                          (($AF) == GPIO_AF_MCO!())       || (($AF) == GPIO_AF_TAMPER!()) ||
                                          (($AF) == GPIO_AF_SWJ!())       || (($AF) == GPIO_AF_TRACE!())  ||
                                          (($AF) == GPIO_AF_TIM1!())      || (($AF) == GPIO_AF_TIM2!())   ||
                                          (($AF) == GPIO_AF_TIM3!())      || (($AF) == GPIO_AF_TIM4!())   ||
                                          (($AF) == GPIO_AF_TIM5!())      || (($AF) == GPIO_AF_TIM8!())   ||
                                          (($AF) == GPIO_AF_I2C1!())      || (($AF) == GPIO_AF_I2C2!())   ||
                                          (($AF) == GPIO_AF_I2C3!())      || (($AF) == GPIO_AF_SPI1!())   ||
                                          (($AF) == GPIO_AF_SPI2!())      || (($AF) == GPIO_AF_TIM13!())  ||
                                          (($AF) == GPIO_AF_SPI3!())      || (($AF) == GPIO_AF_TIM14!())  ||
                                          (($AF) == GPIO_AF_USART1!())    || (($AF) == GPIO_AF_USART2!()) ||
                                          (($AF) == GPIO_AF_USART3!())    || (($AF) == GPIO_AF_UART4!())  ||
                                          (($AF) == GPIO_AF_UART5!())     || (($AF) == GPIO_AF_USART6!()) ||
                                          (($AF) == GPIO_AF_CAN1!())      || (($AF) == GPIO_AF_CAN2!())   ||
                                          (($AF) == GPIO_AF_OTG_FS!())    || (($AF) == GPIO_AF_OTG_HS!()) ||
                                          (($AF) == GPIO_AF_ETH!())       || (($AF) == GPIO_AF_FSMC!())   ||
                                          (($AF) == GPIO_AF_OTG_HS_FS!()) || (($AF) == GPIO_AF_SDIO!())   ||
                                          (($AF) == GPIO_AF_DCMI!())      || (($AF) == GPIO_AF_EVENTOUT!()));}

/* GPIO_Legacy 
  */
    
macro_rules! GPIO_Mode_AIN {() =>        (GPIO_Mode_AN!());}

macro_rules! GPIO_AF_OTG1_FS {() =>      (GPIO_AF_OTG_FS!());}
macro_rules! GPIO_AF_OTG2_HS {() =>      (GPIO_AF_OTG_HS!());}
macro_rules! GPIO_AF_OTG2_FS {() =>      (GPIO_AF_OTG_HS_FS!());}

// /* Exported macro ------------------------------------------------------------*/
// /* Exported functions --------------------------------------------------------*/ 

// /*  Function used to set the GPIO configuration to the default reset state ****/
// void GPIO_DeInit(GPIO_TypeDef* GPIOx);

// /* Initialization and Configuration functions *********************************/
// void GPIO_Init(GPIO_TypeDef* GPIOx, GPIO_InitTypeDef* GPIO_InitStruct);
// void GPIO_StructInit(GPIO_InitTypeDef* GPIO_InitStruct);
// void GPIO_PinLockConfig(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);

// /* GPIO Read and Write functions **********************************************/
// uint8_t GPIO_ReadInputDataBit(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);
// uint16_t GPIO_ReadInputData(GPIO_TypeDef* GPIOx);
// uint8_t GPIO_ReadOutputDataBit(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);
// uint16_t GPIO_ReadOutputData(GPIO_TypeDef* GPIOx);
// void GPIO_SetBits(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);
// void GPIO_ResetBits(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);
// void GPIO_WriteBit(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin, BitAction BitVal);
// void GPIO_Write(GPIO_TypeDef* GPIOx, uint16_t PortVal);
// void GPIO_ToggleBits(GPIO_TypeDef* GPIOx, uint16_t GPIO_Pin);

// /* GPIO Alternate functions configuration function ****************************/
// void GPIO_PinAFConfig(GPIO_TypeDef* GPIOx, uint16_t GPIO_PinSource, uint8_t GPIO_AF);

