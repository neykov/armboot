#[macro_escape];

use zero::std_types::*;

/*
 * STM32F4XX Interrupt Number Definition, according to the selected device 
 *        in Library_configuration_section 
 */

pub type Enum_IRQn = c_int;
pub macro_rules! NonMaskableInt_IRQn (() => (-14 as c_int);)
pub macro_rules! MemoryManagement_IRQn (() => (-12 as c_int);)
pub macro_rules! BusFault_IRQn (() => (-11 as c_int);)
pub macro_rules! UsageFault_IRQn (() => (-10 as c_int);)
pub macro_rules! SVCall_IRQn (() => (-5 as c_int);)
pub macro_rules! DebugMonitor_IRQn (() => (-4 as c_int);)
pub macro_rules! PendSV_IRQn (() => (-2 as c_int);)
pub macro_rules! SysTick_IRQn (() => (-1 as c_int);)
pub macro_rules! WWDG_IRQn (() => (0 as c_int);)
pub macro_rules! PVD_IRQn (() => (1 as c_int);)
pub macro_rules! TAMP_STAMP_IRQn (() => (2 as c_int);)
pub macro_rules! RTC_WKUP_IRQn (() => (3 as c_int);)
pub macro_rules! FLASH_IRQn (() => (4 as c_int);)
pub macro_rules! RCC_IRQn (() => (5 as c_int);)
pub macro_rules! EXTI0_IRQn (() => (6 as c_int);)
pub macro_rules! EXTI1_IRQn (() => (7 as c_int);)
pub macro_rules! EXTI2_IRQn (() => (8 as c_int);)
pub macro_rules! EXTI3_IRQn (() => (9 as c_int);)
pub macro_rules! EXTI4_IRQn (() => (10 as c_int);)
pub macro_rules! DMA1_Stream0_IRQn (() => (11 as c_int);)
pub macro_rules! DMA1_Stream1_IRQn (() => (12 as c_int);)
pub macro_rules! DMA1_Stream2_IRQn (() => (13 as c_int);)
pub macro_rules! DMA1_Stream3_IRQn (() => (14 as c_int);)
pub macro_rules! DMA1_Stream4_IRQn (() => (15 as c_int);)
pub macro_rules! DMA1_Stream5_IRQn (() => (16 as c_int);)
pub macro_rules! DMA1_Stream6_IRQn (() => (17 as c_int);)
pub macro_rules! ADC_IRQn (() => (18 as c_int);)
pub macro_rules! CAN1_TX_IRQn (() => (19 as c_int);)
pub macro_rules! CAN1_RX0_IRQn (() => (20 as c_int);)
pub macro_rules! CAN1_RX1_IRQn (() => (21 as c_int);)
pub macro_rules! CAN1_SCE_IRQn (() => (22 as c_int);)
pub macro_rules! EXTI9_5_IRQn (() => (23 as c_int);)
pub macro_rules! TIM1_BRK_TIM9_IRQn (() => (24 as c_int);)
pub macro_rules! TIM1_UP_TIM10_IRQn (() => (25 as c_int);)
pub macro_rules! TIM1_TRG_COM_TIM11_IRQn (() => (26 as c_int);)
pub macro_rules! TIM1_CC_IRQn (() => (27 as c_int);)
pub macro_rules! TIM2_IRQn (() => (28 as c_int);)
pub macro_rules! TIM3_IRQn (() => (29 as c_int);)
pub macro_rules! TIM4_IRQn (() => (30 as c_int);)
pub macro_rules! I2C1_EV_IRQn (() => (31 as c_int);)
pub macro_rules! I2C1_ER_IRQn (() => (32 as c_int);)
pub macro_rules! I2C2_EV_IRQn (() => (33 as c_int);)
pub macro_rules! I2C2_ER_IRQn (() => (34 as c_int);)
pub macro_rules! SPI1_IRQn (() => (35 as c_int);)
pub macro_rules! SPI2_IRQn (() => (36 as c_int);)
pub macro_rules! USART1_IRQn (() => (37 as c_int);)
pub macro_rules! USART2_IRQn (() => (38 as c_int);)
pub macro_rules! USART3_IRQn (() => (39 as c_int);)
pub macro_rules! EXTI15_10_IRQn (() => (40 as c_int);)
pub macro_rules! RTC_Alarm_IRQn (() => (41 as c_int);)
pub macro_rules! OTG_FS_WKUP_IRQn (() => (42 as c_int);)
pub macro_rules! TIM8_BRK_TIM12_IRQn (() => (43 as c_int);)
pub macro_rules! TIM8_UP_TIM13_IRQn (() => (44 as c_int);)
pub macro_rules! TIM8_TRG_COM_TIM14_IRQn (() => (45 as c_int);)
pub macro_rules! TIM8_CC_IRQn (() => (46 as c_int);)
pub macro_rules! DMA1_Stream7_IRQn (() => (47 as c_int);)
pub macro_rules! FSMC_IRQn (() => (48 as c_int);)
pub macro_rules! SDIO_IRQn (() => (49 as c_int);)
pub macro_rules! TIM5_IRQn (() => (50 as c_int);)
pub macro_rules! SPI3_IRQn (() => (51 as c_int);)
pub macro_rules! UART4_IRQn (() => (52 as c_int);)
pub macro_rules! UART5_IRQn (() => (53 as c_int);)
pub macro_rules! TIM6_DAC_IRQn (() => (54 as c_int);)
pub macro_rules! TIM7_IRQn (() => (55 as c_int);)
pub macro_rules! DMA2_Stream0_IRQn (() => (56 as c_int);)
pub macro_rules! DMA2_Stream1_IRQn (() => (57 as c_int);)
pub macro_rules! DMA2_Stream2_IRQn (() => (58 as c_int);)
pub macro_rules! DMA2_Stream3_IRQn (() => (59 as c_int);)
pub macro_rules! DMA2_Stream4_IRQn (() => (60 as c_int);)
pub macro_rules! ETH_IRQn (() => (61 as c_int);)
pub macro_rules! ETH_WKUP_IRQn (() => (62 as c_int);)
pub macro_rules! CAN2_TX_IRQn (() => (63 as c_int);)
pub macro_rules! CAN2_RX0_IRQn (() => (64 as c_int);)
pub macro_rules! CAN2_RX1_IRQn (() => (65 as c_int);)
pub macro_rules! CAN2_SCE_IRQn (() => (66 as c_int);)
pub macro_rules! OTG_FS_IRQn (() => (67 as c_int);)
pub macro_rules! DMA2_Stream5_IRQn (() => (68 as c_int);)
pub macro_rules! DMA2_Stream6_IRQn (() => (69 as c_int);)
pub macro_rules! DMA2_Stream7_IRQn (() => (70 as c_int);)
pub macro_rules! USART6_IRQn (() => (71 as c_int);)
pub macro_rules! I2C3_EV_IRQn (() => (72 as c_int);)
pub macro_rules! I2C3_ER_IRQn (() => (73 as c_int);)
pub macro_rules! OTG_HS_EP1_OUT_IRQn (() => (74 as c_int);)
pub macro_rules! OTG_HS_EP1_IN_IRQn (() => (75 as c_int);)
pub macro_rules! OTG_HS_WKUP_IRQn (() => (76 as c_int);)
pub macro_rules! OTG_HS_IRQn (() => (77 as c_int);)
pub macro_rules! DCMI_IRQn (() => (78 as c_int);)
pub macro_rules! CRYP_IRQn (() => (79 as c_int);)
pub macro_rules! HASH_RNG_IRQn (() => (80 as c_int);)
pub macro_rules! FPU_IRQn (() => (81 as c_int);)
pub type IRQn_Type = Enum_IRQn;
pub type int32_t = c_int;
pub type int16_t = c_short;
pub type int8_t = c_schar;
pub type uint32_t = c_uint;
pub type uint16_t = c_ushort;
pub type uint8_t = c_uchar;
pub type s32 = int32_t;
pub type s16 = int16_t;
pub type s8 = int8_t;
pub type sc32 = int32_t;
pub type sc16 = int16_t;
pub type sc8 = int8_t;
pub type vs32 = int32_t;
pub type vs16 = int16_t;
pub type vs8 = int8_t;
pub type vsc32 = int32_t;
pub type vsc16 = int16_t;
pub type vsc8 = int8_t;
pub type _u32 = uint32_t;
pub type _u16 = uint16_t;
pub type _u8 = uint8_t;
pub type uc32 = uint32_t;
pub type uc16 = uint16_t;
pub type uc8 = uint8_t;
pub type vu32 = uint32_t;
pub type vu16 = uint16_t;
pub type vu8 = uint8_t;
pub type vuc32 = uint32_t;
pub type vuc16 = uint16_t;
pub type vuc8 = uint8_t;
pub type FlagStatus = c_uint;
pub macro_rules! RESET (() => (0 as c_uint);)
pub macro_rules! SET (() => (1 as c_uint);)
pub type ITStatus = FlagStatus;
pub type FunctionalState = c_uint;
pub macro_rules! DISABLE (() => (0 as c_uint);)
pub macro_rules! ENABLE (() => (1 as c_uint);)
pub type ErrorStatus = c_uint;
pub macro_rules! ERROR (() => (0 as c_uint);)
pub macro_rules! SUCCESS (() => (1 as c_uint);)

pub struct ADCType {
    pub SR: uint32_t,
    pub CR1: uint32_t,
    pub CR2: uint32_t,
    pub SMPR1: uint32_t,
    pub SMPR2: uint32_t,
    pub JOFR1: uint32_t,
    pub JOFR2: uint32_t,
    pub JOFR3: uint32_t,
    pub JOFR4: uint32_t,
    pub HTR: uint32_t,
    pub LTR: uint32_t,
    pub SQR1: uint32_t,
    pub SQR2: uint32_t,
    pub SQR3: uint32_t,
    pub JSQR: uint32_t,
    pub JDR1: uint32_t,
    pub JDR2: uint32_t,
    pub JDR3: uint32_t,
    pub JDR4: uint32_t,
    pub DR: uint32_t,
}
pub struct ADC_CommonType {
    pub CSR: uint32_t,
    pub CCR: uint32_t,
    pub CDR: uint32_t,
}
pub struct CAN_TxMailBoxType {
    pub TIR: uint32_t,
    pub TDTR: uint32_t,
    pub TDLR: uint32_t,
    pub TDHR: uint32_t,
}
pub struct CAN_FIFOMailBoxType {
    pub RIR: uint32_t,
    pub RDTR: uint32_t,
    pub RDLR: uint32_t,
    pub RDHR: uint32_t,
}
pub struct CAN_FilterRegisterType {
    pub FR1: uint32_t,
    pub FR2: uint32_t,
}
pub struct CANType {
    pub MCR: uint32_t,
    pub MSR: uint32_t,
    pub TSR: uint32_t,
    pub RF0R: uint32_t,
    pub RF1R: uint32_t,
    pub IER: uint32_t,
    pub ESR: uint32_t,
    pub BTR: uint32_t,
    pub RESERVED0: [uint32_t, ..88u],
    pub sTxMailBox: [CAN_TxMailBoxType, ..3u],
    pub sFIFOMailBox: [CAN_FIFOMailBoxType, ..2u],
    pub RESERVED1: [uint32_t, ..12u],
    pub FMR: uint32_t,
    pub FM1R: uint32_t,
    pub RESERVED2: uint32_t,
    pub FS1R: uint32_t,
    pub RESERVED3: uint32_t,
    pub FFA1R: uint32_t,
    pub RESERVED4: uint32_t,
    pub FA1R: uint32_t,
    pub RESERVED5: [uint32_t, ..8u],
    pub sFilterRegister: [CAN_FilterRegisterType, ..28u],
}
pub struct CRCType {
    pub DR: uint32_t,
    pub IDR: uint8_t,
    pub RESERVED0: uint8_t,
    pub RESERVED1: uint16_t,
    pub CR: uint32_t,
}
pub struct DACType {
    pub CR: uint32_t,
    pub SWTRIGR: uint32_t,
    pub DHR12R1: uint32_t,
    pub DHR12L1: uint32_t,
    pub DHR8R1: uint32_t,
    pub DHR12R2: uint32_t,
    pub DHR12L2: uint32_t,
    pub DHR8R2: uint32_t,
    pub DHR12RD: uint32_t,
    pub DHR12LD: uint32_t,
    pub DHR8RD: uint32_t,
    pub DOR1: uint32_t,
    pub DOR2: uint32_t,
    pub SR: uint32_t,
}
pub struct DBGMCUType {
    pub IDCODE: uint32_t,
    pub CR: uint32_t,
    pub APB1FZ: uint32_t,
    pub APB2FZ: uint32_t,
}
pub struct DCMIType {
    pub CR: uint32_t,
    pub SR: uint32_t,
    pub RISR: uint32_t,
    pub IER: uint32_t,
    pub MISR: uint32_t,
    pub ICR: uint32_t,
    pub ESCR: uint32_t,
    pub ESUR: uint32_t,
    pub CWSTRTR: uint32_t,
    pub CWSIZER: uint32_t,
    pub DR: uint32_t,
}
pub struct DMA_StreamType {
    pub CR: uint32_t,
    pub NDTR: uint32_t,
    pub PAR: uint32_t,
    pub M0AR: uint32_t,
    pub M1AR: uint32_t,
    pub FCR: uint32_t,
}
pub struct DMAType {
    pub LISR: uint32_t,
    pub HISR: uint32_t,
    pub LIFCR: uint32_t,
    pub HIFCR: uint32_t,
}
pub struct ETHType {
    pub MACCR: uint32_t,
    pub MACFFR: uint32_t,
    pub MACHTHR: uint32_t,
    pub MACHTLR: uint32_t,
    pub MACMIIAR: uint32_t,
    pub MACMIIDR: uint32_t,
    pub MACFCR: uint32_t,
    pub MACVLANTR: uint32_t,
    pub RESERVED0: [uint32_t, ..2u],
    pub MACRWUFFR: uint32_t,
    pub MACPMTCSR: uint32_t,
    pub RESERVED1: [uint32_t, ..2u],
    pub MACSR: uint32_t,
    pub MACIMR: uint32_t,
    pub MACA0HR: uint32_t,
    pub MACA0LR: uint32_t,
    pub MACA1HR: uint32_t,
    pub MACA1LR: uint32_t,
    pub MACA2HR: uint32_t,
    pub MACA2LR: uint32_t,
    pub MACA3HR: uint32_t,
    pub MACA3LR: uint32_t,
    pub RESERVED2: [uint32_t, ..40u],
    pub MMCCR: uint32_t,
    pub MMCRIR: uint32_t,
    pub MMCTIR: uint32_t,
    pub MMCRIMR: uint32_t,
    pub MMCTIMR: uint32_t,
    pub RESERVED3: [uint32_t, ..14u],
    pub MMCTGFSCCR: uint32_t,
    pub MMCTGFMSCCR: uint32_t,
    pub RESERVED4: [uint32_t, ..5u],
    pub MMCTGFCR: uint32_t,
    pub RESERVED5: [uint32_t, ..10u],
    pub MMCRFCECR: uint32_t,
    pub MMCRFAECR: uint32_t,
    pub RESERVED6: [uint32_t, ..10u],
    pub MMCRGUFCR: uint32_t,
    pub RESERVED7: [uint32_t, ..334u],
    pub PTPTSCR: uint32_t,
    pub PTPSSIR: uint32_t,
    pub PTPTSHR: uint32_t,
    pub PTPTSLR: uint32_t,
    pub PTPTSHUR: uint32_t,
    pub PTPTSLUR: uint32_t,
    pub PTPTSAR: uint32_t,
    pub PTPTTHR: uint32_t,
    pub PTPTTLR: uint32_t,
    pub RESERVED8: uint32_t,
    pub PTPTSSR: uint32_t,
    pub RESERVED9: [uint32_t, ..565u],
    pub DMABMR: uint32_t,
    pub DMATPDR: uint32_t,
    pub DMARPDR: uint32_t,
    pub DMARDLAR: uint32_t,
    pub DMATDLAR: uint32_t,
    pub DMASR: uint32_t,
    pub DMAOMR: uint32_t,
    pub DMAIER: uint32_t,
    pub DMAMFBOCR: uint32_t,
    pub DMARSWTR: uint32_t,
    pub RESERVED10: [uint32_t, ..8u],
    pub DMACHTDR: uint32_t,
    pub DMACHRDR: uint32_t,
    pub DMACHTBAR: uint32_t,
    pub DMACHRBAR: uint32_t,
}
pub struct EXTIType {
    pub IMR: uint32_t,
    pub EMR: uint32_t,
    pub RTSR: uint32_t,
    pub FTSR: uint32_t,
    pub SWIER: uint32_t,
    pub PR: uint32_t,
}
pub struct FLASHType {
    pub ACR: uint32_t,
    pub KEYR: uint32_t,
    pub OPTKEYR: uint32_t,
    pub SR: uint32_t,
    pub CR: uint32_t,
    pub OPTCR: uint32_t,
}
pub struct FSMC_Bank1Type {
    pub BTCR: [uint32_t, ..8u],
}
pub struct FSMC_Bank1EType {
    pub BWTR: [uint32_t, ..7u],
}
pub struct FSMC_Bank2Type {
    pub PCR2: uint32_t,
    pub SR2: uint32_t,
    pub PMEM2: uint32_t,
    pub PATT2: uint32_t,
    pub RESERVED0: uint32_t,
    pub ECCR2: uint32_t,
}
pub struct FSMC_Bank3Type {
    pub PCR3: uint32_t,
    pub SR3: uint32_t,
    pub PMEM3: uint32_t,
    pub PATT3: uint32_t,
    pub RESERVED0: uint32_t,
    pub ECCR3: uint32_t,
}
pub struct FSMC_Bank4Type {
    pub PCR4: uint32_t,
    pub SR4: uint32_t,
    pub PMEM4: uint32_t,
    pub PATT4: uint32_t,
    pub PIO4: uint32_t,
}
pub struct GPIOType {
    pub MODER: uint32_t,
    pub OTYPER: uint32_t,
    pub OSPEEDR: uint32_t,
    pub PUPDR: uint32_t,
    pub IDR: uint32_t,
    pub ODR: uint32_t,
    pub BSRRL: uint16_t,
    pub BSRRH: uint16_t,
    pub LCKR: uint32_t,
    pub AFR: [uint32_t, ..2u],
}
pub struct SYSCFGType {
    pub MEMRMP: uint32_t,
    pub PMC: uint32_t,
    pub EXTICR: [uint32_t, ..4u],
    pub RESERVED: [uint32_t, ..2u],
    pub CMPCR: uint32_t,
}
pub struct I2CType {
    pub CR1: uint16_t,
    pub RESERVED0: uint16_t,
    pub CR2: uint16_t,
    pub RESERVED1: uint16_t,
    pub OAR1: uint16_t,
    pub RESERVED2: uint16_t,
    pub OAR2: uint16_t,
    pub RESERVED3: uint16_t,
    pub DR: uint16_t,
    pub RESERVED4: uint16_t,
    pub SR1: uint16_t,
    pub RESERVED5: uint16_t,
    pub SR2: uint16_t,
    pub RESERVED6: uint16_t,
    pub CCR: uint16_t,
    pub RESERVED7: uint16_t,
    pub TRISE: uint16_t,
    pub RESERVED8: uint16_t,
}
pub struct IWDGType {
    pub KR: uint32_t,
    pub PR: uint32_t,
    pub RLR: uint32_t,
    pub SR: uint32_t,
}
pub struct PWRType {
    pub CR: uint32_t,
    pub CSR: uint32_t,
}
pub struct RCCType {
    pub CR: uint32_t,
    pub PLLCFGR: uint32_t,
    pub CFGR: uint32_t,
    pub CIR: uint32_t,
    pub AHB1RSTR: uint32_t,
    pub AHB2RSTR: uint32_t,
    pub AHB3RSTR: uint32_t,
    pub RESERVED0: uint32_t,
    pub APB1RSTR: uint32_t,
    pub APB2RSTR: uint32_t,
    pub RESERVED1: [uint32_t, ..2u],
    pub AHB1ENR: uint32_t,
    pub AHB2ENR: uint32_t,
    pub AHB3ENR: uint32_t,
    pub RESERVED2: uint32_t,
    pub APB1ENR: uint32_t,
    pub APB2ENR: uint32_t,
    pub RESERVED3: [uint32_t, ..2u],
    pub AHB1LPENR: uint32_t,
    pub AHB2LPENR: uint32_t,
    pub AHB3LPENR: uint32_t,
    pub RESERVED4: uint32_t,
    pub APB1LPENR: uint32_t,
    pub APB2LPENR: uint32_t,
    pub RESERVED5: [uint32_t, ..2u],
    pub BDCR: uint32_t,
    pub CSR: uint32_t,
    pub RESERVED6: [uint32_t, ..2u],
    pub SSCGR: uint32_t,
    pub PLLI2SCFGR: uint32_t,
}
pub struct RTCType {
    pub TR: uint32_t,
    pub DR: uint32_t,
    pub CR: uint32_t,
    pub ISR: uint32_t,
    pub PRER: uint32_t,
    pub WUTR: uint32_t,
    pub CALIBR: uint32_t,
    pub ALRMAR: uint32_t,
    pub ALRMBR: uint32_t,
    pub WPR: uint32_t,
    pub SSR: uint32_t,
    pub SHIFTR: uint32_t,
    pub TSTR: uint32_t,
    pub TSDR: uint32_t,
    pub TSSSR: uint32_t,
    pub CALR: uint32_t,
    pub TAFCR: uint32_t,
    pub ALRMASSR: uint32_t,
    pub ALRMBSSR: uint32_t,
    pub RESERVED7: uint32_t,
    pub BKP0R: uint32_t,
    pub BKP1R: uint32_t,
    pub BKP2R: uint32_t,
    pub BKP3R: uint32_t,
    pub BKP4R: uint32_t,
    pub BKP5R: uint32_t,
    pub BKP6R: uint32_t,
    pub BKP7R: uint32_t,
    pub BKP8R: uint32_t,
    pub BKP9R: uint32_t,
    pub BKP10R: uint32_t,
    pub BKP11R: uint32_t,
    pub BKP12R: uint32_t,
    pub BKP13R: uint32_t,
    pub BKP14R: uint32_t,
    pub BKP15R: uint32_t,
    pub BKP16R: uint32_t,
    pub BKP17R: uint32_t,
    pub BKP18R: uint32_t,
    pub BKP19R: uint32_t,
}
pub struct SDIOType {
    pub POWER: uint32_t,
    pub CLKCR: uint32_t,
    pub ARG: uint32_t,
    pub CMD: uint32_t,
    pub RESPCMD: uint32_t,
    pub RESP1: uint32_t,
    pub RESP2: uint32_t,
    pub RESP3: uint32_t,
    pub RESP4: uint32_t,
    pub DTIMER: uint32_t,
    pub DLEN: uint32_t,
    pub DCTRL: uint32_t,
    pub DCOUNT: uint32_t,
    pub STA: uint32_t,
    pub ICR: uint32_t,
    pub MASK: uint32_t,
    pub RESERVED0: [uint32_t, ..2u],
    pub FIFOCNT: uint32_t,
    pub RESERVED1: [uint32_t, ..13u],
    pub FIFO: uint32_t,
}
pub struct SPIType {
    pub CR1: uint16_t,
    pub RESERVED0: uint16_t,
    pub CR2: uint16_t,
    pub RESERVED1: uint16_t,
    pub SR: uint16_t,
    pub RESERVED2: uint16_t,
    pub DR: uint16_t,
    pub RESERVED3: uint16_t,
    pub CRCPR: uint16_t,
    pub RESERVED4: uint16_t,
    pub RXCRCR: uint16_t,
    pub RESERVED5: uint16_t,
    pub TXCRCR: uint16_t,
    pub RESERVED6: uint16_t,
    pub I2SCFGR: uint16_t,
    pub RESERVED7: uint16_t,
    pub I2SPR: uint16_t,
    pub RESERVED8: uint16_t,
}
pub struct TIMType {
    pub CR1: uint16_t,
    pub RESERVED0: uint16_t,
    pub CR2: uint16_t,
    pub RESERVED1: uint16_t,
    pub SMCR: uint16_t,
    pub RESERVED2: uint16_t,
    pub DIER: uint16_t,
    pub RESERVED3: uint16_t,
    pub SR: uint16_t,
    pub RESERVED4: uint16_t,
    pub EGR: uint16_t,
    pub RESERVED5: uint16_t,
    pub CCMR1: uint16_t,
    pub RESERVED6: uint16_t,
    pub CCMR2: uint16_t,
    pub RESERVED7: uint16_t,
    pub CCER: uint16_t,
    pub RESERVED8: uint16_t,
    pub CNT: uint32_t,
    pub PSC: uint16_t,
    pub RESERVED9: uint16_t,
    pub ARR: uint32_t,
    pub RCR: uint16_t,
    pub RESERVED10: uint16_t,
    pub CCR1: uint32_t,
    pub CCR2: uint32_t,
    pub CCR3: uint32_t,
    pub CCR4: uint32_t,
    pub BDTR: uint16_t,
    pub RESERVED11: uint16_t,
    pub DCR: uint16_t,
    pub RESERVED12: uint16_t,
    pub DMAR: uint16_t,
    pub RESERVED13: uint16_t,
    pub OR: uint16_t,
    pub RESERVED14: uint16_t,
}
pub struct USARTType {
    pub SR: uint16_t,
    pub RESERVED0: uint16_t,
    pub DR: uint16_t,
    pub RESERVED1: uint16_t,
    pub BRR: uint16_t,
    pub RESERVED2: uint16_t,
    pub CR1: uint16_t,
    pub RESERVED3: uint16_t,
    pub CR2: uint16_t,
    pub RESERVED4: uint16_t,
    pub CR3: uint16_t,
    pub RESERVED5: uint16_t,
    pub GTPR: uint16_t,
    pub RESERVED6: uint16_t,
}
pub struct WWDGType {
    pub CR: uint32_t,
    pub CFR: uint32_t,
    pub SR: uint32_t,
}
pub struct CRYPType {
    pub CR: uint32_t,
    pub SR: uint32_t,
    pub DR: uint32_t,
    pub DOUT: uint32_t,
    pub DMACR: uint32_t,
    pub IMSCR: uint32_t,
    pub RISR: uint32_t,
    pub MISR: uint32_t,
    pub K0LR: uint32_t,
    pub K0RR: uint32_t,
    pub K1LR: uint32_t,
    pub K1RR: uint32_t,
    pub K2LR: uint32_t,
    pub K2RR: uint32_t,
    pub K3LR: uint32_t,
    pub K3RR: uint32_t,
    pub IV0LR: uint32_t,
    pub IV0RR: uint32_t,
    pub IV1LR: uint32_t,
    pub IV1RR: uint32_t,
}
pub struct HASHType {
    pub CR: uint32_t,
    pub DIN: uint32_t,
    pub STR: uint32_t,
    pub HR: [uint32_t, ..5u],
    pub IMR: uint32_t,
    pub SR: uint32_t,
    pub RESERVED: [uint32_t, ..52u],
    pub CSR: [uint32_t, ..51u],
}
pub struct RNGType {
    pub CR: uint32_t,
    pub SR: uint32_t,
    pub DR: uint32_t,
}


/*Peripheral_memory_map */
pub macro_rules! FLASH_BASE (() => (0x08000000 as uint32_t);)


pub macro_rules! FLASH_BASE (() =>            (0x08000000 as uint32_t);) /*< FLASH(up to 1 MB) base address in the alias region                         */
pub macro_rules! CCMDATARAM_BASE (() =>       (0x10000000 as uint32_t);) /*< CCM(core coupled memory) data RAM(64 KB) base address in the alias region  */
pub macro_rules! SRAM1_BASE (() =>            (0x20000000 as uint32_t);) /*< SRAM1(112 KB) base address in the alias region                             */
pub macro_rules! SRAM2_BASE (() =>            (0x2001C000 as uint32_t);) /*< SRAM2(16 KB) base address in the alias region                              */
pub macro_rules! PERIPH_BASE (() =>           (0x40000000 as uint32_t);) /*< Peripheral base address in the alias region                                */
pub macro_rules! BKPSRAM_BASE (() =>          (0x40024000 as uint32_t);) /*< Backup SRAM(4 KB) base address in the alias region                         */
pub macro_rules! FSMC_R_BASE (() =>           (0xA0000000 as uint32_t);) /*< FSMC registers base address                                                */

pub macro_rules! CCMDATARAM_BB_BASE (() =>    (0x12000000 as uint32_t);) /*< CCM(core coupled memory) data RAM(64 KB) base address in the bit-band region  */
pub macro_rules! SRAM1_BB_BASE (() =>         (0x22000000 as uint32_t);) /*< SRAM1(112 KB) base address in the bit-band region                             */
pub macro_rules! SRAM2_BB_BASE (() =>         (0x2201C000 as uint32_t);) /*< SRAM2(16 KB) base address in the bit-band region                              */
pub macro_rules! PERIPH_BB_BASE (() =>        (0x42000000 as uint32_t);) /*< Peripheral base address in the bit-band region                                */
pub macro_rules! BKPSRAM_BB_BASE (() =>       (0x42024000 as uint32_t);) /*< Backup SRAM(4 KB) base address in the bit-band region                         */

/* Legacy defines */
pub macro_rules! SRAM_BASE (() =>             (SRAM1_BASE!());)
pub macro_rules! SRAM_BB_BASE (() =>          (SRAM1_BB_BASE!());)


/*< Peripheral memory map */
pub macro_rules! APB1PERIPH_BASE (() =>       (PERIPH_BASE!());)
pub macro_rules! APB2PERIPH_BASE (() =>       (PERIPH_BASE!() + 0x00010000);)
pub macro_rules! AHB1PERIPH_BASE (() =>       (PERIPH_BASE!() + 0x00020000);)
pub macro_rules! AHB2PERIPH_BASE (() =>       (PERIPH_BASE!() + 0x10000000);)

/*< APB1 peripherals */
pub macro_rules! TIM2_BASE (() =>             (APB1PERIPH_BASE!() + 0x0000);)
pub macro_rules! TIM3_BASE (() =>             (APB1PERIPH_BASE!() + 0x0400);)
pub macro_rules! TIM4_BASE (() =>             (APB1PERIPH_BASE!() + 0x0800);)
pub macro_rules! TIM5_BASE (() =>             (APB1PERIPH_BASE!() + 0x0C00);)
pub macro_rules! TIM6_BASE (() =>             (APB1PERIPH_BASE!() + 0x1000);)
pub macro_rules! TIM7_BASE (() =>             (APB1PERIPH_BASE!() + 0x1400);)
pub macro_rules! TIM12_BASE (() =>            (APB1PERIPH_BASE!() + 0x1800);)
pub macro_rules! TIM13_BASE (() =>            (APB1PERIPH_BASE!() + 0x1C00);)
pub macro_rules! TIM14_BASE (() =>            (APB1PERIPH_BASE!() + 0x2000);)
pub macro_rules! RTC_BASE (() =>              (APB1PERIPH_BASE!() + 0x2800);)
pub macro_rules! WWDG_BASE (() =>             (APB1PERIPH_BASE!() + 0x2C00);)
pub macro_rules! IWDG_BASE (() =>             (APB1PERIPH_BASE!() + 0x3000);)
pub macro_rules! I2S2ext_BASE (() =>          (APB1PERIPH_BASE!() + 0x3400);)
pub macro_rules! SPI2_BASE (() =>             (APB1PERIPH_BASE!() + 0x3800);)
pub macro_rules! SPI3_BASE (() =>             (APB1PERIPH_BASE!() + 0x3C00);)
pub macro_rules! I2S3ext_BASE (() =>          (APB1PERIPH_BASE!() + 0x4000);)
pub macro_rules! USART2_BASE (() =>           (APB1PERIPH_BASE!() + 0x4400);)
pub macro_rules! USART3_BASE (() =>           (APB1PERIPH_BASE!() + 0x4800);)
pub macro_rules! UART4_BASE (() =>            (APB1PERIPH_BASE!() + 0x4C00);)
pub macro_rules! UART5_BASE (() =>            (APB1PERIPH_BASE!() + 0x5000);)
pub macro_rules! I2C1_BASE (() =>             (APB1PERIPH_BASE!() + 0x5400);)
pub macro_rules! I2C2_BASE (() =>             (APB1PERIPH_BASE!() + 0x5800);)
pub macro_rules! I2C3_BASE (() =>             (APB1PERIPH_BASE!() + 0x5C00);)
pub macro_rules! CAN1_BASE (() =>             (APB1PERIPH_BASE!() + 0x6400);)
pub macro_rules! CAN2_BASE (() =>             (APB1PERIPH_BASE!() + 0x6800);)
pub macro_rules! PWR_BASE (() =>              (APB1PERIPH_BASE!() + 0x7000);)
pub macro_rules! DAC_BASE (() =>              (APB1PERIPH_BASE!() + 0x7400);)

/*< APB2 peripherals */
pub macro_rules! TIM1_BASE (() =>             (APB2PERIPH_BASE!() + 0x0000);)
pub macro_rules! TIM8_BASE (() =>             (APB2PERIPH_BASE!() + 0x0400);)
pub macro_rules! USART1_BASE (() =>           (APB2PERIPH_BASE!() + 0x1000);)
pub macro_rules! USART6_BASE (() =>           (APB2PERIPH_BASE!() + 0x1400);)
pub macro_rules! ADC1_BASE (() =>             (APB2PERIPH_BASE!() + 0x2000);)
pub macro_rules! ADC2_BASE (() =>             (APB2PERIPH_BASE!() + 0x2100);)
pub macro_rules! ADC3_BASE (() =>             (APB2PERIPH_BASE!() + 0x2200);)
pub macro_rules! ADC_BASE (() =>              (APB2PERIPH_BASE!() + 0x2300);)
pub macro_rules! SDIO_BASE (() =>             (APB2PERIPH_BASE!() + 0x2C00);)
pub macro_rules! SPI1_BASE (() =>             (APB2PERIPH_BASE!() + 0x3000);)
pub macro_rules! SYSCFG_BASE (() =>           (APB2PERIPH_BASE!() + 0x3800);)
pub macro_rules! EXTI_BASE (() =>             (APB2PERIPH_BASE!() + 0x3C00);)
pub macro_rules! TIM9_BASE (() =>             (APB2PERIPH_BASE!() + 0x4000);)
pub macro_rules! TIM10_BASE (() =>            (APB2PERIPH_BASE!() + 0x4400);)
pub macro_rules! TIM11_BASE (() =>            (APB2PERIPH_BASE!() + 0x4800);)

/*< AHB1 peripherals */
pub macro_rules! GPIOA_BASE (() =>            (AHB1PERIPH_BASE!() + 0x0000);)
pub macro_rules! GPIOB_BASE (() =>            (AHB1PERIPH_BASE!() + 0x0400);)
pub macro_rules! GPIOC_BASE (() =>            (AHB1PERIPH_BASE!() + 0x0800);)
pub macro_rules! GPIOD_BASE (() =>            (AHB1PERIPH_BASE!() + 0x0C00);)
pub macro_rules! GPIOE_BASE (() =>            (AHB1PERIPH_BASE!() + 0x1000);)
pub macro_rules! GPIOF_BASE (() =>            (AHB1PERIPH_BASE!() + 0x1400);)
pub macro_rules! GPIOG_BASE (() =>            (AHB1PERIPH_BASE!() + 0x1800);)
pub macro_rules! GPIOH_BASE (() =>            (AHB1PERIPH_BASE!() + 0x1C00);)
pub macro_rules! GPIOI_BASE (() =>            (AHB1PERIPH_BASE!() + 0x2000);)
pub macro_rules! CRC_BASE (() =>              (AHB1PERIPH_BASE!() + 0x3000);)
pub macro_rules! RCC_BASE (() =>              (AHB1PERIPH_BASE!() + 0x3800);)
pub macro_rules! FLASH_R_BASE (() =>          (AHB1PERIPH_BASE!() + 0x3C00);)
pub macro_rules! DMA1_BASE (() =>             (AHB1PERIPH_BASE!() + 0x6000);)
pub macro_rules! DMA1_Stream0_BASE (() =>     (DMA1_BASE!() + 0x010);)
pub macro_rules! DMA1_Stream1_BASE (() =>     (DMA1_BASE!() + 0x028);)
pub macro_rules! DMA1_Stream2_BASE (() =>     (DMA1_BASE!() + 0x040);)
pub macro_rules! DMA1_Stream3_BASE (() =>     (DMA1_BASE!() + 0x058);)
pub macro_rules! DMA1_Stream4_BASE (() =>     (DMA1_BASE!() + 0x070);)
pub macro_rules! DMA1_Stream5_BASE (() =>     (DMA1_BASE!() + 0x088);)
pub macro_rules! DMA1_Stream6_BASE (() =>     (DMA1_BASE!() + 0x0A0);)
pub macro_rules! DMA1_Stream7_BASE (() =>     (DMA1_BASE!() + 0x0B8);)
pub macro_rules! DMA2_BASE (() =>             (AHB1PERIPH_BASE!() + 0x6400);)
pub macro_rules! DMA2_Stream0_BASE (() =>     (DMA2_BASE!() + 0x010);)
pub macro_rules! DMA2_Stream1_BASE (() =>     (DMA2_BASE!() + 0x028);)
pub macro_rules! DMA2_Stream2_BASE (() =>     (DMA2_BASE!() + 0x040);)
pub macro_rules! DMA2_Stream3_BASE (() =>     (DMA2_BASE!() + 0x058);)
pub macro_rules! DMA2_Stream4_BASE (() =>     (DMA2_BASE!() + 0x070);)
pub macro_rules! DMA2_Stream5_BASE (() =>     (DMA2_BASE!() + 0x088);)
pub macro_rules! DMA2_Stream6_BASE (() =>     (DMA2_BASE!() + 0x0A0);)
pub macro_rules! DMA2_Stream7_BASE (() =>     (DMA2_BASE!() + 0x0B8);)
pub macro_rules! ETH_BASE (() =>              (AHB1PERIPH_BASE!() + 0x8000);)
pub macro_rules! ETH_MAC_BASE (() =>          (ETH_BASE!());)
pub macro_rules! ETH_MMC_BASE (() =>          (ETH_BASE!() + 0x0100);)
pub macro_rules! ETH_PTP_BASE (() =>          (ETH_BASE!() + 0x0700);)
pub macro_rules! ETH_DMA_BASE (() =>          (ETH_BASE!() + 0x1000);)

/*< AHB2 peripherals */
pub macro_rules! DCMI_BASE (() =>             (AHB2PERIPH_BASE!() + 0x50000);)
pub macro_rules! CRYP_BASE (() =>             (AHB2PERIPH_BASE!() + 0x60000);)
pub macro_rules! HASH_BASE (() =>             (AHB2PERIPH_BASE!() + 0x60400);)
pub macro_rules! RNG_BASE (() =>              (AHB2PERIPH_BASE!() + 0x60800);)

/*< FSMC Bankx registers base address */
pub macro_rules! FSMC_Bank1_R_BASE (() =>     (FSMC_R_BASE!() + 0x0000);)
pub macro_rules! FSMC_Bank1E_R_BASE (() =>    (FSMC_R_BASE!() + 0x0104);)
pub macro_rules! FSMC_Bank2_R_BASE (() =>     (FSMC_R_BASE!() + 0x0060);)
pub macro_rules! FSMC_Bank3_R_BASE (() =>     (FSMC_R_BASE!() + 0x0080);)
pub macro_rules! FSMC_Bank4_R_BASE (() =>     (FSMC_R_BASE!() + 0x00A0);)

/* Debug MCU registers base address */
pub macro_rules! DBGMCU_BASE (() =>           (0xE0042000 as uint32_t);)
  
/* Exported_constants
  */
  
  /* Peripheral_Registers_Bits_Definition
  */
    
/*
 * Peripheral Registers_Bits_Definition
 */

/*
 *Analog to Digital Converter
 */
/* Bit definition for ADC_SR register  */
pub macro_rules! ADC_SR_AWD (() =>                          (0x01 as uint8_t);)               /*<Analog watchdog flag */
pub macro_rules! ADC_SR_EOC (() =>                          (0x02 as uint8_t);)               /*<End of conversion */
pub macro_rules! ADC_SR_JEOC (() =>                         (0x04 as uint8_t);)               /*<Injected channel end of conversion */
pub macro_rules! ADC_SR_JSTRT (() =>                        (0x08 as uint8_t);)               /*<Injected channel Start flag */
pub macro_rules! ADC_SR_STRT (() =>                         (0x10 as uint8_t);)               /*<Regular channel Start flag */
pub macro_rules! ADC_SR_OVR (() =>                          (0x20 as uint8_t);)               /*<Overrun flag */

/*  Bit definition for ADC_CR1 register  */
pub macro_rules! ADC_CR1_AWDCH (() =>                       (0x0000001F as uint32_t);)        /*<AWDCH[4:0] bits (Analog watchdog channel select bits) */
pub macro_rules! ADC_CR1_AWDCH_0 (() =>                     (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CR1_AWDCH_1 (() =>                     (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CR1_AWDCH_2 (() =>                     (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_CR1_AWDCH_3 (() =>                     (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_CR1_AWDCH_4 (() =>                     (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_CR1_EOCIE (() =>                       (0x00000020 as uint32_t);)        /*<Interrupt enable for EOC */
pub macro_rules! ADC_CR1_AWDIE (() =>                       (0x00000040 as uint32_t);)        /*<AAnalog Watchdog interrupt enable */
pub macro_rules! ADC_CR1_JEOCIE (() =>                      (0x00000080 as uint32_t);)        /*<Interrupt enable for injected channels */
pub macro_rules! ADC_CR1_SCAN (() =>                        (0x00000100 as uint32_t);)        /*<Scan mode */
pub macro_rules! ADC_CR1_AWDSGL (() =>                      (0x00000200 as uint32_t);)        /*<Enable the watchdog on a single channel in scan mode */
pub macro_rules! ADC_CR1_JAUTO (() =>                       (0x00000400 as uint32_t);)        /*<Automatic injected group conversion */
pub macro_rules! ADC_CR1_DISCEN (() =>                      (0x00000800 as uint32_t);)        /*<Discontinuous mode on regular channels */
pub macro_rules! ADC_CR1_JDISCEN (() =>                     (0x00001000 as uint32_t);)        /*<Discontinuous mode on injected channels */
pub macro_rules! ADC_CR1_DISCNUM (() =>                     (0x0000E000 as uint32_t);)        /*<DISCNUM[2:0] bits (Discontinuous mode channel count) */
pub macro_rules! ADC_CR1_DISCNUM_0 (() =>                   (0x00002000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CR1_DISCNUM_1 (() =>                   (0x00004000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CR1_DISCNUM_2 (() =>                   (0x00008000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_CR1_JAWDEN (() =>                      (0x00400000 as uint32_t);)        /*<Analog watchdog enable on injected channels */
pub macro_rules! ADC_CR1_AWDEN (() =>                       (0x00800000 as uint32_t);)        /*<Analog watchdog enable on regular channels */
pub macro_rules! ADC_CR1_RES (() =>                         (0x03000000 as uint32_t);)        /*<RES[2:0] bits (Resolution) */
pub macro_rules! ADC_CR1_RES_0 (() =>                       (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CR1_RES_1 (() =>                       (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CR1_OVRIE (() =>                       (0x04000000 as uint32_t);)         /*<overrun interrupt enable */
  
/*  Bit definition for ADC_CR2 register  */
pub macro_rules! ADC_CR2_ADON (() =>                        (0x00000001 as uint32_t);)        /*<A/D Converter ON / OFF */
pub macro_rules! ADC_CR2_CONT (() =>                        (0x00000002 as uint32_t);)        /*<Continuous Conversion */
pub macro_rules! ADC_CR2_DMA (() =>                         (0x00000100 as uint32_t);)        /*<Direct Memory access mode */
pub macro_rules! ADC_CR2_DDS (() =>                         (0x00000200 as uint32_t);)        /*<DMA disable selection (Single ADC) */
pub macro_rules! ADC_CR2_EOCS (() =>                        (0x00000400 as uint32_t);)        /*<End of conversion selection */
pub macro_rules! ADC_CR2_ALIGN (() =>                       (0x00000800 as uint32_t);)        /*<Data Alignment */
pub macro_rules! ADC_CR2_JEXTSEL (() =>                     (0x000F0000 as uint32_t);)        /*<JEXTSEL[3:0] bits (External event select for injected group) */
pub macro_rules! ADC_CR2_JEXTSEL_0 (() =>                   (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CR2_JEXTSEL_1 (() =>                   (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CR2_JEXTSEL_2 (() =>                   (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_CR2_JEXTSEL_3 (() =>                   (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_CR2_JEXTEN (() =>                      (0x00300000 as uint32_t);)        /*<JEXTEN[1:0] bits (External Trigger Conversion mode for injected channelsp) */
pub macro_rules! ADC_CR2_JEXTEN_0 (() =>                    (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CR2_JEXTEN_1 (() =>                    (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CR2_JSWSTART (() =>                    (0x00400000 as uint32_t);)        /*<Start Conversion of injected channels */
pub macro_rules! ADC_CR2_EXTSEL (() =>                      (0x0F000000 as uint32_t);)        /*<EXTSEL[3:0] bits (External Event Select for regular group) */
pub macro_rules! ADC_CR2_EXTSEL_0 (() =>                    (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CR2_EXTSEL_1 (() =>                    (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CR2_EXTSEL_2 (() =>                    (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_CR2_EXTSEL_3 (() =>                    (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_CR2_EXTEN (() =>                       (0x30000000 as uint32_t);)        /*<EXTEN[1:0] bits (External Trigger Conversion mode for regular channelsp) */
pub macro_rules! ADC_CR2_EXTEN_0 (() =>                     (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CR2_EXTEN_1 (() =>                     (0x20000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CR2_SWSTART (() =>                     (0x40000000 as uint32_t);)        /*<Start Conversion of regular channels */

/*  Bit definition for ADC_SMPR1 register  */
pub macro_rules! ADC_SMPR1_SMP10 (() =>                     (0x00000007 as uint32_t);)        /*<SMP10[2:0] bits (Channel 10 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP10_0 (() =>                   (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP10_1 (() =>                   (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP10_2 (() =>                   (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP11 (() =>                     (0x00000038 as uint32_t);)        /*<SMP11[2:0] bits (Channel 11 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP11_0 (() =>                   (0x00000008 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP11_1 (() =>                   (0x00000010 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP11_2 (() =>                   (0x00000020 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP12 (() =>                     (0x000001C0 as uint32_t);)        /*<SMP12[2:0] bits (Channel 12 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP12_0 (() =>                   (0x00000040 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP12_1 (() =>                   (0x00000080 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP12_2 (() =>                   (0x00000100 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP13 (() =>                     (0x00000E00 as uint32_t);)        /*<SMP13[2:0] bits (Channel 13 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP13_0 (() =>                   (0x00000200 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP13_1 (() =>                   (0x00000400 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP13_2 (() =>                   (0x00000800 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP14 (() =>                     (0x00007000 as uint32_t);)        /*<SMP14[2:0] bits (Channel 14 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP14_0 (() =>                   (0x00001000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP14_1 (() =>                   (0x00002000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP14_2 (() =>                   (0x00004000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP15 (() =>                     (0x00038000 as uint32_t);)        /*<SMP15[2:0] bits (Channel 15 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP15_0 (() =>                   (0x00008000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP15_1 (() =>                   (0x00010000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP15_2 (() =>                   (0x00020000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP16 (() =>                     (0x001C0000 as uint32_t);)        /*<SMP16[2:0] bits (Channel 16 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP16_0 (() =>                   (0x00040000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP16_1 (() =>                   (0x00080000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP16_2 (() =>                   (0x00100000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP17 (() =>                     (0x00E00000 as uint32_t);)        /*<SMP17[2:0] bits (Channel 17 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP17_0 (() =>                   (0x00200000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP17_1 (() =>                   (0x00400000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP17_2 (() =>                   (0x00800000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR1_SMP18 (() =>                     (0x07000000 as uint32_t);)        /*<SMP18[2:0] bits (Channel 18 Sample time selection) */
pub macro_rules! ADC_SMPR1_SMP18_0 (() =>                   (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR1_SMP18_1 (() =>                   (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR1_SMP18_2 (() =>                   (0x04000000 as uint32_t);)        /*<Bit 2 */

/*  Bit definition for ADC_SMPR2 register  */
pub macro_rules! ADC_SMPR2_SMP0 (() =>                      (0x00000007 as uint32_t);)        /*<SMP0[2:0] bits (Channel 0 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP0_0 (() =>                    (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP0_1 (() =>                    (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP0_2 (() =>                    (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP1 (() =>                      (0x00000038 as uint32_t);)        /*<SMP1[2:0] bits (Channel 1 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP1_0 (() =>                    (0x00000008 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP1_1 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP1_2 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP2 (() =>                      (0x000001C0 as uint32_t);)        /*<SMP2[2:0] bits (Channel 2 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP2_0 (() =>                    (0x00000040 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP2_1 (() =>                    (0x00000080 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP2_2 (() =>                    (0x00000100 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP3 (() =>                      (0x00000E00 as uint32_t);)        /*<SMP3[2:0] bits (Channel 3 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP3_0 (() =>                    (0x00000200 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP3_1 (() =>                    (0x00000400 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP3_2 (() =>                    (0x00000800 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP4 (() =>                      (0x00007000 as uint32_t);)        /*<SMP4[2:0] bits (Channel 4 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP4_0 (() =>                    (0x00001000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP4_1 (() =>                    (0x00002000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP4_2 (() =>                    (0x00004000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP5 (() =>                      (0x00038000 as uint32_t);)        /*<SMP5[2:0] bits (Channel 5 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP5_0 (() =>                    (0x00008000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP5_1 (() =>                    (0x00010000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP5_2 (() =>                    (0x00020000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP6 (() =>                      (0x001C0000 as uint32_t);)        /*<SMP6[2:0] bits (Channel 6 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP6_0 (() =>                    (0x00040000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP6_1 (() =>                    (0x00080000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP6_2 (() =>                    (0x00100000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP7 (() =>                      (0x00E00000 as uint32_t);)        /*<SMP7[2:0] bits (Channel 7 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP7_0 (() =>                    (0x00200000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP7_1 (() =>                    (0x00400000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP7_2 (() =>                    (0x00800000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP8 (() =>                      (0x07000000 as uint32_t);)        /*<SMP8[2:0] bits (Channel 8 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP8_0 (() =>                    (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP8_1 (() =>                    (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP8_2 (() =>                    (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SMPR2_SMP9 (() =>                      (0x38000000 as uint32_t);)        /*<SMP9[2:0] bits (Channel 9 Sample time selection) */
pub macro_rules! ADC_SMPR2_SMP9_0 (() =>                    (0x08000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SMPR2_SMP9_1 (() =>                    (0x10000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SMPR2_SMP9_2 (() =>                    (0x20000000 as uint32_t);)        /*<Bit 2 */

/*  Bit definition for ADC_JOFR1 register  */
pub macro_rules! ADC_JOFR1_JOFFSET1 (() =>                  (0x0FFF as uint16_t);)            /*<Data offset for injected channel 1 */

/*  Bit definition for ADC_JOFR2 register  */
pub macro_rules! ADC_JOFR2_JOFFSET2 (() =>                  (0x0FFF as uint16_t);)            /*<Data offset for injected channel 2 */

/*  Bit definition for ADC_JOFR3 register  */
pub macro_rules! ADC_JOFR3_JOFFSET3 (() =>                  (0x0FFF as uint16_t);)            /*<Data offset for injected channel 3 */

/*  Bit definition for ADC_JOFR4 register  */
pub macro_rules! ADC_JOFR4_JOFFSET4 (() =>                  (0x0FFF as uint16_t);)            /*<Data offset for injected channel 4 */

/*  Bit definition for ADC_HTR register  */
pub macro_rules! ADC_HTR_HT (() =>                          (0x0FFF as uint16_t);)            /*<Analog watchdog high threshold */

/*  Bit definition for ADC_LTR register  */
pub macro_rules! ADC_LTR_LT (() =>                          (0x0FFF as uint16_t);)            /*<Analog watchdog low threshold */

/*  Bit definition for ADC_SQR1 register  */
pub macro_rules! ADC_SQR1_SQ13 (() =>                       (0x0000001F as uint32_t);)        /*<SQ13[4:0] bits (13th conversion in regular sequence) */
pub macro_rules! ADC_SQR1_SQ13_0 (() =>                     (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR1_SQ13_1 (() =>                     (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR1_SQ13_2 (() =>                     (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR1_SQ13_3 (() =>                     (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR1_SQ13_4 (() =>                     (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR1_SQ14 (() =>                       (0x000003E0 as uint32_t);)        /*<SQ14[4:0] bits (14th conversion in regular sequence) */
pub macro_rules! ADC_SQR1_SQ14_0 (() =>                     (0x00000020 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR1_SQ14_1 (() =>                     (0x00000040 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR1_SQ14_2 (() =>                     (0x00000080 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR1_SQ14_3 (() =>                     (0x00000100 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR1_SQ14_4 (() =>                     (0x00000200 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR1_SQ15 (() =>                       (0x00007C00 as uint32_t);)        /*<SQ15[4:0] bits (15th conversion in regular sequence) */
pub macro_rules! ADC_SQR1_SQ15_0 (() =>                     (0x00000400 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR1_SQ15_1 (() =>                     (0x00000800 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR1_SQ15_2 (() =>                     (0x00001000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR1_SQ15_3 (() =>                     (0x00002000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR1_SQ15_4 (() =>                     (0x00004000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR1_SQ16 (() =>                       (0x000F8000 as uint32_t);)        /*<SQ16[4:0] bits (16th conversion in regular sequence) */
pub macro_rules! ADC_SQR1_SQ16_0 (() =>                     (0x00008000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR1_SQ16_1 (() =>                     (0x00010000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR1_SQ16_2 (() =>                     (0x00020000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR1_SQ16_3 (() =>                     (0x00040000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR1_SQ16_4 (() =>                     (0x00080000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR1_L (() =>                          (0x00F00000 as uint32_t);)        /*<L[3:0] bits (Regular channel sequence length) */
pub macro_rules! ADC_SQR1_L_0 (() =>                        (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR1_L_1 (() =>                        (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR1_L_2 (() =>                        (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR1_L_3 (() =>                        (0x00800000 as uint32_t);)        /*<Bit 3 */

/*  Bit definition for ADC_SQR2 register  */
pub macro_rules! ADC_SQR2_SQ7 (() =>                        (0x0000001F as uint32_t);)        /*<SQ7[4:0] bits (7th conversion in regular sequence) */
pub macro_rules! ADC_SQR2_SQ7_0 (() =>                      (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR2_SQ7_1 (() =>                      (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR2_SQ7_2 (() =>                      (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR2_SQ7_3 (() =>                      (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR2_SQ7_4 (() =>                      (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR2_SQ8 (() =>                        (0x000003E0 as uint32_t);)        /*<SQ8[4:0] bits (8th conversion in regular sequence) */
pub macro_rules! ADC_SQR2_SQ8_0 (() =>                      (0x00000020 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR2_SQ8_1 (() =>                      (0x00000040 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR2_SQ8_2 (() =>                      (0x00000080 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR2_SQ8_3 (() =>                      (0x00000100 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR2_SQ8_4 (() =>                      (0x00000200 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR2_SQ9 (() =>                        (0x00007C00 as uint32_t);)        /*<SQ9[4:0] bits (9th conversion in regular sequence) */
pub macro_rules! ADC_SQR2_SQ9_0 (() =>                      (0x00000400 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR2_SQ9_1 (() =>                      (0x00000800 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR2_SQ9_2 (() =>                      (0x00001000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR2_SQ9_3 (() =>                      (0x00002000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR2_SQ9_4 (() =>                      (0x00004000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR2_SQ10 (() =>                       (0x000F8000 as uint32_t);)        /*<SQ10[4:0] bits (10th conversion in regular sequence) */
pub macro_rules! ADC_SQR2_SQ10_0 (() =>                     (0x00008000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR2_SQ10_1 (() =>                     (0x00010000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR2_SQ10_2 (() =>                     (0x00020000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR2_SQ10_3 (() =>                     (0x00040000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR2_SQ10_4 (() =>                     (0x00080000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR2_SQ11 (() =>                       (0x01F00000 as uint32_t);)        /*<SQ11[4:0] bits (11th conversion in regular sequence) */
pub macro_rules! ADC_SQR2_SQ11_0 (() =>                     (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR2_SQ11_1 (() =>                     (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR2_SQ11_2 (() =>                     (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR2_SQ11_3 (() =>                     (0x00800000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR2_SQ11_4 (() =>                     (0x01000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR2_SQ12 (() =>                       (0x3E000000 as uint32_t);)        /*<SQ12[4:0] bits (12th conversion in regular sequence) */
pub macro_rules! ADC_SQR2_SQ12_0 (() =>                     (0x02000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR2_SQ12_1 (() =>                     (0x04000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR2_SQ12_2 (() =>                     (0x08000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR2_SQ12_3 (() =>                     (0x10000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR2_SQ12_4 (() =>                     (0x20000000 as uint32_t);)        /*<Bit 4 */

/*  Bit definition for ADC_SQR3 register  */
pub macro_rules! ADC_SQR3_SQ1 (() =>                        (0x0000001F as uint32_t);)        /*<SQ1[4:0] bits (1st conversion in regular sequence) */
pub macro_rules! ADC_SQR3_SQ1_0 (() =>                      (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR3_SQ1_1 (() =>                      (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR3_SQ1_2 (() =>                      (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR3_SQ1_3 (() =>                      (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR3_SQ1_4 (() =>                      (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR3_SQ2 (() =>                        (0x000003E0 as uint32_t);)        /*<SQ2[4:0] bits (2nd conversion in regular sequence) */
pub macro_rules! ADC_SQR3_SQ2_0 (() =>                      (0x00000020 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR3_SQ2_1 (() =>                      (0x00000040 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR3_SQ2_2 (() =>                      (0x00000080 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR3_SQ2_3 (() =>                      (0x00000100 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR3_SQ2_4 (() =>                      (0x00000200 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR3_SQ3 (() =>                        (0x00007C00 as uint32_t);)        /*<SQ3[4:0] bits (3rd conversion in regular sequence) */
pub macro_rules! ADC_SQR3_SQ3_0 (() =>                      (0x00000400 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR3_SQ3_1 (() =>                      (0x00000800 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR3_SQ3_2 (() =>                      (0x00001000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR3_SQ3_3 (() =>                      (0x00002000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR3_SQ3_4 (() =>                      (0x00004000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR3_SQ4 (() =>                        (0x000F8000 as uint32_t);)        /*<SQ4[4:0] bits (4th conversion in regular sequence) */
pub macro_rules! ADC_SQR3_SQ4_0 (() =>                      (0x00008000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR3_SQ4_1 (() =>                      (0x00010000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR3_SQ4_2 (() =>                      (0x00020000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR3_SQ4_3 (() =>                      (0x00040000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR3_SQ4_4 (() =>                      (0x00080000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR3_SQ5 (() =>                        (0x01F00000 as uint32_t);)        /*<SQ5[4:0] bits (5th conversion in regular sequence) */
pub macro_rules! ADC_SQR3_SQ5_0 (() =>                      (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR3_SQ5_1 (() =>                      (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR3_SQ5_2 (() =>                      (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR3_SQ5_3 (() =>                      (0x00800000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR3_SQ5_4 (() =>                      (0x01000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_SQR3_SQ6 (() =>                        (0x3E000000 as uint32_t);)        /*<SQ6[4:0] bits (6th conversion in regular sequence) */
pub macro_rules! ADC_SQR3_SQ6_0 (() =>                      (0x02000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_SQR3_SQ6_1 (() =>                      (0x04000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_SQR3_SQ6_2 (() =>                      (0x08000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_SQR3_SQ6_3 (() =>                      (0x10000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_SQR3_SQ6_4 (() =>                      (0x20000000 as uint32_t);)        /*<Bit 4 */

/*  Bit definition for ADC_JSQR register  */
pub macro_rules! ADC_JSQR_JSQ1 (() =>                       (0x0000001F as uint32_t);)        /*<JSQ1[4:0] bits (1st conversion in injected sequence) */  
pub macro_rules! ADC_JSQR_JSQ1_0 (() =>                     (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_JSQR_JSQ1_1 (() =>                     (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_JSQR_JSQ1_2 (() =>                     (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_JSQR_JSQ1_3 (() =>                     (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_JSQR_JSQ1_4 (() =>                     (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_JSQR_JSQ2 (() =>                       (0x000003E0 as uint32_t);)        /*<JSQ2[4:0] bits (2nd conversion in injected sequence) */
pub macro_rules! ADC_JSQR_JSQ2_0 (() =>                     (0x00000020 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_JSQR_JSQ2_1 (() =>                     (0x00000040 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_JSQR_JSQ2_2 (() =>                     (0x00000080 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_JSQR_JSQ2_3 (() =>                     (0x00000100 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_JSQR_JSQ2_4 (() =>                     (0x00000200 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_JSQR_JSQ3 (() =>                       (0x00007C00 as uint32_t);)        /*<JSQ3[4:0] bits (3rd conversion in injected sequence) */
pub macro_rules! ADC_JSQR_JSQ3_0 (() =>                     (0x00000400 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_JSQR_JSQ3_1 (() =>                     (0x00000800 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_JSQR_JSQ3_2 (() =>                     (0x00001000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_JSQR_JSQ3_3 (() =>                     (0x00002000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_JSQR_JSQ3_4 (() =>                     (0x00004000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_JSQR_JSQ4 (() =>                       (0x000F8000 as uint32_t);)        /*<JSQ4[4:0] bits (4th conversion in injected sequence) */
pub macro_rules! ADC_JSQR_JSQ4_0 (() =>                     (0x00008000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_JSQR_JSQ4_1 (() =>                     (0x00010000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_JSQR_JSQ4_2 (() =>                     (0x00020000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_JSQR_JSQ4_3 (() =>                     (0x00040000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_JSQR_JSQ4_4 (() =>                     (0x00080000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_JSQR_JL (() =>                         (0x00300000 as uint32_t);)        /*<JL[1:0] bits (Injected Sequence length) */
pub macro_rules! ADC_JSQR_JL_0 (() =>                       (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_JSQR_JL_1 (() =>                       (0x00200000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for ADC_JDR1 register  */
pub macro_rules! ADC_JDR1_JDATA (() =>                      (0xFFFF as uint16_t);)            /*<Injected data */

/*  Bit definition for ADC_JDR2 register  */
pub macro_rules! ADC_JDR2_JDATA (() =>                      (0xFFFF as uint16_t);)            /*<Injected data */

/*  Bit definition for ADC_JDR3 register  */
pub macro_rules! ADC_JDR3_JDATA (() =>                      (0xFFFF as uint16_t);)            /*<Injected data */

/*  Bit definition for ADC_JDR4 register  */
pub macro_rules! ADC_JDR4_JDATA (() =>                      (0xFFFF as uint16_t);)            /*<Injected data */

/*  Bit definition for ADC_DR register  */
pub macro_rules! ADC_DR_DATA (() =>                         (0x0000FFFF as uint32_t);)        /*<Regular data */
pub macro_rules! ADC_DR_ADC2DATA (() =>                     (0xFFFF0000 as uint32_t);)        /*<ADC2 data */

/*  Bit definition for ADC_CSR register  */
pub macro_rules! ADC_CSR_AWD1 (() =>                        (0x00000001 as uint32_t);)        /*<ADC1 Analog watchdog flag */
pub macro_rules! ADC_CSR_EOC1 (() =>                        (0x00000002 as uint32_t);)        /*<ADC1 End of conversion */
pub macro_rules! ADC_CSR_JEOC1 (() =>                       (0x00000004 as uint32_t);)        /*<ADC1 Injected channel end of conversion */
pub macro_rules! ADC_CSR_JSTRT1 (() =>                      (0x00000008 as uint32_t);)        /*<ADC1 Injected channel Start flag */
pub macro_rules! ADC_CSR_STRT1 (() =>                       (0x00000010 as uint32_t);)        /*<ADC1 Regular channel Start flag */
pub macro_rules! ADC_CSR_DOVR1 (() =>                       (0x00000020 as uint32_t);)        /*<ADC1 DMA overrun  flag */
pub macro_rules! ADC_CSR_AWD2 (() =>                        (0x00000100 as uint32_t);)        /*<ADC2 Analog watchdog flag */
pub macro_rules! ADC_CSR_EOC2 (() =>                        (0x00000200 as uint32_t);)        /*<ADC2 End of conversion */
pub macro_rules! ADC_CSR_JEOC2 (() =>                       (0x00000400 as uint32_t);)        /*<ADC2 Injected channel end of conversion */
pub macro_rules! ADC_CSR_JSTRT2 (() =>                      (0x00000800 as uint32_t);)        /*<ADC2 Injected channel Start flag */
pub macro_rules! ADC_CSR_STRT2 (() =>                       (0x00001000 as uint32_t);)        /*<ADC2 Regular channel Start flag */
pub macro_rules! ADC_CSR_DOVR2 (() =>                       (0x00002000 as uint32_t);)        /*<ADC2 DMA overrun  flag */
pub macro_rules! ADC_CSR_AWD3 (() =>                        (0x00010000 as uint32_t);)        /*<ADC3 Analog watchdog flag */
pub macro_rules! ADC_CSR_EOC3 (() =>                        (0x00020000 as uint32_t);)        /*<ADC3 End of conversion */
pub macro_rules! ADC_CSR_JEOC3 (() =>                       (0x00040000 as uint32_t);)        /*<ADC3 Injected channel end of conversion */
pub macro_rules! ADC_CSR_JSTRT3 (() =>                      (0x00080000 as uint32_t);)        /*<ADC3 Injected channel Start flag */
pub macro_rules! ADC_CSR_STRT3 (() =>                       (0x00100000 as uint32_t);)        /*<ADC3 Regular channel Start flag */
pub macro_rules! ADC_CSR_DOVR3 (() =>                       (0x00200000 as uint32_t);)        /*<ADC3 DMA overrun  flag */

/*  Bit definition for ADC_CCR register  */
pub macro_rules! ADC_CCR_MULTI (() =>                       (0x0000001F as uint32_t);)        /*<MULTI[4:0] bits (Multi-ADC mode selection) */  
pub macro_rules! ADC_CCR_MULTI_0 (() =>                     (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CCR_MULTI_1 (() =>                     (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CCR_MULTI_2 (() =>                     (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_CCR_MULTI_3 (() =>                     (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_CCR_MULTI_4 (() =>                     (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! ADC_CCR_DELAY (() =>                       (0x00000F00 as uint32_t);)        /*<DELAY[3:0] bits (Delay between 2 sampling phases) */  
pub macro_rules! ADC_CCR_DELAY_0 (() =>                     (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CCR_DELAY_1 (() =>                     (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CCR_DELAY_2 (() =>                     (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! ADC_CCR_DELAY_3 (() =>                     (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! ADC_CCR_DDS (() =>                         (0x00002000 as uint32_t);)        /*<DMA disable selection (Multi-ADC mode) */
pub macro_rules! ADC_CCR_DMA (() =>                         (0x0000C000 as uint32_t);)        /*<DMA[1:0] bits (Direct Memory Access mode for multimode) */  
pub macro_rules! ADC_CCR_DMA_0 (() =>                       (0x00004000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CCR_DMA_1 (() =>                       (0x00008000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CCR_ADCPRE (() =>                      (0x00030000 as uint32_t);)        /*<ADCPRE[1:0] bits (ADC prescaler) */  
pub macro_rules! ADC_CCR_ADCPRE_0 (() =>                    (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! ADC_CCR_ADCPRE_1 (() =>                    (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! ADC_CCR_VBATE (() =>                       (0x00400000 as uint32_t);)        /*<VBAT Enable */
pub macro_rules! ADC_CCR_TSVREFE (() =>                     (0x00800000 as uint32_t);)        /*<Temperature Sensor and VREFINT Enable */

/*  Bit definition for ADC_CDR register  */
pub macro_rules! ADC_CDR_DATA1 (() =>                      (0x0000FFFF as uint32_t);)         /*<1st data of a pair of regular conversions */
pub macro_rules! ADC_CDR_DATA2 (() =>                      (0xFFFF0000 as uint32_t);)         /*<2nd data of a pair of regular conversions */


/*                                                                            */
/*                         Controller Area Network                            */
/*                                                                            */

/*<CAN control and status registers */
/*  Bit definition for CAN_MCR register  */
pub macro_rules! CAN_MCR_INRQ (() =>                        (0x0001 as uint16_t);)            /*<Initialization Request */
pub macro_rules! CAN_MCR_SLEEP (() =>                       (0x0002 as uint16_t);)            /*<Sleep Mode Request */
pub macro_rules! CAN_MCR_TXFP (() =>                        (0x0004 as uint16_t);)            /*<Transmit FIFO Priority */
pub macro_rules! CAN_MCR_RFLM (() =>                        (0x0008 as uint16_t);)            /*<Receive FIFO Locked Mode */
pub macro_rules! CAN_MCR_NART (() =>                        (0x0010 as uint16_t);)            /*<No Automatic Retransmission */
pub macro_rules! CAN_MCR_AWUM (() =>                        (0x0020 as uint16_t);)            /*<Automatic Wakeup Mode */
pub macro_rules! CAN_MCR_ABOM (() =>                        (0x0040 as uint16_t);)            /*<Automatic Bus-Off Management */
pub macro_rules! CAN_MCR_TTCM (() =>                        (0x0080 as uint16_t);)            /*<Time Triggered Communication Mode */
pub macro_rules! CAN_MCR_RESET (() =>                       (0x8000 as uint16_t);)            /*<bxCAN software master reset */

/*  Bit definition for CAN_MSR register  */
pub macro_rules! CAN_MSR_INAK (() =>                        (0x0001 as uint16_t);)            /*<Initialization Acknowledge */
pub macro_rules! CAN_MSR_SLAK (() =>                        (0x0002 as uint16_t);)            /*<Sleep Acknowledge */
pub macro_rules! CAN_MSR_ERRI (() =>                        (0x0004 as uint16_t);)            /*<Error Interrupt */
pub macro_rules! CAN_MSR_WKUI (() =>                        (0x0008 as uint16_t);)            /*<Wakeup Interrupt */
pub macro_rules! CAN_MSR_SLAKI (() =>                       (0x0010 as uint16_t);)            /*<Sleep Acknowledge Interrupt */
pub macro_rules! CAN_MSR_TXM (() =>                         (0x0100 as uint16_t);)            /*<Transmit Mode */
pub macro_rules! CAN_MSR_RXM (() =>                         (0x0200 as uint16_t);)            /*<Receive Mode */
pub macro_rules! CAN_MSR_SAMP (() =>                        (0x0400 as uint16_t);)            /*<Last Sample Point */
pub macro_rules! CAN_MSR_RX (() =>                          (0x0800 as uint16_t);)            /*<CAN Rx Signal */

/*  Bit definition for CAN_TSR register  */
pub macro_rules! CAN_TSR_RQCP0 (() =>                       (0x00000001 as uint32_t);)        /*<Request Completed Mailbox0 */
pub macro_rules! CAN_TSR_TXOK0 (() =>                       (0x00000002 as uint32_t);)        /*<Transmission OK of Mailbox0 */
pub macro_rules! CAN_TSR_ALST0 (() =>                       (0x00000004 as uint32_t);)        /*<Arbitration Lost for Mailbox0 */
pub macro_rules! CAN_TSR_TERR0 (() =>                       (0x00000008 as uint32_t);)        /*<Transmission Error of Mailbox0 */
pub macro_rules! CAN_TSR_ABRQ0 (() =>                       (0x00000080 as uint32_t);)        /*<Abort Request for Mailbox0 */
pub macro_rules! CAN_TSR_RQCP1 (() =>                       (0x00000100 as uint32_t);)        /*<Request Completed Mailbox1 */
pub macro_rules! CAN_TSR_TXOK1 (() =>                       (0x00000200 as uint32_t);)        /*<Transmission OK of Mailbox1 */
pub macro_rules! CAN_TSR_ALST1 (() =>                       (0x00000400 as uint32_t);)        /*<Arbitration Lost for Mailbox1 */
pub macro_rules! CAN_TSR_TERR1 (() =>                       (0x00000800 as uint32_t);)        /*<Transmission Error of Mailbox1 */
pub macro_rules! CAN_TSR_ABRQ1 (() =>                       (0x00008000 as uint32_t);)        /*<Abort Request for Mailbox 1 */
pub macro_rules! CAN_TSR_RQCP2 (() =>                       (0x00010000 as uint32_t);)        /*<Request Completed Mailbox2 */
pub macro_rules! CAN_TSR_TXOK2 (() =>                       (0x00020000 as uint32_t);)        /*<Transmission OK of Mailbox 2 */
pub macro_rules! CAN_TSR_ALST2 (() =>                       (0x00040000 as uint32_t);)        /*<Arbitration Lost for mailbox 2 */
pub macro_rules! CAN_TSR_TERR2 (() =>                       (0x00080000 as uint32_t);)        /*<Transmission Error of Mailbox 2 */
pub macro_rules! CAN_TSR_ABRQ2 (() =>                       (0x00800000 as uint32_t);)        /*<Abort Request for Mailbox 2 */
pub macro_rules! CAN_TSR_CODE (() =>                        (0x03000000 as uint32_t);)        /*<Mailbox Code */

pub macro_rules! CAN_TSR_TME (() =>                         (0x1C000000 as uint32_t);)        /*<TME[2:0] bits */
pub macro_rules! CAN_TSR_TME0 (() =>                        (0x04000000 as uint32_t);)        /*<Transmit Mailbox 0 Empty */
pub macro_rules! CAN_TSR_TME1 (() =>                        (0x08000000 as uint32_t);)        /*<Transmit Mailbox 1 Empty */
pub macro_rules! CAN_TSR_TME2 (() =>                        (0x10000000 as uint32_t);)        /*<Transmit Mailbox 2 Empty */

pub macro_rules! CAN_TSR_LOW (() =>                         (0xE0000000 as uint32_t);)        /*<LOW[2:0] bits */
pub macro_rules! CAN_TSR_LOW0 (() =>                        (0x20000000 as uint32_t);)        /*<Lowest Priority Flag for Mailbox 0 */
pub macro_rules! CAN_TSR_LOW1 (() =>                        (0x40000000 as uint32_t);)        /*<Lowest Priority Flag for Mailbox 1 */
pub macro_rules! CAN_TSR_LOW2 (() =>                        (0x80000000 as uint32_t);)        /*<Lowest Priority Flag for Mailbox 2 */

/*  Bit definition for CAN_RF0R register  */
pub macro_rules! CAN_RF0R_FMP0 (() =>                       (0x03 as uint8_t);)               /*<FIFO 0 Message Pending */
pub macro_rules! CAN_RF0R_FULL0 (() =>                      (0x08 as uint8_t);)               /*<FIFO 0 Full */
pub macro_rules! CAN_RF0R_FOVR0 (() =>                      (0x10 as uint8_t);)               /*<FIFO 0 Overrun */
pub macro_rules! CAN_RF0R_RFOM0 (() =>                      (0x20 as uint8_t);)               /*<Release FIFO 0 Output Mailbox */

/*  Bit definition for CAN_RF1R register  */
pub macro_rules! CAN_RF1R_FMP1 (() =>                       (0x03 as uint8_t);)               /*<FIFO 1 Message Pending */
pub macro_rules! CAN_RF1R_FULL1 (() =>                      (0x08 as uint8_t);)               /*<FIFO 1 Full */
pub macro_rules! CAN_RF1R_FOVR1 (() =>                      (0x10 as uint8_t);)               /*<FIFO 1 Overrun */
pub macro_rules! CAN_RF1R_RFOM1 (() =>                      (0x20 as uint8_t);)               /*<Release FIFO 1 Output Mailbox */

/*  Bit definition for CAN_IER register  */
pub macro_rules! CAN_IER_TMEIE (() =>                       (0x00000001 as uint32_t);)        /*<Transmit Mailbox Empty Interrupt Enable */
pub macro_rules! CAN_IER_FMPIE0 (() =>                      (0x00000002 as uint32_t);)        /*<FIFO Message Pending Interrupt Enable */
pub macro_rules! CAN_IER_FFIE0 (() =>                       (0x00000004 as uint32_t);)        /*<FIFO Full Interrupt Enable */
pub macro_rules! CAN_IER_FOVIE0 (() =>                      (0x00000008 as uint32_t);)        /*<FIFO Overrun Interrupt Enable */
pub macro_rules! CAN_IER_FMPIE1 (() =>                      (0x00000010 as uint32_t);)        /*<FIFO Message Pending Interrupt Enable */
pub macro_rules! CAN_IER_FFIE1 (() =>                       (0x00000020 as uint32_t);)        /*<FIFO Full Interrupt Enable */
pub macro_rules! CAN_IER_FOVIE1 (() =>                      (0x00000040 as uint32_t);)        /*<FIFO Overrun Interrupt Enable */
pub macro_rules! CAN_IER_EWGIE (() =>                       (0x00000100 as uint32_t);)        /*<Error Warning Interrupt Enable */
pub macro_rules! CAN_IER_EPVIE (() =>                       (0x00000200 as uint32_t);)        /*<Error Passive Interrupt Enable */
pub macro_rules! CAN_IER_BOFIE (() =>                       (0x00000400 as uint32_t);)        /*<Bus-Off Interrupt Enable */
pub macro_rules! CAN_IER_LECIE (() =>                       (0x00000800 as uint32_t);)        /*<Last Error Code Interrupt Enable */
pub macro_rules! CAN_IER_ERRIE (() =>                       (0x00008000 as uint32_t);)        /*<Error Interrupt Enable */
pub macro_rules! CAN_IER_WKUIE (() =>                       (0x00010000 as uint32_t);)        /*<Wakeup Interrupt Enable */
pub macro_rules! CAN_IER_SLKIE (() =>                       (0x00020000 as uint32_t);)        /*<Sleep Interrupt Enable */

/*  Bit definition for CAN_ESR register  */
pub macro_rules! CAN_ESR_EWGF (() =>                        (0x00000001 as uint32_t);)        /*<Error Warning Flag */
pub macro_rules! CAN_ESR_EPVF (() =>                        (0x00000002 as uint32_t);)        /*<Error Passive Flag */
pub macro_rules! CAN_ESR_BOFF (() =>                        (0x00000004 as uint32_t);)        /*<Bus-Off Flag */

pub macro_rules! CAN_ESR_LEC (() =>                         (0x00000070 as uint32_t);)        /*<LEC[2:0] bits (Last Error Code) */
pub macro_rules! CAN_ESR_LEC_0 (() =>                       (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! CAN_ESR_LEC_1 (() =>                       (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! CAN_ESR_LEC_2 (() =>                       (0x00000040 as uint32_t);)        /*<Bit 2 */

pub macro_rules! CAN_ESR_TEC (() =>                         (0x00FF0000 as uint32_t);)        /*<Least significant byte of the 9-bit Transmit Error Counter */
pub macro_rules! CAN_ESR_REC (() =>                         (0xFF000000 as uint32_t);)        /*<Receive Error Counter */

/*  Bit definition for CAN_BTR register  */
pub macro_rules! CAN_BTR_BRP (() =>                         (0x000003FF as uint32_t);)        /*<Baud Rate Prescaler */
pub macro_rules! CAN_BTR_TS1 (() =>                         (0x000F0000 as uint32_t);)        /*<Time Segment 1 */
pub macro_rules! CAN_BTR_TS2 (() =>                         (0x00700000 as uint32_t);)        /*<Time Segment 2 */
pub macro_rules! CAN_BTR_SJW (() =>                         (0x03000000 as uint32_t);)        /*<Resynchronization Jump Width */
pub macro_rules! CAN_BTR_LBKM (() =>                        (0x40000000 as uint32_t);)        /*<Loop Back Mode (Debug) */
pub macro_rules! CAN_BTR_SILM (() =>                        (0x80000000 as uint32_t);)        /*<Silent Mode */

/*<Mailbox registers */
/*  Bit definition for CAN_TI0R register  */
pub macro_rules! CAN_TI0R_TXRQ (() =>                       (0x00000001 as uint32_t);)        /*<Transmit Mailbox Request */
pub macro_rules! CAN_TI0R_RTR (() =>                        (0x00000002 as uint32_t);)        /*<Remote Transmission Request */
pub macro_rules! CAN_TI0R_IDE (() =>                        (0x00000004 as uint32_t);)        /*<Identifier Extension */
pub macro_rules! CAN_TI0R_EXID (() =>                       (0x001FFFF8 as uint32_t);)        /*<Extended Identifier */
pub macro_rules! CAN_TI0R_STID (() =>                       (0xFFE00000 as uint32_t);)        /*<Standard Identifier or Extended Identifier */

/*  Bit definition for CAN_TDT0R register  */
pub macro_rules! CAN_TDT0R_DLC (() =>                       (0x0000000F as uint32_t);)        /*<Data Length Code */
pub macro_rules! CAN_TDT0R_TGT (() =>                       (0x00000100 as uint32_t);)        /*<Transmit Global Time */
pub macro_rules! CAN_TDT0R_TIME (() =>                      (0xFFFF0000 as uint32_t);)        /*<Message Time Stamp */

/*  Bit definition for CAN_TDL0R register  */
pub macro_rules! CAN_TDL0R_DATA0 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 0 */
pub macro_rules! CAN_TDL0R_DATA1 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 1 */
pub macro_rules! CAN_TDL0R_DATA2 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 2 */
pub macro_rules! CAN_TDL0R_DATA3 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 3 */

/*  Bit definition for CAN_TDH0R register  */
pub macro_rules! CAN_TDH0R_DATA4 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 4 */
pub macro_rules! CAN_TDH0R_DATA5 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 5 */
pub macro_rules! CAN_TDH0R_DATA6 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 6 */
pub macro_rules! CAN_TDH0R_DATA7 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 7 */

/*  Bit definition for CAN_TI1R register  */
pub macro_rules! CAN_TI1R_TXRQ (() =>                       (0x00000001 as uint32_t);)        /*<Transmit Mailbox Request */
pub macro_rules! CAN_TI1R_RTR (() =>                        (0x00000002 as uint32_t);)        /*<Remote Transmission Request */
pub macro_rules! CAN_TI1R_IDE (() =>                        (0x00000004 as uint32_t);)        /*<Identifier Extension */
pub macro_rules! CAN_TI1R_EXID (() =>                       (0x001FFFF8 as uint32_t);)        /*<Extended Identifier */
pub macro_rules! CAN_TI1R_STID (() =>                       (0xFFE00000 as uint32_t);)        /*<Standard Identifier or Extended Identifier */

/*  Bit definition for CAN_TDT1R register  */
pub macro_rules! CAN_TDT1R_DLC (() =>                       (0x0000000F as uint32_t);)        /*<Data Length Code */
pub macro_rules! CAN_TDT1R_TGT (() =>                       (0x00000100 as uint32_t);)        /*<Transmit Global Time */
pub macro_rules! CAN_TDT1R_TIME (() =>                      (0xFFFF0000 as uint32_t);)        /*<Message Time Stamp */

/*  Bit definition for CAN_TDL1R register  */
pub macro_rules! CAN_TDL1R_DATA0 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 0 */
pub macro_rules! CAN_TDL1R_DATA1 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 1 */
pub macro_rules! CAN_TDL1R_DATA2 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 2 */
pub macro_rules! CAN_TDL1R_DATA3 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 3 */

/*  Bit definition for CAN_TDH1R register  */
pub macro_rules! CAN_TDH1R_DATA4 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 4 */
pub macro_rules! CAN_TDH1R_DATA5 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 5 */
pub macro_rules! CAN_TDH1R_DATA6 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 6 */
pub macro_rules! CAN_TDH1R_DATA7 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 7 */

/*  Bit definition for CAN_TI2R register  */
pub macro_rules! CAN_TI2R_TXRQ (() =>                       (0x00000001 as uint32_t);)        /*<Transmit Mailbox Request */
pub macro_rules! CAN_TI2R_RTR (() =>                        (0x00000002 as uint32_t);)        /*<Remote Transmission Request */
pub macro_rules! CAN_TI2R_IDE (() =>                        (0x00000004 as uint32_t);)        /*<Identifier Extension */
pub macro_rules! CAN_TI2R_EXID (() =>                       (0x001FFFF8 as uint32_t);)        /*<Extended identifier */
pub macro_rules! CAN_TI2R_STID (() =>                       (0xFFE00000 as uint32_t);)        /*<Standard Identifier or Extended Identifier */

/*  Bit definition for CAN_TDT2R register  */  
pub macro_rules! CAN_TDT2R_DLC (() =>                       (0x0000000F as uint32_t);)        /*<Data Length Code */
pub macro_rules! CAN_TDT2R_TGT (() =>                       (0x00000100 as uint32_t);)        /*<Transmit Global Time */
pub macro_rules! CAN_TDT2R_TIME (() =>                      (0xFFFF0000 as uint32_t);)        /*<Message Time Stamp */

/*  Bit definition for CAN_TDL2R register  */
pub macro_rules! CAN_TDL2R_DATA0 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 0 */
pub macro_rules! CAN_TDL2R_DATA1 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 1 */
pub macro_rules! CAN_TDL2R_DATA2 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 2 */
pub macro_rules! CAN_TDL2R_DATA3 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 3 */

/*  Bit definition for CAN_TDH2R register  */
pub macro_rules! CAN_TDH2R_DATA4 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 4 */
pub macro_rules! CAN_TDH2R_DATA5 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 5 */
pub macro_rules! CAN_TDH2R_DATA6 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 6 */
pub macro_rules! CAN_TDH2R_DATA7 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 7 */

/*  Bit definition for CAN_RI0R register  */
pub macro_rules! CAN_RI0R_RTR (() =>                        (0x00000002 as uint32_t);)        /*<Remote Transmission Request */
pub macro_rules! CAN_RI0R_IDE (() =>                        (0x00000004 as uint32_t);)        /*<Identifier Extension */
pub macro_rules! CAN_RI0R_EXID (() =>                       (0x001FFFF8 as uint32_t);)        /*<Extended Identifier */
pub macro_rules! CAN_RI0R_STID (() =>                       (0xFFE00000 as uint32_t);)        /*<Standard Identifier or Extended Identifier */

/*  Bit definition for CAN_RDT0R register  */
pub macro_rules! CAN_RDT0R_DLC (() =>                       (0x0000000F as uint32_t);)        /*<Data Length Code */
pub macro_rules! CAN_RDT0R_FMI (() =>                       (0x0000FF00 as uint32_t);)        /*<Filter Match Index */
pub macro_rules! CAN_RDT0R_TIME (() =>                      (0xFFFF0000 as uint32_t);)        /*<Message Time Stamp */

/*  Bit definition for CAN_RDL0R register  */
pub macro_rules! CAN_RDL0R_DATA0 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 0 */
pub macro_rules! CAN_RDL0R_DATA1 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 1 */
pub macro_rules! CAN_RDL0R_DATA2 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 2 */
pub macro_rules! CAN_RDL0R_DATA3 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 3 */

/*  Bit definition for CAN_RDH0R register  */
pub macro_rules! CAN_RDH0R_DATA4 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 4 */
pub macro_rules! CAN_RDH0R_DATA5 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 5 */
pub macro_rules! CAN_RDH0R_DATA6 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 6 */
pub macro_rules! CAN_RDH0R_DATA7 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 7 */

/*  Bit definition for CAN_RI1R register  */
pub macro_rules! CAN_RI1R_RTR (() =>                        (0x00000002 as uint32_t);)        /*<Remote Transmission Request */
pub macro_rules! CAN_RI1R_IDE (() =>                        (0x00000004 as uint32_t);)        /*<Identifier Extension */
pub macro_rules! CAN_RI1R_EXID (() =>                       (0x001FFFF8 as uint32_t);)        /*<Extended identifier */
pub macro_rules! CAN_RI1R_STID (() =>                       (0xFFE00000 as uint32_t);)        /*<Standard Identifier or Extended Identifier */

/*  Bit definition for CAN_RDT1R register  */
pub macro_rules! CAN_RDT1R_DLC (() =>                       (0x0000000F as uint32_t);)        /*<Data Length Code */
pub macro_rules! CAN_RDT1R_FMI (() =>                       (0x0000FF00 as uint32_t);)        /*<Filter Match Index */
pub macro_rules! CAN_RDT1R_TIME (() =>                      (0xFFFF0000 as uint32_t);)        /*<Message Time Stamp */

/*  Bit definition for CAN_RDL1R register  */
pub macro_rules! CAN_RDL1R_DATA0 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 0 */
pub macro_rules! CAN_RDL1R_DATA1 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 1 */
pub macro_rules! CAN_RDL1R_DATA2 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 2 */
pub macro_rules! CAN_RDL1R_DATA3 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 3 */

/*  Bit definition for CAN_RDH1R register  */
pub macro_rules! CAN_RDH1R_DATA4 (() =>                     (0x000000FF as uint32_t);)        /*<Data byte 4 */
pub macro_rules! CAN_RDH1R_DATA5 (() =>                     (0x0000FF00 as uint32_t);)        /*<Data byte 5 */
pub macro_rules! CAN_RDH1R_DATA6 (() =>                     (0x00FF0000 as uint32_t);)        /*<Data byte 6 */
pub macro_rules! CAN_RDH1R_DATA7 (() =>                     (0xFF000000 as uint32_t);)        /*<Data byte 7 */

/*<CAN filter registers */
/*  Bit definition for CAN_FMR register  */
pub macro_rules! CAN_FMR_FINIT (() =>                       (0x01 as uint8_t);)               /*<Filter Init Mode */

/*  Bit definition for CAN_FM1R register  */
pub macro_rules! CAN_FM1R_FBM (() =>                        (0x3FFF as uint16_t);)            /*<Filter Mode */
pub macro_rules! CAN_FM1R_FBM0 (() =>                       (0x0001 as uint16_t);)            /*<Filter Init Mode bit 0 */
pub macro_rules! CAN_FM1R_FBM1 (() =>                       (0x0002 as uint16_t);)            /*<Filter Init Mode bit 1 */
pub macro_rules! CAN_FM1R_FBM2 (() =>                       (0x0004 as uint16_t);)            /*<Filter Init Mode bit 2 */
pub macro_rules! CAN_FM1R_FBM3 (() =>                       (0x0008 as uint16_t);)            /*<Filter Init Mode bit 3 */
pub macro_rules! CAN_FM1R_FBM4 (() =>                       (0x0010 as uint16_t);)            /*<Filter Init Mode bit 4 */
pub macro_rules! CAN_FM1R_FBM5 (() =>                       (0x0020 as uint16_t);)            /*<Filter Init Mode bit 5 */
pub macro_rules! CAN_FM1R_FBM6 (() =>                       (0x0040 as uint16_t);)            /*<Filter Init Mode bit 6 */
pub macro_rules! CAN_FM1R_FBM7 (() =>                       (0x0080 as uint16_t);)            /*<Filter Init Mode bit 7 */
pub macro_rules! CAN_FM1R_FBM8 (() =>                       (0x0100 as uint16_t);)            /*<Filter Init Mode bit 8 */
pub macro_rules! CAN_FM1R_FBM9 (() =>                       (0x0200 as uint16_t);)            /*<Filter Init Mode bit 9 */
pub macro_rules! CAN_FM1R_FBM10 (() =>                      (0x0400 as uint16_t);)            /*<Filter Init Mode bit 10 */
pub macro_rules! CAN_FM1R_FBM11 (() =>                      (0x0800 as uint16_t);)            /*<Filter Init Mode bit 11 */
pub macro_rules! CAN_FM1R_FBM12 (() =>                      (0x1000 as uint16_t);)            /*<Filter Init Mode bit 12 */
pub macro_rules! CAN_FM1R_FBM13 (() =>                      (0x2000 as uint16_t);)            /*<Filter Init Mode bit 13 */

/*  Bit definition for CAN_FS1R register  */
pub macro_rules! CAN_FS1R_FSC (() =>                        (0x3FFF as uint16_t);)            /*<Filter Scale Configuration */
pub macro_rules! CAN_FS1R_FSC0 (() =>                       (0x0001 as uint16_t);)            /*<Filter Scale Configuration bit 0 */
pub macro_rules! CAN_FS1R_FSC1 (() =>                       (0x0002 as uint16_t);)            /*<Filter Scale Configuration bit 1 */
pub macro_rules! CAN_FS1R_FSC2 (() =>                       (0x0004 as uint16_t);)            /*<Filter Scale Configuration bit 2 */
pub macro_rules! CAN_FS1R_FSC3 (() =>                       (0x0008 as uint16_t);)            /*<Filter Scale Configuration bit 3 */
pub macro_rules! CAN_FS1R_FSC4 (() =>                       (0x0010 as uint16_t);)            /*<Filter Scale Configuration bit 4 */
pub macro_rules! CAN_FS1R_FSC5 (() =>                       (0x0020 as uint16_t);)            /*<Filter Scale Configuration bit 5 */
pub macro_rules! CAN_FS1R_FSC6 (() =>                       (0x0040 as uint16_t);)            /*<Filter Scale Configuration bit 6 */
pub macro_rules! CAN_FS1R_FSC7 (() =>                       (0x0080 as uint16_t);)            /*<Filter Scale Configuration bit 7 */
pub macro_rules! CAN_FS1R_FSC8 (() =>                       (0x0100 as uint16_t);)            /*<Filter Scale Configuration bit 8 */
pub macro_rules! CAN_FS1R_FSC9 (() =>                       (0x0200 as uint16_t);)            /*<Filter Scale Configuration bit 9 */
pub macro_rules! CAN_FS1R_FSC10 (() =>                      (0x0400 as uint16_t);)            /*<Filter Scale Configuration bit 10 */
pub macro_rules! CAN_FS1R_FSC11 (() =>                      (0x0800 as uint16_t);)            /*<Filter Scale Configuration bit 11 */
pub macro_rules! CAN_FS1R_FSC12 (() =>                      (0x1000 as uint16_t);)            /*<Filter Scale Configuration bit 12 */
pub macro_rules! CAN_FS1R_FSC13 (() =>                      (0x2000 as uint16_t);)            /*<Filter Scale Configuration bit 13 */

/*  Bit definition for CAN_FFA1R register  */
pub macro_rules! CAN_FFA1R_FFA (() =>                       (0x3FFF as uint16_t);)            /*<Filter FIFO Assignment */
pub macro_rules! CAN_FFA1R_FFA0 (() =>                      (0x0001 as uint16_t);)            /*<Filter FIFO Assignment for Filter 0 */
pub macro_rules! CAN_FFA1R_FFA1 (() =>                      (0x0002 as uint16_t);)            /*<Filter FIFO Assignment for Filter 1 */
pub macro_rules! CAN_FFA1R_FFA2 (() =>                      (0x0004 as uint16_t);)            /*<Filter FIFO Assignment for Filter 2 */
pub macro_rules! CAN_FFA1R_FFA3 (() =>                      (0x0008 as uint16_t);)            /*<Filter FIFO Assignment for Filter 3 */
pub macro_rules! CAN_FFA1R_FFA4 (() =>                      (0x0010 as uint16_t);)            /*<Filter FIFO Assignment for Filter 4 */
pub macro_rules! CAN_FFA1R_FFA5 (() =>                      (0x0020 as uint16_t);)            /*<Filter FIFO Assignment for Filter 5 */
pub macro_rules! CAN_FFA1R_FFA6 (() =>                      (0x0040 as uint16_t);)            /*<Filter FIFO Assignment for Filter 6 */
pub macro_rules! CAN_FFA1R_FFA7 (() =>                      (0x0080 as uint16_t);)            /*<Filter FIFO Assignment for Filter 7 */
pub macro_rules! CAN_FFA1R_FFA8 (() =>                      (0x0100 as uint16_t);)            /*<Filter FIFO Assignment for Filter 8 */
pub macro_rules! CAN_FFA1R_FFA9 (() =>                      (0x0200 as uint16_t);)            /*<Filter FIFO Assignment for Filter 9 */
pub macro_rules! CAN_FFA1R_FFA10 (() =>                     (0x0400 as uint16_t);)            /*<Filter FIFO Assignment for Filter 10 */
pub macro_rules! CAN_FFA1R_FFA11 (() =>                     (0x0800 as uint16_t);)            /*<Filter FIFO Assignment for Filter 11 */
pub macro_rules! CAN_FFA1R_FFA12 (() =>                     (0x1000 as uint16_t);)            /*<Filter FIFO Assignment for Filter 12 */
pub macro_rules! CAN_FFA1R_FFA13 (() =>                     (0x2000 as uint16_t);)            /*<Filter FIFO Assignment for Filter 13 */

/*  Bit definition for CAN_FA1R register  */
pub macro_rules! CAN_FA1R_FACT (() =>                       (0x3FFF as uint16_t);)            /*<Filter Active */
pub macro_rules! CAN_FA1R_FACT0 (() =>                      (0x0001 as uint16_t);)            /*<Filter 0 Active */
pub macro_rules! CAN_FA1R_FACT1 (() =>                      (0x0002 as uint16_t);)            /*<Filter 1 Active */
pub macro_rules! CAN_FA1R_FACT2 (() =>                      (0x0004 as uint16_t);)            /*<Filter 2 Active */
pub macro_rules! CAN_FA1R_FACT3 (() =>                      (0x0008 as uint16_t);)            /*<Filter 3 Active */
pub macro_rules! CAN_FA1R_FACT4 (() =>                      (0x0010 as uint16_t);)            /*<Filter 4 Active */
pub macro_rules! CAN_FA1R_FACT5 (() =>                      (0x0020 as uint16_t);)            /*<Filter 5 Active */
pub macro_rules! CAN_FA1R_FACT6 (() =>                      (0x0040 as uint16_t);)            /*<Filter 6 Active */
pub macro_rules! CAN_FA1R_FACT7 (() =>                      (0x0080 as uint16_t);)            /*<Filter 7 Active */
pub macro_rules! CAN_FA1R_FACT8 (() =>                      (0x0100 as uint16_t);)            /*<Filter 8 Active */
pub macro_rules! CAN_FA1R_FACT9 (() =>                      (0x0200 as uint16_t);)            /*<Filter 9 Active */
pub macro_rules! CAN_FA1R_FACT10 (() =>                     (0x0400 as uint16_t);)            /*<Filter 10 Active */
pub macro_rules! CAN_FA1R_FACT11 (() =>                     (0x0800 as uint16_t);)            /*<Filter 11 Active */
pub macro_rules! CAN_FA1R_FACT12 (() =>                     (0x1000 as uint16_t);)            /*<Filter 12 Active */
pub macro_rules! CAN_FA1R_FACT13 (() =>                     (0x2000 as uint16_t);)            /*<Filter 13 Active */

/*  Bit definition for CAN_F0R1 register  */
pub macro_rules! CAN_F0R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F0R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F0R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F0R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F0R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F0R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F0R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F0R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F0R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F0R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F0R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F0R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F0R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F0R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F0R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F0R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F0R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F0R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F0R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F0R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F0R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F0R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F0R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F0R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F0R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F0R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F0R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F0R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F0R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F0R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F0R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F0R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F1R1 register  */
pub macro_rules! CAN_F1R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F1R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F1R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F1R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F1R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F1R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F1R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F1R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F1R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F1R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F1R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F1R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F1R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F1R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F1R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F1R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F1R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F1R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F1R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F1R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F1R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F1R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F1R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F1R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F1R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F1R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F1R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F1R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F1R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F1R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F1R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F1R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F2R1 register  */
pub macro_rules! CAN_F2R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F2R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F2R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F2R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F2R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F2R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F2R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F2R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F2R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F2R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F2R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F2R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F2R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F2R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F2R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F2R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F2R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F2R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F2R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F2R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F2R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F2R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F2R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F2R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F2R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F2R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F2R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F2R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F2R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F2R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F2R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F2R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F3R1 register  */
pub macro_rules! CAN_F3R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F3R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F3R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F3R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F3R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F3R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F3R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F3R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F3R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F3R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F3R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F3R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F3R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F3R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F3R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F3R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F3R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F3R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F3R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F3R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F3R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F3R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F3R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F3R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F3R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F3R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F3R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F3R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F3R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F3R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F3R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F3R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F4R1 register  */
pub macro_rules! CAN_F4R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F4R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F4R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F4R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F4R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F4R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F4R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F4R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F4R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F4R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F4R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F4R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F4R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F4R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F4R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F4R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F4R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F4R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F4R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F4R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F4R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F4R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F4R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F4R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F4R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F4R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F4R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F4R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F4R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F4R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F4R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F4R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F5R1 register  */
pub macro_rules! CAN_F5R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F5R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F5R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F5R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F5R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F5R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F5R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F5R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F5R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F5R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F5R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F5R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F5R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F5R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F5R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F5R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F5R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F5R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F5R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F5R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F5R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F5R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F5R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F5R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F5R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F5R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F5R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F5R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F5R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F5R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F5R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F5R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F6R1 register  */
pub macro_rules! CAN_F6R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F6R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F6R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F6R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F6R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F6R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F6R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F6R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F6R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F6R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F6R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F6R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F6R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F6R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F6R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F6R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F6R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F6R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F6R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F6R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F6R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F6R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F6R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F6R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F6R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F6R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F6R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F6R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F6R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F6R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F6R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F6R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F7R1 register  */
pub macro_rules! CAN_F7R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F7R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F7R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F7R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F7R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F7R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F7R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F7R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F7R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F7R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F7R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F7R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F7R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F7R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F7R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F7R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F7R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F7R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F7R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F7R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F7R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F7R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F7R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F7R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F7R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F7R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F7R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F7R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F7R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F7R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F7R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F7R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F8R1 register  */
pub macro_rules! CAN_F8R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F8R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F8R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F8R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F8R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F8R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F8R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F8R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F8R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F8R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F8R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F8R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F8R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F8R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F8R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F8R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F8R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F8R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F8R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F8R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F8R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F8R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F8R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F8R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F8R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F8R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F8R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F8R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F8R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F8R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F8R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F8R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F9R1 register  */
pub macro_rules! CAN_F9R1_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F9R1_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F9R1_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F9R1_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F9R1_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F9R1_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F9R1_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F9R1_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F9R1_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F9R1_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F9R1_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F9R1_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F9R1_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F9R1_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F9R1_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F9R1_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F9R1_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F9R1_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F9R1_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F9R1_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F9R1_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F9R1_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F9R1_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F9R1_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F9R1_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F9R1_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F9R1_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F9R1_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F9R1_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F9R1_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F9R1_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F9R1_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F10R1 register  */
pub macro_rules! CAN_F10R1_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F10R1_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F10R1_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F10R1_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F10R1_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F10R1_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F10R1_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F10R1_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F10R1_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F10R1_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F10R1_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F10R1_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F10R1_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F10R1_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F10R1_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F10R1_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F10R1_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F10R1_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F10R1_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F10R1_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F10R1_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F10R1_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F10R1_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F10R1_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F10R1_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F10R1_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F10R1_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F10R1_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F10R1_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F10R1_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F10R1_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F10R1_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F11R1 register  */
pub macro_rules! CAN_F11R1_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F11R1_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F11R1_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F11R1_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F11R1_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F11R1_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F11R1_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F11R1_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F11R1_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F11R1_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F11R1_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F11R1_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F11R1_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F11R1_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F11R1_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F11R1_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F11R1_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F11R1_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F11R1_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F11R1_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F11R1_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F11R1_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F11R1_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F11R1_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F11R1_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F11R1_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F11R1_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F11R1_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F11R1_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F11R1_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F11R1_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F11R1_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F12R1 register  */
pub macro_rules! CAN_F12R1_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F12R1_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F12R1_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F12R1_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F12R1_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F12R1_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F12R1_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F12R1_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F12R1_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F12R1_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F12R1_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F12R1_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F12R1_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F12R1_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F12R1_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F12R1_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F12R1_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F12R1_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F12R1_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F12R1_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F12R1_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F12R1_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F12R1_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F12R1_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F12R1_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F12R1_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F12R1_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F12R1_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F12R1_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F12R1_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F12R1_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F12R1_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F13R1 register  */
pub macro_rules! CAN_F13R1_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F13R1_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F13R1_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F13R1_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F13R1_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F13R1_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F13R1_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F13R1_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F13R1_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F13R1_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F13R1_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F13R1_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F13R1_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F13R1_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F13R1_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F13R1_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F13R1_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F13R1_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F13R1_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F13R1_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F13R1_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F13R1_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F13R1_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F13R1_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F13R1_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F13R1_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F13R1_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F13R1_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F13R1_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F13R1_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F13R1_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F13R1_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F0R2 register  */
pub macro_rules! CAN_F0R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F0R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F0R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F0R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F0R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F0R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F0R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F0R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F0R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F0R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F0R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F0R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F0R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F0R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F0R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F0R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F0R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F0R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F0R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F0R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F0R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F0R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F0R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F0R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F0R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F0R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F0R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F0R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F0R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F0R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F0R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F0R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F1R2 register  */
pub macro_rules! CAN_F1R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F1R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F1R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F1R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F1R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F1R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F1R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F1R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F1R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F1R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F1R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F1R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F1R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F1R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F1R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F1R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F1R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F1R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F1R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F1R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F1R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F1R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F1R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F1R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F1R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F1R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F1R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F1R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F1R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F1R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F1R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F1R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F2R2 register  */
pub macro_rules! CAN_F2R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F2R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F2R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F2R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F2R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F2R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F2R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F2R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F2R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F2R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F2R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F2R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F2R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F2R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F2R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F2R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F2R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F2R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F2R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F2R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F2R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F2R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F2R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F2R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F2R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F2R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F2R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F2R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F2R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F2R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F2R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F2R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F3R2 register  */
pub macro_rules! CAN_F3R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F3R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F3R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F3R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F3R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F3R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F3R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F3R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F3R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F3R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F3R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F3R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F3R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F3R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F3R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F3R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F3R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F3R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F3R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F3R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F3R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F3R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F3R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F3R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F3R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F3R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F3R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F3R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F3R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F3R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F3R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F3R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F4R2 register  */
pub macro_rules! CAN_F4R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F4R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F4R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F4R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F4R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F4R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F4R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F4R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F4R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F4R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F4R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F4R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F4R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F4R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F4R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F4R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F4R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F4R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F4R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F4R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F4R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F4R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F4R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F4R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F4R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F4R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F4R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F4R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F4R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F4R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F4R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F4R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F5R2 register  */
pub macro_rules! CAN_F5R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F5R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F5R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F5R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F5R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F5R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F5R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F5R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F5R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F5R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F5R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F5R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F5R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F5R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F5R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F5R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F5R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F5R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F5R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F5R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F5R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F5R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F5R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F5R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F5R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F5R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F5R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F5R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F5R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F5R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F5R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F5R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F6R2 register  */
pub macro_rules! CAN_F6R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F6R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F6R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F6R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F6R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F6R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F6R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F6R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F6R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F6R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F6R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F6R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F6R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F6R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F6R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F6R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F6R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F6R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F6R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F6R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F6R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F6R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F6R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F6R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F6R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F6R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F6R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F6R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F6R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F6R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F6R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F6R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F7R2 register  */
pub macro_rules! CAN_F7R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F7R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F7R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F7R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F7R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F7R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F7R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F7R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F7R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F7R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F7R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F7R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F7R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F7R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F7R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F7R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F7R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F7R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F7R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F7R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F7R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F7R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F7R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F7R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F7R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F7R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F7R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F7R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F7R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F7R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F7R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F7R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F8R2 register  */
pub macro_rules! CAN_F8R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F8R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F8R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F8R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F8R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F8R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F8R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F8R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F8R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F8R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F8R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F8R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F8R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F8R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F8R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F8R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F8R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F8R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F8R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F8R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F8R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F8R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F8R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F8R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F8R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F8R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F8R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F8R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F8R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F8R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F8R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F8R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F9R2 register  */
pub macro_rules! CAN_F9R2_FB0 (() =>                        (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F9R2_FB1 (() =>                        (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F9R2_FB2 (() =>                        (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F9R2_FB3 (() =>                        (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F9R2_FB4 (() =>                        (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F9R2_FB5 (() =>                        (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F9R2_FB6 (() =>                        (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F9R2_FB7 (() =>                        (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F9R2_FB8 (() =>                        (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F9R2_FB9 (() =>                        (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F9R2_FB10 (() =>                       (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F9R2_FB11 (() =>                       (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F9R2_FB12 (() =>                       (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F9R2_FB13 (() =>                       (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F9R2_FB14 (() =>                       (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F9R2_FB15 (() =>                       (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F9R2_FB16 (() =>                       (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F9R2_FB17 (() =>                       (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F9R2_FB18 (() =>                       (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F9R2_FB19 (() =>                       (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F9R2_FB20 (() =>                       (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F9R2_FB21 (() =>                       (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F9R2_FB22 (() =>                       (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F9R2_FB23 (() =>                       (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F9R2_FB24 (() =>                       (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F9R2_FB25 (() =>                       (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F9R2_FB26 (() =>                       (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F9R2_FB27 (() =>                       (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F9R2_FB28 (() =>                       (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F9R2_FB29 (() =>                       (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F9R2_FB30 (() =>                       (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F9R2_FB31 (() =>                       (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F10R2 register  */
pub macro_rules! CAN_F10R2_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F10R2_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F10R2_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F10R2_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F10R2_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F10R2_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F10R2_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F10R2_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F10R2_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F10R2_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F10R2_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F10R2_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F10R2_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F10R2_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F10R2_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F10R2_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F10R2_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F10R2_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F10R2_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F10R2_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F10R2_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F10R2_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F10R2_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F10R2_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F10R2_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F10R2_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F10R2_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F10R2_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F10R2_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F10R2_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F10R2_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F10R2_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F11R2 register  */
pub macro_rules! CAN_F11R2_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F11R2_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F11R2_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F11R2_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F11R2_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F11R2_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F11R2_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F11R2_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F11R2_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F11R2_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F11R2_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F11R2_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F11R2_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F11R2_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F11R2_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F11R2_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F11R2_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F11R2_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F11R2_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F11R2_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F11R2_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F11R2_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F11R2_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F11R2_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F11R2_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F11R2_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F11R2_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F11R2_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F11R2_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F11R2_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F11R2_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F11R2_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F12R2 register  */
pub macro_rules! CAN_F12R2_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F12R2_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F12R2_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F12R2_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F12R2_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F12R2_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F12R2_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F12R2_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F12R2_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F12R2_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F12R2_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F12R2_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F12R2_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F12R2_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F12R2_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F12R2_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F12R2_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F12R2_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F12R2_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F12R2_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F12R2_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F12R2_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F12R2_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F12R2_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F12R2_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F12R2_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F12R2_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F12R2_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F12R2_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F12R2_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F12R2_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F12R2_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */

/*  Bit definition for CAN_F13R2 register  */
pub macro_rules! CAN_F13R2_FB0 (() =>                       (0x00000001 as uint32_t);)        /*<Filter bit 0 */
pub macro_rules! CAN_F13R2_FB1 (() =>                       (0x00000002 as uint32_t);)        /*<Filter bit 1 */
pub macro_rules! CAN_F13R2_FB2 (() =>                       (0x00000004 as uint32_t);)        /*<Filter bit 2 */
pub macro_rules! CAN_F13R2_FB3 (() =>                       (0x00000008 as uint32_t);)        /*<Filter bit 3 */
pub macro_rules! CAN_F13R2_FB4 (() =>                       (0x00000010 as uint32_t);)        /*<Filter bit 4 */
pub macro_rules! CAN_F13R2_FB5 (() =>                       (0x00000020 as uint32_t);)        /*<Filter bit 5 */
pub macro_rules! CAN_F13R2_FB6 (() =>                       (0x00000040 as uint32_t);)        /*<Filter bit 6 */
pub macro_rules! CAN_F13R2_FB7 (() =>                       (0x00000080 as uint32_t);)        /*<Filter bit 7 */
pub macro_rules! CAN_F13R2_FB8 (() =>                       (0x00000100 as uint32_t);)        /*<Filter bit 8 */
pub macro_rules! CAN_F13R2_FB9 (() =>                       (0x00000200 as uint32_t);)        /*<Filter bit 9 */
pub macro_rules! CAN_F13R2_FB10 (() =>                      (0x00000400 as uint32_t);)        /*<Filter bit 10 */
pub macro_rules! CAN_F13R2_FB11 (() =>                      (0x00000800 as uint32_t);)        /*<Filter bit 11 */
pub macro_rules! CAN_F13R2_FB12 (() =>                      (0x00001000 as uint32_t);)        /*<Filter bit 12 */
pub macro_rules! CAN_F13R2_FB13 (() =>                      (0x00002000 as uint32_t);)        /*<Filter bit 13 */
pub macro_rules! CAN_F13R2_FB14 (() =>                      (0x00004000 as uint32_t);)        /*<Filter bit 14 */
pub macro_rules! CAN_F13R2_FB15 (() =>                      (0x00008000 as uint32_t);)        /*<Filter bit 15 */
pub macro_rules! CAN_F13R2_FB16 (() =>                      (0x00010000 as uint32_t);)        /*<Filter bit 16 */
pub macro_rules! CAN_F13R2_FB17 (() =>                      (0x00020000 as uint32_t);)        /*<Filter bit 17 */
pub macro_rules! CAN_F13R2_FB18 (() =>                      (0x00040000 as uint32_t);)        /*<Filter bit 18 */
pub macro_rules! CAN_F13R2_FB19 (() =>                      (0x00080000 as uint32_t);)        /*<Filter bit 19 */
pub macro_rules! CAN_F13R2_FB20 (() =>                      (0x00100000 as uint32_t);)        /*<Filter bit 20 */
pub macro_rules! CAN_F13R2_FB21 (() =>                      (0x00200000 as uint32_t);)        /*<Filter bit 21 */
pub macro_rules! CAN_F13R2_FB22 (() =>                      (0x00400000 as uint32_t);)        /*<Filter bit 22 */
pub macro_rules! CAN_F13R2_FB23 (() =>                      (0x00800000 as uint32_t);)        /*<Filter bit 23 */
pub macro_rules! CAN_F13R2_FB24 (() =>                      (0x01000000 as uint32_t);)        /*<Filter bit 24 */
pub macro_rules! CAN_F13R2_FB25 (() =>                      (0x02000000 as uint32_t);)        /*<Filter bit 25 */
pub macro_rules! CAN_F13R2_FB26 (() =>                      (0x04000000 as uint32_t);)        /*<Filter bit 26 */
pub macro_rules! CAN_F13R2_FB27 (() =>                      (0x08000000 as uint32_t);)        /*<Filter bit 27 */
pub macro_rules! CAN_F13R2_FB28 (() =>                      (0x10000000 as uint32_t);)        /*<Filter bit 28 */
pub macro_rules! CAN_F13R2_FB29 (() =>                      (0x20000000 as uint32_t);)        /*<Filter bit 29 */
pub macro_rules! CAN_F13R2_FB30 (() =>                      (0x40000000 as uint32_t);)        /*<Filter bit 30 */
pub macro_rules! CAN_F13R2_FB31 (() =>                      (0x80000000 as uint32_t);)        /*<Filter bit 31 */


/*                                                                            */
/*                          CRC calculation unit                              */
/*                                                                            */

/*  Bit definition for CRC_DR register  */
pub macro_rules! CRC_DR_DR (() =>                           (0xFFFFFFFF as uint32_t);) /*< Data register bits */


/*  Bit definition for CRC_IDR register  */
pub macro_rules! CRC_IDR_IDR (() =>                         (0xFF as uint8_t);)        /*< General-purpose 8-bit data register bits */


/*  Bit definition for CRC_CR register  */
pub macro_rules! CRC_CR_RESET (() =>                        (0x01 as uint8_t);)        /*< RESET bit */


/*                                                                            */
/*                            Crypto Processor                                */
/*                                                                            */

/* Bits definition for CRYP_CR register  */
pub macro_rules! CRYP_CR_ALGODIR (() =>                      (0x00000004 as uint32_t);)

pub macro_rules! CRYP_CR_ALGOMODE (() =>                     (0x00000038 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_0 (() =>                   (0x00000008 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_1 (() =>                   (0x00000010 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_2 (() =>                   (0x00000020 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_TDES_ECB (() =>            (0x00000000 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_TDES_CBC (() =>            (0x00000008 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_DES_ECB (() =>             (0x00000010 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_DES_CBC (() =>             (0x00000018 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_AES_ECB (() =>             (0x00000020 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_AES_CBC (() =>             (0x00000028 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_AES_CTR (() =>             (0x00000030 as uint32_t);)
pub macro_rules! CRYP_CR_ALGOMODE_AES_KEY (() =>             (0x00000038 as uint32_t);)

pub macro_rules! CRYP_CR_DATATYPE (() =>                     (0x000000C0 as uint32_t);)
pub macro_rules! CRYP_CR_DATATYPE_0 (() =>                   (0x00000040 as uint32_t);)
pub macro_rules! CRYP_CR_DATATYPE_1 (() =>                   (0x00000080 as uint32_t);)
pub macro_rules! CRYP_CR_KEYSIZE (() =>                      (0x00000300 as uint32_t);)
pub macro_rules! CRYP_CR_KEYSIZE_0 (() =>                    (0x00000100 as uint32_t);)
pub macro_rules! CRYP_CR_KEYSIZE_1 (() =>                    (0x00000200 as uint32_t);)
pub macro_rules! CRYP_CR_FFLUSH (() =>                       (0x00004000 as uint32_t);)
pub macro_rules! CRYP_CR_CRYPEN (() =>                       (0x00008000 as uint32_t);)
/* Bits definition for CRYP_SR register  */
pub macro_rules! CRYP_SR_IFEM (() =>                         (0x00000001 as uint32_t);)
pub macro_rules! CRYP_SR_IFNF (() =>                         (0x00000002 as uint32_t);)
pub macro_rules! CRYP_SR_OFNE (() =>                         (0x00000004 as uint32_t);)
pub macro_rules! CRYP_SR_OFFU (() =>                         (0x00000008 as uint32_t);)
pub macro_rules! CRYP_SR_BUSY (() =>                         (0x00000010 as uint32_t);)
/* Bits definition for CRYP_DMACR register  */
pub macro_rules! CRYP_DMACR_DIEN (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! CRYP_DMACR_DOEN (() =>                      (0x00000002 as uint32_t);)
/*  Bits definition for CRYP_IMSCR register  */
pub macro_rules! CRYP_IMSCR_INIM (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! CRYP_IMSCR_OUTIM (() =>                     (0x00000002 as uint32_t);)
/* Bits definition for CRYP_RISR register  */
pub macro_rules! CRYP_RISR_OUTRIS (() =>                     (0x00000001 as uint32_t);)
pub macro_rules! CRYP_RISR_INRIS (() =>                      (0x00000002 as uint32_t);)
/* Bits definition for CRYP_MISR register  */
pub macro_rules! CRYP_MISR_INMIS (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! CRYP_MISR_OUTMIS (() =>                     (0x00000002 as uint32_t);)


/*                                                                            */
/*                      Digital to Analog Converter                           */
/*                                                                            */

/*  Bit definition for DAC_CR register  */
pub macro_rules! DAC_CR_EN1 (() =>                          (0x00000001 as uint32_t);)        /*<DAC channel1 enable */
pub macro_rules! DAC_CR_BOFF1 (() =>                        (0x00000002 as uint32_t);)        /*<DAC channel1 output buffer disable */
pub macro_rules! DAC_CR_TEN1 (() =>                         (0x00000004 as uint32_t);)        /*<DAC channel1 Trigger enable */

pub macro_rules! DAC_CR_TSEL1 (() =>                        (0x00000038 as uint32_t);)        /*<TSEL1[2:0] (DAC channel1 Trigger selection) */
pub macro_rules! DAC_CR_TSEL1_0 (() =>                      (0x00000008 as uint32_t);)        /*<Bit 0 */
pub macro_rules! DAC_CR_TSEL1_1 (() =>                      (0x00000010 as uint32_t);)        /*<Bit 1 */
pub macro_rules! DAC_CR_TSEL1_2 (() =>                      (0x00000020 as uint32_t);)        /*<Bit 2 */

pub macro_rules! DAC_CR_WAVE1 (() =>                        (0x000000C0 as uint32_t);)        /*<WAVE1[1:0] (DAC channel1 noise/triangle wave generation enable) */
pub macro_rules! DAC_CR_WAVE1_0 (() =>                      (0x00000040 as uint32_t);)        /*<Bit 0 */
pub macro_rules! DAC_CR_WAVE1_1 (() =>                      (0x00000080 as uint32_t);)        /*<Bit 1 */

pub macro_rules! DAC_CR_MAMP1 (() =>                        (0x00000F00 as uint32_t);)        /*<MAMP1[3:0] (DAC channel1 Mask/Amplitude selector) */
pub macro_rules! DAC_CR_MAMP1_0 (() =>                      (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! DAC_CR_MAMP1_1 (() =>                      (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! DAC_CR_MAMP1_2 (() =>                      (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! DAC_CR_MAMP1_3 (() =>                      (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! DAC_CR_DMAEN1 (() =>                       (0x00001000 as uint32_t);)        /*<DAC channel1 DMA enable */
pub macro_rules! DAC_CR_EN2 (() =>                          (0x00010000 as uint32_t);)        /*<DAC channel2 enable */
pub macro_rules! DAC_CR_BOFF2 (() =>                        (0x00020000 as uint32_t);)        /*<DAC channel2 output buffer disable */
pub macro_rules! DAC_CR_TEN2 (() =>                         (0x00040000 as uint32_t);)        /*<DAC channel2 Trigger enable */

pub macro_rules! DAC_CR_TSEL2 (() =>                        (0x00380000 as uint32_t);)        /*<TSEL2[2:0] (DAC channel2 Trigger selection) */
pub macro_rules! DAC_CR_TSEL2_0 (() =>                      (0x00080000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! DAC_CR_TSEL2_1 (() =>                      (0x00100000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! DAC_CR_TSEL2_2 (() =>                      (0x00200000 as uint32_t);)        /*<Bit 2 */

pub macro_rules! DAC_CR_WAVE2 (() =>                        (0x00C00000 as uint32_t);)        /*<WAVE2[1:0] (DAC channel2 noise/triangle wave generation enable) */
pub macro_rules! DAC_CR_WAVE2_0 (() =>                      (0x00400000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! DAC_CR_WAVE2_1 (() =>                      (0x00800000 as uint32_t);)        /*<Bit 1 */

pub macro_rules! DAC_CR_MAMP2 (() =>                        (0x0F000000 as uint32_t);)        /*<MAMP2[3:0] (DAC channel2 Mask/Amplitude selector) */
pub macro_rules! DAC_CR_MAMP2_0 (() =>                      (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! DAC_CR_MAMP2_1 (() =>                      (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! DAC_CR_MAMP2_2 (() =>                      (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! DAC_CR_MAMP2_3 (() =>                      (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! DAC_CR_DMAEN2 (() =>                       (0x10000000 as uint32_t);)        /*<DAC channel2 DMA enabled */

/*  Bit definition for DAC_SWTRIGR register  */
pub macro_rules! DAC_SWTRIGR_SWTRIG1 (() =>                 (0x01 as uint8_t);)               /*<DAC channel1 software trigger */
pub macro_rules! DAC_SWTRIGR_SWTRIG2 (() =>                 (0x02 as uint8_t);)               /*<DAC channel2 software trigger */

/*  Bit definition for DAC_DHR12R1 register  */
pub macro_rules! DAC_DHR12R1_DACC1DHR (() =>                (0x0FFF as uint16_t);)            /*<DAC channel1 12-bit Right aligned data */

/*  Bit definition for DAC_DHR12L1 register  */
pub macro_rules! DAC_DHR12L1_DACC1DHR (() =>                (0xFFF0 as uint16_t);)            /*<DAC channel1 12-bit Left aligned data */

/*  Bit definition for DAC_DHR8R1 register  */
pub macro_rules! DAC_DHR8R1_DACC1DHR (() =>                 (0xFF as uint8_t);)               /*<DAC channel1 8-bit Right aligned data */

/*  Bit definition for DAC_DHR12R2 register  */
pub macro_rules! DAC_DHR12R2_DACC2DHR (() =>                (0x0FFF as uint16_t);)            /*<DAC channel2 12-bit Right aligned data */

/*  Bit definition for DAC_DHR12L2 register  */
pub macro_rules! DAC_DHR12L2_DACC2DHR (() =>                (0xFFF0 as uint16_t);)            /*<DAC channel2 12-bit Left aligned data */

/*  Bit definition for DAC_DHR8R2 register  */
pub macro_rules! DAC_DHR8R2_DACC2DHR (() =>                 (0xFF as uint8_t);)               /*<DAC channel2 8-bit Right aligned data */

/*  Bit definition for DAC_DHR12RD register  */
pub macro_rules! DAC_DHR12RD_DACC1DHR (() =>                (0x00000FFF as uint32_t);)        /*<DAC channel1 12-bit Right aligned data */
pub macro_rules! DAC_DHR12RD_DACC2DHR (() =>                (0x0FFF0000 as uint32_t);)        /*<DAC channel2 12-bit Right aligned data */

/*  Bit definition for DAC_DHR12LD register  */
pub macro_rules! DAC_DHR12LD_DACC1DHR (() =>                (0x0000FFF0 as uint32_t);)        /*<DAC channel1 12-bit Left aligned data */
pub macro_rules! DAC_DHR12LD_DACC2DHR (() =>                (0xFFF00000 as uint32_t);)        /*<DAC channel2 12-bit Left aligned data */

/*  Bit definition for DAC_DHR8RD register  */
pub macro_rules! DAC_DHR8RD_DACC1DHR (() =>                 (0x00FF as uint16_t);)            /*<DAC channel1 8-bit Right aligned data */
pub macro_rules! DAC_DHR8RD_DACC2DHR (() =>                 (0xFF00 as uint16_t);)            /*<DAC channel2 8-bit Right aligned data */

/*  Bit definition for DAC_DOR1 register  */
pub macro_rules! DAC_DOR1_DACC1DOR (() =>                   (0x0FFF as uint16_t);)            /*<DAC channel1 data output */

/*  Bit definition for DAC_DOR2 register  */
pub macro_rules! DAC_DOR2_DACC2DOR (() =>                   (0x0FFF as uint16_t);)            /*<DAC channel2 data output */

/*  Bit definition for DAC_SR register  */
pub macro_rules! DAC_SR_DMAUDR1 (() =>                      (0x00002000 as uint32_t);)        /*<DAC channel1 DMA underrun flag */
pub macro_rules! DAC_SR_DMAUDR2 (() =>                      (0x20000000 as uint32_t);)        /*<DAC channel2 DMA underrun flag */


/*                                                                            */
/*                                 Debug MCU                                  */
/*                                                                            */



/*                                                                            */
/*                                    DCMI                                    */
/*                                                                            */

/*  Bits definition for DCMI_CR register  */
pub macro_rules! DCMI_CR_CAPTURE (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! DCMI_CR_CM (() =>                           (0x00000002 as uint32_t);)
pub macro_rules! DCMI_CR_CROP (() =>                         (0x00000004 as uint32_t);)
pub macro_rules! DCMI_CR_JPEG (() =>                         (0x00000008 as uint32_t);)
pub macro_rules! DCMI_CR_ESS (() =>                          (0x00000010 as uint32_t);)
pub macro_rules! DCMI_CR_PCKPOL (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! DCMI_CR_HSPOL (() =>                        (0x00000040 as uint32_t);)
pub macro_rules! DCMI_CR_VSPOL (() =>                        (0x00000080 as uint32_t);)
pub macro_rules! DCMI_CR_FCRC_0 (() =>                       (0x00000100 as uint32_t);)
pub macro_rules! DCMI_CR_FCRC_1 (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! DCMI_CR_EDM_0 (() =>                        (0x00000400 as uint32_t);)
pub macro_rules! DCMI_CR_EDM_1 (() =>                        (0x00000800 as uint32_t);)
pub macro_rules! DCMI_CR_CRE (() =>                          (0x00001000 as uint32_t);)
pub macro_rules! DCMI_CR_ENABLE (() =>                       (0x00004000 as uint32_t);)

/*  Bits definition for DCMI_SR register  */
pub macro_rules! DCMI_SR_HSYNC (() =>                        (0x00000001 as uint32_t);)
pub macro_rules! DCMI_SR_VSYNC (() =>                        (0x00000002 as uint32_t);)
pub macro_rules! DCMI_SR_FNE (() =>                          (0x00000004 as uint32_t);)

/*  Bits definition for DCMI_RISR register  */
pub macro_rules! DCMI_RISR_FRAME_RIS (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! DCMI_RISR_OVF_RIS (() =>                    (0x00000002 as uint32_t);)
pub macro_rules! DCMI_RISR_ERR_RIS (() =>                    (0x00000004 as uint32_t);)
pub macro_rules! DCMI_RISR_VSYNC_RIS (() =>                  (0x00000008 as uint32_t);)
pub macro_rules! DCMI_RISR_LINE_RIS (() =>                   (0x00000010 as uint32_t);)

/*  Bits definition for DCMI_IER register  */
pub macro_rules! DCMI_IER_FRAME_IE (() =>                    (0x00000001 as uint32_t);)
pub macro_rules! DCMI_IER_OVF_IE (() =>                      (0x00000002 as uint32_t);)
pub macro_rules! DCMI_IER_ERR_IE (() =>                      (0x00000004 as uint32_t);)
pub macro_rules! DCMI_IER_VSYNC_IE (() =>                    (0x00000008 as uint32_t);)
pub macro_rules! DCMI_IER_LINE_IE (() =>                     (0x00000010 as uint32_t);)

/*  Bits definition for DCMI_MISR register  */
pub macro_rules! DCMI_MISR_FRAME_MIS (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! DCMI_MISR_OVF_MIS (() =>                    (0x00000002 as uint32_t);)
pub macro_rules! DCMI_MISR_ERR_MIS (() =>                    (0x00000004 as uint32_t);)
pub macro_rules! DCMI_MISR_VSYNC_MIS (() =>                  (0x00000008 as uint32_t);)
pub macro_rules! DCMI_MISR_LINE_MIS (() =>                   (0x00000010 as uint32_t);)

/*  Bits definition for DCMI_ICR register  */
pub macro_rules! DCMI_ICR_FRAME_ISC (() =>                   (0x00000001 as uint32_t);)
pub macro_rules! DCMI_ICR_OVF_ISC (() =>                     (0x00000002 as uint32_t);)
pub macro_rules! DCMI_ICR_ERR_ISC (() =>                     (0x00000004 as uint32_t);)
pub macro_rules! DCMI_ICR_VSYNC_ISC (() =>                   (0x00000008 as uint32_t);)
pub macro_rules! DCMI_ICR_LINE_ISC (() =>                    (0x00000010 as uint32_t);)


/*                                                                            */
/*                             DMA Controller                                 */
/*                                                                            */

/*  Bits definition for DMA_SxCR register  */ 
pub macro_rules! DMA_SxCR_CHSEL (() =>                       (0x0E000000 as uint32_t);)
pub macro_rules! DMA_SxCR_CHSEL_0 (() =>                     (0x02000000 as uint32_t);)
pub macro_rules! DMA_SxCR_CHSEL_1 (() =>                     (0x04000000 as uint32_t);)
pub macro_rules! DMA_SxCR_CHSEL_2 (() =>                     (0x08000000 as uint32_t);) 
pub macro_rules! DMA_SxCR_MBURST (() =>                      (0x01800000 as uint32_t);)
pub macro_rules! DMA_SxCR_MBURST_0 (() =>                    (0x00800000 as uint32_t);)
pub macro_rules! DMA_SxCR_MBURST_1 (() =>                    (0x01000000 as uint32_t);)
pub macro_rules! DMA_SxCR_PBURST (() =>                      (0x00600000 as uint32_t);)
pub macro_rules! DMA_SxCR_PBURST_0 (() =>                    (0x00200000 as uint32_t);)
pub macro_rules! DMA_SxCR_PBURST_1 (() =>                    (0x00400000 as uint32_t);)
pub macro_rules! DMA_SxCR_ACK (() =>                         (0x00100000 as uint32_t);)
pub macro_rules! DMA_SxCR_CT (() =>                          (0x00080000 as uint32_t);)  
pub macro_rules! DMA_SxCR_DBM (() =>                         (0x00040000 as uint32_t);)
pub macro_rules! DMA_SxCR_PL (() =>                          (0x00030000 as uint32_t);)
pub macro_rules! DMA_SxCR_PL_0 (() =>                        (0x00010000 as uint32_t);)
pub macro_rules! DMA_SxCR_PL_1 (() =>                        (0x00020000 as uint32_t);)
pub macro_rules! DMA_SxCR_PINCOS (() =>                      (0x00008000 as uint32_t);)
pub macro_rules! DMA_SxCR_MSIZE (() =>                       (0x00006000 as uint32_t);)
pub macro_rules! DMA_SxCR_MSIZE_0 (() =>                     (0x00002000 as uint32_t);)
pub macro_rules! DMA_SxCR_MSIZE_1 (() =>                     (0x00004000 as uint32_t);)
pub macro_rules! DMA_SxCR_PSIZE (() =>                       (0x00001800 as uint32_t);)
pub macro_rules! DMA_SxCR_PSIZE_0 (() =>                     (0x00000800 as uint32_t);)
pub macro_rules! DMA_SxCR_PSIZE_1 (() =>                     (0x00001000 as uint32_t);)
pub macro_rules! DMA_SxCR_MINC (() =>                        (0x00000400 as uint32_t);)
pub macro_rules! DMA_SxCR_PINC (() =>                        (0x00000200 as uint32_t);)
pub macro_rules! DMA_SxCR_CIRC (() =>                        (0x00000100 as uint32_t);)
pub macro_rules! DMA_SxCR_DIR (() =>                         (0x000000C0 as uint32_t);)
pub macro_rules! DMA_SxCR_DIR_0 (() =>                       (0x00000040 as uint32_t);)
pub macro_rules! DMA_SxCR_DIR_1 (() =>                       (0x00000080 as uint32_t);)
pub macro_rules! DMA_SxCR_PFCTRL (() =>                      (0x00000020 as uint32_t);)
pub macro_rules! DMA_SxCR_TCIE (() =>                        (0x00000010 as uint32_t);)
pub macro_rules! DMA_SxCR_HTIE (() =>                        (0x00000008 as uint32_t);)
pub macro_rules! DMA_SxCR_TEIE (() =>                        (0x00000004 as uint32_t);)
pub macro_rules! DMA_SxCR_DMEIE (() =>                       (0x00000002 as uint32_t);)
pub macro_rules! DMA_SxCR_EN (() =>                          (0x00000001 as uint32_t);)

/*  Bits definition for DMA_SxCNDTR register  */
pub macro_rules! DMA_SxNDT (() =>                            (0x0000FFFF as uint32_t);)
pub macro_rules! DMA_SxNDT_0 (() =>                          (0x00000001 as uint32_t);)
pub macro_rules! DMA_SxNDT_1 (() =>                          (0x00000002 as uint32_t);)
pub macro_rules! DMA_SxNDT_2 (() =>                          (0x00000004 as uint32_t);)
pub macro_rules! DMA_SxNDT_3 (() =>                          (0x00000008 as uint32_t);)
pub macro_rules! DMA_SxNDT_4 (() =>                          (0x00000010 as uint32_t);)
pub macro_rules! DMA_SxNDT_5 (() =>                          (0x00000020 as uint32_t);)
pub macro_rules! DMA_SxNDT_6 (() =>                          (0x00000040 as uint32_t);)
pub macro_rules! DMA_SxNDT_7 (() =>                          (0x00000080 as uint32_t);)
pub macro_rules! DMA_SxNDT_8 (() =>                          (0x00000100 as uint32_t);)
pub macro_rules! DMA_SxNDT_9 (() =>                          (0x00000200 as uint32_t);)
pub macro_rules! DMA_SxNDT_10 (() =>                         (0x00000400 as uint32_t);)
pub macro_rules! DMA_SxNDT_11 (() =>                         (0x00000800 as uint32_t);)
pub macro_rules! DMA_SxNDT_12 (() =>                         (0x00001000 as uint32_t);)
pub macro_rules! DMA_SxNDT_13 (() =>                         (0x00002000 as uint32_t);)
pub macro_rules! DMA_SxNDT_14 (() =>                         (0x00004000 as uint32_t);)
pub macro_rules! DMA_SxNDT_15 (() =>                         (0x00008000 as uint32_t);)

/*  Bits definition for DMA_SxFCR register  */ 
pub macro_rules! DMA_SxFCR_FEIE (() =>                       (0x00000080 as uint32_t);)
pub macro_rules! DMA_SxFCR_FS (() =>                         (0x00000038 as uint32_t);)
pub macro_rules! DMA_SxFCR_FS_0 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! DMA_SxFCR_FS_1 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! DMA_SxFCR_FS_2 (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! DMA_SxFCR_DMDIS (() =>                      (0x00000004 as uint32_t);)
pub macro_rules! DMA_SxFCR_FTH (() =>                        (0x00000003 as uint32_t);)
pub macro_rules! DMA_SxFCR_FTH_0 (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! DMA_SxFCR_FTH_1 (() =>                      (0x00000002 as uint32_t);)

/*  Bits definition for DMA_LISR register  */ 
pub macro_rules! DMA_LISR_TCIF3 (() =>                       (0x08000000 as uint32_t);)
pub macro_rules! DMA_LISR_HTIF3 (() =>                       (0x04000000 as uint32_t);)
pub macro_rules! DMA_LISR_TEIF3 (() =>                       (0x02000000 as uint32_t);)
pub macro_rules! DMA_LISR_DMEIF3 (() =>                      (0x01000000 as uint32_t);)
pub macro_rules! DMA_LISR_FEIF3 (() =>                       (0x00400000 as uint32_t);)
pub macro_rules! DMA_LISR_TCIF2 (() =>                       (0x00200000 as uint32_t);)
pub macro_rules! DMA_LISR_HTIF2 (() =>                       (0x00100000 as uint32_t);)
pub macro_rules! DMA_LISR_TEIF2 (() =>                       (0x00080000 as uint32_t);)
pub macro_rules! DMA_LISR_DMEIF2 (() =>                      (0x00040000 as uint32_t);)
pub macro_rules! DMA_LISR_FEIF2 (() =>                       (0x00010000 as uint32_t);)
pub macro_rules! DMA_LISR_TCIF1 (() =>                       (0x00000800 as uint32_t);)
pub macro_rules! DMA_LISR_HTIF1 (() =>                       (0x00000400 as uint32_t);)
pub macro_rules! DMA_LISR_TEIF1 (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! DMA_LISR_DMEIF1 (() =>                      (0x00000100 as uint32_t);)
pub macro_rules! DMA_LISR_FEIF1 (() =>                       (0x00000040 as uint32_t);)
pub macro_rules! DMA_LISR_TCIF0 (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! DMA_LISR_HTIF0 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! DMA_LISR_TEIF0 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! DMA_LISR_DMEIF0 (() =>                      (0x00000004 as uint32_t);)
pub macro_rules! DMA_LISR_FEIF0 (() =>                       (0x00000001 as uint32_t);)

/*  Bits definition for DMA_HISR register  */ 
pub macro_rules! DMA_HISR_TCIF7 (() =>                       (0x08000000 as uint32_t);)
pub macro_rules! DMA_HISR_HTIF7 (() =>                       (0x04000000 as uint32_t);)
pub macro_rules! DMA_HISR_TEIF7 (() =>                       (0x02000000 as uint32_t);)
pub macro_rules! DMA_HISR_DMEIF7 (() =>                      (0x01000000 as uint32_t);)
pub macro_rules! DMA_HISR_FEIF7 (() =>                       (0x00400000 as uint32_t);)
pub macro_rules! DMA_HISR_TCIF6 (() =>                       (0x00200000 as uint32_t);)
pub macro_rules! DMA_HISR_HTIF6 (() =>                       (0x00100000 as uint32_t);)
pub macro_rules! DMA_HISR_TEIF6 (() =>                       (0x00080000 as uint32_t);)
pub macro_rules! DMA_HISR_DMEIF6 (() =>                      (0x00040000 as uint32_t);)
pub macro_rules! DMA_HISR_FEIF6 (() =>                       (0x00010000 as uint32_t);)
pub macro_rules! DMA_HISR_TCIF5 (() =>                       (0x00000800 as uint32_t);)
pub macro_rules! DMA_HISR_HTIF5 (() =>                       (0x00000400 as uint32_t);)
pub macro_rules! DMA_HISR_TEIF5 (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! DMA_HISR_DMEIF5 (() =>                      (0x00000100 as uint32_t);)
pub macro_rules! DMA_HISR_FEIF5 (() =>                       (0x00000040 as uint32_t);)
pub macro_rules! DMA_HISR_TCIF4 (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! DMA_HISR_HTIF4 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! DMA_HISR_TEIF4 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! DMA_HISR_DMEIF4 (() =>                      (0x00000004 as uint32_t);)
pub macro_rules! DMA_HISR_FEIF4 (() =>                       (0x00000001 as uint32_t);)

/*  Bits definition for DMA_LIFCR register  */ 
pub macro_rules! DMA_LIFCR_CTCIF3 (() =>                     (0x08000000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CHTIF3 (() =>                     (0x04000000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CTEIF3 (() =>                     (0x02000000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CDMEIF3 (() =>                    (0x01000000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CFEIF3 (() =>                     (0x00400000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CTCIF2 (() =>                     (0x00200000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CHTIF2 (() =>                     (0x00100000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CTEIF2 (() =>                     (0x00080000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CDMEIF2 (() =>                    (0x00040000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CFEIF2 (() =>                     (0x00010000 as uint32_t);)
pub macro_rules! DMA_LIFCR_CTCIF1 (() =>                     (0x00000800 as uint32_t);)
pub macro_rules! DMA_LIFCR_CHTIF1 (() =>                     (0x00000400 as uint32_t);)
pub macro_rules! DMA_LIFCR_CTEIF1 (() =>                     (0x00000200 as uint32_t);)
pub macro_rules! DMA_LIFCR_CDMEIF1 (() =>                    (0x00000100 as uint32_t);)
pub macro_rules! DMA_LIFCR_CFEIF1 (() =>                     (0x00000040 as uint32_t);)
pub macro_rules! DMA_LIFCR_CTCIF0 (() =>                     (0x00000020 as uint32_t);)
pub macro_rules! DMA_LIFCR_CHTIF0 (() =>                     (0x00000010 as uint32_t);)
pub macro_rules! DMA_LIFCR_CTEIF0 (() =>                     (0x00000008 as uint32_t);)
pub macro_rules! DMA_LIFCR_CDMEIF0 (() =>                    (0x00000004 as uint32_t);)
pub macro_rules! DMA_LIFCR_CFEIF0 (() =>                     (0x00000001 as uint32_t);)

/*  Bits definition for DMA_HIFCR  register  */ 
pub macro_rules! DMA_HIFCR_CTCIF7 (() =>                     (0x08000000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CHTIF7 (() =>                     (0x04000000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CTEIF7 (() =>                     (0x02000000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CDMEIF7 (() =>                    (0x01000000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CFEIF7 (() =>                     (0x00400000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CTCIF6 (() =>                     (0x00200000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CHTIF6 (() =>                     (0x00100000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CTEIF6 (() =>                     (0x00080000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CDMEIF6 (() =>                    (0x00040000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CFEIF6 (() =>                     (0x00010000 as uint32_t);)
pub macro_rules! DMA_HIFCR_CTCIF5 (() =>                     (0x00000800 as uint32_t);)
pub macro_rules! DMA_HIFCR_CHTIF5 (() =>                     (0x00000400 as uint32_t);)
pub macro_rules! DMA_HIFCR_CTEIF5 (() =>                     (0x00000200 as uint32_t);)
pub macro_rules! DMA_HIFCR_CDMEIF5 (() =>                    (0x00000100 as uint32_t);)
pub macro_rules! DMA_HIFCR_CFEIF5 (() =>                     (0x00000040 as uint32_t);)
pub macro_rules! DMA_HIFCR_CTCIF4 (() =>                     (0x00000020 as uint32_t);)
pub macro_rules! DMA_HIFCR_CHTIF4 (() =>                     (0x00000010 as uint32_t);)
pub macro_rules! DMA_HIFCR_CTEIF4 (() =>                     (0x00000008 as uint32_t);)
pub macro_rules! DMA_HIFCR_CDMEIF4 (() =>                    (0x00000004 as uint32_t);)
pub macro_rules! DMA_HIFCR_CFEIF4 (() =>                     (0x00000001 as uint32_t);)


/*                                                                            */
/*                    External Interrupt/Event Controller                     */
/*                                                                            */

/*  Bit definition for EXTI_IMR register  */
pub macro_rules! EXTI_IMR_MR0 (() =>                        (0x00000001 as uint32_t);)        /*< Interrupt Mask on line 0 */
pub macro_rules! EXTI_IMR_MR1 (() =>                        (0x00000002 as uint32_t);)        /*< Interrupt Mask on line 1 */
pub macro_rules! EXTI_IMR_MR2 (() =>                        (0x00000004 as uint32_t);)        /*< Interrupt Mask on line 2 */
pub macro_rules! EXTI_IMR_MR3 (() =>                        (0x00000008 as uint32_t);)        /*< Interrupt Mask on line 3 */
pub macro_rules! EXTI_IMR_MR4 (() =>                        (0x00000010 as uint32_t);)        /*< Interrupt Mask on line 4 */
pub macro_rules! EXTI_IMR_MR5 (() =>                        (0x00000020 as uint32_t);)        /*< Interrupt Mask on line 5 */
pub macro_rules! EXTI_IMR_MR6 (() =>                        (0x00000040 as uint32_t);)        /*< Interrupt Mask on line 6 */
pub macro_rules! EXTI_IMR_MR7 (() =>                        (0x00000080 as uint32_t);)        /*< Interrupt Mask on line 7 */
pub macro_rules! EXTI_IMR_MR8 (() =>                        (0x00000100 as uint32_t);)        /*< Interrupt Mask on line 8 */
pub macro_rules! EXTI_IMR_MR9 (() =>                        (0x00000200 as uint32_t);)        /*< Interrupt Mask on line 9 */
pub macro_rules! EXTI_IMR_MR10 (() =>                       (0x00000400 as uint32_t);)        /*< Interrupt Mask on line 10 */
pub macro_rules! EXTI_IMR_MR11 (() =>                       (0x00000800 as uint32_t);)        /*< Interrupt Mask on line 11 */
pub macro_rules! EXTI_IMR_MR12 (() =>                       (0x00001000 as uint32_t);)        /*< Interrupt Mask on line 12 */
pub macro_rules! EXTI_IMR_MR13 (() =>                       (0x00002000 as uint32_t);)        /*< Interrupt Mask on line 13 */
pub macro_rules! EXTI_IMR_MR14 (() =>                       (0x00004000 as uint32_t);)        /*< Interrupt Mask on line 14 */
pub macro_rules! EXTI_IMR_MR15 (() =>                       (0x00008000 as uint32_t);)        /*< Interrupt Mask on line 15 */
pub macro_rules! EXTI_IMR_MR16 (() =>                       (0x00010000 as uint32_t);)        /*< Interrupt Mask on line 16 */
pub macro_rules! EXTI_IMR_MR17 (() =>                       (0x00020000 as uint32_t);)        /*< Interrupt Mask on line 17 */
pub macro_rules! EXTI_IMR_MR18 (() =>                       (0x00040000 as uint32_t);)        /*< Interrupt Mask on line 18 */
pub macro_rules! EXTI_IMR_MR19 (() =>                       (0x00080000 as uint32_t);)        /*< Interrupt Mask on line 19 */

/*  Bit definition for EXTI_EMR register  */
pub macro_rules! EXTI_EMR_MR0 (() =>                        (0x00000001 as uint32_t);)        /*< Event Mask on line 0 */
pub macro_rules! EXTI_EMR_MR1 (() =>                        (0x00000002 as uint32_t);)        /*< Event Mask on line 1 */
pub macro_rules! EXTI_EMR_MR2 (() =>                        (0x00000004 as uint32_t);)        /*< Event Mask on line 2 */
pub macro_rules! EXTI_EMR_MR3 (() =>                        (0x00000008 as uint32_t);)        /*< Event Mask on line 3 */
pub macro_rules! EXTI_EMR_MR4 (() =>                        (0x00000010 as uint32_t);)        /*< Event Mask on line 4 */
pub macro_rules! EXTI_EMR_MR5 (() =>                        (0x00000020 as uint32_t);)        /*< Event Mask on line 5 */
pub macro_rules! EXTI_EMR_MR6 (() =>                        (0x00000040 as uint32_t);)        /*< Event Mask on line 6 */
pub macro_rules! EXTI_EMR_MR7 (() =>                        (0x00000080 as uint32_t);)        /*< Event Mask on line 7 */
pub macro_rules! EXTI_EMR_MR8 (() =>                        (0x00000100 as uint32_t);)        /*< Event Mask on line 8 */
pub macro_rules! EXTI_EMR_MR9 (() =>                        (0x00000200 as uint32_t);)        /*< Event Mask on line 9 */
pub macro_rules! EXTI_EMR_MR10 (() =>                       (0x00000400 as uint32_t);)        /*< Event Mask on line 10 */
pub macro_rules! EXTI_EMR_MR11 (() =>                       (0x00000800 as uint32_t);)        /*< Event Mask on line 11 */
pub macro_rules! EXTI_EMR_MR12 (() =>                       (0x00001000 as uint32_t);)        /*< Event Mask on line 12 */
pub macro_rules! EXTI_EMR_MR13 (() =>                       (0x00002000 as uint32_t);)        /*< Event Mask on line 13 */
pub macro_rules! EXTI_EMR_MR14 (() =>                       (0x00004000 as uint32_t);)        /*< Event Mask on line 14 */
pub macro_rules! EXTI_EMR_MR15 (() =>                       (0x00008000 as uint32_t);)        /*< Event Mask on line 15 */
pub macro_rules! EXTI_EMR_MR16 (() =>                       (0x00010000 as uint32_t);)        /*< Event Mask on line 16 */
pub macro_rules! EXTI_EMR_MR17 (() =>                       (0x00020000 as uint32_t);)        /*< Event Mask on line 17 */
pub macro_rules! EXTI_EMR_MR18 (() =>                       (0x00040000 as uint32_t);)        /*< Event Mask on line 18 */
pub macro_rules! EXTI_EMR_MR19 (() =>                       (0x00080000 as uint32_t);)        /*< Event Mask on line 19 */

/*  Bit definition for EXTI_RTSR register  */
pub macro_rules! EXTI_RTSR_TR0 (() =>                       (0x00000001 as uint32_t);)        /*< Rising trigger event configuration bit of line 0 */
pub macro_rules! EXTI_RTSR_TR1 (() =>                       (0x00000002 as uint32_t);)        /*< Rising trigger event configuration bit of line 1 */
pub macro_rules! EXTI_RTSR_TR2 (() =>                       (0x00000004 as uint32_t);)        /*< Rising trigger event configuration bit of line 2 */
pub macro_rules! EXTI_RTSR_TR3 (() =>                       (0x00000008 as uint32_t);)        /*< Rising trigger event configuration bit of line 3 */
pub macro_rules! EXTI_RTSR_TR4 (() =>                       (0x00000010 as uint32_t);)        /*< Rising trigger event configuration bit of line 4 */
pub macro_rules! EXTI_RTSR_TR5 (() =>                       (0x00000020 as uint32_t);)        /*< Rising trigger event configuration bit of line 5 */
pub macro_rules! EXTI_RTSR_TR6 (() =>                       (0x00000040 as uint32_t);)        /*< Rising trigger event configuration bit of line 6 */
pub macro_rules! EXTI_RTSR_TR7 (() =>                       (0x00000080 as uint32_t);)        /*< Rising trigger event configuration bit of line 7 */
pub macro_rules! EXTI_RTSR_TR8 (() =>                       (0x00000100 as uint32_t);)        /*< Rising trigger event configuration bit of line 8 */
pub macro_rules! EXTI_RTSR_TR9 (() =>                       (0x00000200 as uint32_t);)        /*< Rising trigger event configuration bit of line 9 */
pub macro_rules! EXTI_RTSR_TR10 (() =>                      (0x00000400 as uint32_t);)        /*< Rising trigger event configuration bit of line 10 */
pub macro_rules! EXTI_RTSR_TR11 (() =>                      (0x00000800 as uint32_t);)        /*< Rising trigger event configuration bit of line 11 */
pub macro_rules! EXTI_RTSR_TR12 (() =>                      (0x00001000 as uint32_t);)        /*< Rising trigger event configuration bit of line 12 */
pub macro_rules! EXTI_RTSR_TR13 (() =>                      (0x00002000 as uint32_t);)        /*< Rising trigger event configuration bit of line 13 */
pub macro_rules! EXTI_RTSR_TR14 (() =>                      (0x00004000 as uint32_t);)        /*< Rising trigger event configuration bit of line 14 */
pub macro_rules! EXTI_RTSR_TR15 (() =>                      (0x00008000 as uint32_t);)        /*< Rising trigger event configuration bit of line 15 */
pub macro_rules! EXTI_RTSR_TR16 (() =>                      (0x00010000 as uint32_t);)        /*< Rising trigger event configuration bit of line 16 */
pub macro_rules! EXTI_RTSR_TR17 (() =>                      (0x00020000 as uint32_t);)        /*< Rising trigger event configuration bit of line 17 */
pub macro_rules! EXTI_RTSR_TR18 (() =>                      (0x00040000 as uint32_t);)        /*< Rising trigger event configuration bit of line 18 */
pub macro_rules! EXTI_RTSR_TR19 (() =>                      (0x00080000 as uint32_t);)        /*< Rising trigger event configuration bit of line 19 */

/*  Bit definition for EXTI_FTSR register  */
pub macro_rules! EXTI_FTSR_TR0 (() =>                       (0x00000001 as uint32_t);)        /*< Falling trigger event configuration bit of line 0 */
pub macro_rules! EXTI_FTSR_TR1 (() =>                       (0x00000002 as uint32_t);)        /*< Falling trigger event configuration bit of line 1 */
pub macro_rules! EXTI_FTSR_TR2 (() =>                       (0x00000004 as uint32_t);)        /*< Falling trigger event configuration bit of line 2 */
pub macro_rules! EXTI_FTSR_TR3 (() =>                       (0x00000008 as uint32_t);)        /*< Falling trigger event configuration bit of line 3 */
pub macro_rules! EXTI_FTSR_TR4 (() =>                       (0x00000010 as uint32_t);)        /*< Falling trigger event configuration bit of line 4 */
pub macro_rules! EXTI_FTSR_TR5 (() =>                       (0x00000020 as uint32_t);)        /*< Falling trigger event configuration bit of line 5 */
pub macro_rules! EXTI_FTSR_TR6 (() =>                       (0x00000040 as uint32_t);)        /*< Falling trigger event configuration bit of line 6 */
pub macro_rules! EXTI_FTSR_TR7 (() =>                       (0x00000080 as uint32_t);)        /*< Falling trigger event configuration bit of line 7 */
pub macro_rules! EXTI_FTSR_TR8 (() =>                       (0x00000100 as uint32_t);)        /*< Falling trigger event configuration bit of line 8 */
pub macro_rules! EXTI_FTSR_TR9 (() =>                       (0x00000200 as uint32_t);)        /*< Falling trigger event configuration bit of line 9 */
pub macro_rules! EXTI_FTSR_TR10 (() =>                      (0x00000400 as uint32_t);)        /*< Falling trigger event configuration bit of line 10 */
pub macro_rules! EXTI_FTSR_TR11 (() =>                      (0x00000800 as uint32_t);)        /*< Falling trigger event configuration bit of line 11 */
pub macro_rules! EXTI_FTSR_TR12 (() =>                      (0x00001000 as uint32_t);)        /*< Falling trigger event configuration bit of line 12 */
pub macro_rules! EXTI_FTSR_TR13 (() =>                      (0x00002000 as uint32_t);)        /*< Falling trigger event configuration bit of line 13 */
pub macro_rules! EXTI_FTSR_TR14 (() =>                      (0x00004000 as uint32_t);)        /*< Falling trigger event configuration bit of line 14 */
pub macro_rules! EXTI_FTSR_TR15 (() =>                      (0x00008000 as uint32_t);)        /*< Falling trigger event configuration bit of line 15 */
pub macro_rules! EXTI_FTSR_TR16 (() =>                      (0x00010000 as uint32_t);)        /*< Falling trigger event configuration bit of line 16 */
pub macro_rules! EXTI_FTSR_TR17 (() =>                      (0x00020000 as uint32_t);)        /*< Falling trigger event configuration bit of line 17 */
pub macro_rules! EXTI_FTSR_TR18 (() =>                      (0x00040000 as uint32_t);)        /*< Falling trigger event configuration bit of line 18 */
pub macro_rules! EXTI_FTSR_TR19 (() =>                      (0x00080000 as uint32_t);)        /*< Falling trigger event configuration bit of line 19 */

/*  Bit definition for EXTI_SWIER register  */
pub macro_rules! EXTI_SWIER_SWIER0 (() =>                   (0x00000001 as uint32_t);)        /*< Software Interrupt on line 0 */
pub macro_rules! EXTI_SWIER_SWIER1 (() =>                   (0x00000002 as uint32_t);)        /*< Software Interrupt on line 1 */
pub macro_rules! EXTI_SWIER_SWIER2 (() =>                   (0x00000004 as uint32_t);)        /*< Software Interrupt on line 2 */
pub macro_rules! EXTI_SWIER_SWIER3 (() =>                   (0x00000008 as uint32_t);)        /*< Software Interrupt on line 3 */
pub macro_rules! EXTI_SWIER_SWIER4 (() =>                   (0x00000010 as uint32_t);)        /*< Software Interrupt on line 4 */
pub macro_rules! EXTI_SWIER_SWIER5 (() =>                   (0x00000020 as uint32_t);)        /*< Software Interrupt on line 5 */
pub macro_rules! EXTI_SWIER_SWIER6 (() =>                   (0x00000040 as uint32_t);)        /*< Software Interrupt on line 6 */
pub macro_rules! EXTI_SWIER_SWIER7 (() =>                   (0x00000080 as uint32_t);)        /*< Software Interrupt on line 7 */
pub macro_rules! EXTI_SWIER_SWIER8 (() =>                   (0x00000100 as uint32_t);)        /*< Software Interrupt on line 8 */
pub macro_rules! EXTI_SWIER_SWIER9 (() =>                   (0x00000200 as uint32_t);)        /*< Software Interrupt on line 9 */
pub macro_rules! EXTI_SWIER_SWIER10 (() =>                  (0x00000400 as uint32_t);)        /*< Software Interrupt on line 10 */
pub macro_rules! EXTI_SWIER_SWIER11 (() =>                  (0x00000800 as uint32_t);)        /*< Software Interrupt on line 11 */
pub macro_rules! EXTI_SWIER_SWIER12 (() =>                  (0x00001000 as uint32_t);)        /*< Software Interrupt on line 12 */
pub macro_rules! EXTI_SWIER_SWIER13 (() =>                  (0x00002000 as uint32_t);)        /*< Software Interrupt on line 13 */
pub macro_rules! EXTI_SWIER_SWIER14 (() =>                  (0x00004000 as uint32_t);)        /*< Software Interrupt on line 14 */
pub macro_rules! EXTI_SWIER_SWIER15 (() =>                  (0x00008000 as uint32_t);)        /*< Software Interrupt on line 15 */
pub macro_rules! EXTI_SWIER_SWIER16 (() =>                  (0x00010000 as uint32_t);)        /*< Software Interrupt on line 16 */
pub macro_rules! EXTI_SWIER_SWIER17 (() =>                  (0x00020000 as uint32_t);)        /*< Software Interrupt on line 17 */
pub macro_rules! EXTI_SWIER_SWIER18 (() =>                  (0x00040000 as uint32_t);)        /*< Software Interrupt on line 18 */
pub macro_rules! EXTI_SWIER_SWIER19 (() =>                  (0x00080000 as uint32_t);)        /*< Software Interrupt on line 19 */

/*  Bit definition for EXTI_PR register  */
pub macro_rules! EXTI_PR_PR0 (() =>                         (0x00000001 as uint32_t);)        /*< Pending bit for line 0 */
pub macro_rules! EXTI_PR_PR1 (() =>                         (0x00000002 as uint32_t);)        /*< Pending bit for line 1 */
pub macro_rules! EXTI_PR_PR2 (() =>                         (0x00000004 as uint32_t);)        /*< Pending bit for line 2 */
pub macro_rules! EXTI_PR_PR3 (() =>                         (0x00000008 as uint32_t);)        /*< Pending bit for line 3 */
pub macro_rules! EXTI_PR_PR4 (() =>                         (0x00000010 as uint32_t);)        /*< Pending bit for line 4 */
pub macro_rules! EXTI_PR_PR5 (() =>                         (0x00000020 as uint32_t);)        /*< Pending bit for line 5 */
pub macro_rules! EXTI_PR_PR6 (() =>                         (0x00000040 as uint32_t);)        /*< Pending bit for line 6 */
pub macro_rules! EXTI_PR_PR7 (() =>                         (0x00000080 as uint32_t);)        /*< Pending bit for line 7 */
pub macro_rules! EXTI_PR_PR8 (() =>                         (0x00000100 as uint32_t);)        /*< Pending bit for line 8 */
pub macro_rules! EXTI_PR_PR9 (() =>                         (0x00000200 as uint32_t);)        /*< Pending bit for line 9 */
pub macro_rules! EXTI_PR_PR10 (() =>                        (0x00000400 as uint32_t);)        /*< Pending bit for line 10 */
pub macro_rules! EXTI_PR_PR11 (() =>                        (0x00000800 as uint32_t);)        /*< Pending bit for line 11 */
pub macro_rules! EXTI_PR_PR12 (() =>                        (0x00001000 as uint32_t);)        /*< Pending bit for line 12 */
pub macro_rules! EXTI_PR_PR13 (() =>                        (0x00002000 as uint32_t);)        /*< Pending bit for line 13 */
pub macro_rules! EXTI_PR_PR14 (() =>                        (0x00004000 as uint32_t);)        /*< Pending bit for line 14 */
pub macro_rules! EXTI_PR_PR15 (() =>                        (0x00008000 as uint32_t);)        /*< Pending bit for line 15 */
pub macro_rules! EXTI_PR_PR16 (() =>                        (0x00010000 as uint32_t);)        /*< Pending bit for line 16 */
pub macro_rules! EXTI_PR_PR17 (() =>                        (0x00020000 as uint32_t);)        /*< Pending bit for line 17 */
pub macro_rules! EXTI_PR_PR18 (() =>                        (0x00040000 as uint32_t);)        /*< Pending bit for line 18 */
pub macro_rules! EXTI_PR_PR19 (() =>                        (0x00080000 as uint32_t);)        /*< Pending bit for line 19 */


/*                                                                            */
/*                                    FLASH                                   */
/*                                                                            */

/*  Bits definition for FLASH_ACR register  */
pub macro_rules! FLASH_ACR_LATENCY (() =>                    (0x00000007 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_0WS (() =>                (0x00000000 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_1WS (() =>                (0x00000001 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_2WS (() =>                (0x00000002 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_3WS (() =>                (0x00000003 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_4WS (() =>                (0x00000004 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_5WS (() =>                (0x00000005 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_6WS (() =>                (0x00000006 as uint32_t);)
pub macro_rules! FLASH_ACR_LATENCY_7WS (() =>                (0x00000007 as uint32_t);)

pub macro_rules! FLASH_ACR_PRFTEN (() =>                     (0x00000100 as uint32_t);)
pub macro_rules! FLASH_ACR_ICEN (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! FLASH_ACR_DCEN (() =>                       (0x00000400 as uint32_t);)
pub macro_rules! FLASH_ACR_ICRST (() =>                      (0x00000800 as uint32_t);)
pub macro_rules! FLASH_ACR_DCRST (() =>                      (0x00001000 as uint32_t);)
pub macro_rules! FLASH_ACR_BYTE0_ADDRESS (() =>              (0x40023C00 as uint32_t);)
pub macro_rules! FLASH_ACR_BYTE2_ADDRESS (() =>              (0x40023C03 as uint32_t);)

/*  Bits definition for FLASH_SR register  */
pub macro_rules! FLASH_SR_EOP (() =>                         (0x00000001 as uint32_t);)
pub macro_rules! FLASH_SR_SOP (() =>                         (0x00000002 as uint32_t);)
pub macro_rules! FLASH_SR_WRPERR (() =>                      (0x00000010 as uint32_t);)
pub macro_rules! FLASH_SR_PGAERR (() =>                      (0x00000020 as uint32_t);)
pub macro_rules! FLASH_SR_PGPERR (() =>                      (0x00000040 as uint32_t);)
pub macro_rules! FLASH_SR_PGSERR (() =>                      (0x00000080 as uint32_t);)
pub macro_rules! FLASH_SR_BSY (() =>                         (0x00010000 as uint32_t);)

/*  Bits definition for FLASH_CR register  */
pub macro_rules! FLASH_CR_PG (() =>                          (0x00000001 as uint32_t);)
pub macro_rules! FLASH_CR_SER (() =>                         (0x00000002 as uint32_t);)
pub macro_rules! FLASH_CR_MER (() =>                         (0x00000004 as uint32_t);)
pub macro_rules! FLASH_CR_SNB_0 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! FLASH_CR_SNB_1 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! FLASH_CR_SNB_2 (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! FLASH_CR_SNB_3 (() =>                       (0x00000040 as uint32_t);)
pub macro_rules! FLASH_CR_PSIZE_0 (() =>                     (0x00000100 as uint32_t);)
pub macro_rules! FLASH_CR_PSIZE_1 (() =>                     (0x00000200 as uint32_t);)
pub macro_rules! FLASH_CR_STRT (() =>                        (0x00010000 as uint32_t);)
pub macro_rules! FLASH_CR_EOPIE (() =>                       (0x01000000 as uint32_t);)
pub macro_rules! FLASH_CR_LOCK (() =>                        (0x80000000 as uint32_t);)

/*  Bits definition for FLASH_OPTCR register  */
pub macro_rules! FLASH_OPTCR_OPTLOCK (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! FLASH_OPTCR_OPTSTRT (() =>                  (0x00000002 as uint32_t);)
pub macro_rules! FLASH_OPTCR_BOR_LEV_0 (() =>                (0x00000004 as uint32_t);)
pub macro_rules! FLASH_OPTCR_BOR_LEV_1 (() =>                (0x00000008 as uint32_t);)
pub macro_rules! FLASH_OPTCR_BOR_LEV (() =>                  (0x0000000C as uint32_t);)
pub macro_rules! FLASH_OPTCR_WDG_SW (() =>                   (0x00000020 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nRST_STOP (() =>                (0x00000040 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nRST_STDBY (() =>               (0x00000080 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_0 (() =>                    (0x00000100 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_1 (() =>                    (0x00000200 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_2 (() =>                    (0x00000400 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_3 (() =>                    (0x00000800 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_4 (() =>                    (0x00001000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_5 (() =>                    (0x00002000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_6 (() =>                    (0x00004000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_RDP_7 (() =>                    (0x00008000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_0 (() =>                   (0x00010000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_1 (() =>                   (0x00020000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_2 (() =>                   (0x00040000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_3 (() =>                   (0x00080000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_4 (() =>                   (0x00100000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_5 (() =>                   (0x00200000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_6 (() =>                   (0x00400000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_7 (() =>                   (0x00800000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_8 (() =>                   (0x01000000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_9 (() =>                   (0x02000000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_10 (() =>                  (0x04000000 as uint32_t);)
pub macro_rules! FLASH_OPTCR_nWRP_11 (() =>                  (0x08000000 as uint32_t);)


/*                                                                            */
/*                       Flexible Static Memory Controller                    */
/*                                                                            */

/*  Bit definition for FSMC_BCR1 register  */
pub macro_rules! FSMC_BCR1_MBKEN (() =>                     (0x00000001 as uint32_t);)        /*<Memory bank enable bit */
pub macro_rules! FSMC_BCR1_MUXEN (() =>                     (0x00000002 as uint32_t);)        /*<Address/data multiplexing enable bit */

pub macro_rules! FSMC_BCR1_MTYP (() =>                      (0x0000000C as uint32_t);)        /*<MTYP[1:0] bits (Memory type) */
pub macro_rules! FSMC_BCR1_MTYP_0 (() =>                    (0x00000004 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR1_MTYP_1 (() =>                    (0x00000008 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR1_MWID (() =>                      (0x00000030 as uint32_t);)        /*<MWID[1:0] bits (Memory data bus width) */
pub macro_rules! FSMC_BCR1_MWID_0 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR1_MWID_1 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR1_FACCEN (() =>                    (0x00000040 as uint32_t);)        /*<Flash access enable */
pub macro_rules! FSMC_BCR1_BURSTEN (() =>                   (0x00000100 as uint32_t);)        /*<Burst enable bit */
pub macro_rules! FSMC_BCR1_WAITPOL (() =>                   (0x00000200 as uint32_t);)        /*<Wait signal polarity bit */
pub macro_rules! FSMC_BCR1_WRAPMOD (() =>                   (0x00000400 as uint32_t);)        /*<Wrapped burst mode support */
pub macro_rules! FSMC_BCR1_WAITCFG (() =>                   (0x00000800 as uint32_t);)        /*<Wait timing configuration */
pub macro_rules! FSMC_BCR1_WREN (() =>                      (0x00001000 as uint32_t);)        /*<Write enable bit */
pub macro_rules! FSMC_BCR1_WAITEN (() =>                    (0x00002000 as uint32_t);)        /*<Wait enable bit */
pub macro_rules! FSMC_BCR1_EXTMOD (() =>                    (0x00004000 as uint32_t);)        /*<Extended mode enable */
pub macro_rules! FSMC_BCR1_ASYNCWAIT (() =>                 (0x00008000 as uint32_t);)        /*<Asynchronous wait */
pub macro_rules! FSMC_BCR1_CBURSTRW (() =>                  (0x00080000 as uint32_t);)        /*<Write burst enable */

/*  Bit definition for FSMC_BCR2 register  */
pub macro_rules! FSMC_BCR2_MBKEN (() =>                     (0x00000001 as uint32_t);)        /*<Memory bank enable bit */
pub macro_rules! FSMC_BCR2_MUXEN (() =>                     (0x00000002 as uint32_t);)        /*<Address/data multiplexing enable bit */

pub macro_rules! FSMC_BCR2_MTYP (() =>                      (0x0000000C as uint32_t);)        /*<MTYP[1:0] bits (Memory type) */
pub macro_rules! FSMC_BCR2_MTYP_0 (() =>                    (0x00000004 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR2_MTYP_1 (() =>                    (0x00000008 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR2_MWID (() =>                      (0x00000030 as uint32_t);)        /*<MWID[1:0] bits (Memory data bus width) */
pub macro_rules! FSMC_BCR2_MWID_0 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR2_MWID_1 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR2_FACCEN (() =>                    (0x00000040 as uint32_t);)        /*<Flash access enable */
pub macro_rules! FSMC_BCR2_BURSTEN (() =>                   (0x00000100 as uint32_t);)        /*<Burst enable bit */
pub macro_rules! FSMC_BCR2_WAITPOL (() =>                   (0x00000200 as uint32_t);)        /*<Wait signal polarity bit */
pub macro_rules! FSMC_BCR2_WRAPMOD (() =>                   (0x00000400 as uint32_t);)        /*<Wrapped burst mode support */
pub macro_rules! FSMC_BCR2_WAITCFG (() =>                   (0x00000800 as uint32_t);)        /*<Wait timing configuration */
pub macro_rules! FSMC_BCR2_WREN (() =>                      (0x00001000 as uint32_t);)        /*<Write enable bit */
pub macro_rules! FSMC_BCR2_WAITEN (() =>                    (0x00002000 as uint32_t);)        /*<Wait enable bit */
pub macro_rules! FSMC_BCR2_EXTMOD (() =>                    (0x00004000 as uint32_t);)        /*<Extended mode enable */
pub macro_rules! FSMC_BCR2_ASYNCWAIT (() =>                 (0x00008000 as uint32_t);)        /*<Asynchronous wait */
pub macro_rules! FSMC_BCR2_CBURSTRW (() =>                  (0x00080000 as uint32_t);)        /*<Write burst enable */

/*  Bit definition for FSMC_BCR3 register  */
pub macro_rules! FSMC_BCR3_MBKEN (() =>                     (0x00000001 as uint32_t);)        /*<Memory bank enable bit */
pub macro_rules! FSMC_BCR3_MUXEN (() =>                     (0x00000002 as uint32_t);)        /*<Address/data multiplexing enable bit */

pub macro_rules! FSMC_BCR3_MTYP (() =>                      (0x0000000C as uint32_t);)        /*<MTYP[1:0] bits (Memory type) */
pub macro_rules! FSMC_BCR3_MTYP_0 (() =>                    (0x00000004 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR3_MTYP_1 (() =>                    (0x00000008 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR3_MWID (() =>                      (0x00000030 as uint32_t);)        /*<MWID[1:0] bits (Memory data bus width) */
pub macro_rules! FSMC_BCR3_MWID_0 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR3_MWID_1 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR3_FACCEN (() =>                    (0x00000040 as uint32_t);)        /*<Flash access enable */
pub macro_rules! FSMC_BCR3_BURSTEN (() =>                   (0x00000100 as uint32_t);)        /*<Burst enable bit */
pub macro_rules! FSMC_BCR3_WAITPOL (() =>                   (0x00000200 as uint32_t);)        /*<Wait signal polarity bit. */
pub macro_rules! FSMC_BCR3_WRAPMOD (() =>                   (0x00000400 as uint32_t);)        /*<Wrapped burst mode support */
pub macro_rules! FSMC_BCR3_WAITCFG (() =>                   (0x00000800 as uint32_t);)        /*<Wait timing configuration */
pub macro_rules! FSMC_BCR3_WREN (() =>                      (0x00001000 as uint32_t);)        /*<Write enable bit */
pub macro_rules! FSMC_BCR3_WAITEN (() =>                    (0x00002000 as uint32_t);)        /*<Wait enable bit */
pub macro_rules! FSMC_BCR3_EXTMOD (() =>                    (0x00004000 as uint32_t);)        /*<Extended mode enable */
pub macro_rules! FSMC_BCR3_ASYNCWAIT (() =>                 (0x00008000 as uint32_t);)        /*<Asynchronous wait */
pub macro_rules! FSMC_BCR3_CBURSTRW (() =>                  (0x00080000 as uint32_t);)        /*<Write burst enable */

/*  Bit definition for FSMC_BCR4 register  */
pub macro_rules! FSMC_BCR4_MBKEN (() =>                     (0x00000001 as uint32_t);)        /*<Memory bank enable bit */
pub macro_rules! FSMC_BCR4_MUXEN (() =>                     (0x00000002 as uint32_t);)        /*<Address/data multiplexing enable bit */

pub macro_rules! FSMC_BCR4_MTYP (() =>                      (0x0000000C as uint32_t);)        /*<MTYP[1:0] bits (Memory type) */
pub macro_rules! FSMC_BCR4_MTYP_0 (() =>                    (0x00000004 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR4_MTYP_1 (() =>                    (0x00000008 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR4_MWID (() =>                      (0x00000030 as uint32_t);)        /*<MWID[1:0] bits (Memory data bus width) */
pub macro_rules! FSMC_BCR4_MWID_0 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BCR4_MWID_1 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_BCR4_FACCEN (() =>                    (0x00000040 as uint32_t);)        /*<Flash access enable */
pub macro_rules! FSMC_BCR4_BURSTEN (() =>                   (0x00000100 as uint32_t);)        /*<Burst enable bit */
pub macro_rules! FSMC_BCR4_WAITPOL (() =>                   (0x00000200 as uint32_t);)        /*<Wait signal polarity bit */
pub macro_rules! FSMC_BCR4_WRAPMOD (() =>                   (0x00000400 as uint32_t);)        /*<Wrapped burst mode support */
pub macro_rules! FSMC_BCR4_WAITCFG (() =>                   (0x00000800 as uint32_t);)        /*<Wait timing configuration */
pub macro_rules! FSMC_BCR4_WREN (() =>                      (0x00001000 as uint32_t);)        /*<Write enable bit */
pub macro_rules! FSMC_BCR4_WAITEN (() =>                    (0x00002000 as uint32_t);)        /*<Wait enable bit */
pub macro_rules! FSMC_BCR4_EXTMOD (() =>                    (0x00004000 as uint32_t);)        /*<Extended mode enable */
pub macro_rules! FSMC_BCR4_ASYNCWAIT (() =>                 (0x00008000 as uint32_t);)        /*<Asynchronous wait */
pub macro_rules! FSMC_BCR4_CBURSTRW (() =>                  (0x00080000 as uint32_t);)        /*<Write burst enable */

/*  Bit definition for FSMC_BTR1 register  */
pub macro_rules! FSMC_BTR1_ADDSET (() =>                    (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BTR1_ADDSET_0 (() =>                  (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR1_ADDSET_1 (() =>                  (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR1_ADDSET_2 (() =>                  (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR1_ADDSET_3 (() =>                  (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR1_ADDHLD (() =>                    (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BTR1_ADDHLD_0 (() =>                  (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR1_ADDHLD_1 (() =>                  (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR1_ADDHLD_2 (() =>                  (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR1_ADDHLD_3 (() =>                  (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR1_DATAST (() =>                    (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BTR1_DATAST_0 (() =>                  (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR1_DATAST_1 (() =>                  (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR1_DATAST_2 (() =>                  (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR1_DATAST_3 (() =>                  (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR1_BUSTURN (() =>                   (0x000F0000 as uint32_t);)        /*<BUSTURN[3:0] bits (Bus turnaround phase duration) */
pub macro_rules! FSMC_BTR1_BUSTURN_0 (() =>                 (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR1_BUSTURN_1 (() =>                 (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR1_BUSTURN_2 (() =>                 (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR1_BUSTURN_3 (() =>                 (0x00080000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR1_CLKDIV (() =>                    (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BTR1_CLKDIV_0 (() =>                  (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR1_CLKDIV_1 (() =>                  (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR1_CLKDIV_2 (() =>                  (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR1_CLKDIV_3 (() =>                  (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR1_DATLAT (() =>                    (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BTR1_DATLAT_0 (() =>                  (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR1_DATLAT_1 (() =>                  (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR1_DATLAT_2 (() =>                  (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR1_DATLAT_3 (() =>                  (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR1_ACCMOD (() =>                    (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BTR1_ACCMOD_0 (() =>                  (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR1_ACCMOD_1 (() =>                  (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_BTR2 register  */
pub macro_rules! FSMC_BTR2_ADDSET (() =>                    (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BTR2_ADDSET_0 (() =>                  (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR2_ADDSET_1 (() =>                  (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR2_ADDSET_2 (() =>                  (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR2_ADDSET_3 (() =>                  (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR2_ADDHLD (() =>                    (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BTR2_ADDHLD_0 (() =>                  (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR2_ADDHLD_1 (() =>                  (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR2_ADDHLD_2 (() =>                  (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR2_ADDHLD_3 (() =>                  (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR2_DATAST (() =>                    (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BTR2_DATAST_0 (() =>                  (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR2_DATAST_1 (() =>                  (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR2_DATAST_2 (() =>                  (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR2_DATAST_3 (() =>                  (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR2_BUSTURN (() =>                   (0x000F0000 as uint32_t);)        /*<BUSTURN[3:0] bits (Bus turnaround phase duration) */
pub macro_rules! FSMC_BTR2_BUSTURN_0 (() =>                 (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR2_BUSTURN_1 (() =>                 (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR2_BUSTURN_2 (() =>                 (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR2_BUSTURN_3 (() =>                 (0x00080000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR2_CLKDIV (() =>                    (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BTR2_CLKDIV_0 (() =>                  (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR2_CLKDIV_1 (() =>                  (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR2_CLKDIV_2 (() =>                  (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR2_CLKDIV_3 (() =>                  (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR2_DATLAT (() =>                    (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BTR2_DATLAT_0 (() =>                  (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR2_DATLAT_1 (() =>                  (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR2_DATLAT_2 (() =>                  (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR2_DATLAT_3 (() =>                  (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR2_ACCMOD (() =>                    (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BTR2_ACCMOD_0 (() =>                  (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR2_ACCMOD_1 (() =>                  (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_BTR3 register  */
pub macro_rules! FSMC_BTR3_ADDSET (() =>                    (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BTR3_ADDSET_0 (() =>                  (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR3_ADDSET_1 (() =>                  (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR3_ADDSET_2 (() =>                  (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR3_ADDSET_3 (() =>                  (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR3_ADDHLD (() =>                    (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BTR3_ADDHLD_0 (() =>                  (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR3_ADDHLD_1 (() =>                  (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR3_ADDHLD_2 (() =>                  (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR3_ADDHLD_3 (() =>                  (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR3_DATAST (() =>                    (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BTR3_DATAST_0 (() =>                  (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR3_DATAST_1 (() =>                  (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR3_DATAST_2 (() =>                  (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR3_DATAST_3 (() =>                  (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR3_BUSTURN (() =>                   (0x000F0000 as uint32_t);)        /*<BUSTURN[3:0] bits (Bus turnaround phase duration) */
pub macro_rules! FSMC_BTR3_BUSTURN_0 (() =>                 (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR3_BUSTURN_1 (() =>                 (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR3_BUSTURN_2 (() =>                 (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR3_BUSTURN_3 (() =>                 (0x00080000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR3_CLKDIV (() =>                    (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BTR3_CLKDIV_0 (() =>                  (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR3_CLKDIV_1 (() =>                  (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR3_CLKDIV_2 (() =>                  (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR3_CLKDIV_3 (() =>                  (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR3_DATLAT (() =>                    (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BTR3_DATLAT_0 (() =>                  (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR3_DATLAT_1 (() =>                  (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR3_DATLAT_2 (() =>                  (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR3_DATLAT_3 (() =>                  (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR3_ACCMOD (() =>                    (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BTR3_ACCMOD_0 (() =>                  (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR3_ACCMOD_1 (() =>                  (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_BTR4 register  */
pub macro_rules! FSMC_BTR4_ADDSET (() =>                    (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BTR4_ADDSET_0 (() =>                  (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR4_ADDSET_1 (() =>                  (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR4_ADDSET_2 (() =>                  (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR4_ADDSET_3 (() =>                  (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR4_ADDHLD (() =>                    (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BTR4_ADDHLD_0 (() =>                  (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR4_ADDHLD_1 (() =>                  (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR4_ADDHLD_2 (() =>                  (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR4_ADDHLD_3 (() =>                  (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR4_DATAST (() =>                    (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BTR4_DATAST_0 (() =>                  (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR4_DATAST_1 (() =>                  (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR4_DATAST_2 (() =>                  (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR4_DATAST_3 (() =>                  (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR4_BUSTURN (() =>                   (0x000F0000 as uint32_t);)        /*<BUSTURN[3:0] bits (Bus turnaround phase duration) */
pub macro_rules! FSMC_BTR4_BUSTURN_0 (() =>                 (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR4_BUSTURN_1 (() =>                 (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR4_BUSTURN_2 (() =>                 (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR4_BUSTURN_3 (() =>                 (0x00080000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR4_CLKDIV (() =>                    (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BTR4_CLKDIV_0 (() =>                  (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR4_CLKDIV_1 (() =>                  (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR4_CLKDIV_2 (() =>                  (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR4_CLKDIV_3 (() =>                  (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR4_DATLAT (() =>                    (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BTR4_DATLAT_0 (() =>                  (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR4_DATLAT_1 (() =>                  (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BTR4_DATLAT_2 (() =>                  (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BTR4_DATLAT_3 (() =>                  (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BTR4_ACCMOD (() =>                    (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BTR4_ACCMOD_0 (() =>                  (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BTR4_ACCMOD_1 (() =>                  (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_BWTR1 register  */
pub macro_rules! FSMC_BWTR1_ADDSET (() =>                   (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BWTR1_ADDSET_0 (() =>                 (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR1_ADDSET_1 (() =>                 (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR1_ADDSET_2 (() =>                 (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR1_ADDSET_3 (() =>                 (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR1_ADDHLD (() =>                   (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BWTR1_ADDHLD_0 (() =>                 (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR1_ADDHLD_1 (() =>                 (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR1_ADDHLD_2 (() =>                 (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR1_ADDHLD_3 (() =>                 (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR1_DATAST (() =>                   (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BWTR1_DATAST_0 (() =>                 (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR1_DATAST_1 (() =>                 (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR1_DATAST_2 (() =>                 (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR1_DATAST_3 (() =>                 (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR1_CLKDIV (() =>                   (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BWTR1_CLKDIV_0 (() =>                 (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR1_CLKDIV_1 (() =>                 (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR1_CLKDIV_2 (() =>                 (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR1_CLKDIV_3 (() =>                 (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR1_DATLAT (() =>                   (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BWTR1_DATLAT_0 (() =>                 (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR1_DATLAT_1 (() =>                 (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR1_DATLAT_2 (() =>                 (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR1_DATLAT_3 (() =>                 (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR1_ACCMOD (() =>                   (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BWTR1_ACCMOD_0 (() =>                 (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR1_ACCMOD_1 (() =>                 (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_BWTR2 register  */
pub macro_rules! FSMC_BWTR2_ADDSET (() =>                   (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BWTR2_ADDSET_0 (() =>                 (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR2_ADDSET_1 (() =>                 (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR2_ADDSET_2 (() =>                 (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR2_ADDSET_3 (() =>                 (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR2_ADDHLD (() =>                   (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BWTR2_ADDHLD_0 (() =>                 (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR2_ADDHLD_1 (() =>                 (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR2_ADDHLD_2 (() =>                 (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR2_ADDHLD_3 (() =>                 (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR2_DATAST (() =>                   (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BWTR2_DATAST_0 (() =>                 (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR2_DATAST_1 (() =>                 (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR2_DATAST_2 (() =>                 (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR2_DATAST_3 (() =>                 (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR2_CLKDIV (() =>                   (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BWTR2_CLKDIV_0 (() =>                 (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR2_CLKDIV_1 (() =>                 (0x00200000 as uint32_t);)        /*<Bit 1*/
pub macro_rules! FSMC_BWTR2_CLKDIV_2 (() =>                 (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR2_CLKDIV_3 (() =>                 (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR2_DATLAT (() =>                   (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BWTR2_DATLAT_0 (() =>                 (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR2_DATLAT_1 (() =>                 (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR2_DATLAT_2 (() =>                 (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR2_DATLAT_3 (() =>                 (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR2_ACCMOD (() =>                   (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BWTR2_ACCMOD_0 (() =>                 (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR2_ACCMOD_1 (() =>                 (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_BWTR3 register  */
pub macro_rules! FSMC_BWTR3_ADDSET (() =>                   (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BWTR3_ADDSET_0 (() =>                 (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR3_ADDSET_1 (() =>                 (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR3_ADDSET_2 (() =>                 (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR3_ADDSET_3 (() =>                 (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR3_ADDHLD (() =>                   (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BWTR3_ADDHLD_0 (() =>                 (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR3_ADDHLD_1 (() =>                 (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR3_ADDHLD_2 (() =>                 (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR3_ADDHLD_3 (() =>                 (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR3_DATAST (() =>                   (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BWTR3_DATAST_0 (() =>                 (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR3_DATAST_1 (() =>                 (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR3_DATAST_2 (() =>                 (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR3_DATAST_3 (() =>                 (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR3_CLKDIV (() =>                   (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BWTR3_CLKDIV_0 (() =>                 (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR3_CLKDIV_1 (() =>                 (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR3_CLKDIV_2 (() =>                 (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR3_CLKDIV_3 (() =>                 (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR3_DATLAT (() =>                   (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BWTR3_DATLAT_0 (() =>                 (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR3_DATLAT_1 (() =>                 (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR3_DATLAT_2 (() =>                 (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR3_DATLAT_3 (() =>                 (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR3_ACCMOD (() =>                   (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BWTR3_ACCMOD_0 (() =>                 (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR3_ACCMOD_1 (() =>                 (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_BWTR4 register  */
pub macro_rules! FSMC_BWTR4_ADDSET (() =>                   (0x0000000F as uint32_t);)        /*<ADDSET[3:0] bits (Address setup phase duration) */
pub macro_rules! FSMC_BWTR4_ADDSET_0 (() =>                 (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR4_ADDSET_1 (() =>                 (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR4_ADDSET_2 (() =>                 (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR4_ADDSET_3 (() =>                 (0x00000008 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR4_ADDHLD (() =>                   (0x000000F0 as uint32_t);)        /*<ADDHLD[3:0] bits (Address-hold phase duration) */
pub macro_rules! FSMC_BWTR4_ADDHLD_0 (() =>                 (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR4_ADDHLD_1 (() =>                 (0x00000020 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR4_ADDHLD_2 (() =>                 (0x00000040 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR4_ADDHLD_3 (() =>                 (0x00000080 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR4_DATAST (() =>                   (0x0000FF00 as uint32_t);)        /*<DATAST [3:0] bits (Data-phase duration) */
pub macro_rules! FSMC_BWTR4_DATAST_0 (() =>                 (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR4_DATAST_1 (() =>                 (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR4_DATAST_2 (() =>                 (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR4_DATAST_3 (() =>                 (0x00000800 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR4_CLKDIV (() =>                   (0x00F00000 as uint32_t);)        /*<CLKDIV[3:0] bits (Clock divide ratio) */
pub macro_rules! FSMC_BWTR4_CLKDIV_0 (() =>                 (0x00100000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR4_CLKDIV_1 (() =>                 (0x00200000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR4_CLKDIV_2 (() =>                 (0x00400000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR4_CLKDIV_3 (() =>                 (0x00800000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR4_DATLAT (() =>                   (0x0F000000 as uint32_t);)        /*<DATLA[3:0] bits (Data latency) */
pub macro_rules! FSMC_BWTR4_DATLAT_0 (() =>                 (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR4_DATLAT_1 (() =>                 (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_BWTR4_DATLAT_2 (() =>                 (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_BWTR4_DATLAT_3 (() =>                 (0x08000000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_BWTR4_ACCMOD (() =>                   (0x30000000 as uint32_t);)        /*<ACCMOD[1:0] bits (Access mode) */
pub macro_rules! FSMC_BWTR4_ACCMOD_0 (() =>                 (0x10000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_BWTR4_ACCMOD_1 (() =>                 (0x20000000 as uint32_t);)        /*<Bit 1 */

/*  Bit definition for FSMC_PCR2 register  */
pub macro_rules! FSMC_PCR2_PWAITEN (() =>                   (0x00000002 as uint32_t);)        /*<Wait feature enable bit */
pub macro_rules! FSMC_PCR2_PBKEN (() =>                     (0x00000004 as uint32_t);)        /*<PC Card/NAND Flash memory bank enable bit */
pub macro_rules! FSMC_PCR2_PTYP (() =>                      (0x00000008 as uint32_t);)        /*<Memory type */

pub macro_rules! FSMC_PCR2_PWID (() =>                      (0x00000030 as uint32_t);)        /*<PWID[1:0] bits (NAND Flash databus width) */
pub macro_rules! FSMC_PCR2_PWID_0 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR2_PWID_1 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_PCR2_ECCEN (() =>                     (0x00000040 as uint32_t);)        /*<ECC computation logic enable bit */

pub macro_rules! FSMC_PCR2_TCLR (() =>                      (0x00001E00 as uint32_t);)        /*<TCLR[3:0] bits (CLE to RE delay) */
pub macro_rules! FSMC_PCR2_TCLR_0 (() =>                    (0x00000200 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR2_TCLR_1 (() =>                    (0x00000400 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR2_TCLR_2 (() =>                    (0x00000800 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PCR2_TCLR_3 (() =>                    (0x00001000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_PCR2_TAR (() =>                       (0x0001E000 as uint32_t);)        /*<TAR[3:0] bits (ALE to RE delay) */
pub macro_rules! FSMC_PCR2_TAR_0 (() =>                     (0x00002000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR2_TAR_1 (() =>                     (0x00004000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR2_TAR_2 (() =>                     (0x00008000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PCR2_TAR_3 (() =>                     (0x00010000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_PCR2_ECCPS (() =>                     (0x000E0000 as uint32_t);)        /*<ECCPS[1:0] bits (ECC page size) */
pub macro_rules! FSMC_PCR2_ECCPS_0 (() =>                   (0x00020000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR2_ECCPS_1 (() =>                   (0x00040000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR2_ECCPS_2 (() =>                   (0x00080000 as uint32_t);)        /*<Bit 2 */

/*  Bit definition for FSMC_PCR3 register  */
pub macro_rules! FSMC_PCR3_PWAITEN (() =>                   (0x00000002 as uint32_t);)        /*<Wait feature enable bit */
pub macro_rules! FSMC_PCR3_PBKEN (() =>                     (0x00000004 as uint32_t);)        /*<PC Card/NAND Flash memory bank enable bit */
pub macro_rules! FSMC_PCR3_PTYP (() =>                      (0x00000008 as uint32_t);)        /*<Memory type */

pub macro_rules! FSMC_PCR3_PWID (() =>                      (0x00000030 as uint32_t);)        /*<PWID[1:0] bits (NAND Flash databus width) */
pub macro_rules! FSMC_PCR3_PWID_0 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR3_PWID_1 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_PCR3_ECCEN (() =>                     (0x00000040 as uint32_t);)        /*<ECC computation logic enable bit */

pub macro_rules! FSMC_PCR3_TCLR (() =>                      (0x00001E00 as uint32_t);)        /*<TCLR[3:0] bits (CLE to RE delay) */
pub macro_rules! FSMC_PCR3_TCLR_0 (() =>                    (0x00000200 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR3_TCLR_1 (() =>                    (0x00000400 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR3_TCLR_2 (() =>                    (0x00000800 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PCR3_TCLR_3 (() =>                    (0x00001000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_PCR3_TAR (() =>                       (0x0001E000 as uint32_t);)        /*<TAR[3:0] bits (ALE to RE delay) */
pub macro_rules! FSMC_PCR3_TAR_0 (() =>                     (0x00002000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR3_TAR_1 (() =>                     (0x00004000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR3_TAR_2 (() =>                     (0x00008000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PCR3_TAR_3 (() =>                     (0x00010000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_PCR3_ECCPS (() =>                     (0x000E0000 as uint32_t);)        /*<ECCPS[2:0] bits (ECC page size) */
pub macro_rules! FSMC_PCR3_ECCPS_0 (() =>                   (0x00020000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR3_ECCPS_1 (() =>                   (0x00040000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR3_ECCPS_2 (() =>                   (0x00080000 as uint32_t);)        /*<Bit 2 */

/*  Bit definition for FSMC_PCR4 register  */
pub macro_rules! FSMC_PCR4_PWAITEN (() =>                   (0x00000002 as uint32_t);)        /*<Wait feature enable bit */
pub macro_rules! FSMC_PCR4_PBKEN (() =>                     (0x00000004 as uint32_t);)        /*<PC Card/NAND Flash memory bank enable bit */
pub macro_rules! FSMC_PCR4_PTYP (() =>                      (0x00000008 as uint32_t);)        /*<Memory type */

pub macro_rules! FSMC_PCR4_PWID (() =>                      (0x00000030 as uint32_t);)        /*<PWID[1:0] bits (NAND Flash databus width) */
pub macro_rules! FSMC_PCR4_PWID_0 (() =>                    (0x00000010 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR4_PWID_1 (() =>                    (0x00000020 as uint32_t);)        /*<Bit 1 */

pub macro_rules! FSMC_PCR4_ECCEN (() =>                     (0x00000040 as uint32_t);)        /*<ECC computation logic enable bit */

pub macro_rules! FSMC_PCR4_TCLR (() =>                      (0x00001E00 as uint32_t);)        /*<TCLR[3:0] bits (CLE to RE delay) */
pub macro_rules! FSMC_PCR4_TCLR_0 (() =>                    (0x00000200 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR4_TCLR_1 (() =>                    (0x00000400 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR4_TCLR_2 (() =>                    (0x00000800 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PCR4_TCLR_3 (() =>                    (0x00001000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_PCR4_TAR (() =>                       (0x0001E000 as uint32_t);)        /*<TAR[3:0] bits (ALE to RE delay) */
pub macro_rules! FSMC_PCR4_TAR_0 (() =>                     (0x00002000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR4_TAR_1 (() =>                     (0x00004000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR4_TAR_2 (() =>                     (0x00008000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PCR4_TAR_3 (() =>                     (0x00010000 as uint32_t);)        /*<Bit 3 */

pub macro_rules! FSMC_PCR4_ECCPS (() =>                     (0x000E0000 as uint32_t);)        /*<ECCPS[2:0] bits (ECC page size) */
pub macro_rules! FSMC_PCR4_ECCPS_0 (() =>                   (0x00020000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PCR4_ECCPS_1 (() =>                   (0x00040000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PCR4_ECCPS_2 (() =>                   (0x00080000 as uint32_t);)        /*<Bit 2 */

/*  Bit definition for FSMC_SR2 register  */
pub macro_rules! FSMC_SR2_IRS (() =>                        (0x01 as uint8_t);)               /*<Interrupt Rising Edge status */
pub macro_rules! FSMC_SR2_ILS (() =>                        (0x02 as uint8_t);)               /*<Interrupt Level status */
pub macro_rules! FSMC_SR2_IFS (() =>                        (0x04 as uint8_t);)               /*<Interrupt Falling Edge status */
pub macro_rules! FSMC_SR2_IREN (() =>                       (0x08 as uint8_t);)               /*<Interrupt Rising Edge detection Enable bit */
pub macro_rules! FSMC_SR2_ILEN (() =>                       (0x10 as uint8_t);)               /*<Interrupt Level detection Enable bit */
pub macro_rules! FSMC_SR2_IFEN (() =>                       (0x20 as uint8_t);)               /*<Interrupt Falling Edge detection Enable bit */
pub macro_rules! FSMC_SR2_FEMPT (() =>                      (0x40 as uint8_t);)               /*<FIFO empty */

/*  Bit definition for FSMC_SR3 register  */
pub macro_rules! FSMC_SR3_IRS (() =>                        (0x01 as uint8_t);)               /*<Interrupt Rising Edge status */
pub macro_rules! FSMC_SR3_ILS (() =>                        (0x02 as uint8_t);)               /*<Interrupt Level status */
pub macro_rules! FSMC_SR3_IFS (() =>                        (0x04 as uint8_t);)               /*<Interrupt Falling Edge status */
pub macro_rules! FSMC_SR3_IREN (() =>                       (0x08 as uint8_t);)               /*<Interrupt Rising Edge detection Enable bit */
pub macro_rules! FSMC_SR3_ILEN (() =>                       (0x10 as uint8_t);)               /*<Interrupt Level detection Enable bit */
pub macro_rules! FSMC_SR3_IFEN (() =>                       (0x20 as uint8_t);)               /*<Interrupt Falling Edge detection Enable bit */
pub macro_rules! FSMC_SR3_FEMPT (() =>                      (0x40 as uint8_t);)               /*<FIFO empty */

/*  Bit definition for FSMC_SR4 register  */
pub macro_rules! FSMC_SR4_IRS (() =>                        (0x01 as uint8_t);)               /*<Interrupt Rising Edge status */
pub macro_rules! FSMC_SR4_ILS (() =>                        (0x02 as uint8_t);)               /*<Interrupt Level status */
pub macro_rules! FSMC_SR4_IFS (() =>                        (0x04 as uint8_t);)               /*<Interrupt Falling Edge status */
pub macro_rules! FSMC_SR4_IREN (() =>                       (0x08 as uint8_t);)               /*<Interrupt Rising Edge detection Enable bit */
pub macro_rules! FSMC_SR4_ILEN (() =>                       (0x10 as uint8_t);)               /*<Interrupt Level detection Enable bit */
pub macro_rules! FSMC_SR4_IFEN (() =>                       (0x20 as uint8_t);)               /*<Interrupt Falling Edge detection Enable bit */
pub macro_rules! FSMC_SR4_FEMPT (() =>                      (0x40 as uint8_t);)               /*<FIFO empty */

/*  Bit definition for FSMC_PMEM2 register  */
pub macro_rules! FSMC_PMEM2_MEMSET2 (() =>                  (0x000000FF as uint32_t);)        /*<MEMSET2[7:0] bits (Common memory 2 setup time) */
pub macro_rules! FSMC_PMEM2_MEMSET2_0 (() =>                (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM2_MEMSET2_1 (() =>                (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM2_MEMSET2_2 (() =>                (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM2_MEMSET2_3 (() =>                (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM2_MEMSET2_4 (() =>                (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM2_MEMSET2_5 (() =>                (0x00000020 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM2_MEMSET2_6 (() =>                (0x00000040 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM2_MEMSET2_7 (() =>                (0x00000080 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM2_MEMWAIT2 (() =>                 (0x0000FF00 as uint32_t);)        /*<MEMWAIT2[7:0] bits (Common memory 2 wait time) */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_0 (() =>               (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_1 (() =>               (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_2 (() =>               (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_3 (() =>               (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_4 (() =>               (0x00001000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_5 (() =>               (0x00002000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_6 (() =>               (0x00004000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM2_MEMWAIT2_7 (() =>               (0x00008000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM2_MEMHOLD2 (() =>                 (0x00FF0000 as uint32_t);)        /*<MEMHOLD2[7:0] bits (Common memory 2 hold time) */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_0 (() =>               (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_1 (() =>               (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_2 (() =>               (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_3 (() =>               (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_4 (() =>               (0x00100000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_5 (() =>               (0x00200000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_6 (() =>               (0x00400000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM2_MEMHOLD2_7 (() =>               (0x00800000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM2_MEMHIZ2 (() =>                  (0xFF000000 as uint32_t);)        /*<MEMHIZ2[7:0] bits (Common memory 2 databus HiZ time) */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_0 (() =>                (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_1 (() =>                (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_2 (() =>                (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_3 (() =>                (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_4 (() =>                (0x10000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_5 (() =>                (0x20000000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_6 (() =>                (0x40000000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM2_MEMHIZ2_7 (() =>                (0x80000000 as uint32_t);)        /*<Bit 7 */

/*  Bit definition for FSMC_PMEM3 register  */
pub macro_rules! FSMC_PMEM3_MEMSET3 (() =>                  (0x000000FF as uint32_t);)        /*<MEMSET3[7:0] bits (Common memory 3 setup time) */
pub macro_rules! FSMC_PMEM3_MEMSET3_0 (() =>                (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM3_MEMSET3_1 (() =>                (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM3_MEMSET3_2 (() =>                (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM3_MEMSET3_3 (() =>                (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM3_MEMSET3_4 (() =>                (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM3_MEMSET3_5 (() =>                (0x00000020 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM3_MEMSET3_6 (() =>                (0x00000040 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM3_MEMSET3_7 (() =>                (0x00000080 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM3_MEMWAIT3 (() =>                 (0x0000FF00 as uint32_t);)        /*<MEMWAIT3[7:0] bits (Common memory 3 wait time) */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_0 (() =>               (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_1 (() =>               (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_2 (() =>               (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_3 (() =>               (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_4 (() =>               (0x00001000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_5 (() =>               (0x00002000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_6 (() =>               (0x00004000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM3_MEMWAIT3_7 (() =>               (0x00008000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM3_MEMHOLD3 (() =>                 (0x00FF0000 as uint32_t);)        /*<MEMHOLD3[7:0] bits (Common memory 3 hold time) */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_0 (() =>               (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_1 (() =>               (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_2 (() =>               (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_3 (() =>               (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_4 (() =>               (0x00100000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_5 (() =>               (0x00200000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_6 (() =>               (0x00400000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM3_MEMHOLD3_7 (() =>               (0x00800000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM3_MEMHIZ3 (() =>                  (0xFF000000 as uint32_t);)        /*<MEMHIZ3[7:0] bits (Common memory 3 databus HiZ time) */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_0 (() =>                (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_1 (() =>                (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_2 (() =>                (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_3 (() =>                (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_4 (() =>                (0x10000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_5 (() =>                (0x20000000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_6 (() =>                (0x40000000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM3_MEMHIZ3_7 (() =>                (0x80000000 as uint32_t);)        /*<Bit 7 */

/*  Bit definition for FSMC_PMEM4 register  */
pub macro_rules! FSMC_PMEM4_MEMSET4 (() =>                  (0x000000FF as uint32_t);)        /*<MEMSET4[7:0] bits (Common memory 4 setup time) */
pub macro_rules! FSMC_PMEM4_MEMSET4_0 (() =>                (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM4_MEMSET4_1 (() =>                (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM4_MEMSET4_2 (() =>                (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM4_MEMSET4_3 (() =>                (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM4_MEMSET4_4 (() =>                (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM4_MEMSET4_5 (() =>                (0x00000020 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM4_MEMSET4_6 (() =>                (0x00000040 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM4_MEMSET4_7 (() =>                (0x00000080 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM4_MEMWAIT4 (() =>                 (0x0000FF00 as uint32_t);)        /*<MEMWAIT4[7:0] bits (Common memory 4 wait time) */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_0 (() =>               (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_1 (() =>               (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_2 (() =>               (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_3 (() =>               (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_4 (() =>               (0x00001000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_5 (() =>               (0x00002000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_6 (() =>               (0x00004000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM4_MEMWAIT4_7 (() =>               (0x00008000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM4_MEMHOLD4 (() =>                 (0x00FF0000 as uint32_t);)        /*<MEMHOLD4[7:0] bits (Common memory 4 hold time) */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_0 (() =>               (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_1 (() =>               (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_2 (() =>               (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_3 (() =>               (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_4 (() =>               (0x00100000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_5 (() =>               (0x00200000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_6 (() =>               (0x00400000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM4_MEMHOLD4_7 (() =>               (0x00800000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PMEM4_MEMHIZ4 (() =>                  (0xFF000000 as uint32_t);)        /*<MEMHIZ4[7:0] bits (Common memory 4 databus HiZ time) */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_0 (() =>                (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_1 (() =>                (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_2 (() =>                (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_3 (() =>                (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_4 (() =>                (0x10000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_5 (() =>                (0x20000000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_6 (() =>                (0x40000000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PMEM4_MEMHIZ4_7 (() =>                (0x80000000 as uint32_t);)        /*<Bit 7 */

/*  Bit definition for FSMC_PATT2 register  */
pub macro_rules! FSMC_PATT2_ATTSET2 (() =>                  (0x000000FF as uint32_t);)        /*<ATTSET2[7:0] bits (Attribute memory 2 setup time) */
pub macro_rules! FSMC_PATT2_ATTSET2_0 (() =>                (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT2_ATTSET2_1 (() =>                (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT2_ATTSET2_2 (() =>                (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT2_ATTSET2_3 (() =>                (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT2_ATTSET2_4 (() =>                (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT2_ATTSET2_5 (() =>                (0x00000020 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT2_ATTSET2_6 (() =>                (0x00000040 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT2_ATTSET2_7 (() =>                (0x00000080 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT2_ATTWAIT2 (() =>                 (0x0000FF00 as uint32_t);)        /*<ATTWAIT2[7:0] bits (Attribute memory 2 wait time) */
pub macro_rules! FSMC_PATT2_ATTWAIT2_0 (() =>               (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT2_ATTWAIT2_1 (() =>               (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT2_ATTWAIT2_2 (() =>               (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT2_ATTWAIT2_3 (() =>               (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT2_ATTWAIT2_4 (() =>               (0x00001000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT2_ATTWAIT2_5 (() =>               (0x00002000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT2_ATTWAIT2_6 (() =>               (0x00004000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT2_ATTWAIT2_7 (() =>               (0x00008000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT2_ATTHOLD2 (() =>                 (0x00FF0000 as uint32_t);)        /*<ATTHOLD2[7:0] bits (Attribute memory 2 hold time) */
pub macro_rules! FSMC_PATT2_ATTHOLD2_0 (() =>               (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT2_ATTHOLD2_1 (() =>               (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT2_ATTHOLD2_2 (() =>               (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT2_ATTHOLD2_3 (() =>               (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT2_ATTHOLD2_4 (() =>               (0x00100000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT2_ATTHOLD2_5 (() =>               (0x00200000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT2_ATTHOLD2_6 (() =>               (0x00400000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT2_ATTHOLD2_7 (() =>               (0x00800000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT2_ATTHIZ2 (() =>                  (0xFF000000 as uint32_t);)        /*<ATTHIZ2[7:0] bits (Attribute memory 2 databus HiZ time) */
pub macro_rules! FSMC_PATT2_ATTHIZ2_0 (() =>                (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT2_ATTHIZ2_1 (() =>                (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT2_ATTHIZ2_2 (() =>                (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT2_ATTHIZ2_3 (() =>                (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT2_ATTHIZ2_4 (() =>                (0x10000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT2_ATTHIZ2_5 (() =>                (0x20000000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT2_ATTHIZ2_6 (() =>                (0x40000000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT2_ATTHIZ2_7 (() =>                (0x80000000 as uint32_t);)        /*<Bit 7 */

/*  Bit definition for FSMC_PATT3 register  */
pub macro_rules! FSMC_PATT3_ATTSET3 (() =>                  (0x000000FF as uint32_t);)        /*<ATTSET3[7:0] bits (Attribute memory 3 setup time) */
pub macro_rules! FSMC_PATT3_ATTSET3_0 (() =>                (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT3_ATTSET3_1 (() =>                (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT3_ATTSET3_2 (() =>                (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT3_ATTSET3_3 (() =>                (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT3_ATTSET3_4 (() =>                (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT3_ATTSET3_5 (() =>                (0x00000020 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT3_ATTSET3_6 (() =>                (0x00000040 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT3_ATTSET3_7 (() =>                (0x00000080 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT3_ATTWAIT3 (() =>                 (0x0000FF00 as uint32_t);)        /*<ATTWAIT3[7:0] bits (Attribute memory 3 wait time) */
pub macro_rules! FSMC_PATT3_ATTWAIT3_0 (() =>               (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT3_ATTWAIT3_1 (() =>               (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT3_ATTWAIT3_2 (() =>               (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT3_ATTWAIT3_3 (() =>               (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT3_ATTWAIT3_4 (() =>               (0x00001000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT3_ATTWAIT3_5 (() =>               (0x00002000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT3_ATTWAIT3_6 (() =>               (0x00004000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT3_ATTWAIT3_7 (() =>               (0x00008000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT3_ATTHOLD3 (() =>                 (0x00FF0000 as uint32_t);)        /*<ATTHOLD3[7:0] bits (Attribute memory 3 hold time) */
pub macro_rules! FSMC_PATT3_ATTHOLD3_0 (() =>               (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT3_ATTHOLD3_1 (() =>               (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT3_ATTHOLD3_2 (() =>               (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT3_ATTHOLD3_3 (() =>               (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT3_ATTHOLD3_4 (() =>               (0x00100000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT3_ATTHOLD3_5 (() =>               (0x00200000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT3_ATTHOLD3_6 (() =>               (0x00400000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT3_ATTHOLD3_7 (() =>               (0x00800000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT3_ATTHIZ3 (() =>                  (0xFF000000 as uint32_t);)        /*<ATTHIZ3[7:0] bits (Attribute memory 3 databus HiZ time) */
pub macro_rules! FSMC_PATT3_ATTHIZ3_0 (() =>                (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT3_ATTHIZ3_1 (() =>                (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT3_ATTHIZ3_2 (() =>                (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT3_ATTHIZ3_3 (() =>                (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT3_ATTHIZ3_4 (() =>                (0x10000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT3_ATTHIZ3_5 (() =>                (0x20000000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT3_ATTHIZ3_6 (() =>                (0x40000000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT3_ATTHIZ3_7 (() =>                (0x80000000 as uint32_t);)        /*<Bit 7 */

/*  Bit definition for FSMC_PATT4 register  */
pub macro_rules! FSMC_PATT4_ATTSET4 (() =>                  (0x000000FF as uint32_t);)        /*<ATTSET4[7:0] bits (Attribute memory 4 setup time) */
pub macro_rules! FSMC_PATT4_ATTSET4_0 (() =>                (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT4_ATTSET4_1 (() =>                (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT4_ATTSET4_2 (() =>                (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT4_ATTSET4_3 (() =>                (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT4_ATTSET4_4 (() =>                (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT4_ATTSET4_5 (() =>                (0x00000020 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT4_ATTSET4_6 (() =>                (0x00000040 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT4_ATTSET4_7 (() =>                (0x00000080 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT4_ATTWAIT4 (() =>                 (0x0000FF00 as uint32_t);)        /*<ATTWAIT4[7:0] bits (Attribute memory 4 wait time) */
pub macro_rules! FSMC_PATT4_ATTWAIT4_0 (() =>               (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT4_ATTWAIT4_1 (() =>               (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT4_ATTWAIT4_2 (() =>               (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT4_ATTWAIT4_3 (() =>               (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT4_ATTWAIT4_4 (() =>               (0x00001000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT4_ATTWAIT4_5 (() =>               (0x00002000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT4_ATTWAIT4_6 (() =>               (0x00004000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT4_ATTWAIT4_7 (() =>               (0x00008000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT4_ATTHOLD4 (() =>                 (0x00FF0000 as uint32_t);)        /*<ATTHOLD4[7:0] bits (Attribute memory 4 hold time) */
pub macro_rules! FSMC_PATT4_ATTHOLD4_0 (() =>               (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT4_ATTHOLD4_1 (() =>               (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT4_ATTHOLD4_2 (() =>               (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT4_ATTHOLD4_3 (() =>               (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT4_ATTHOLD4_4 (() =>               (0x00100000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT4_ATTHOLD4_5 (() =>               (0x00200000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT4_ATTHOLD4_6 (() =>               (0x00400000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT4_ATTHOLD4_7 (() =>               (0x00800000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PATT4_ATTHIZ4 (() =>                  (0xFF000000 as uint32_t);)        /*<ATTHIZ4[7:0] bits (Attribute memory 4 databus HiZ time) */
pub macro_rules! FSMC_PATT4_ATTHIZ4_0 (() =>                (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PATT4_ATTHIZ4_1 (() =>                (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PATT4_ATTHIZ4_2 (() =>                (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PATT4_ATTHIZ4_3 (() =>                (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PATT4_ATTHIZ4_4 (() =>                (0x10000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PATT4_ATTHIZ4_5 (() =>                (0x20000000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PATT4_ATTHIZ4_6 (() =>                (0x40000000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PATT4_ATTHIZ4_7 (() =>                (0x80000000 as uint32_t);)        /*<Bit 7 */

/*  Bit definition for FSMC_PIO4 register  */
pub macro_rules! FSMC_PIO4_IOSET4 (() =>                    (0x000000FF as uint32_t);)        /*<IOSET4[7:0] bits (I/O 4 setup time) */
pub macro_rules! FSMC_PIO4_IOSET4_0 (() =>                  (0x00000001 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PIO4_IOSET4_1 (() =>                  (0x00000002 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PIO4_IOSET4_2 (() =>                  (0x00000004 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PIO4_IOSET4_3 (() =>                  (0x00000008 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PIO4_IOSET4_4 (() =>                  (0x00000010 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PIO4_IOSET4_5 (() =>                  (0x00000020 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PIO4_IOSET4_6 (() =>                  (0x00000040 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PIO4_IOSET4_7 (() =>                  (0x00000080 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PIO4_IOWAIT4 (() =>                   (0x0000FF00 as uint32_t);)        /*<IOWAIT4[7:0] bits (I/O 4 wait time) */
pub macro_rules! FSMC_PIO4_IOWAIT4_0 (() =>                 (0x00000100 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PIO4_IOWAIT4_1 (() =>                 (0x00000200 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PIO4_IOWAIT4_2 (() =>                 (0x00000400 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PIO4_IOWAIT4_3 (() =>                 (0x00000800 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PIO4_IOWAIT4_4 (() =>                 (0x00001000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PIO4_IOWAIT4_5 (() =>                 (0x00002000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PIO4_IOWAIT4_6 (() =>                 (0x00004000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PIO4_IOWAIT4_7 (() =>                 (0x00008000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PIO4_IOHOLD4 (() =>                   (0x00FF0000 as uint32_t);)        /*<IOHOLD4[7:0] bits (I/O 4 hold time) */
pub macro_rules! FSMC_PIO4_IOHOLD4_0 (() =>                 (0x00010000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PIO4_IOHOLD4_1 (() =>                 (0x00020000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PIO4_IOHOLD4_2 (() =>                 (0x00040000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PIO4_IOHOLD4_3 (() =>                 (0x00080000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PIO4_IOHOLD4_4 (() =>                 (0x00100000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PIO4_IOHOLD4_5 (() =>                 (0x00200000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PIO4_IOHOLD4_6 (() =>                 (0x00400000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PIO4_IOHOLD4_7 (() =>                 (0x00800000 as uint32_t);)        /*<Bit 7 */

pub macro_rules! FSMC_PIO4_IOHIZ4 (() =>                    (0xFF000000 as uint32_t);)        /*<IOHIZ4[7:0] bits (I/O 4 databus HiZ time) */
pub macro_rules! FSMC_PIO4_IOHIZ4_0 (() =>                  (0x01000000 as uint32_t);)        /*<Bit 0 */
pub macro_rules! FSMC_PIO4_IOHIZ4_1 (() =>                  (0x02000000 as uint32_t);)        /*<Bit 1 */
pub macro_rules! FSMC_PIO4_IOHIZ4_2 (() =>                  (0x04000000 as uint32_t);)        /*<Bit 2 */
pub macro_rules! FSMC_PIO4_IOHIZ4_3 (() =>                  (0x08000000 as uint32_t);)        /*<Bit 3 */
pub macro_rules! FSMC_PIO4_IOHIZ4_4 (() =>                  (0x10000000 as uint32_t);)        /*<Bit 4 */
pub macro_rules! FSMC_PIO4_IOHIZ4_5 (() =>                  (0x20000000 as uint32_t);)        /*<Bit 5 */
pub macro_rules! FSMC_PIO4_IOHIZ4_6 (() =>                  (0x40000000 as uint32_t);)        /*<Bit 6 */
pub macro_rules! FSMC_PIO4_IOHIZ4_7 (() =>                  (0x80000000 as uint32_t);)        /*<Bit 7 */

/*  Bit definition for FSMC_ECCR2 register  */
pub macro_rules! FSMC_ECCR2_ECC2 (() =>                     (0xFFFFFFFF as uint32_t);)        /*<ECC result */

/*  Bit definition for FSMC_ECCR3 register  */
pub macro_rules! FSMC_ECCR3_ECC3 (() =>                     (0xFFFFFFFF as uint32_t);)        /*<ECC result */


/*                                                                            */
/*                            General Purpose I/O                             */
/*                                                                            */

/*  Bits definition for GPIO_MODER register  */
pub macro_rules! GPIO_MODER_MODER0 (() =>                    (0x00000003 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER0_0 (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER0_1 (() =>                  (0x00000002 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER1 (() =>                    (0x0000000C as uint32_t);)
pub macro_rules! GPIO_MODER_MODER1_0 (() =>                  (0x00000004 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER1_1 (() =>                  (0x00000008 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER2 (() =>                    (0x00000030 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER2_0 (() =>                  (0x00000010 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER2_1 (() =>                  (0x00000020 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER3 (() =>                    (0x000000C0 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER3_0 (() =>                  (0x00000040 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER3_1 (() =>                  (0x00000080 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER4 (() =>                    (0x00000300 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER4_0 (() =>                  (0x00000100 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER4_1 (() =>                  (0x00000200 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER5 (() =>                    (0x00000C00 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER5_0 (() =>                  (0x00000400 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER5_1 (() =>                  (0x00000800 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER6 (() =>                    (0x00003000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER6_0 (() =>                  (0x00001000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER6_1 (() =>                  (0x00002000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER7 (() =>                    (0x0000C000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER7_0 (() =>                  (0x00004000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER7_1 (() =>                  (0x00008000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER8 (() =>                    (0x00030000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER8_0 (() =>                  (0x00010000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER8_1 (() =>                  (0x00020000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER9 (() =>                    (0x000C0000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER9_0 (() =>                  (0x00040000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER9_1 (() =>                  (0x00080000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER10 (() =>                   (0x00300000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER10_0 (() =>                 (0x00100000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER10_1 (() =>                 (0x00200000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER11 (() =>                   (0x00C00000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER11_0 (() =>                 (0x00400000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER11_1 (() =>                 (0x00800000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER12 (() =>                   (0x03000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER12_0 (() =>                 (0x01000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER12_1 (() =>                 (0x02000000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER13 (() =>                   (0x0C000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER13_0 (() =>                 (0x04000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER13_1 (() =>                 (0x08000000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER14 (() =>                   (0x30000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER14_0 (() =>                 (0x10000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER14_1 (() =>                 (0x20000000 as uint32_t);)

pub macro_rules! GPIO_MODER_MODER15 (() =>                   (0xC0000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER15_0 (() =>                 (0x40000000 as uint32_t);)
pub macro_rules! GPIO_MODER_MODER15_1 (() =>                 (0x80000000 as uint32_t);)

/*  Bits definition for GPIO_OTYPER register  */
pub macro_rules! GPIO_OTYPER_OT_0 (() =>                     (0x00000001 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_1 (() =>                     (0x00000002 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_2 (() =>                     (0x00000004 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_3 (() =>                     (0x00000008 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_4 (() =>                     (0x00000010 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_5 (() =>                     (0x00000020 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_6 (() =>                     (0x00000040 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_7 (() =>                     (0x00000080 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_8 (() =>                     (0x00000100 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_9 (() =>                     (0x00000200 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_10 (() =>                    (0x00000400 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_11 (() =>                    (0x00000800 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_12 (() =>                    (0x00001000 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_13 (() =>                    (0x00002000 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_14 (() =>                    (0x00004000 as uint32_t);)
pub macro_rules! GPIO_OTYPER_OT_15 (() =>                    (0x00008000 as uint32_t);)

/*  Bits definition for GPIO_OSPEEDR register  */
pub macro_rules! GPIO_OSPEEDER_OSPEEDR0 (() =>               (0x00000003 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR0_0 (() =>             (0x00000001 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR0_1 (() =>             (0x00000002 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR1 (() =>               (0x0000000C as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR1_0 (() =>             (0x00000004 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR1_1 (() =>             (0x00000008 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR2 (() =>               (0x00000030 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR2_0 (() =>             (0x00000010 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR2_1 (() =>             (0x00000020 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR3 (() =>               (0x000000C0 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR3_0 (() =>             (0x00000040 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR3_1 (() =>             (0x00000080 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR4 (() =>               (0x00000300 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR4_0 (() =>             (0x00000100 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR4_1 (() =>             (0x00000200 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR5 (() =>               (0x00000C00 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR5_0 (() =>             (0x00000400 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR5_1 (() =>             (0x00000800 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR6 (() =>               (0x00003000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR6_0 (() =>             (0x00001000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR6_1 (() =>             (0x00002000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR7 (() =>               (0x0000C000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR7_0 (() =>             (0x00004000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR7_1 (() =>             (0x00008000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR8 (() =>               (0x00030000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR8_0 (() =>             (0x00010000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR8_1 (() =>             (0x00020000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR9 (() =>               (0x000C0000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR9_0 (() =>             (0x00040000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR9_1 (() =>             (0x00080000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR10 (() =>              (0x00300000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR10_0 (() =>            (0x00100000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR10_1 (() =>            (0x00200000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR11 (() =>              (0x00C00000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR11_0 (() =>            (0x00400000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR11_1 (() =>            (0x00800000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR12 (() =>              (0x03000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR12_0 (() =>            (0x01000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR12_1 (() =>            (0x02000000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR13 (() =>              (0x0C000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR13_0 (() =>            (0x04000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR13_1 (() =>            (0x08000000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR14 (() =>              (0x30000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR14_0 (() =>            (0x10000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR14_1 (() =>            (0x20000000 as uint32_t);)

pub macro_rules! GPIO_OSPEEDER_OSPEEDR15 (() =>              (0xC0000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR15_0 (() =>            (0x40000000 as uint32_t);)
pub macro_rules! GPIO_OSPEEDER_OSPEEDR15_1 (() =>            (0x80000000 as uint32_t);)

/*  Bits definition for GPIO_PUPDR register  */
pub macro_rules! GPIO_PUPDR_PUPDR0 (() =>                    (0x00000003 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR0_0 (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR0_1 (() =>                  (0x00000002 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR1 (() =>                    (0x0000000C as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR1_0 (() =>                  (0x00000004 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR1_1 (() =>                  (0x00000008 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR2 (() =>                    (0x00000030 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR2_0 (() =>                  (0x00000010 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR2_1 (() =>                  (0x00000020 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR3 (() =>                    (0x000000C0 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR3_0 (() =>                  (0x00000040 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR3_1 (() =>                  (0x00000080 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR4 (() =>                    (0x00000300 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR4_0 (() =>                  (0x00000100 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR4_1 (() =>                  (0x00000200 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR5 (() =>                    (0x00000C00 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR5_0 (() =>                  (0x00000400 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR5_1 (() =>                  (0x00000800 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR6 (() =>                    (0x00003000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR6_0 (() =>                  (0x00001000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR6_1 (() =>                  (0x00002000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR7 (() =>                    (0x0000C000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR7_0 (() =>                  (0x00004000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR7_1 (() =>                  (0x00008000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR8 (() =>                    (0x00030000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR8_0 (() =>                  (0x00010000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR8_1 (() =>                  (0x00020000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR9 (() =>                    (0x000C0000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR9_0 (() =>                  (0x00040000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR9_1 (() =>                  (0x00080000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR10 (() =>                   (0x00300000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR10_0 (() =>                 (0x00100000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR10_1 (() =>                 (0x00200000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR11 (() =>                   (0x00C00000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR11_0 (() =>                 (0x00400000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR11_1 (() =>                 (0x00800000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR12 (() =>                   (0x03000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR12_0 (() =>                 (0x01000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR12_1 (() =>                 (0x02000000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR13 (() =>                   (0x0C000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR13_0 (() =>                 (0x04000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR13_1 (() =>                 (0x08000000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR14 (() =>                   (0x30000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR14_0 (() =>                 (0x10000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR14_1 (() =>                 (0x20000000 as uint32_t);)

pub macro_rules! GPIO_PUPDR_PUPDR15 (() =>                   (0xC0000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR15_0 (() =>                 (0x40000000 as uint32_t);)
pub macro_rules! GPIO_PUPDR_PUPDR15_1 (() =>                 (0x80000000 as uint32_t);)

/*  Bits definition for GPIO_IDR register  */
pub macro_rules! GPIO_IDR_IDR_0 (() =>                       (0x00000001 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_1 (() =>                       (0x00000002 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_2 (() =>                       (0x00000004 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_3 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_4 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_5 (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_6 (() =>                       (0x00000040 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_7 (() =>                       (0x00000080 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_8 (() =>                       (0x00000100 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_9 (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_10 (() =>                      (0x00000400 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_11 (() =>                      (0x00000800 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_12 (() =>                      (0x00001000 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_13 (() =>                      (0x00002000 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_14 (() =>                      (0x00004000 as uint32_t);)
pub macro_rules! GPIO_IDR_IDR_15 (() =>                      (0x00008000 as uint32_t);)
/* Old GPIO_IDR register bits definition, maintained for legacy purpose */
pub macro_rules! GPIO_OTYPER_IDR_0 (() =>                    (GPIO_IDR_IDR_0!());)
pub macro_rules! GPIO_OTYPER_IDR_1 (() =>                    (GPIO_IDR_IDR_1!());)
pub macro_rules! GPIO_OTYPER_IDR_2 (() =>                    (GPIO_IDR_IDR_2!());)
pub macro_rules! GPIO_OTYPER_IDR_3 (() =>                    (GPIO_IDR_IDR_3!());)
pub macro_rules! GPIO_OTYPER_IDR_4 (() =>                    (GPIO_IDR_IDR_4!());)
pub macro_rules! GPIO_OTYPER_IDR_5 (() =>                    (GPIO_IDR_IDR_5!());)
pub macro_rules! GPIO_OTYPER_IDR_6 (() =>                    (GPIO_IDR_IDR_6!());)
pub macro_rules! GPIO_OTYPER_IDR_7 (() =>                    (GPIO_IDR_IDR_7!());)
pub macro_rules! GPIO_OTYPER_IDR_8 (() =>                    (GPIO_IDR_IDR_8!());)
pub macro_rules! GPIO_OTYPER_IDR_9 (() =>                    (GPIO_IDR_IDR_9!());)
pub macro_rules! GPIO_OTYPER_IDR_10 (() =>                   (GPIO_IDR_IDR_10!());)
pub macro_rules! GPIO_OTYPER_IDR_11 (() =>                   (GPIO_IDR_IDR_11!());)
pub macro_rules! GPIO_OTYPER_IDR_12 (() =>                   (GPIO_IDR_IDR_12!());)
pub macro_rules! GPIO_OTYPER_IDR_13 (() =>                   (GPIO_IDR_IDR_13!());)
pub macro_rules! GPIO_OTYPER_IDR_14 (() =>                   (GPIO_IDR_IDR_14!());)
pub macro_rules! GPIO_OTYPER_IDR_15 (() =>                   (GPIO_IDR_IDR_15!());)

/*  Bits definition for GPIO_ODR register  */
pub macro_rules! GPIO_ODR_ODR_0 (() =>                       (0x00000001 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_1 (() =>                       (0x00000002 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_2 (() =>                       (0x00000004 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_3 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_4 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_5 (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_6 (() =>                       (0x00000040 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_7 (() =>                       (0x00000080 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_8 (() =>                       (0x00000100 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_9 (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_10 (() =>                      (0x00000400 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_11 (() =>                      (0x00000800 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_12 (() =>                      (0x00001000 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_13 (() =>                      (0x00002000 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_14 (() =>                      (0x00004000 as uint32_t);)
pub macro_rules! GPIO_ODR_ODR_15 (() =>                      (0x00008000 as uint32_t);)
/* Old GPIO_ODR register bits definition, maintained for legacy purpose */
pub macro_rules! GPIO_OTYPER_ODR_0 (() =>                    (GPIO_ODR_ODR_0!());)
pub macro_rules! GPIO_OTYPER_ODR_1 (() =>                    (GPIO_ODR_ODR_1!());)
pub macro_rules! GPIO_OTYPER_ODR_2 (() =>                    (GPIO_ODR_ODR_2!());)
pub macro_rules! GPIO_OTYPER_ODR_3 (() =>                    (GPIO_ODR_ODR_3!());)
pub macro_rules! GPIO_OTYPER_ODR_4 (() =>                    (GPIO_ODR_ODR_4!());)
pub macro_rules! GPIO_OTYPER_ODR_5 (() =>                    (GPIO_ODR_ODR_5!());)
pub macro_rules! GPIO_OTYPER_ODR_6 (() =>                    (GPIO_ODR_ODR_6!());)
pub macro_rules! GPIO_OTYPER_ODR_7 (() =>                    (GPIO_ODR_ODR_7!());)
pub macro_rules! GPIO_OTYPER_ODR_8 (() =>                    (GPIO_ODR_ODR_8!());)
pub macro_rules! GPIO_OTYPER_ODR_9 (() =>                    (GPIO_ODR_ODR_9!());)
pub macro_rules! GPIO_OTYPER_ODR_10 (() =>                   (GPIO_ODR_ODR_10!());)
pub macro_rules! GPIO_OTYPER_ODR_11 (() =>                   (GPIO_ODR_ODR_11!());)
pub macro_rules! GPIO_OTYPER_ODR_12 (() =>                   (GPIO_ODR_ODR_12!());)
pub macro_rules! GPIO_OTYPER_ODR_13 (() =>                   (GPIO_ODR_ODR_13!());)
pub macro_rules! GPIO_OTYPER_ODR_14 (() =>                   (GPIO_ODR_ODR_14!());)
pub macro_rules! GPIO_OTYPER_ODR_15 (() =>                   (GPIO_ODR_ODR_15!());)

/*  Bits definition for GPIO_BSRR register  */
pub macro_rules! GPIO_BSRR_BS_0 (() =>                       (0x00000001 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_1 (() =>                       (0x00000002 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_2 (() =>                       (0x00000004 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_3 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_4 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_5 (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_6 (() =>                       (0x00000040 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_7 (() =>                       (0x00000080 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_8 (() =>                       (0x00000100 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_9 (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_10 (() =>                      (0x00000400 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_11 (() =>                      (0x00000800 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_12 (() =>                      (0x00001000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_13 (() =>                      (0x00002000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_14 (() =>                      (0x00004000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BS_15 (() =>                      (0x00008000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_0 (() =>                       (0x00010000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_1 (() =>                       (0x00020000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_2 (() =>                       (0x00040000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_3 (() =>                       (0x00080000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_4 (() =>                       (0x00100000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_5 (() =>                       (0x00200000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_6 (() =>                       (0x00400000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_7 (() =>                       (0x00800000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_8 (() =>                       (0x01000000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_9 (() =>                       (0x02000000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_10 (() =>                      (0x04000000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_11 (() =>                      (0x08000000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_12 (() =>                      (0x10000000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_13 (() =>                      (0x20000000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_14 (() =>                      (0x40000000 as uint32_t);)
pub macro_rules! GPIO_BSRR_BR_15 (() =>                      (0x80000000 as uint32_t);)


/*                                                                            */
/*                                    HASH                                    */
/*                                                                            */

/*  Bits definition for HASH_CR register  */
pub macro_rules! HASH_CR_INIT (() =>                         (0x00000004 as uint32_t);)
pub macro_rules! HASH_CR_DMAE (() =>                         (0x00000008 as uint32_t);)
pub macro_rules! HASH_CR_DATATYPE (() =>                     (0x00000030 as uint32_t);)
pub macro_rules! HASH_CR_DATATYPE_0 (() =>                   (0x00000010 as uint32_t);)
pub macro_rules! HASH_CR_DATATYPE_1 (() =>                   (0x00000020 as uint32_t);)
pub macro_rules! HASH_CR_MODE (() =>                         (0x00000040 as uint32_t);)
pub macro_rules! HASH_CR_ALGO (() =>                         (0x00000080 as uint32_t);)
pub macro_rules! HASH_CR_NBW (() =>                          (0x00000F00 as uint32_t);)
pub macro_rules! HASH_CR_NBW_0 (() =>                        (0x00000100 as uint32_t);)
pub macro_rules! HASH_CR_NBW_1 (() =>                        (0x00000200 as uint32_t);)
pub macro_rules! HASH_CR_NBW_2 (() =>                        (0x00000400 as uint32_t);)
pub macro_rules! HASH_CR_NBW_3 (() =>                        (0x00000800 as uint32_t);)
pub macro_rules! HASH_CR_DINNE (() =>                        (0x00001000 as uint32_t);)
pub macro_rules! HASH_CR_LKEY (() =>                         (0x00010000 as uint32_t);)

/*  Bits definition for HASH_STR register  */
pub macro_rules! HASH_STR_NBW (() =>                         (0x0000001F as uint32_t);)
pub macro_rules! HASH_STR_NBW_0 (() =>                       (0x00000001 as uint32_t);)
pub macro_rules! HASH_STR_NBW_1 (() =>                       (0x00000002 as uint32_t);)
pub macro_rules! HASH_STR_NBW_2 (() =>                       (0x00000004 as uint32_t);)
pub macro_rules! HASH_STR_NBW_3 (() =>                       (0x00000008 as uint32_t);)
pub macro_rules! HASH_STR_NBW_4 (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! HASH_STR_DCAL (() =>                        (0x00000100 as uint32_t);)

/*  Bits definition for HASH_IMR register  */
pub macro_rules! HASH_IMR_DINIM (() =>                       (0x00000001 as uint32_t);)
pub macro_rules! HASH_IMR_DCIM (() =>                        (0x00000002 as uint32_t);)

/*  Bits definition for HASH_SR register  */
pub macro_rules! HASH_SR_DINIS (() =>                        (0x00000001 as uint32_t);)
pub macro_rules! HASH_SR_DCIS (() =>                         (0x00000002 as uint32_t);)
pub macro_rules! HASH_SR_DMAS (() =>                         (0x00000004 as uint32_t);)
pub macro_rules! HASH_SR_BUSY (() =>                         (0x00000008 as uint32_t);)


/*                                                                            */
/*                      Inter-integrated Circuit Interface                    */
/*                                                                            */

/*  Bit definition for I2C_CR1 register  */
pub macro_rules! I2C_CR1_PE (() =>                          (0x0001 as uint16_t);)            /*<Peripheral Enable */
pub macro_rules! I2C_CR1_SMBUS (() =>                       (0x0002 as uint16_t);)            /*<SMBus Mode */
pub macro_rules! I2C_CR1_SMBTYPE (() =>                     (0x0008 as uint16_t);)            /*<SMBus Type */
pub macro_rules! I2C_CR1_ENARP (() =>                       (0x0010 as uint16_t);)            /*<ARP Enable */
pub macro_rules! I2C_CR1_ENPEC (() =>                       (0x0020 as uint16_t);)            /*<PEC Enable */
pub macro_rules! I2C_CR1_ENGC (() =>                        (0x0040 as uint16_t);)            /*<General Call Enable */
pub macro_rules! I2C_CR1_NOSTRETCH (() =>                   (0x0080 as uint16_t);)            /*<Clock Stretching Disable (Slave mode) */
pub macro_rules! I2C_CR1_START (() =>                       (0x0100 as uint16_t);)            /*<Start Generation */
pub macro_rules! I2C_CR1_STOP (() =>                        (0x0200 as uint16_t);)            /*<Stop Generation */
pub macro_rules! I2C_CR1_ACK (() =>                         (0x0400 as uint16_t);)            /*<Acknowledge Enable */
pub macro_rules! I2C_CR1_POS (() =>                         (0x0800 as uint16_t);)            /*<Acknowledge/PEC Position (for data reception) */
pub macro_rules! I2C_CR1_PEC (() =>                         (0x1000 as uint16_t);)            /*<Packet Error Checking */
pub macro_rules! I2C_CR1_ALERT (() =>                       (0x2000 as uint16_t);)            /*<SMBus Alert */
pub macro_rules! I2C_CR1_SWRST (() =>                       (0x8000 as uint16_t);)            /*<Software Reset */

/*  Bit definition for I2C_CR2 register  */
pub macro_rules! I2C_CR2_FREQ (() =>                        (0x003F as uint16_t);)            /*<FREQ[5:0] bits (Peripheral Clock Frequency) */
pub macro_rules! I2C_CR2_FREQ_0 (() =>                      (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! I2C_CR2_FREQ_1 (() =>                      (0x0002 as uint16_t);)            /*<Bit 1 */
pub macro_rules! I2C_CR2_FREQ_2 (() =>                      (0x0004 as uint16_t);)            /*<Bit 2 */
pub macro_rules! I2C_CR2_FREQ_3 (() =>                      (0x0008 as uint16_t);)            /*<Bit 3 */
pub macro_rules! I2C_CR2_FREQ_4 (() =>                      (0x0010 as uint16_t);)            /*<Bit 4 */
pub macro_rules! I2C_CR2_FREQ_5 (() =>                      (0x0020 as uint16_t);)            /*<Bit 5 */

pub macro_rules! I2C_CR2_ITERREN (() =>                     (0x0100 as uint16_t);)            /*<Error Interrupt Enable */
pub macro_rules! I2C_CR2_ITEVTEN (() =>                     (0x0200 as uint16_t);)            /*<Event Interrupt Enable */
pub macro_rules! I2C_CR2_ITBUFEN (() =>                     (0x0400 as uint16_t);)            /*<Buffer Interrupt Enable */
pub macro_rules! I2C_CR2_DMAEN (() =>                       (0x0800 as uint16_t);)            /*<DMA Requests Enable */
pub macro_rules! I2C_CR2_LAST (() =>                        (0x1000 as uint16_t);)            /*<DMA Last Transfer */

/*  Bit definition for I2C_OAR1 register  */
pub macro_rules! I2C_OAR1_ADD1_7 (() =>                     (0x00FE as uint16_t);)            /*<Interface Address */
pub macro_rules! I2C_OAR1_ADD8_9 (() =>                     (0x0300 as uint16_t);)            /*<Interface Address */

pub macro_rules! I2C_OAR1_ADD0 (() =>                       (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! I2C_OAR1_ADD1 (() =>                       (0x0002 as uint16_t);)            /*<Bit 1 */
pub macro_rules! I2C_OAR1_ADD2 (() =>                       (0x0004 as uint16_t);)            /*<Bit 2 */
pub macro_rules! I2C_OAR1_ADD3 (() =>                       (0x0008 as uint16_t);)            /*<Bit 3 */
pub macro_rules! I2C_OAR1_ADD4 (() =>                       (0x0010 as uint16_t);)            /*<Bit 4 */
pub macro_rules! I2C_OAR1_ADD5 (() =>                       (0x0020 as uint16_t);)            /*<Bit 5 */
pub macro_rules! I2C_OAR1_ADD6 (() =>                       (0x0040 as uint16_t);)            /*<Bit 6 */
pub macro_rules! I2C_OAR1_ADD7 (() =>                       (0x0080 as uint16_t);)            /*<Bit 7 */
pub macro_rules! I2C_OAR1_ADD8 (() =>                       (0x0100 as uint16_t);)            /*<Bit 8 */
pub macro_rules! I2C_OAR1_ADD9 (() =>                       (0x0200 as uint16_t);)            /*<Bit 9 */

pub macro_rules! I2C_OAR1_ADDMODE (() =>                    (0x8000 as uint16_t);)            /*<Addressing Mode (Slave mode) */

/*  Bit definition for I2C_OAR2 register  */
pub macro_rules! I2C_OAR2_ENDUAL (() =>                     (0x01 as uint8_t);)               /*<Dual addressing mode enable */
pub macro_rules! I2C_OAR2_ADD2 (() =>                       (0xFE as uint8_t);)               /*<Interface address */

/*  Bit definition for I2C_DR register  */
pub macro_rules! I2C_DR_DR (() =>                           (0xFF as uint8_t);)               /*<8-bit Data Register */

/*  Bit definition for I2C_SR1 register  */
pub macro_rules! I2C_SR1_SB (() =>                          (0x0001 as uint16_t);)            /*<Start Bit (Master mode) */
pub macro_rules! I2C_SR1_ADDR (() =>                        (0x0002 as uint16_t);)            /*<Address sent (master mode)/matched (slave mode) */
pub macro_rules! I2C_SR1_BTF (() =>                         (0x0004 as uint16_t);)            /*<Byte Transfer Finished */
pub macro_rules! I2C_SR1_ADD10 (() =>                       (0x0008 as uint16_t);)            /*<10-bit header sent (Master mode) */
pub macro_rules! I2C_SR1_STOPF (() =>                       (0x0010 as uint16_t);)            /*<Stop detection (Slave mode) */
pub macro_rules! I2C_SR1_RXNE (() =>                        (0x0040 as uint16_t);)            /*<Data Register not Empty (receivers) */
pub macro_rules! I2C_SR1_TXE (() =>                         (0x0080 as uint16_t);)            /*<Data Register Empty (transmitters) */
pub macro_rules! I2C_SR1_BERR (() =>                        (0x0100 as uint16_t);)            /*<Bus Error */
pub macro_rules! I2C_SR1_ARLO (() =>                        (0x0200 as uint16_t);)            /*<Arbitration Lost (master mode) */
pub macro_rules! I2C_SR1_AF (() =>                          (0x0400 as uint16_t);)            /*<Acknowledge Failure */
pub macro_rules! I2C_SR1_OVR (() =>                         (0x0800 as uint16_t);)            /*<Overrun/Underrun */
pub macro_rules! I2C_SR1_PECERR (() =>                      (0x1000 as uint16_t);)            /*<PEC Error in reception */
pub macro_rules! I2C_SR1_TIMEOUT (() =>                     (0x4000 as uint16_t);)            /*<Timeout or Tlow Error */
pub macro_rules! I2C_SR1_SMBALERT (() =>                    (0x8000 as uint16_t);)            /*<SMBus Alert */

/*  Bit definition for I2C_SR2 register  */
pub macro_rules! I2C_SR2_MSL (() =>                         (0x0001 as uint16_t);)            /*<Master/Slave */
pub macro_rules! I2C_SR2_BUSY (() =>                        (0x0002 as uint16_t);)            /*<Bus Busy */
pub macro_rules! I2C_SR2_TRA (() =>                         (0x0004 as uint16_t);)            /*<Transmitter/Receiver */
pub macro_rules! I2C_SR2_GENCALL (() =>                     (0x0010 as uint16_t);)            /*<General Call Address (Slave mode) */
pub macro_rules! I2C_SR2_SMBDEFAULT (() =>                  (0x0020 as uint16_t);)            /*<SMBus Device Default Address (Slave mode) */
pub macro_rules! I2C_SR2_SMBHOST (() =>                     (0x0040 as uint16_t);)            /*<SMBus Host Header (Slave mode) */
pub macro_rules! I2C_SR2_DUALF (() =>                       (0x0080 as uint16_t);)            /*<Dual Flag (Slave mode) */
pub macro_rules! I2C_SR2_PEC (() =>                         (0xFF00 as uint16_t);)            /*<Packet Error Checking Register */

/*  Bit definition for I2C_CCR register  */
pub macro_rules! I2C_CCR_CCR (() =>                         (0x0FFF as uint16_t);)            /*<Clock Control Register in Fast/Standard mode (Master mode) */
pub macro_rules! I2C_CCR_DUTY (() =>                        (0x4000 as uint16_t);)            /*<Fast Mode Duty Cycle */
pub macro_rules! I2C_CCR_FS (() =>                          (0x8000 as uint16_t);)            /*<I2C Master Mode Selection */

/*  Bit definition for I2C_TRISE register  */
pub macro_rules! I2C_TRISE_TRISE (() =>                     (0x3F as uint8_t);)               /*<Maximum Rise Time in Fast/Standard mode (Master mode) */


/*                                                                            */
/*                           Independent WATCHDOG                             */
/*                                                                            */

/*  Bit definition for IWDG_KR register  */
pub macro_rules! IWDG_KR_KEY (() =>                         (0xFFFF as uint16_t);)            /*<Key value (write only, read 0000h) */

/*  Bit definition for IWDG_PR register  */
pub macro_rules! IWDG_PR_PR (() =>                          (0x07 as uint8_t);)               /*<PR[2:0] (Prescaler divider) */
pub macro_rules! IWDG_PR_PR_0 (() =>                        (0x01 as uint8_t);)               /*<Bit 0 */
pub macro_rules! IWDG_PR_PR_1 (() =>                        (0x02 as uint8_t);)               /*<Bit 1 */
pub macro_rules! IWDG_PR_PR_2 (() =>                        (0x04 as uint8_t);)               /*<Bit 2 */

/*  Bit definition for IWDG_RLR register  */
pub macro_rules! IWDG_RLR_RL (() =>                         (0x0FFF as uint16_t);)            /*<Watchdog counter reload value */

/*  Bit definition for IWDG_SR register  */
pub macro_rules! IWDG_SR_PVU (() =>                         (0x01 as uint8_t);)               /*<Watchdog prescaler value update */
pub macro_rules! IWDG_SR_RVU (() =>                         (0x02 as uint8_t);)               /*<Watchdog counter reload value update */


/*                                                                            */
/*                             Power Control                                  */
/*                                                                            */

/*  Bit definition for PWR_CR register  */
pub macro_rules! PWR_CR_LPDS (() =>                         (0x0001 as uint16_t);)     /*< Low-Power Deepsleep */
pub macro_rules! PWR_CR_PDDS (() =>                         (0x0002 as uint16_t);)     /*< Power Down Deepsleep */
pub macro_rules! PWR_CR_CWUF (() =>                         (0x0004 as uint16_t);)     /*< Clear Wakeup Flag */
pub macro_rules! PWR_CR_CSBF (() =>                         (0x0008 as uint16_t);)     /*< Clear Standby Flag */
pub macro_rules! PWR_CR_PVDE (() =>                         (0x0010 as uint16_t);)     /*< Power Voltage Detector Enable */

pub macro_rules! PWR_CR_PLS (() =>                          (0x00E0 as uint16_t);)     /*< PLS[2:0] bits (PVD Level Selection) */
pub macro_rules! PWR_CR_PLS_0 (() =>                        (0x0020 as uint16_t);)     /*< Bit 0 */
pub macro_rules! PWR_CR_PLS_1 (() =>                        (0x0040 as uint16_t);)     /*< Bit 1 */
pub macro_rules! PWR_CR_PLS_2 (() =>                        (0x0080 as uint16_t);)     /*< Bit 2 */


/*< PVD level configuration */
pub macro_rules! PWR_CR_PLS_LEV0 (() =>                     (0x0000 as uint16_t);)     /*< PVD level 0 */
pub macro_rules! PWR_CR_PLS_LEV1 (() =>                     (0x0020 as uint16_t);)     /*< PVD level 1 */
pub macro_rules! PWR_CR_PLS_LEV2 (() =>                     (0x0040 as uint16_t);)     /*< PVD level 2 */
pub macro_rules! PWR_CR_PLS_LEV3 (() =>                     (0x0060 as uint16_t);)     /*< PVD level 3 */
pub macro_rules! PWR_CR_PLS_LEV4 (() =>                     (0x0080 as uint16_t);)     /*< PVD level 4 */
pub macro_rules! PWR_CR_PLS_LEV5 (() =>                     (0x00A0 as uint16_t);)     /*< PVD level 5 */
pub macro_rules! PWR_CR_PLS_LEV6 (() =>                     (0x00C0 as uint16_t);)     /*< PVD level 6 */
pub macro_rules! PWR_CR_PLS_LEV7 (() =>                     (0x00E0 as uint16_t);)     /*< PVD level 7 */

pub macro_rules! PWR_CR_DBP (() =>                          (0x0100 as uint16_t);)     /*< Disable Backup Domain write protection */
pub macro_rules! PWR_CR_FPDS (() =>                         (0x0200 as uint16_t);)     /*< Flash power down in Stop mode */
pub macro_rules! PWR_CR_VOS (() =>                          (0x4000 as uint16_t);)     /*< Regulator voltage scaling output selection */
/* Legacy define */
pub macro_rules! PWR_CR_PMODE (() =>                        (PWR_CR_VOS!());)

/*  Bit definition for PWR_CSR register  */
pub macro_rules! PWR_CSR_WUF (() =>                         (0x0001 as uint16_t);)     /*< Wakeup Flag */
pub macro_rules! PWR_CSR_SBF (() =>                         (0x0002 as uint16_t);)     /*< Standby Flag */
pub macro_rules! PWR_CSR_PVDO (() =>                        (0x0004 as uint16_t);)     /*< PVD Output */
pub macro_rules! PWR_CSR_BRR (() =>                         (0x0008 as uint16_t);)     /*< Backup regulator ready */
pub macro_rules! PWR_CSR_EWUP (() =>                        (0x0100 as uint16_t);)     /*< Enable WKUP pin */
pub macro_rules! PWR_CSR_BRE (() =>                         (0x0200 as uint16_t);)     /*< Backup regulator enable */
pub macro_rules! PWR_CSR_VOSRDY (() =>                      (0x4000 as uint16_t);)     /*< Regulator voltage scaling output selection ready */
/* Legacy define */
pub macro_rules! PWR_CSR_REGRDY (() =>                      (PWR_CSR_VOSRDY!());)


/*                                                                            */
/*                         Reset and Clock Control                            */
/*                                                                            */

/*  Bit definition for RCC_CR register  */
pub macro_rules! RCC_CR_HSION (() =>                        (0x00000001 as uint32_t);)
pub macro_rules! RCC_CR_HSIRDY (() =>                       (0x00000002 as uint32_t);)

pub macro_rules! RCC_CR_HSITRIM (() =>                      (0x000000F8 as uint32_t);)
pub macro_rules! RCC_CR_HSITRIM_0 (() =>                    (0x00000008 as uint32_t);)/*<Bit 0 */
pub macro_rules! RCC_CR_HSITRIM_1 (() =>                    (0x00000010 as uint32_t);)/*<Bit 1 */
pub macro_rules! RCC_CR_HSITRIM_2 (() =>                    (0x00000020 as uint32_t);)/*<Bit 2 */
pub macro_rules! RCC_CR_HSITRIM_3 (() =>                    (0x00000040 as uint32_t);)/*<Bit 3 */
pub macro_rules! RCC_CR_HSITRIM_4 (() =>                    (0x00000080 as uint32_t);)/*<Bit 4 */

pub macro_rules! RCC_CR_HSICAL (() =>                       (0x0000FF00 as uint32_t);)
pub macro_rules! RCC_CR_HSICAL_0 (() =>                     (0x00000100 as uint32_t);)/*<Bit 0 */
pub macro_rules! RCC_CR_HSICAL_1 (() =>                     (0x00000200 as uint32_t);)/*<Bit 1 */
pub macro_rules! RCC_CR_HSICAL_2 (() =>                     (0x00000400 as uint32_t);)/*<Bit 2 */
pub macro_rules! RCC_CR_HSICAL_3 (() =>                     (0x00000800 as uint32_t);)/*<Bit 3 */
pub macro_rules! RCC_CR_HSICAL_4 (() =>                     (0x00001000 as uint32_t);)/*<Bit 4 */
pub macro_rules! RCC_CR_HSICAL_5 (() =>                     (0x00002000 as uint32_t);)/*<Bit 5 */
pub macro_rules! RCC_CR_HSICAL_6 (() =>                     (0x00004000 as uint32_t);)/*<Bit 6 */
pub macro_rules! RCC_CR_HSICAL_7 (() =>                     (0x00008000 as uint32_t);)/*<Bit 7 */

pub macro_rules! RCC_CR_HSEON (() =>                        (0x00010000 as uint32_t);)
pub macro_rules! RCC_CR_HSERDY (() =>                       (0x00020000 as uint32_t);)
pub macro_rules! RCC_CR_HSEBYP (() =>                       (0x00040000 as uint32_t);)
pub macro_rules! RCC_CR_CSSON (() =>                        (0x00080000 as uint32_t);)
pub macro_rules! RCC_CR_PLLON (() =>                        (0x01000000 as uint32_t);)
pub macro_rules! RCC_CR_PLLRDY (() =>                       (0x02000000 as uint32_t);)
pub macro_rules! RCC_CR_PLLI2SON (() =>                     (0x04000000 as uint32_t);)
pub macro_rules! RCC_CR_PLLI2SRDY (() =>                    (0x08000000 as uint32_t);)

/*  Bit definition for RCC_PLLCFGR register  */
pub macro_rules! RCC_PLLCFGR_PLLM (() =>                    (0x0000003F as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLM_0 (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLM_1 (() =>                  (0x00000002 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLM_2 (() =>                  (0x00000004 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLM_3 (() =>                  (0x00000008 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLM_4 (() =>                  (0x00000010 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLM_5 (() =>                  (0x00000020 as uint32_t);)

pub macro_rules! RCC_PLLCFGR_PLLN (() =>                     (0x00007FC0 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_0 (() =>                   (0x00000040 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_1 (() =>                   (0x00000080 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_2 (() =>                   (0x00000100 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_3 (() =>                   (0x00000200 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_4 (() =>                   (0x00000400 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_5 (() =>                   (0x00000800 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_6 (() =>                   (0x00001000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_7 (() =>                   (0x00002000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLN_8 (() =>                   (0x00004000 as uint32_t);)

pub macro_rules! RCC_PLLCFGR_PLLP (() =>                    (0x00030000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLP_0 (() =>                  (0x00010000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLP_1 (() =>                  (0x00020000 as uint32_t);)

pub macro_rules! RCC_PLLCFGR_PLLSRC (() =>                  (0x00400000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLSRC_HSE (() =>              (0x00400000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLSRC_HSI (() =>              (0x00000000 as uint32_t);)

pub macro_rules! RCC_PLLCFGR_PLLQ (() =>                    (0x0F000000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLQ_0 (() =>                  (0x01000000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLQ_1 (() =>                  (0x02000000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLQ_2 (() =>                  (0x04000000 as uint32_t);)
pub macro_rules! RCC_PLLCFGR_PLLQ_3 (() =>                  (0x08000000 as uint32_t);)

/*  Bit definition for RCC_CFGR register  */
/*< SW configuration */
pub macro_rules! RCC_CFGR_SW (() =>                         (0x00000003 as uint32_t);)        /*< SW[1:0] bits (System clock Switch) */
pub macro_rules! RCC_CFGR_SW_0 (() =>                       (0x00000001 as uint32_t);)        /*< Bit 0 */
pub macro_rules! RCC_CFGR_SW_1 (() =>                       (0x00000002 as uint32_t);)        /*< Bit 1 */

pub macro_rules! RCC_CFGR_SW_HSI (() =>                     (0x00000000 as uint32_t);)        /*< HSI selected as system clock */
pub macro_rules! RCC_CFGR_SW_HSE (() =>                     (0x00000001 as uint32_t);)        /*< HSE selected as system clock */
pub macro_rules! RCC_CFGR_SW_PLL (() =>                     (0x00000002 as uint32_t);)        /*< PLL selected as system clock */

/*< SWS configuration */
pub macro_rules! RCC_CFGR_SWS (() =>                        (0x0000000C as uint32_t);)        /*< SWS[1:0] bits (System Clock Switch Status) */
pub macro_rules! RCC_CFGR_SWS_0 (() =>                      (0x00000004 as uint32_t);)        /*< Bit 0 */
pub macro_rules! RCC_CFGR_SWS_1 (() =>                      (0x00000008 as uint32_t);)        /*< Bit 1 */

pub macro_rules! RCC_CFGR_SWS_HSI (() =>                    (0x00000000 as uint32_t);)        /*< HSI oscillator used as system clock */
pub macro_rules! RCC_CFGR_SWS_HSE (() =>                    (0x00000004 as uint32_t);)        /*< HSE oscillator used as system clock */
pub macro_rules! RCC_CFGR_SWS_PLL (() =>                    (0x00000008 as uint32_t);)        /*< PLL used as system clock */

/*< HPRE configuration */
pub macro_rules! RCC_CFGR_HPRE (() =>                       (0x000000F0 as uint32_t);)        /*< HPRE[3:0] bits (AHB prescaler) */
pub macro_rules! RCC_CFGR_HPRE_0 (() =>                     (0x00000010 as uint32_t);)        /*< Bit 0 */
pub macro_rules! RCC_CFGR_HPRE_1 (() =>                     (0x00000020 as uint32_t);)        /*< Bit 1 */
pub macro_rules! RCC_CFGR_HPRE_2 (() =>                     (0x00000040 as uint32_t);)        /*< Bit 2 */
pub macro_rules! RCC_CFGR_HPRE_3 (() =>                     (0x00000080 as uint32_t);)        /*< Bit 3 */

pub macro_rules! RCC_CFGR_HPRE_DIV1 (() =>                  (0x00000000 as uint32_t);)        /*< SYSCLK not divided */
pub macro_rules! RCC_CFGR_HPRE_DIV2 (() =>                  (0x00000080 as uint32_t);)        /*< SYSCLK divided by 2 */
pub macro_rules! RCC_CFGR_HPRE_DIV4 (() =>                  (0x00000090 as uint32_t);)        /*< SYSCLK divided by 4 */
pub macro_rules! RCC_CFGR_HPRE_DIV8 (() =>                  (0x000000A0 as uint32_t);)        /*< SYSCLK divided by 8 */
pub macro_rules! RCC_CFGR_HPRE_DIV16 (() =>                 (0x000000B0 as uint32_t);)        /*< SYSCLK divided by 16 */
pub macro_rules! RCC_CFGR_HPRE_DIV64 (() =>                 (0x000000C0 as uint32_t);)        /*< SYSCLK divided by 64 */
pub macro_rules! RCC_CFGR_HPRE_DIV128 (() =>                (0x000000D0 as uint32_t);)        /*< SYSCLK divided by 128 */
pub macro_rules! RCC_CFGR_HPRE_DIV256 (() =>                (0x000000E0 as uint32_t);)        /*< SYSCLK divided by 256 */
pub macro_rules! RCC_CFGR_HPRE_DIV512 (() =>                (0x000000F0 as uint32_t);)        /*< SYSCLK divided by 512 */

/*< PPRE1 configuration */
pub macro_rules! RCC_CFGR_PPRE1 (() =>                      (0x00001C00 as uint32_t);)        /*< PRE1[2:0] bits (APB1 prescaler) */
pub macro_rules! RCC_CFGR_PPRE1_0 (() =>                    (0x00000400 as uint32_t);)        /*< Bit 0 */
pub macro_rules! RCC_CFGR_PPRE1_1 (() =>                    (0x00000800 as uint32_t);)        /*< Bit 1 */
pub macro_rules! RCC_CFGR_PPRE1_2 (() =>                    (0x00001000 as uint32_t);)        /*< Bit 2 */

pub macro_rules! RCC_CFGR_PPRE1_DIV1 (() =>                 (0x00000000 as uint32_t);)        /*< HCLK not divided */
pub macro_rules! RCC_CFGR_PPRE1_DIV2 (() =>                 (0x00001000 as uint32_t);)        /*< HCLK divided by 2 */
pub macro_rules! RCC_CFGR_PPRE1_DIV4 (() =>                 (0x00001400 as uint32_t);)        /*< HCLK divided by 4 */
pub macro_rules! RCC_CFGR_PPRE1_DIV8 (() =>                 (0x00001800 as uint32_t);)        /*< HCLK divided by 8 */
pub macro_rules! RCC_CFGR_PPRE1_DIV16 (() =>                (0x00001C00 as uint32_t);)        /*< HCLK divided by 16 */

/*< PPRE2 configuration */
pub macro_rules! RCC_CFGR_PPRE2 (() =>                      (0x0000E000 as uint32_t);)        /*< PRE2[2:0] bits (APB2 prescaler) */
pub macro_rules! RCC_CFGR_PPRE2_0 (() =>                    (0x00002000 as uint32_t);)        /*< Bit 0 */
pub macro_rules! RCC_CFGR_PPRE2_1 (() =>                    (0x00004000 as uint32_t);)        /*< Bit 1 */
pub macro_rules! RCC_CFGR_PPRE2_2 (() =>                    (0x00008000 as uint32_t);)        /*< Bit 2 */

pub macro_rules! RCC_CFGR_PPRE2_DIV1 (() =>                 (0x00000000 as uint32_t);)        /*< HCLK not divided */
pub macro_rules! RCC_CFGR_PPRE2_DIV2 (() =>                 (0x00008000 as uint32_t);)        /*< HCLK divided by 2 */
pub macro_rules! RCC_CFGR_PPRE2_DIV4 (() =>                 (0x0000A000 as uint32_t);)        /*< HCLK divided by 4 */
pub macro_rules! RCC_CFGR_PPRE2_DIV8 (() =>                 (0x0000C000 as uint32_t);)        /*< HCLK divided by 8 */
pub macro_rules! RCC_CFGR_PPRE2_DIV16 (() =>                (0x0000E000 as uint32_t);)        /*< HCLK divided by 16 */

/*< RTCPRE configuration */
pub macro_rules! RCC_CFGR_RTCPRE (() =>                     (0x001F0000 as uint32_t);)
pub macro_rules! RCC_CFGR_RTCPRE_0 (() =>                   (0x00010000 as uint32_t);)
pub macro_rules! RCC_CFGR_RTCPRE_1 (() =>                   (0x00020000 as uint32_t);)
pub macro_rules! RCC_CFGR_RTCPRE_2 (() =>                   (0x00040000 as uint32_t);)
pub macro_rules! RCC_CFGR_RTCPRE_3 (() =>                   (0x00080000 as uint32_t);)
pub macro_rules! RCC_CFGR_RTCPRE_4 (() =>                   (0x00100000 as uint32_t);)

/*< MCO1 configuration */
pub macro_rules! RCC_CFGR_MCO1 (() =>                       (0x00600000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO1_0 (() =>                     (0x00200000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO1_1 (() =>                     (0x00400000 as uint32_t);)

pub macro_rules! RCC_CFGR_I2SSRC (() =>                     (0x00800000 as uint32_t);)

pub macro_rules! RCC_CFGR_MCO1PRE (() =>                    (0x07000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO1PRE_0 (() =>                  (0x01000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO1PRE_1 (() =>                  (0x02000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO1PRE_2 (() =>                  (0x04000000 as uint32_t);)

pub macro_rules! RCC_CFGR_MCO2PRE (() =>                    (0x38000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO2PRE_0 (() =>                  (0x08000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO2PRE_1 (() =>                  (0x10000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO2PRE_2 (() =>                  (0x20000000 as uint32_t);)

pub macro_rules! RCC_CFGR_MCO2 (() =>                       (0xC0000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO2_0 (() =>                     (0x40000000 as uint32_t);)
pub macro_rules! RCC_CFGR_MCO2_1 (() =>                     (0x80000000 as uint32_t);)

/*  Bit definition for RCC_CIR register  */
pub macro_rules! RCC_CIR_LSIRDYF (() =>                     (0x00000001 as uint32_t);)
pub macro_rules! RCC_CIR_LSERDYF (() =>                     (0x00000002 as uint32_t);)
pub macro_rules! RCC_CIR_HSIRDYF (() =>                     (0x00000004 as uint32_t);)
pub macro_rules! RCC_CIR_HSERDYF (() =>                     (0x00000008 as uint32_t);)
pub macro_rules! RCC_CIR_PLLRDYF (() =>                     (0x00000010 as uint32_t);)
pub macro_rules! RCC_CIR_PLLI2SRDYF (() =>                  (0x00000020 as uint32_t);)
pub macro_rules! RCC_CIR_CSSF (() =>                        (0x00000080 as uint32_t);)
pub macro_rules! RCC_CIR_LSIRDYIE (() =>                    (0x00000100 as uint32_t);)
pub macro_rules! RCC_CIR_LSERDYIE (() =>                    (0x00000200 as uint32_t);)
pub macro_rules! RCC_CIR_HSIRDYIE (() =>                    (0x00000400 as uint32_t);)
pub macro_rules! RCC_CIR_HSERDYIE (() =>                    (0x00000800 as uint32_t);)
pub macro_rules! RCC_CIR_PLLRDYIE (() =>                    (0x00001000 as uint32_t);)
pub macro_rules! RCC_CIR_PLLI2SRDYIE (() =>                 (0x00002000 as uint32_t);)
pub macro_rules! RCC_CIR_LSIRDYC (() =>                     (0x00010000 as uint32_t);)
pub macro_rules! RCC_CIR_LSERDYC (() =>                     (0x00020000 as uint32_t);)
pub macro_rules! RCC_CIR_HSIRDYC (() =>                     (0x00040000 as uint32_t);)
pub macro_rules! RCC_CIR_HSERDYC (() =>                     (0x00080000 as uint32_t);)
pub macro_rules! RCC_CIR_PLLRDYC (() =>                     (0x00100000 as uint32_t);)
pub macro_rules! RCC_CIR_PLLI2SRDYC (() =>                  (0x00200000 as uint32_t);)
pub macro_rules! RCC_CIR_CSSC (() =>                        (0x00800000 as uint32_t);)

/*  Bit definition for RCC_AHB1RSTR register  */
pub macro_rules! RCC_AHB1RSTR_GPIOARST (() =>               (0x00000001 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIOBRST (() =>               (0x00000002 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIOCRST (() =>               (0x00000004 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIODRST (() =>               (0x00000008 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIOERST (() =>               (0x00000010 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIOFRST (() =>               (0x00000020 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIOGRST (() =>               (0x00000040 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIOHRST (() =>               (0x00000080 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_GPIOIRST (() =>               (0x00000100 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_CRCRST (() =>                 (0x00001000 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_DMA1RST (() =>                (0x00200000 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_DMA2RST (() =>                (0x00400000 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_ETHMACRST (() =>              (0x02000000 as uint32_t);)
pub macro_rules! RCC_AHB1RSTR_OTGHRST (() =>                (0x10000000 as uint32_t);)

/*  Bit definition for RCC_AHB2RSTR register  */
pub macro_rules! RCC_AHB2RSTR_DCMIRST (() =>                (0x00000001 as uint32_t);)
pub macro_rules! RCC_AHB2RSTR_CRYPRST (() =>                (0x00000010 as uint32_t);)
pub macro_rules! RCC_AHB2RSTR_HSAHRST (() =>                (0x00000020 as uint32_t);)
pub macro_rules! RCC_AHB2RSTR_RNGRST (() =>                 (0x00000040 as uint32_t);)
pub macro_rules! RCC_AHB2RSTR_OTGFSRST (() =>               (0x00000080 as uint32_t);)

/*  Bit definition for RCC_AHB3RSTR register  */
pub macro_rules! RCC_AHB3RSTR_FSMCRST (() =>                (0x00000001 as uint32_t);)

/*  Bit definition for RCC_APB1RSTR register  */
pub macro_rules! RCC_APB1RSTR_TIM2RST (() =>                (0x00000001 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM3RST (() =>                (0x00000002 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM4RST (() =>                (0x00000004 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM5RST (() =>                (0x00000008 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM6RST (() =>                (0x00000010 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM7RST (() =>                (0x00000020 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM12RST (() =>               (0x00000040 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM13RST (() =>               (0x00000080 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_TIM14RST (() =>               (0x00000100 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_WWDGEN (() =>                 (0x00000800 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_SPI2RST (() =>                (0x00008000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_SPI3RST (() =>                (0x00010000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_USART2RST (() =>              (0x00020000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_USART3RST (() =>              (0x00040000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_UART4RST (() =>               (0x00080000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_UART5RST (() =>               (0x00100000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_I2C1RST (() =>                (0x00200000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_I2C2RST (() =>                (0x00400000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_I2C3RST (() =>                (0x00800000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_CAN1RST (() =>                (0x02000000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_CAN2RST (() =>                (0x04000000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_PWRRST (() =>                 (0x10000000 as uint32_t);)
pub macro_rules! RCC_APB1RSTR_DACRST (() =>                 (0x20000000 as uint32_t);)

/*  Bit definition for RCC_APB2RSTR register  */
pub macro_rules! RCC_APB2RSTR_TIM1RST (() =>                (0x00000001 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_TIM8RST (() =>                (0x00000002 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_USART1RST (() =>              (0x00000010 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_USART6RST (() =>              (0x00000020 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_ADCRST (() =>                 (0x00000100 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_SDIORST (() =>                (0x00000800 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_SPI1RST (() =>                (0x00001000 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_SYSCFGRST (() =>              (0x00004000 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_TIM9RST (() =>                (0x00010000 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_TIM10RST (() =>               (0x00020000 as uint32_t);)
pub macro_rules! RCC_APB2RSTR_TIM11RST (() =>               (0x00040000 as uint32_t);)
/* Old SPI1RST bit definition, maintained for legacy purpose */
pub macro_rules! RCC_APB2RSTR_SPI1 (() =>                   (RCC_APB2RSTR_SPI1RST!());)

/*  Bit definition for RCC_AHB1ENR register  */
pub macro_rules! RCC_AHB1ENR_GPIOAEN (() =>                 (0x00000001 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIOBEN (() =>                 (0x00000002 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIOCEN (() =>                 (0x00000004 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIODEN (() =>                 (0x00000008 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIOEEN (() =>                 (0x00000010 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIOFEN (() =>                 (0x00000020 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIOGEN (() =>                 (0x00000040 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIOHEN (() =>                 (0x00000080 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_GPIOIEN (() =>                 (0x00000100 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_CRCEN (() =>                   (0x00001000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_BKPSRAMEN (() =>               (0x00040000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_CCMDATARAMEN (() =>            (0x00100000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_DMA1EN (() =>                  (0x00200000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_DMA2EN (() =>                  (0x00400000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_ETHMACEN (() =>                (0x02000000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_ETHMACTXEN (() =>              (0x04000000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_ETHMACRXEN (() =>              (0x08000000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_ETHMACPTPEN (() =>             (0x10000000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_OTGHSEN (() =>                 (0x20000000 as uint32_t);)
pub macro_rules! RCC_AHB1ENR_OTGHSULPIEN (() =>             (0x40000000 as uint32_t);)

/*  Bit definition for RCC_AHB2ENR register  */
pub macro_rules! RCC_AHB2ENR_DCMIEN (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! RCC_AHB2ENR_CRYPEN (() =>                  (0x00000010 as uint32_t);)
pub macro_rules! RCC_AHB2ENR_HASHEN (() =>                  (0x00000020 as uint32_t);)
pub macro_rules! RCC_AHB2ENR_RNGEN (() =>                   (0x00000040 as uint32_t);)
pub macro_rules! RCC_AHB2ENR_OTGFSEN (() =>                 (0x00000080 as uint32_t);)

/*  Bit definition for RCC_AHB3ENR register  */
pub macro_rules! RCC_AHB3ENR_FSMCEN (() =>                  (0x00000001 as uint32_t);)

/*  Bit definition for RCC_APB1ENR register  */
pub macro_rules! RCC_APB1ENR_TIM2EN (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM3EN (() =>                  (0x00000002 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM4EN (() =>                  (0x00000004 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM5EN (() =>                  (0x00000008 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM6EN (() =>                  (0x00000010 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM7EN (() =>                  (0x00000020 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM12EN (() =>                 (0x00000040 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM13EN (() =>                 (0x00000080 as uint32_t);)
pub macro_rules! RCC_APB1ENR_TIM14EN (() =>                 (0x00000100 as uint32_t);)
pub macro_rules! RCC_APB1ENR_WWDGEN (() =>                  (0x00000800 as uint32_t);)
pub macro_rules! RCC_APB1ENR_SPI2EN (() =>                  (0x00004000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_SPI3EN (() =>                  (0x00008000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_USART2EN (() =>                (0x00020000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_USART3EN (() =>                (0x00040000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_UART4EN (() =>                 (0x00080000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_UART5EN (() =>                 (0x00100000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_I2C1EN (() =>                  (0x00200000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_I2C2EN (() =>                  (0x00400000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_I2C3EN (() =>                  (0x00800000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_CAN1EN (() =>                  (0x02000000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_CAN2EN (() =>                  (0x04000000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_PWREN (() =>                   (0x10000000 as uint32_t);)
pub macro_rules! RCC_APB1ENR_DACEN (() =>                   (0x20000000 as uint32_t);)

/*  Bit definition for RCC_APB2ENR register  */
pub macro_rules! RCC_APB2ENR_TIM1EN (() =>                  (0x00000001 as uint32_t);)
pub macro_rules! RCC_APB2ENR_TIM8EN (() =>                  (0x00000002 as uint32_t);)
pub macro_rules! RCC_APB2ENR_USART1EN (() =>                (0x00000010 as uint32_t);)
pub macro_rules! RCC_APB2ENR_USART6EN (() =>                (0x00000020 as uint32_t);)
pub macro_rules! RCC_APB2ENR_ADC1EN (() =>                  (0x00000100 as uint32_t);)
pub macro_rules! RCC_APB2ENR_ADC2EN (() =>                  (0x00000200 as uint32_t);)
pub macro_rules! RCC_APB2ENR_ADC3EN (() =>                  (0x00000400 as uint32_t);)
pub macro_rules! RCC_APB2ENR_SDIOEN (() =>                  (0x00000800 as uint32_t);)
pub macro_rules! RCC_APB2ENR_SPI1EN (() =>                  (0x00001000 as uint32_t);)
pub macro_rules! RCC_APB2ENR_SYSCFGEN (() =>                (0x00004000 as uint32_t);)
pub macro_rules! RCC_APB2ENR_TIM11EN (() =>                 (0x00040000 as uint32_t);)
pub macro_rules! RCC_APB2ENR_TIM10EN (() =>                 (0x00020000 as uint32_t);)
pub macro_rules! RCC_APB2ENR_TIM9EN (() =>                  (0x00010000 as uint32_t);)

/*  Bit definition for RCC_AHB1LPENR register  */
pub macro_rules! RCC_AHB1LPENR_GPIOALPEN (() =>             (0x00000001 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIOBLPEN (() =>             (0x00000002 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIOCLPEN (() =>             (0x00000004 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIODLPEN (() =>             (0x00000008 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIOELPEN (() =>             (0x00000010 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIOFLPEN (() =>             (0x00000020 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIOGLPEN (() =>             (0x00000040 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIOHLPEN (() =>             (0x00000080 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_GPIOILPEN (() =>             (0x00000100 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_CRCLPEN (() =>               (0x00001000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_FLITFLPEN (() =>             (0x00008000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_SRAM1LPEN (() =>             (0x00010000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_SRAM2LPEN (() =>             (0x00020000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_BKPSRAMLPEN (() =>           (0x00040000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_DMA1LPEN (() =>              (0x00200000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_DMA2LPEN (() =>              (0x00400000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_ETHMACLPEN (() =>            (0x02000000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_ETHMACTXLPEN (() =>          (0x04000000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_ETHMACRXLPEN (() =>          (0x08000000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_ETHMACPTPLPEN (() =>         (0x10000000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_OTGHSLPEN (() =>             (0x20000000 as uint32_t);)
pub macro_rules! RCC_AHB1LPENR_OTGHSULPILPEN (() =>         (0x40000000 as uint32_t);)

/*  Bit definition for RCC_AHB2LPENR register  */
pub macro_rules! RCC_AHB2LPENR_DCMILPEN (() =>              (0x00000001 as uint32_t);)
pub macro_rules! RCC_AHB2LPENR_CRYPLPEN (() =>              (0x00000010 as uint32_t);)
pub macro_rules! RCC_AHB2LPENR_HASHLPEN (() =>              (0x00000020 as uint32_t);)
pub macro_rules! RCC_AHB2LPENR_RNGLPEN (() =>               (0x00000040 as uint32_t);)
pub macro_rules! RCC_AHB2LPENR_OTGFSLPEN (() =>             (0x00000080 as uint32_t);)

/*  Bit definition for RCC_AHB3LPENR register  */
pub macro_rules! RCC_AHB3LPENR_FSMCLPEN (() =>              (0x00000001 as uint32_t);)

/*  Bit definition for RCC_APB1LPENR register  */
pub macro_rules! RCC_APB1LPENR_TIM2LPEN (() =>              (0x00000001 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM3LPEN (() =>              (0x00000002 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM4LPEN (() =>              (0x00000004 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM5LPEN (() =>              (0x00000008 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM6LPEN (() =>              (0x00000010 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM7LPEN (() =>              (0x00000020 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM12LPEN (() =>             (0x00000040 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM13LPEN (() =>             (0x00000080 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_TIM14LPEN (() =>             (0x00000100 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_WWDGLPEN (() =>              (0x00000800 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_SPI2LPEN (() =>              (0x00004000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_SPI3LPEN (() =>              (0x00008000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_USART2LPEN (() =>            (0x00020000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_USART3LPEN (() =>            (0x00040000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_UART4LPEN (() =>             (0x00080000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_UART5LPEN (() =>             (0x00100000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_I2C1LPEN (() =>              (0x00200000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_I2C2LPEN (() =>              (0x00400000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_I2C3LPEN (() =>              (0x00800000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_CAN1LPEN (() =>              (0x02000000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_CAN2LPEN (() =>              (0x04000000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_PWRLPEN (() =>               (0x10000000 as uint32_t);)
pub macro_rules! RCC_APB1LPENR_DACLPEN (() =>               (0x20000000 as uint32_t);)

/*  Bit definition for RCC_APB2LPENR register  */
pub macro_rules! RCC_APB2LPENR_TIM1LPEN (() =>              (0x00000001 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_TIM8LPEN (() =>              (0x00000002 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_USART1LPEN (() =>            (0x00000010 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_USART6LPEN (() =>            (0x00000020 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_ADC1LPEN (() =>              (0x00000100 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_ADC2PEN (() =>               (0x00000200 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_ADC3LPEN (() =>              (0x00000400 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_SDIOLPEN (() =>              (0x00000800 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_SPI1LPEN (() =>              (0x00001000 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_SYSCFGLPEN (() =>            (0x00004000 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_TIM9LPEN (() =>              (0x00010000 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_TIM10LPEN (() =>             (0x00020000 as uint32_t);)
pub macro_rules! RCC_APB2LPENR_TIM11LPEN (() =>             (0x00040000 as uint32_t);)

/*  Bit definition for RCC_BDCR register  */
pub macro_rules! RCC_BDCR_LSEON (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! RCC_BDCR_LSERDY (() =>                     (0x00000002 as uint32_t);)
pub macro_rules! RCC_BDCR_LSEBYP (() =>                     (0x00000004 as uint32_t);)

pub macro_rules! RCC_BDCR_RTCSEL (() =>                    (0x00000300 as uint32_t);)
pub macro_rules! RCC_BDCR_RTCSEL_0 (() =>                  (0x00000100 as uint32_t);)
pub macro_rules! RCC_BDCR_RTCSEL_1 (() =>                  (0x00000200 as uint32_t);)

pub macro_rules! RCC_BDCR_RTCEN (() =>                      (0x00008000 as uint32_t);)
pub macro_rules! RCC_BDCR_BDRST (() =>                      (0x00010000 as uint32_t);)

/*  Bit definition for RCC_CSR register  */
pub macro_rules! RCC_CSR_LSION (() =>                       (0x00000001 as uint32_t);)
pub macro_rules! RCC_CSR_LSIRDY (() =>                      (0x00000002 as uint32_t);)
pub macro_rules! RCC_CSR_RMVF (() =>                        (0x01000000 as uint32_t);)
pub macro_rules! RCC_CSR_BORRSTF (() =>                     (0x02000000 as uint32_t);)
pub macro_rules! RCC_CSR_PADRSTF (() =>                     (0x04000000 as uint32_t);)
pub macro_rules! RCC_CSR_PORRSTF (() =>                     (0x08000000 as uint32_t);)
pub macro_rules! RCC_CSR_SFTRSTF (() =>                     (0x10000000 as uint32_t);)
pub macro_rules! RCC_CSR_WDGRSTF (() =>                     (0x20000000 as uint32_t);)
pub macro_rules! RCC_CSR_WWDGRSTF (() =>                    (0x40000000 as uint32_t);)
pub macro_rules! RCC_CSR_LPWRRSTF (() =>                    (0x80000000 as uint32_t);)

/*  Bit definition for RCC_SSCGR register  */
pub macro_rules! RCC_SSCGR_MODPER (() =>                    (0x00001FFF as uint32_t);)
pub macro_rules! RCC_SSCGR_INCSTEP (() =>                   (0x0FFFE000 as uint32_t);)
pub macro_rules! RCC_SSCGR_SPREADSEL (() =>                 (0x40000000 as uint32_t);)
pub macro_rules! RCC_SSCGR_SSCGEN (() =>                    (0x80000000 as uint32_t);)

/*  Bit definition for RCC_PLLI2SCFGR register  */
pub macro_rules! RCC_PLLI2SCFGR_PLLI2SN (() =>              (0x00007FC0 as uint32_t);)
pub macro_rules! RCC_PLLI2SCFGR_PLLI2SR (() =>              (0x70000000 as uint32_t);)


/*                                                                            */
/*                                    RNG                                     */
/*                                                                            */

/*  Bits definition for RNG_CR register  */
pub macro_rules! RNG_CR_RNGEN (() =>                         (0x00000004 as uint32_t);)
pub macro_rules! RNG_CR_IE (() =>                            (0x00000008 as uint32_t);)

/*  Bits definition for RNG_SR register  */
pub macro_rules! RNG_SR_DRDY (() =>                          (0x00000001 as uint32_t);)
pub macro_rules! RNG_SR_CECS (() =>                          (0x00000002 as uint32_t);)
pub macro_rules! RNG_SR_SECS (() =>                          (0x00000004 as uint32_t);)
pub macro_rules! RNG_SR_CEIS (() =>                          (0x00000020 as uint32_t);)
pub macro_rules! RNG_SR_SEIS (() =>                          (0x00000040 as uint32_t);)


/*                                                                            */
/*                           Real-Time Clock (RTC)                            */
/*                                                                            */

/*  Bits definition for RTC_TR register  */
pub macro_rules! RTC_TR_PM (() =>                            (0x00400000 as uint32_t);)
pub macro_rules! RTC_TR_HT (() =>                            (0x00300000 as uint32_t);)
pub macro_rules! RTC_TR_HT_0 (() =>                          (0x00100000 as uint32_t);)
pub macro_rules! RTC_TR_HT_1 (() =>                          (0x00200000 as uint32_t);)
pub macro_rules! RTC_TR_HU (() =>                            (0x000F0000 as uint32_t);)
pub macro_rules! RTC_TR_HU_0 (() =>                          (0x00010000 as uint32_t);)
pub macro_rules! RTC_TR_HU_1 (() =>                          (0x00020000 as uint32_t);)
pub macro_rules! RTC_TR_HU_2 (() =>                          (0x00040000 as uint32_t);)
pub macro_rules! RTC_TR_HU_3 (() =>                          (0x00080000 as uint32_t);)
pub macro_rules! RTC_TR_MNT (() =>                           (0x00007000 as uint32_t);)
pub macro_rules! RTC_TR_MNT_0 (() =>                         (0x00001000 as uint32_t);)
pub macro_rules! RTC_TR_MNT_1 (() =>                         (0x00002000 as uint32_t);)
pub macro_rules! RTC_TR_MNT_2 (() =>                         (0x00004000 as uint32_t);)
pub macro_rules! RTC_TR_MNU (() =>                           (0x00000F00 as uint32_t);)
pub macro_rules! RTC_TR_MNU_0 (() =>                         (0x00000100 as uint32_t);)
pub macro_rules! RTC_TR_MNU_1 (() =>                         (0x00000200 as uint32_t);)
pub macro_rules! RTC_TR_MNU_2 (() =>                         (0x00000400 as uint32_t);)
pub macro_rules! RTC_TR_MNU_3 (() =>                         (0x00000800 as uint32_t);)
pub macro_rules! RTC_TR_ST (() =>                            (0x00000070 as uint32_t);)
pub macro_rules! RTC_TR_ST_0 (() =>                          (0x00000010 as uint32_t);)
pub macro_rules! RTC_TR_ST_1 (() =>                          (0x00000020 as uint32_t);)
pub macro_rules! RTC_TR_ST_2 (() =>                          (0x00000040 as uint32_t);)
pub macro_rules! RTC_TR_SU (() =>                            (0x0000000F as uint32_t);)
pub macro_rules! RTC_TR_SU_0 (() =>                          (0x00000001 as uint32_t);)
pub macro_rules! RTC_TR_SU_1 (() =>                          (0x00000002 as uint32_t);)
pub macro_rules! RTC_TR_SU_2 (() =>                          (0x00000004 as uint32_t);)
pub macro_rules! RTC_TR_SU_3 (() =>                          (0x00000008 as uint32_t);)

/*  Bits definition for RTC_DR register  */
pub macro_rules! RTC_DR_YT (() =>                            (0x00F00000 as uint32_t);)
pub macro_rules! RTC_DR_YT_0 (() =>                          (0x00100000 as uint32_t);)
pub macro_rules! RTC_DR_YT_1 (() =>                          (0x00200000 as uint32_t);)
pub macro_rules! RTC_DR_YT_2 (() =>                          (0x00400000 as uint32_t);)
pub macro_rules! RTC_DR_YT_3 (() =>                          (0x00800000 as uint32_t);)
pub macro_rules! RTC_DR_YU (() =>                            (0x000F0000 as uint32_t);)
pub macro_rules! RTC_DR_YU_0 (() =>                          (0x00010000 as uint32_t);)
pub macro_rules! RTC_DR_YU_1 (() =>                          (0x00020000 as uint32_t);)
pub macro_rules! RTC_DR_YU_2 (() =>                          (0x00040000 as uint32_t);)
pub macro_rules! RTC_DR_YU_3 (() =>                          (0x00080000 as uint32_t);)
pub macro_rules! RTC_DR_WDU (() =>                           (0x0000E000 as uint32_t);)
pub macro_rules! RTC_DR_WDU_0 (() =>                         (0x00002000 as uint32_t);)
pub macro_rules! RTC_DR_WDU_1 (() =>                         (0x00004000 as uint32_t);)
pub macro_rules! RTC_DR_WDU_2 (() =>                         (0x00008000 as uint32_t);)
pub macro_rules! RTC_DR_MT (() =>                            (0x00001000 as uint32_t);)
pub macro_rules! RTC_DR_MU (() =>                            (0x00000F00 as uint32_t);)
pub macro_rules! RTC_DR_MU_0 (() =>                          (0x00000100 as uint32_t);)
pub macro_rules! RTC_DR_MU_1 (() =>                          (0x00000200 as uint32_t);)
pub macro_rules! RTC_DR_MU_2 (() =>                          (0x00000400 as uint32_t);)
pub macro_rules! RTC_DR_MU_3 (() =>                          (0x00000800 as uint32_t);)
pub macro_rules! RTC_DR_DT (() =>                            (0x00000030 as uint32_t);)
pub macro_rules! RTC_DR_DT_0 (() =>                          (0x00000010 as uint32_t);)
pub macro_rules! RTC_DR_DT_1 (() =>                          (0x00000020 as uint32_t);)
pub macro_rules! RTC_DR_DU (() =>                            (0x0000000F as uint32_t);)
pub macro_rules! RTC_DR_DU_0 (() =>                          (0x00000001 as uint32_t);)
pub macro_rules! RTC_DR_DU_1 (() =>                          (0x00000002 as uint32_t);)
pub macro_rules! RTC_DR_DU_2 (() =>                          (0x00000004 as uint32_t);)
pub macro_rules! RTC_DR_DU_3 (() =>                          (0x00000008 as uint32_t);)

/*  Bits definition for RTC_CR register  */
pub macro_rules! RTC_CR_COE (() =>                           (0x00800000 as uint32_t);)
pub macro_rules! RTC_CR_OSEL (() =>                          (0x00600000 as uint32_t);)
pub macro_rules! RTC_CR_OSEL_0 (() =>                        (0x00200000 as uint32_t);)
pub macro_rules! RTC_CR_OSEL_1 (() =>                        (0x00400000 as uint32_t);)
pub macro_rules! RTC_CR_POL (() =>                           (0x00100000 as uint32_t);)
pub macro_rules! RTC_CR_COSEL (() =>                         (0x00080000 as uint32_t);)
pub macro_rules! RTC_CR_BCK (() =>                           (0x00040000 as uint32_t);)
pub macro_rules! RTC_CR_SUB1H (() =>                         (0x00020000 as uint32_t);)
pub macro_rules! RTC_CR_ADD1H (() =>                         (0x00010000 as uint32_t);)
pub macro_rules! RTC_CR_TSIE (() =>                          (0x00008000 as uint32_t);)
pub macro_rules! RTC_CR_WUTIE (() =>                         (0x00004000 as uint32_t);)
pub macro_rules! RTC_CR_ALRBIE (() =>                        (0x00002000 as uint32_t);)
pub macro_rules! RTC_CR_ALRAIE (() =>                        (0x00001000 as uint32_t);)
pub macro_rules! RTC_CR_TSE (() =>                           (0x00000800 as uint32_t);)
pub macro_rules! RTC_CR_WUTE (() =>                          (0x00000400 as uint32_t);)
pub macro_rules! RTC_CR_ALRBE (() =>                         (0x00000200 as uint32_t);)
pub macro_rules! RTC_CR_ALRAE (() =>                         (0x00000100 as uint32_t);)
pub macro_rules! RTC_CR_DCE (() =>                           (0x00000080 as uint32_t);)
pub macro_rules! RTC_CR_FMT (() =>                           (0x00000040 as uint32_t);)
pub macro_rules! RTC_CR_BYPSHAD (() =>                       (0x00000020 as uint32_t);)
pub macro_rules! RTC_CR_REFCKON (() =>                       (0x00000010 as uint32_t);)
pub macro_rules! RTC_CR_TSEDGE (() =>                        (0x00000008 as uint32_t);)
pub macro_rules! RTC_CR_WUCKSEL (() =>                       (0x00000007 as uint32_t);)
pub macro_rules! RTC_CR_WUCKSEL_0 (() =>                     (0x00000001 as uint32_t);)
pub macro_rules! RTC_CR_WUCKSEL_1 (() =>                     (0x00000002 as uint32_t);)
pub macro_rules! RTC_CR_WUCKSEL_2 (() =>                     (0x00000004 as uint32_t);)

/*  Bits definition for RTC_ISR register  */
pub macro_rules! RTC_ISR_RECALPF (() =>                      (0x00010000 as uint32_t);)
pub macro_rules! RTC_ISR_TAMP1F (() =>                       (0x00002000 as uint32_t);)
pub macro_rules! RTC_ISR_TSOVF (() =>                        (0x00001000 as uint32_t);)
pub macro_rules! RTC_ISR_TSF (() =>                          (0x00000800 as uint32_t);)
pub macro_rules! RTC_ISR_WUTF (() =>                         (0x00000400 as uint32_t);)
pub macro_rules! RTC_ISR_ALRBF (() =>                        (0x00000200 as uint32_t);)
pub macro_rules! RTC_ISR_ALRAF (() =>                        (0x00000100 as uint32_t);)
pub macro_rules! RTC_ISR_INIT (() =>                         (0x00000080 as uint32_t);)
pub macro_rules! RTC_ISR_INITF (() =>                        (0x00000040 as uint32_t);)
pub macro_rules! RTC_ISR_RSF (() =>                          (0x00000020 as uint32_t);)
pub macro_rules! RTC_ISR_INITS (() =>                        (0x00000010 as uint32_t);)
pub macro_rules! RTC_ISR_SHPF (() =>                         (0x00000008 as uint32_t);)
pub macro_rules! RTC_ISR_WUTWF (() =>                        (0x00000004 as uint32_t);)
pub macro_rules! RTC_ISR_ALRBWF (() =>                       (0x00000002 as uint32_t);)
pub macro_rules! RTC_ISR_ALRAWF (() =>                       (0x00000001 as uint32_t);)

/*  Bits definition for RTC_PRER register  */
pub macro_rules! RTC_PRER_PREDIV_A (() =>                    (0x007F0000 as uint32_t);)
pub macro_rules! RTC_PRER_PREDIV_S (() =>                    (0x00001FFF as uint32_t);)

/*  Bits definition for RTC_WUTR register  */
pub macro_rules! RTC_WUTR_WUT (() =>                         (0x0000FFFF as uint32_t);)

/*  Bits definition for RTC_CALIBR register  */
pub macro_rules! RTC_CALIBR_DCS (() =>                       (0x00000080 as uint32_t);)
pub macro_rules! RTC_CALIBR_DC (() =>                        (0x0000001F as uint32_t);)

/*  Bits definition for RTC_ALRMAR register  */
pub macro_rules! RTC_ALRMAR_MSK4 (() =>                      (0x80000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_WDSEL (() =>                     (0x40000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DT (() =>                        (0x30000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DT_0 (() =>                      (0x10000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DT_1 (() =>                      (0x20000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DU (() =>                        (0x0F000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DU_0 (() =>                      (0x01000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DU_1 (() =>                      (0x02000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DU_2 (() =>                      (0x04000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_DU_3 (() =>                      (0x08000000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MSK3 (() =>                      (0x00800000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_PM (() =>                        (0x00400000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HT (() =>                        (0x00300000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HT_0 (() =>                      (0x00100000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HT_1 (() =>                      (0x00200000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HU (() =>                        (0x000F0000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HU_0 (() =>                      (0x00010000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HU_1 (() =>                      (0x00020000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HU_2 (() =>                      (0x00040000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_HU_3 (() =>                      (0x00080000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MSK2 (() =>                      (0x00008000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNT (() =>                       (0x00007000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNT_0 (() =>                     (0x00001000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNT_1 (() =>                     (0x00002000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNT_2 (() =>                     (0x00004000 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNU (() =>                       (0x00000F00 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNU_0 (() =>                     (0x00000100 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNU_1 (() =>                     (0x00000200 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNU_2 (() =>                     (0x00000400 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MNU_3 (() =>                     (0x00000800 as uint32_t);)
pub macro_rules! RTC_ALRMAR_MSK1 (() =>                      (0x00000080 as uint32_t);)
pub macro_rules! RTC_ALRMAR_ST (() =>                        (0x00000070 as uint32_t);)
pub macro_rules! RTC_ALRMAR_ST_0 (() =>                      (0x00000010 as uint32_t);)
pub macro_rules! RTC_ALRMAR_ST_1 (() =>                      (0x00000020 as uint32_t);)
pub macro_rules! RTC_ALRMAR_ST_2 (() =>                      (0x00000040 as uint32_t);)
pub macro_rules! RTC_ALRMAR_SU (() =>                        (0x0000000F as uint32_t);)
pub macro_rules! RTC_ALRMAR_SU_0 (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! RTC_ALRMAR_SU_1 (() =>                      (0x00000002 as uint32_t);)
pub macro_rules! RTC_ALRMAR_SU_2 (() =>                      (0x00000004 as uint32_t);)
pub macro_rules! RTC_ALRMAR_SU_3 (() =>                      (0x00000008 as uint32_t);)

/*  Bits definition for RTC_ALRMBR register  */
pub macro_rules! RTC_ALRMBR_MSK4 (() =>                      (0x80000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_WDSEL (() =>                     (0x40000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DT (() =>                        (0x30000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DT_0 (() =>                      (0x10000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DT_1 (() =>                      (0x20000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DU (() =>                        (0x0F000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DU_0 (() =>                      (0x01000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DU_1 (() =>                      (0x02000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DU_2 (() =>                      (0x04000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_DU_3 (() =>                      (0x08000000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MSK3 (() =>                      (0x00800000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_PM (() =>                        (0x00400000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HT (() =>                        (0x00300000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HT_0 (() =>                      (0x00100000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HT_1 (() =>                      (0x00200000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HU (() =>                        (0x000F0000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HU_0 (() =>                      (0x00010000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HU_1 (() =>                      (0x00020000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HU_2 (() =>                      (0x00040000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_HU_3 (() =>                      (0x00080000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MSK2 (() =>                      (0x00008000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNT (() =>                       (0x00007000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNT_0 (() =>                     (0x00001000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNT_1 (() =>                     (0x00002000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNT_2 (() =>                     (0x00004000 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNU (() =>                       (0x00000F00 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNU_0 (() =>                     (0x00000100 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNU_1 (() =>                     (0x00000200 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNU_2 (() =>                     (0x00000400 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MNU_3 (() =>                     (0x00000800 as uint32_t);)
pub macro_rules! RTC_ALRMBR_MSK1 (() =>                      (0x00000080 as uint32_t);)
pub macro_rules! RTC_ALRMBR_ST (() =>                        (0x00000070 as uint32_t);)
pub macro_rules! RTC_ALRMBR_ST_0 (() =>                      (0x00000010 as uint32_t);)
pub macro_rules! RTC_ALRMBR_ST_1 (() =>                      (0x00000020 as uint32_t);)
pub macro_rules! RTC_ALRMBR_ST_2 (() =>                      (0x00000040 as uint32_t);)
pub macro_rules! RTC_ALRMBR_SU (() =>                        (0x0000000F as uint32_t);)
pub macro_rules! RTC_ALRMBR_SU_0 (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! RTC_ALRMBR_SU_1 (() =>                      (0x00000002 as uint32_t);)
pub macro_rules! RTC_ALRMBR_SU_2 (() =>                      (0x00000004 as uint32_t);)
pub macro_rules! RTC_ALRMBR_SU_3 (() =>                      (0x00000008 as uint32_t);)

/*  Bits definition for RTC_WPR register  */
pub macro_rules! RTC_WPR_KEY (() =>                          (0x000000FF as uint32_t);)

/*  Bits definition for RTC_SSR register  */
pub macro_rules! RTC_SSR_SS (() =>                           (0x0000FFFF as uint32_t);)

/*  Bits definition for RTC_SHIFTR register  */
pub macro_rules! RTC_SHIFTR_SUBFS (() =>                     (0x00007FFF as uint32_t);)
pub macro_rules! RTC_SHIFTR_ADD1S (() =>                     (0x80000000 as uint32_t);)

/*  Bits definition for RTC_TSTR register  */
pub macro_rules! RTC_TSTR_PM (() =>                          (0x00400000 as uint32_t);)
pub macro_rules! RTC_TSTR_HT (() =>                          (0x00300000 as uint32_t);)
pub macro_rules! RTC_TSTR_HT_0 (() =>                        (0x00100000 as uint32_t);)
pub macro_rules! RTC_TSTR_HT_1 (() =>                        (0x00200000 as uint32_t);)
pub macro_rules! RTC_TSTR_HU (() =>                          (0x000F0000 as uint32_t);)
pub macro_rules! RTC_TSTR_HU_0 (() =>                        (0x00010000 as uint32_t);)
pub macro_rules! RTC_TSTR_HU_1 (() =>                        (0x00020000 as uint32_t);)
pub macro_rules! RTC_TSTR_HU_2 (() =>                        (0x00040000 as uint32_t);)
pub macro_rules! RTC_TSTR_HU_3 (() =>                        (0x00080000 as uint32_t);)
pub macro_rules! RTC_TSTR_MNT (() =>                         (0x00007000 as uint32_t);)
pub macro_rules! RTC_TSTR_MNT_0 (() =>                       (0x00001000 as uint32_t);)
pub macro_rules! RTC_TSTR_MNT_1 (() =>                       (0x00002000 as uint32_t);)
pub macro_rules! RTC_TSTR_MNT_2 (() =>                       (0x00004000 as uint32_t);)
pub macro_rules! RTC_TSTR_MNU (() =>                         (0x00000F00 as uint32_t);)
pub macro_rules! RTC_TSTR_MNU_0 (() =>                       (0x00000100 as uint32_t);)
pub macro_rules! RTC_TSTR_MNU_1 (() =>                       (0x00000200 as uint32_t);)
pub macro_rules! RTC_TSTR_MNU_2 (() =>                       (0x00000400 as uint32_t);)
pub macro_rules! RTC_TSTR_MNU_3 (() =>                       (0x00000800 as uint32_t);)
pub macro_rules! RTC_TSTR_ST (() =>                          (0x00000070 as uint32_t);)
pub macro_rules! RTC_TSTR_ST_0 (() =>                        (0x00000010 as uint32_t);)
pub macro_rules! RTC_TSTR_ST_1 (() =>                        (0x00000020 as uint32_t);)
pub macro_rules! RTC_TSTR_ST_2 (() =>                        (0x00000040 as uint32_t);)
pub macro_rules! RTC_TSTR_SU (() =>                          (0x0000000F as uint32_t);)
pub macro_rules! RTC_TSTR_SU_0 (() =>                        (0x00000001 as uint32_t);)
pub macro_rules! RTC_TSTR_SU_1 (() =>                        (0x00000002 as uint32_t);)
pub macro_rules! RTC_TSTR_SU_2 (() =>                        (0x00000004 as uint32_t);)
pub macro_rules! RTC_TSTR_SU_3 (() =>                        (0x00000008 as uint32_t);)

/*  Bits definition for RTC_TSDR register  */
pub macro_rules! RTC_TSDR_WDU (() =>                         (0x0000E000 as uint32_t);)
pub macro_rules! RTC_TSDR_WDU_0 (() =>                       (0x00002000 as uint32_t);)
pub macro_rules! RTC_TSDR_WDU_1 (() =>                       (0x00004000 as uint32_t);)
pub macro_rules! RTC_TSDR_WDU_2 (() =>                       (0x00008000 as uint32_t);)
pub macro_rules! RTC_TSDR_MT (() =>                          (0x00001000 as uint32_t);)
pub macro_rules! RTC_TSDR_MU (() =>                          (0x00000F00 as uint32_t);)
pub macro_rules! RTC_TSDR_MU_0 (() =>                        (0x00000100 as uint32_t);)
pub macro_rules! RTC_TSDR_MU_1 (() =>                        (0x00000200 as uint32_t);)
pub macro_rules! RTC_TSDR_MU_2 (() =>                        (0x00000400 as uint32_t);)
pub macro_rules! RTC_TSDR_MU_3 (() =>                        (0x00000800 as uint32_t);)
pub macro_rules! RTC_TSDR_DT (() =>                          (0x00000030 as uint32_t);)
pub macro_rules! RTC_TSDR_DT_0 (() =>                        (0x00000010 as uint32_t);)
pub macro_rules! RTC_TSDR_DT_1 (() =>                        (0x00000020 as uint32_t);)
pub macro_rules! RTC_TSDR_DU (() =>                          (0x0000000F as uint32_t);)
pub macro_rules! RTC_TSDR_DU_0 (() =>                        (0x00000001 as uint32_t);)
pub macro_rules! RTC_TSDR_DU_1 (() =>                        (0x00000002 as uint32_t);)
pub macro_rules! RTC_TSDR_DU_2 (() =>                        (0x00000004 as uint32_t);)
pub macro_rules! RTC_TSDR_DU_3 (() =>                        (0x00000008 as uint32_t);)

/*  Bits definition for RTC_TSSSR register  */
pub macro_rules! RTC_TSSSR_SS (() =>                         (0x0000FFFF as uint32_t);)

/*  Bits definition for RTC_CAL register  */
pub macro_rules! RTC_CALR_CALP (() =>                        (0x00008000 as uint32_t);)
pub macro_rules! RTC_CALR_CALW8 (() =>                       (0x00004000 as uint32_t);)
pub macro_rules! RTC_CALR_CALW16 (() =>                      (0x00002000 as uint32_t);)
pub macro_rules! RTC_CALR_CALM (() =>                        (0x000001FF as uint32_t);)
pub macro_rules! RTC_CALR_CALM_0 (() =>                      (0x00000001 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_1 (() =>                      (0x00000002 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_2 (() =>                      (0x00000004 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_3 (() =>                      (0x00000008 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_4 (() =>                      (0x00000010 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_5 (() =>                      (0x00000020 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_6 (() =>                      (0x00000040 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_7 (() =>                      (0x00000080 as uint32_t);)
pub macro_rules! RTC_CALR_CALM_8 (() =>                      (0x00000100 as uint32_t);)

/*  Bits definition for RTC_TAFCR register  */
pub macro_rules! RTC_TAFCR_ALARMOUTTYPE (() =>               (0x00040000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TSINSEL (() =>                    (0x00020000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPINSEL (() =>                  (0x00010000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPPUDIS (() =>                  (0x00008000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPPRCH (() =>                   (0x00006000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPPRCH_0 (() =>                 (0x00002000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPPRCH_1 (() =>                 (0x00004000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPFLT (() =>                    (0x00001800 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPFLT_0 (() =>                  (0x00000800 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPFLT_1 (() =>                  (0x00001000 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPFREQ (() =>                   (0x00000700 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPFREQ_0 (() =>                 (0x00000100 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPFREQ_1 (() =>                 (0x00000200 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPFREQ_2 (() =>                 (0x00000400 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPTS (() =>                     (0x00000080 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMPIE (() =>                     (0x00000004 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMP1TRG (() =>                   (0x00000002 as uint32_t);)
pub macro_rules! RTC_TAFCR_TAMP1E (() =>                     (0x00000001 as uint32_t);)

/*  Bits definition for RTC_ALRMASSR register  */
pub macro_rules! RTC_ALRMASSR_MASKSS (() =>                  (0x0F000000 as uint32_t);)
pub macro_rules! RTC_ALRMASSR_MASKSS_0 (() =>                (0x01000000 as uint32_t);)
pub macro_rules! RTC_ALRMASSR_MASKSS_1 (() =>                (0x02000000 as uint32_t);)
pub macro_rules! RTC_ALRMASSR_MASKSS_2 (() =>                (0x04000000 as uint32_t);)
pub macro_rules! RTC_ALRMASSR_MASKSS_3 (() =>                (0x08000000 as uint32_t);)
pub macro_rules! RTC_ALRMASSR_SS (() =>                      (0x00007FFF as uint32_t);)

/*  Bits definition for RTC_ALRMBSSR register  */
pub macro_rules! RTC_ALRMBSSR_MASKSS (() =>                  (0x0F000000 as uint32_t);)
pub macro_rules! RTC_ALRMBSSR_MASKSS_0 (() =>                (0x01000000 as uint32_t);)
pub macro_rules! RTC_ALRMBSSR_MASKSS_1 (() =>                (0x02000000 as uint32_t);)
pub macro_rules! RTC_ALRMBSSR_MASKSS_2 (() =>                (0x04000000 as uint32_t);)
pub macro_rules! RTC_ALRMBSSR_MASKSS_3 (() =>                (0x08000000 as uint32_t);)
pub macro_rules! RTC_ALRMBSSR_SS (() =>                      (0x00007FFF as uint32_t);)

/*  Bits definition for RTC_BKP0R register  */
pub macro_rules! RTC_BKP0R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP1R register  */
pub macro_rules! RTC_BKP1R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP2R register  */
pub macro_rules! RTC_BKP2R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP3R register  */
pub macro_rules! RTC_BKP3R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP4R register  */
pub macro_rules! RTC_BKP4R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP5R register  */
pub macro_rules! RTC_BKP5R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP6R register  */
pub macro_rules! RTC_BKP6R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP7R register  */
pub macro_rules! RTC_BKP7R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP8R register  */
pub macro_rules! RTC_BKP8R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP9R register  */
pub macro_rules! RTC_BKP9R (() =>                            (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP10R register  */
pub macro_rules! RTC_BKP10R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP11R register  */
pub macro_rules! RTC_BKP11R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP12R register  */
pub macro_rules! RTC_BKP12R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP13R register  */
pub macro_rules! RTC_BKP13R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP14R register  */
pub macro_rules! RTC_BKP14R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP15R register  */
pub macro_rules! RTC_BKP15R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP16R register  */
pub macro_rules! RTC_BKP16R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP17R register  */
pub macro_rules! RTC_BKP17R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP18R register  */
pub macro_rules! RTC_BKP18R (() =>                           (0xFFFFFFFF as uint32_t);)

/*  Bits definition for RTC_BKP19R register  */
pub macro_rules! RTC_BKP19R (() =>                           (0xFFFFFFFF as uint32_t);)


/*                                                                            */
/*                          SD host Interface                                 */
/*                                                                            */

/*  Bit definition for SDIO_POWER register  */
pub macro_rules! SDIO_POWER_PWRCTRL (() =>                  (0x03 as uint8_t);)               /*<PWRCTRL[1:0] bits (Power supply control bits) */
pub macro_rules! SDIO_POWER_PWRCTRL_0 (() =>                (0x01 as uint8_t);)               /*<Bit 0 */
pub macro_rules! SDIO_POWER_PWRCTRL_1 (() =>                (0x02 as uint8_t);)               /*<Bit 1 */

/*  Bit definition for SDIO_CLKCR register  */
pub macro_rules! SDIO_CLKCR_CLKDIV (() =>                   (0x00FF as uint16_t);)            /*<Clock divide factor */
pub macro_rules! SDIO_CLKCR_CLKEN (() =>                    (0x0100 as uint16_t);)            /*<Clock enable bit */
pub macro_rules! SDIO_CLKCR_PWRSAV (() =>                   (0x0200 as uint16_t);)            /*<Power saving configuration bit */
pub macro_rules! SDIO_CLKCR_BYPASS (() =>                   (0x0400 as uint16_t);)            /*<Clock divider bypass enable bit */

pub macro_rules! SDIO_CLKCR_WIDBUS (() =>                   (0x1800 as uint16_t);)            /*<WIDBUS[1:0] bits (Wide bus mode enable bit) */
pub macro_rules! SDIO_CLKCR_WIDBUS_0 (() =>                 (0x0800 as uint16_t);)            /*<Bit 0 */
pub macro_rules! SDIO_CLKCR_WIDBUS_1 (() =>                 (0x1000 as uint16_t);)            /*<Bit 1 */

pub macro_rules! SDIO_CLKCR_NEGEDGE (() =>                  (0x2000 as uint16_t);)            /*<SDIO_CK dephasing selection bit */
pub macro_rules! SDIO_CLKCR_HWFC_EN (() =>                  (0x4000 as uint16_t);)            /*<HW Flow Control enable */

/*  Bit definition for SDIO_ARG register  */
pub macro_rules! SDIO_ARG_CMDARG (() =>                     (0xFFFFFFFF as uint32_t);)            /*<Command argument */

/*  Bit definition for SDIO_CMD register  */
pub macro_rules! SDIO_CMD_CMDINDEX (() =>                   (0x003F as uint16_t);)            /*<Command Index */

pub macro_rules! SDIO_CMD_WAITRESP (() =>                   (0x00C0 as uint16_t);)            /*<WAITRESP[1:0] bits (Wait for response bits) */
pub macro_rules! SDIO_CMD_WAITRESP_0 (() =>                 (0x0040 as uint16_t);)            /*< Bit 0 */
pub macro_rules! SDIO_CMD_WAITRESP_1 (() =>                 (0x0080 as uint16_t);)            /*< Bit 1 */

pub macro_rules! SDIO_CMD_WAITINT (() =>                    (0x0100 as uint16_t);)            /*<CPSM Waits for Interrupt Request */
pub macro_rules! SDIO_CMD_WAITPEND (() =>                   (0x0200 as uint16_t);)            /*<CPSM Waits for ends of data transfer (CmdPend internal signal) */
pub macro_rules! SDIO_CMD_CPSMEN (() =>                     (0x0400 as uint16_t);)            /*<Command path state machine (CPSM) Enable bit */
pub macro_rules! SDIO_CMD_SDIOSUSPEND (() =>                (0x0800 as uint16_t);)            /*<SD I/O suspend command */
pub macro_rules! SDIO_CMD_ENCMDCOMPL (() =>                 (0x1000 as uint16_t);)            /*<Enable CMD completion */
pub macro_rules! SDIO_CMD_NIEN (() =>                       (0x2000 as uint16_t);)            /*<Not Interrupt Enable */
pub macro_rules! SDIO_CMD_CEATACMD (() =>                   (0x4000 as uint16_t);)            /*<CE-ATA command */

/*  Bit definition for SDIO_RESPCMD register  */
pub macro_rules! SDIO_RESPCMD_RESPCMD (() =>                (0x3F as uint8_t);)               /*<Response command index */

/*  Bit definition for SDIO_RESP0 register  */
pub macro_rules! SDIO_RESP0_CARDSTATUS0 (() =>              (0xFFFFFFFF as uint32_t);)        /*<Card Status */

/*  Bit definition for SDIO_RESP1 register  */
pub macro_rules! SDIO_RESP1_CARDSTATUS1 (() =>              (0xFFFFFFFF as uint32_t);)        /*<Card Status */

/*  Bit definition for SDIO_RESP2 register  */
pub macro_rules! SDIO_RESP2_CARDSTATUS2 (() =>              (0xFFFFFFFF as uint32_t);)        /*<Card Status */

/*  Bit definition for SDIO_RESP3 register  */
pub macro_rules! SDIO_RESP3_CARDSTATUS3 (() =>              (0xFFFFFFFF as uint32_t);)        /*<Card Status */

/*  Bit definition for SDIO_RESP4 register  */
pub macro_rules! SDIO_RESP4_CARDSTATUS4 (() =>              (0xFFFFFFFF as uint32_t);)        /*<Card Status */

/*  Bit definition for SDIO_DTIMER register  */
pub macro_rules! SDIO_DTIMER_DATATIME (() =>                (0xFFFFFFFF as uint32_t);)        /*<Data timeout period. */

/*  Bit definition for SDIO_DLEN register  */
pub macro_rules! SDIO_DLEN_DATALENGTH (() =>                (0x01FFFFFF as uint32_t);)        /*<Data length value */

/*  Bit definition for SDIO_DCTRL register  */
pub macro_rules! SDIO_DCTRL_DTEN (() =>                     (0x0001 as uint16_t);)            /*<Data transfer enabled bit */
pub macro_rules! SDIO_DCTRL_DTDIR (() =>                    (0x0002 as uint16_t);)            /*<Data transfer direction selection */
pub macro_rules! SDIO_DCTRL_DTMODE (() =>                   (0x0004 as uint16_t);)            /*<Data transfer mode selection */
pub macro_rules! SDIO_DCTRL_DMAEN (() =>                    (0x0008 as uint16_t);)            /*<DMA enabled bit */

pub macro_rules! SDIO_DCTRL_DBLOCKSIZE (() =>               (0x00F0 as uint16_t);)            /*<DBLOCKSIZE[3:0] bits (Data block size) */
pub macro_rules! SDIO_DCTRL_DBLOCKSIZE_0 (() =>             (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! SDIO_DCTRL_DBLOCKSIZE_1 (() =>             (0x0020 as uint16_t);)            /*<Bit 1 */
pub macro_rules! SDIO_DCTRL_DBLOCKSIZE_2 (() =>             (0x0040 as uint16_t);)            /*<Bit 2 */
pub macro_rules! SDIO_DCTRL_DBLOCKSIZE_3 (() =>             (0x0080 as uint16_t);)            /*<Bit 3 */

pub macro_rules! SDIO_DCTRL_RWSTART (() =>                  (0x0100 as uint16_t);)            /*<Read wait start */
pub macro_rules! SDIO_DCTRL_RWSTOP (() =>                   (0x0200 as uint16_t);)            /*<Read wait stop */
pub macro_rules! SDIO_DCTRL_RWMOD (() =>                    (0x0400 as uint16_t);)            /*<Read wait mode */
pub macro_rules! SDIO_DCTRL_SDIOEN (() =>                   (0x0800 as uint16_t);)            /*<SD I/O enable functions */

/*  Bit definition for SDIO_DCOUNT register  */
pub macro_rules! SDIO_DCOUNT_DATACOUNT (() =>               (0x01FFFFFF as uint32_t);)        /*<Data count value */

/*  Bit definition for SDIO_STA register  */
pub macro_rules! SDIO_STA_CCRCFAIL (() =>                   (0x00000001 as uint32_t);)        /*<Command response received (CRC check failed) */
pub macro_rules! SDIO_STA_DCRCFAIL (() =>                   (0x00000002 as uint32_t);)        /*<Data block sent/received (CRC check failed) */
pub macro_rules! SDIO_STA_CTIMEOUT (() =>                   (0x00000004 as uint32_t);)        /*<Command response timeout */
pub macro_rules! SDIO_STA_DTIMEOUT (() =>                   (0x00000008 as uint32_t);)        /*<Data timeout */
pub macro_rules! SDIO_STA_TXUNDERR (() =>                   (0x00000010 as uint32_t);)        /*<Transmit FIFO underrun error */
pub macro_rules! SDIO_STA_RXOVERR (() =>                    (0x00000020 as uint32_t);)        /*<Received FIFO overrun error */
pub macro_rules! SDIO_STA_CMDREND (() =>                    (0x00000040 as uint32_t);)        /*<Command response received (CRC check passed) */
pub macro_rules! SDIO_STA_CMDSENT (() =>                    (0x00000080 as uint32_t);)        /*<Command sent (no response required) */
pub macro_rules! SDIO_STA_DATAEND (() =>                    (0x00000100 as uint32_t);)        /*<Data end (data counter, SDIDCOUNT, is zero) */
pub macro_rules! SDIO_STA_STBITERR (() =>                   (0x00000200 as uint32_t);)        /*<Start bit not detected on all data signals in wide bus mode */
pub macro_rules! SDIO_STA_DBCKEND (() =>                    (0x00000400 as uint32_t);)        /*<Data block sent/received (CRC check passed) */
pub macro_rules! SDIO_STA_CMDACT (() =>                     (0x00000800 as uint32_t);)        /*<Command transfer in progress */
pub macro_rules! SDIO_STA_TXACT (() =>                      (0x00001000 as uint32_t);)        /*<Data transmit in progress */
pub macro_rules! SDIO_STA_RXACT (() =>                      (0x00002000 as uint32_t);)        /*<Data receive in progress */
pub macro_rules! SDIO_STA_TXFIFOHE (() =>                   (0x00004000 as uint32_t);)        /*<Transmit FIFO Half Empty: at least 8 words can be written into the FIFO */
pub macro_rules! SDIO_STA_RXFIFOHF (() =>                   (0x00008000 as uint32_t);)        /*<Receive FIFO Half Full: there are at least 8 words in the FIFO */
pub macro_rules! SDIO_STA_TXFIFOF (() =>                    (0x00010000 as uint32_t);)        /*<Transmit FIFO full */
pub macro_rules! SDIO_STA_RXFIFOF (() =>                    (0x00020000 as uint32_t);)        /*<Receive FIFO full */
pub macro_rules! SDIO_STA_TXFIFOE (() =>                    (0x00040000 as uint32_t);)        /*<Transmit FIFO empty */
pub macro_rules! SDIO_STA_RXFIFOE (() =>                    (0x00080000 as uint32_t);)        /*<Receive FIFO empty */
pub macro_rules! SDIO_STA_TXDAVL (() =>                     (0x00100000 as uint32_t);)        /*<Data available in transmit FIFO */
pub macro_rules! SDIO_STA_RXDAVL (() =>                     (0x00200000 as uint32_t);)        /*<Data available in receive FIFO */
pub macro_rules! SDIO_STA_SDIOIT (() =>                     (0x00400000 as uint32_t);)        /*<SDIO interrupt received */
pub macro_rules! SDIO_STA_CEATAEND (() =>                   (0x00800000 as uint32_t);)        /*<CE-ATA command completion signal received for CMD61 */

/*  Bit definition for SDIO_ICR register  */
pub macro_rules! SDIO_ICR_CCRCFAILC (() =>                  (0x00000001 as uint32_t);)        /*<CCRCFAIL flag clear bit */
pub macro_rules! SDIO_ICR_DCRCFAILC (() =>                  (0x00000002 as uint32_t);)        /*<DCRCFAIL flag clear bit */
pub macro_rules! SDIO_ICR_CTIMEOUTC (() =>                  (0x00000004 as uint32_t);)        /*<CTIMEOUT flag clear bit */
pub macro_rules! SDIO_ICR_DTIMEOUTC (() =>                  (0x00000008 as uint32_t);)        /*<DTIMEOUT flag clear bit */
pub macro_rules! SDIO_ICR_TXUNDERRC (() =>                  (0x00000010 as uint32_t);)        /*<TXUNDERR flag clear bit */
pub macro_rules! SDIO_ICR_RXOVERRC (() =>                   (0x00000020 as uint32_t);)        /*<RXOVERR flag clear bit */
pub macro_rules! SDIO_ICR_CMDRENDC (() =>                   (0x00000040 as uint32_t);)        /*<CMDREND flag clear bit */
pub macro_rules! SDIO_ICR_CMDSENTC (() =>                   (0x00000080 as uint32_t);)        /*<CMDSENT flag clear bit */
pub macro_rules! SDIO_ICR_DATAENDC (() =>                   (0x00000100 as uint32_t);)        /*<DATAEND flag clear bit */
pub macro_rules! SDIO_ICR_STBITERRC (() =>                  (0x00000200 as uint32_t);)        /*<STBITERR flag clear bit */
pub macro_rules! SDIO_ICR_DBCKENDC (() =>                   (0x00000400 as uint32_t);)        /*<DBCKEND flag clear bit */
pub macro_rules! SDIO_ICR_SDIOITC (() =>                    (0x00400000 as uint32_t);)        /*<SDIOIT flag clear bit */
pub macro_rules! SDIO_ICR_CEATAENDC (() =>                  (0x00800000 as uint32_t);)        /*<CEATAEND flag clear bit */

/*  Bit definition for SDIO_MASK register  */
pub macro_rules! SDIO_MASK_CCRCFAILIE (() =>                (0x00000001 as uint32_t);)        /*<Command CRC Fail Interrupt Enable */
pub macro_rules! SDIO_MASK_DCRCFAILIE (() =>                (0x00000002 as uint32_t);)        /*<Data CRC Fail Interrupt Enable */
pub macro_rules! SDIO_MASK_CTIMEOUTIE (() =>                (0x00000004 as uint32_t);)        /*<Command TimeOut Interrupt Enable */
pub macro_rules! SDIO_MASK_DTIMEOUTIE (() =>                (0x00000008 as uint32_t);)        /*<Data TimeOut Interrupt Enable */
pub macro_rules! SDIO_MASK_TXUNDERRIE (() =>                (0x00000010 as uint32_t);)        /*<Tx FIFO UnderRun Error Interrupt Enable */
pub macro_rules! SDIO_MASK_RXOVERRIE (() =>                 (0x00000020 as uint32_t);)        /*<Rx FIFO OverRun Error Interrupt Enable */
pub macro_rules! SDIO_MASK_CMDRENDIE (() =>                 (0x00000040 as uint32_t);)        /*<Command Response Received Interrupt Enable */
pub macro_rules! SDIO_MASK_CMDSENTIE (() =>                 (0x00000080 as uint32_t);)        /*<Command Sent Interrupt Enable */
pub macro_rules! SDIO_MASK_DATAENDIE (() =>                 (0x00000100 as uint32_t);)        /*<Data End Interrupt Enable */
pub macro_rules! SDIO_MASK_STBITERRIE (() =>                (0x00000200 as uint32_t);)        /*<Start Bit Error Interrupt Enable */
pub macro_rules! SDIO_MASK_DBCKENDIE (() =>                 (0x00000400 as uint32_t);)        /*<Data Block End Interrupt Enable */
pub macro_rules! SDIO_MASK_CMDACTIE (() =>                  (0x00000800 as uint32_t);)        /*<CCommand Acting Interrupt Enable */
pub macro_rules! SDIO_MASK_TXACTIE (() =>                   (0x00001000 as uint32_t);)        /*<Data Transmit Acting Interrupt Enable */
pub macro_rules! SDIO_MASK_RXACTIE (() =>                   (0x00002000 as uint32_t);)        /*<Data receive acting interrupt enabled */
pub macro_rules! SDIO_MASK_TXFIFOHEIE (() =>                (0x00004000 as uint32_t);)        /*<Tx FIFO Half Empty interrupt Enable */
pub macro_rules! SDIO_MASK_RXFIFOHFIE (() =>                (0x00008000 as uint32_t);)        /*<Rx FIFO Half Full interrupt Enable */
pub macro_rules! SDIO_MASK_TXFIFOFIE (() =>                 (0x00010000 as uint32_t);)        /*<Tx FIFO Full interrupt Enable */
pub macro_rules! SDIO_MASK_RXFIFOFIE (() =>                 (0x00020000 as uint32_t);)        /*<Rx FIFO Full interrupt Enable */
pub macro_rules! SDIO_MASK_TXFIFOEIE (() =>                 (0x00040000 as uint32_t);)        /*<Tx FIFO Empty interrupt Enable */
pub macro_rules! SDIO_MASK_RXFIFOEIE (() =>                 (0x00080000 as uint32_t);)        /*<Rx FIFO Empty interrupt Enable */
pub macro_rules! SDIO_MASK_TXDAVLIE (() =>                  (0x00100000 as uint32_t);)        /*<Data available in Tx FIFO interrupt Enable */
pub macro_rules! SDIO_MASK_RXDAVLIE (() =>                  (0x00200000 as uint32_t);)        /*<Data available in Rx FIFO interrupt Enable */
pub macro_rules! SDIO_MASK_SDIOITIE (() =>                  (0x00400000 as uint32_t);)        /*<SDIO Mode Interrupt Received interrupt Enable */
pub macro_rules! SDIO_MASK_CEATAENDIE (() =>                (0x00800000 as uint32_t);)        /*<CE-ATA command completion signal received Interrupt Enable */

/*  Bit definition for SDIO_FIFOCNT register  */
pub macro_rules! SDIO_FIFOCNT_FIFOCOUNT (() =>              (0x00FFFFFF as uint32_t);)        /*<Remaining number of words to be written to or read from the FIFO */

/*  Bit definition for SDIO_FIFO register  */
pub macro_rules! SDIO_FIFO_FIFODATA (() =>                  (0xFFFFFFFF as uint32_t);)        /*<Receive and transmit FIFO data */


/*                                                                            */
/*                        Serial Peripheral Interface                         */
/*                                                                            */

/*  Bit definition for SPI_CR1 register  */
pub macro_rules! SPI_CR1_CPHA (() =>                        (0x0001 as uint16_t);)            /*<Clock Phase */
pub macro_rules! SPI_CR1_CPOL (() =>                        (0x0002 as uint16_t);)            /*<Clock Polarity */
pub macro_rules! SPI_CR1_MSTR (() =>                        (0x0004 as uint16_t);)            /*<Master Selection */

pub macro_rules! SPI_CR1_BR (() =>                          (0x0038 as uint16_t);)            /*<BR[2:0] bits (Baud Rate Control) */
pub macro_rules! SPI_CR1_BR_0 (() =>                        (0x0008 as uint16_t);)            /*<Bit 0 */
pub macro_rules! SPI_CR1_BR_1 (() =>                        (0x0010 as uint16_t);)            /*<Bit 1 */
pub macro_rules! SPI_CR1_BR_2 (() =>                        (0x0020 as uint16_t);)            /*<Bit 2 */

pub macro_rules! SPI_CR1_SPE (() =>                         (0x0040 as uint16_t);)            /*<SPI Enable */
pub macro_rules! SPI_CR1_LSBFIRST (() =>                    (0x0080 as uint16_t);)            /*<Frame Format */
pub macro_rules! SPI_CR1_SSI (() =>                         (0x0100 as uint16_t);)            /*<Internal slave select */
pub macro_rules! SPI_CR1_SSM (() =>                         (0x0200 as uint16_t);)            /*<Software slave management */
pub macro_rules! SPI_CR1_RXONLY (() =>                      (0x0400 as uint16_t);)            /*<Receive only */
pub macro_rules! SPI_CR1_DFF (() =>                         (0x0800 as uint16_t);)            /*<Data Frame Format */
pub macro_rules! SPI_CR1_CRCNEXT (() =>                     (0x1000 as uint16_t);)            /*<Transmit CRC next */
pub macro_rules! SPI_CR1_CRCEN (() =>                       (0x2000 as uint16_t);)            /*<Hardware CRC calculation enable */
pub macro_rules! SPI_CR1_BIDIOE (() =>                      (0x4000 as uint16_t);)            /*<Output enable in bidirectional mode */
pub macro_rules! SPI_CR1_BIDIMODE (() =>                    (0x8000 as uint16_t);)            /*<Bidirectional data mode enable */

/*  Bit definition for SPI_CR2 register  */
pub macro_rules! SPI_CR2_RXDMAEN (() =>                     (0x01 as uint8_t);)               /*<Rx Buffer DMA Enable */
pub macro_rules! SPI_CR2_TXDMAEN (() =>                     (0x02 as uint8_t);)               /*<Tx Buffer DMA Enable */
pub macro_rules! SPI_CR2_SSOE (() =>                        (0x04 as uint8_t);)               /*<SS Output Enable */
pub macro_rules! SPI_CR2_ERRIE (() =>                       (0x20 as uint8_t);)               /*<Error Interrupt Enable */
pub macro_rules! SPI_CR2_RXNEIE (() =>                      (0x40 as uint8_t);)               /*<RX buffer Not Empty Interrupt Enable */
pub macro_rules! SPI_CR2_TXEIE (() =>                       (0x80 as uint8_t);)               /*<Tx buffer Empty Interrupt Enable */

/*  Bit definition for SPI_SR register  */
pub macro_rules! SPI_SR_RXNE (() =>                         (0x01 as uint8_t);)               /*<Receive buffer Not Empty */
pub macro_rules! SPI_SR_TXE (() =>                          (0x02 as uint8_t);)               /*<Transmit buffer Empty */
pub macro_rules! SPI_SR_CHSIDE (() =>                       (0x04 as uint8_t);)               /*<Channel side */
pub macro_rules! SPI_SR_UDR (() =>                          (0x08 as uint8_t);)               /*<Underrun flag */
pub macro_rules! SPI_SR_CRCERR (() =>                       (0x10 as uint8_t);)               /*<CRC Error flag */
pub macro_rules! SPI_SR_MODF (() =>                         (0x20 as uint8_t);)               /*<Mode fault */
pub macro_rules! SPI_SR_OVR (() =>                          (0x40 as uint8_t);)               /*<Overrun flag */
pub macro_rules! SPI_SR_BSY (() =>                          (0x80 as uint8_t);)               /*<Busy flag */

/*  Bit definition for SPI_DR register  */
pub macro_rules! SPI_DR_DR (() =>                           (0xFFFF as uint16_t);)            /*<Data Register */

/*  Bit definition for SPI_CRCPR register  */
pub macro_rules! SPI_CRCPR_CRCPOLY (() =>                   (0xFFFF as uint16_t);)            /*<CRC polynomial register */

/*  Bit definition for SPI_RXCRCR register  */
pub macro_rules! SPI_RXCRCR_RXCRC (() =>                    (0xFFFF as uint16_t);)            /*<Rx CRC Register */

/*  Bit definition for SPI_TXCRCR register  */
pub macro_rules! SPI_TXCRCR_TXCRC (() =>                    (0xFFFF as uint16_t);)            /*<Tx CRC Register */

/*  Bit definition for SPI_I2SCFGR register  */
pub macro_rules! SPI_I2SCFGR_CHLEN (() =>                   (0x0001 as uint16_t);)            /*<Channel length (number of bits per audio channel) */

pub macro_rules! SPI_I2SCFGR_DATLEN (() =>                  (0x0006 as uint16_t);)            /*<DATLEN[1:0] bits (Data length to be transferred) */
pub macro_rules! SPI_I2SCFGR_DATLEN_0 (() =>                (0x0002 as uint16_t);)            /*<Bit 0 */
pub macro_rules! SPI_I2SCFGR_DATLEN_1 (() =>                (0x0004 as uint16_t);)            /*<Bit 1 */

pub macro_rules! SPI_I2SCFGR_CKPOL (() =>                   (0x0008 as uint16_t);)            /*<steady state clock polarity */

pub macro_rules! SPI_I2SCFGR_I2SSTD (() =>                  (0x0030 as uint16_t);)            /*<I2SSTD[1:0] bits (I2S standard selection) */
pub macro_rules! SPI_I2SCFGR_I2SSTD_0 (() =>                (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! SPI_I2SCFGR_I2SSTD_1 (() =>                (0x0020 as uint16_t);)            /*<Bit 1 */

pub macro_rules! SPI_I2SCFGR_PCMSYNC (() =>                 (0x0080 as uint16_t);)            /*<PCM frame synchronization */

pub macro_rules! SPI_I2SCFGR_I2SCFG (() =>                  (0x0300 as uint16_t);)            /*<I2SCFG[1:0] bits (I2S configuration mode) */
pub macro_rules! SPI_I2SCFGR_I2SCFG_0 (() =>                (0x0100 as uint16_t);)            /*<Bit 0 */
pub macro_rules! SPI_I2SCFGR_I2SCFG_1 (() =>                (0x0200 as uint16_t);)            /*<Bit 1 */

pub macro_rules! SPI_I2SCFGR_I2SE (() =>                    (0x0400 as uint16_t);)            /*<I2S Enable */
pub macro_rules! SPI_I2SCFGR_I2SMOD (() =>                  (0x0800 as uint16_t);)            /*<I2S mode selection */

/*  Bit definition for SPI_I2SPR register  */
pub macro_rules! SPI_I2SPR_I2SDIV (() =>                    (0x00FF as uint16_t);)            /*<I2S Linear prescaler */
pub macro_rules! SPI_I2SPR_ODD (() =>                       (0x0100 as uint16_t);)            /*<Odd factor for the prescaler */
pub macro_rules! SPI_I2SPR_MCKOE (() =>                     (0x0200 as uint16_t);)            /*<Master Clock Output Enable */


/*                                                                            */
/*                                 SYSCFG                                     */
/*                                                                            */

/*  Bit definition for SYSCFG_MEMRMP register  */  
pub macro_rules! SYSCFG_MEMRMP_MEM_MODE (() =>          (0x00000003 as uint32_t);) /*<SYSCFG_Memory Remap Config */
pub macro_rules! SYSCFG_MEMRMP_MEM_MODE_0 (() =>        (0x00000001 as uint32_t);)
pub macro_rules! SYSCFG_MEMRMP_MEM_MODE_1 (() =>        (0x00000002 as uint32_t);)

/*  Bit definition for SYSCFG_PMC register  */
pub macro_rules! SYSCFG_PMC_MII_RMII_SEL (() =>         (0x00800000 as uint32_t);) /*<Ethernet PHY interface selection */
/* Old MII_RMII_SEL bit definition, maintained for legacy purpose */
pub macro_rules! SYSCFG_PMC_MII_RMII (() =>             (SYSCFG_PMC_MII_RMII_SEL!());)

/*  Bit definition for SYSCFG_EXTICR1 register  */
pub macro_rules! SYSCFG_EXTICR1_EXTI0 (() =>            (0x000F as uint16_t);) /*<EXTI 0 configuration */
pub macro_rules! SYSCFG_EXTICR1_EXTI1 (() =>            (0x00F0 as uint16_t);) /*<EXTI 1 configuration */
pub macro_rules! SYSCFG_EXTICR1_EXTI2 (() =>            (0x0F00 as uint16_t);) /*<EXTI 2 configuration */
pub macro_rules! SYSCFG_EXTICR1_EXTI3 (() =>            (0xF000 as uint16_t);) /*<EXTI 3 configuration */
/* 
  * EXTI0 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PA (() =>         (0x0000 as uint16_t);) /*<PA[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PB (() =>         (0x0001 as uint16_t);) /*<PB[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PC (() =>         (0x0002 as uint16_t);) /*<PC[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PD (() =>         (0x0003 as uint16_t);) /*<PD[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PE (() =>         (0x0004 as uint16_t);) /*<PE[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PF (() =>         (0x0005 as uint16_t);) /*<PF[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PG (() =>         (0x0006 as uint16_t);) /*<PG[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PH (() =>         (0x0007 as uint16_t);) /*<PH[0] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI0_PI (() =>         (0x0008 as uint16_t);) /*<PI[0] pin */
/* 
  * EXTI1 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PA (() =>         (0x0000 as uint16_t);) /*<PA[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PB (() =>         (0x0010 as uint16_t);) /*<PB[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PC (() =>         (0x0020 as uint16_t);) /*<PC[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PD (() =>         (0x0030 as uint16_t);) /*<PD[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PE (() =>         (0x0040 as uint16_t);) /*<PE[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PF (() =>         (0x0050 as uint16_t);) /*<PF[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PG (() =>         (0x0060 as uint16_t);) /*<PG[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PH (() =>         (0x0070 as uint16_t);) /*<PH[1] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI1_PI (() =>         (0x0080 as uint16_t);) /*<PI[1] pin */
/* 
  * EXTI2 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PA (() =>         (0x0000 as uint16_t);) /*<PA[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PB (() =>         (0x0100 as uint16_t);) /*<PB[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PC (() =>         (0x0200 as uint16_t);) /*<PC[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PD (() =>         (0x0300 as uint16_t);) /*<PD[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PE (() =>         (0x0400 as uint16_t);) /*<PE[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PF (() =>         (0x0500 as uint16_t);) /*<PF[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PG (() =>         (0x0600 as uint16_t);) /*<PG[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PH (() =>         (0x0700 as uint16_t);) /*<PH[2] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI2_PI (() =>         (0x0800 as uint16_t);) /*<PI[2] pin */
/* 
  * EXTI3 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PA (() =>         (0x0000 as uint16_t);) /*<PA[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PB (() =>         (0x1000 as uint16_t);) /*<PB[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PC (() =>         (0x2000 as uint16_t);) /*<PC[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PD (() =>         (0x3000 as uint16_t);) /*<PD[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PE (() =>         (0x4000 as uint16_t);) /*<PE[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PF (() =>         (0x5000 as uint16_t);) /*<PF[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PG (() =>         (0x6000 as uint16_t);) /*<PG[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PH (() =>         (0x7000 as uint16_t);) /*<PH[3] pin */
pub macro_rules! SYSCFG_EXTICR1_EXTI3_PI (() =>         (0x8000 as uint16_t);) /*<PI[3] pin */

/*  Bit definition for SYSCFG_EXTICR2 register  */
pub macro_rules! SYSCFG_EXTICR2_EXTI4 (() =>            (0x000F as uint16_t);) /*<EXTI 4 configuration */
pub macro_rules! SYSCFG_EXTICR2_EXTI5 (() =>            (0x00F0 as uint16_t);) /*<EXTI 5 configuration */
pub macro_rules! SYSCFG_EXTICR2_EXTI6 (() =>            (0x0F00 as uint16_t);) /*<EXTI 6 configuration */
pub macro_rules! SYSCFG_EXTICR2_EXTI7 (() =>            (0xF000 as uint16_t);) /*<EXTI 7 configuration */
/* 
  * EXTI4 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PA (() =>         (0x0000 as uint16_t);) /*<PA[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PB (() =>         (0x0001 as uint16_t);) /*<PB[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PC (() =>         (0x0002 as uint16_t);) /*<PC[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PD (() =>         (0x0003 as uint16_t);) /*<PD[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PE (() =>         (0x0004 as uint16_t);) /*<PE[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PF (() =>         (0x0005 as uint16_t);) /*<PF[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PG (() =>         (0x0006 as uint16_t);) /*<PG[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PH (() =>         (0x0007 as uint16_t);) /*<PH[4] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI4_PI (() =>         (0x0008 as uint16_t);) /*<PI[4] pin */
/* 
  * EXTI5 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PA (() =>         (0x0000 as uint16_t);) /*<PA[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PB (() =>         (0x0010 as uint16_t);) /*<PB[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PC (() =>         (0x0020 as uint16_t);) /*<PC[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PD (() =>         (0x0030 as uint16_t);) /*<PD[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PE (() =>         (0x0040 as uint16_t);) /*<PE[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PF (() =>         (0x0050 as uint16_t);) /*<PF[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PG (() =>         (0x0060 as uint16_t);) /*<PG[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PH (() =>         (0x0070 as uint16_t);) /*<PH[5] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI5_PI (() =>         (0x0080 as uint16_t);) /*<PI[5] pin */
/* 
  * EXTI6 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PA (() =>         (0x0000 as uint16_t);) /*<PA[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PB (() =>         (0x0100 as uint16_t);) /*<PB[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PC (() =>         (0x0200 as uint16_t);) /*<PC[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PD (() =>         (0x0300 as uint16_t);) /*<PD[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PE (() =>         (0x0400 as uint16_t);) /*<PE[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PF (() =>         (0x0500 as uint16_t);) /*<PF[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PG (() =>         (0x0600 as uint16_t);) /*<PG[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PH (() =>         (0x0700 as uint16_t);) /*<PH[6] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI6_PI (() =>         (0x0800 as uint16_t);) /*<PI[6] pin */
/* 
  * EXTI7 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PA (() =>         (0x0000 as uint16_t);) /*<PA[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PB (() =>         (0x1000 as uint16_t);) /*<PB[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PC (() =>         (0x2000 as uint16_t);) /*<PC[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PD (() =>         (0x3000 as uint16_t);) /*<PD[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PE (() =>         (0x4000 as uint16_t);) /*<PE[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PF (() =>         (0x5000 as uint16_t);) /*<PF[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PG (() =>         (0x6000 as uint16_t);) /*<PG[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PH (() =>         (0x7000 as uint16_t);) /*<PH[7] pin */
pub macro_rules! SYSCFG_EXTICR2_EXTI7_PI (() =>         (0x8000 as uint16_t);) /*<PI[7] pin */

/*  Bit definition for SYSCFG_EXTICR3 register  */
pub macro_rules! SYSCFG_EXTICR3_EXTI8 (() =>            (0x000F as uint16_t);) /*<EXTI 8 configuration */
pub macro_rules! SYSCFG_EXTICR3_EXTI9 (() =>            (0x00F0 as uint16_t);) /*<EXTI 9 configuration */
pub macro_rules! SYSCFG_EXTICR3_EXTI10 (() =>           (0x0F00 as uint16_t);) /*<EXTI 10 configuration */
pub macro_rules! SYSCFG_EXTICR3_EXTI11 (() =>           (0xF000 as uint16_t);) /*<EXTI 11 configuration */
           
/* 
  * EXTI8 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PA (() =>         (0x0000 as uint16_t);) /*<PA[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PB (() =>         (0x0001 as uint16_t);) /*<PB[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PC (() =>         (0x0002 as uint16_t);) /*<PC[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PD (() =>         (0x0003 as uint16_t);) /*<PD[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PE (() =>         (0x0004 as uint16_t);) /*<PE[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PF (() =>         (0x0005 as uint16_t);) /*<PF[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PG (() =>         (0x0006 as uint16_t);) /*<PG[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PH (() =>         (0x0007 as uint16_t);) /*<PH[8] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI8_PI (() =>         (0x0008 as uint16_t);) /*<PI[8] pin */
/* 
  * EXTI9 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PA (() =>         (0x0000 as uint16_t);) /*<PA[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PB (() =>         (0x0010 as uint16_t);) /*<PB[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PC (() =>         (0x0020 as uint16_t);) /*<PC[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PD (() =>         (0x0030 as uint16_t);) /*<PD[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PE (() =>         (0x0040 as uint16_t);) /*<PE[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PF (() =>         (0x0050 as uint16_t);) /*<PF[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PG (() =>         (0x0060 as uint16_t);) /*<PG[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PH (() =>         (0x0070 as uint16_t);) /*<PH[9] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI9_PI (() =>         (0x0080 as uint16_t);) /*<PI[9] pin */
/* 
  * EXTI10 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PA (() =>        (0x0000 as uint16_t);) /*<PA[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PB (() =>        (0x0100 as uint16_t);) /*<PB[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PC (() =>        (0x0200 as uint16_t);) /*<PC[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PD (() =>        (0x0300 as uint16_t);) /*<PD[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PE (() =>        (0x0400 as uint16_t);) /*<PE[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PF (() =>        (0x0500 as uint16_t);) /*<PF[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PG (() =>        (0x0600 as uint16_t);) /*<PG[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PH (() =>        (0x0700 as uint16_t);) /*<PH[10] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI10_PI (() =>        (0x0800 as uint16_t);) /*<PI[10] pin */
/* 
  * EXTI11 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PA (() =>        (0x0000 as uint16_t);) /*<PA[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PB (() =>        (0x1000 as uint16_t);) /*<PB[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PC (() =>        (0x2000 as uint16_t);) /*<PC[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PD (() =>        (0x3000 as uint16_t);) /*<PD[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PE (() =>        (0x4000 as uint16_t);) /*<PE[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PF (() =>        (0x5000 as uint16_t);) /*<PF[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PG (() =>        (0x6000 as uint16_t);) /*<PG[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PH (() =>        (0x7000 as uint16_t);) /*<PH[11] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI11_PI (() =>        (0x8000 as uint16_t);) /*<PI[11] pin */

/*  Bit definition for SYSCFG_EXTICR4 register  */
pub macro_rules! SYSCFG_EXTICR4_EXTI12 (() =>           (0x000F as uint16_t);) /*<EXTI 12 configuration */
pub macro_rules! SYSCFG_EXTICR4_EXTI13 (() =>           (0x00F0 as uint16_t);) /*<EXTI 13 configuration */
pub macro_rules! SYSCFG_EXTICR4_EXTI14 (() =>           (0x0F00 as uint16_t);) /*<EXTI 14 configuration */
pub macro_rules! SYSCFG_EXTICR4_EXTI15 (() =>           (0xF000 as uint16_t);) /*<EXTI 15 configuration */
/* 
  * EXTI12 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR4_EXTI12_PA (() =>        (0x0000 as uint16_t);) /*<PA[12] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI12_PB (() =>        (0x0001 as uint16_t);) /*<PB[12] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI12_PC (() =>        (0x0002 as uint16_t);) /*<PC[12] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI12_PD (() =>        (0x0003 as uint16_t);) /*<PD[12] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI12_PE (() =>        (0x0004 as uint16_t);) /*<PE[12] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI12_PF (() =>        (0x0005 as uint16_t);) /*<PF[12] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI12_PG (() =>        (0x0006 as uint16_t);) /*<PG[12] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI12_PH (() =>        (0x0007 as uint16_t);) /*<PH[12] pin */
/* 
  * EXTI13 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR4_EXTI13_PA (() =>        (0x0000 as uint16_t);) /*<PA[13] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI13_PB (() =>        (0x0010 as uint16_t);) /*<PB[13] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI13_PC (() =>        (0x0020 as uint16_t);) /*<PC[13] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI13_PD (() =>        (0x0030 as uint16_t);) /*<PD[13] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI13_PE (() =>        (0x0040 as uint16_t);) /*<PE[13] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI13_PF (() =>        (0x0050 as uint16_t);) /*<PF[13] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI13_PG (() =>        (0x0060 as uint16_t);) /*<PG[13] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI13_PH (() =>        (0x0070 as uint16_t);) /*<PH[13] pin */
/* 
  * EXTI14 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR4_EXTI14_PA (() =>        (0x0000 as uint16_t);) /*<PA[14] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI14_PB (() =>        (0x0100 as uint16_t);) /*<PB[14] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI14_PC (() =>        (0x0200 as uint16_t);) /*<PC[14] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI14_PD (() =>        (0x0300 as uint16_t);) /*<PD[14] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI14_PE (() =>        (0x0400 as uint16_t);) /*<PE[14] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI14_PF (() =>        (0x0500 as uint16_t);) /*<PF[14] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI14_PG (() =>        (0x0600 as uint16_t);) /*<PG[14] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI14_PH (() =>        (0x0700 as uint16_t);) /*<PH[14] pin */
/* 
  * EXTI15 configuration  
  */ 
pub macro_rules! SYSCFG_EXTICR4_EXTI15_PA (() =>        (0x0000 as uint16_t);) /*<PA[15] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI15_PB (() =>        (0x1000 as uint16_t);) /*<PB[15] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI15_PC (() =>        (0x2000 as uint16_t);) /*<PC[15] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI15_PD (() =>        (0x3000 as uint16_t);) /*<PD[15] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI15_PE (() =>        (0x4000 as uint16_t);) /*<PE[15] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI15_PF (() =>        (0x5000 as uint16_t);) /*<PF[15] pin */
pub macro_rules! SYSCFG_EXTICR4_EXTI15_PG (() =>        (0x6000 as uint16_t);) /*<PG[15] pin */
pub macro_rules! SYSCFG_EXTICR3_EXTI15_PH (() =>        (0x7000 as uint16_t);) /*<PH[15] pin */

/*  Bit definition for SYSCFG_CMPCR register  */  
pub macro_rules! SYSCFG_CMPCR_CMP_PD (() =>             (0x00000001 as uint32_t);) /*<Compensation cell ready flag */
pub macro_rules! SYSCFG_CMPCR_READY (() =>              (0x00000100 as uint32_t);) /*<Compensation cell power-down */


/*                                                                            */
/*                                    TIM                                     */
/*                                                                            */

/*  Bit definition for TIM_CR1 register  */
pub macro_rules! TIM_CR1_CEN (() =>                         (0x0001 as uint16_t);)            /*<Counter enable */
pub macro_rules! TIM_CR1_UDIS (() =>                        (0x0002 as uint16_t);)            /*<Update disable */
pub macro_rules! TIM_CR1_URS (() =>                         (0x0004 as uint16_t);)            /*<Update request source */
pub macro_rules! TIM_CR1_OPM (() =>                         (0x0008 as uint16_t);)            /*<One pulse mode */
pub macro_rules! TIM_CR1_DIR (() =>                         (0x0010 as uint16_t);)            /*<Direction */

pub macro_rules! TIM_CR1_CMS (() =>                         (0x0060 as uint16_t);)            /*<CMS[1:0] bits (Center-aligned mode selection) */
pub macro_rules! TIM_CR1_CMS_0 (() =>                       (0x0020 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CR1_CMS_1 (() =>                       (0x0040 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CR1_ARPE (() =>                        (0x0080 as uint16_t);)            /*<Auto-reload preload enable */

pub macro_rules! TIM_CR1_CKD (() =>                         (0x0300 as uint16_t);)            /*<CKD[1:0] bits (clock division) */
pub macro_rules! TIM_CR1_CKD_0 (() =>                       (0x0100 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CR1_CKD_1 (() =>                       (0x0200 as uint16_t);)            /*<Bit 1 */

/*  Bit definition for TIM_CR2 register  */
pub macro_rules! TIM_CR2_CCPC (() =>                        (0x0001 as uint16_t);)            /*<Capture/Compare Preloaded Control */
pub macro_rules! TIM_CR2_CCUS (() =>                        (0x0004 as uint16_t);)            /*<Capture/Compare Control Update Selection */
pub macro_rules! TIM_CR2_CCDS (() =>                        (0x0008 as uint16_t);)            /*<Capture/Compare DMA Selection */

pub macro_rules! TIM_CR2_MMS (() =>                         (0x0070 as uint16_t);)            /*<MMS[2:0] bits (Master Mode Selection) */
pub macro_rules! TIM_CR2_MMS_0 (() =>                       (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CR2_MMS_1 (() =>                       (0x0020 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CR2_MMS_2 (() =>                       (0x0040 as uint16_t);)            /*<Bit 2 */

pub macro_rules! TIM_CR2_TI1S (() =>                        (0x0080 as uint16_t);)            /*<TI1 Selection */
pub macro_rules! TIM_CR2_OIS1 (() =>                        (0x0100 as uint16_t);)            /*<Output Idle state 1 (OC1 output) */
pub macro_rules! TIM_CR2_OIS1N (() =>                       (0x0200 as uint16_t);)            /*<Output Idle state 1 (OC1N output) */
pub macro_rules! TIM_CR2_OIS2 (() =>                        (0x0400 as uint16_t);)            /*<Output Idle state 2 (OC2 output) */
pub macro_rules! TIM_CR2_OIS2N (() =>                       (0x0800 as uint16_t);)            /*<Output Idle state 2 (OC2N output) */
pub macro_rules! TIM_CR2_OIS3 (() =>                        (0x1000 as uint16_t);)            /*<Output Idle state 3 (OC3 output) */
pub macro_rules! TIM_CR2_OIS3N (() =>                       (0x2000 as uint16_t);)            /*<Output Idle state 3 (OC3N output) */
pub macro_rules! TIM_CR2_OIS4 (() =>                        (0x4000 as uint16_t);)            /*<Output Idle state 4 (OC4 output) */

/*  Bit definition for TIM_SMCR register  */
pub macro_rules! TIM_SMCR_SMS (() =>                        (0x0007 as uint16_t);)            /*<SMS[2:0] bits (Slave mode selection) */
pub macro_rules! TIM_SMCR_SMS_0 (() =>                      (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_SMCR_SMS_1 (() =>                      (0x0002 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_SMCR_SMS_2 (() =>                      (0x0004 as uint16_t);)            /*<Bit 2 */

pub macro_rules! TIM_SMCR_TS (() =>                         (0x0070 as uint16_t);)            /*<TS[2:0] bits (Trigger selection) */
pub macro_rules! TIM_SMCR_TS_0 (() =>                       (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_SMCR_TS_1 (() =>                       (0x0020 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_SMCR_TS_2 (() =>                       (0x0040 as uint16_t);)            /*<Bit 2 */

pub macro_rules! TIM_SMCR_MSM (() =>                        (0x0080 as uint16_t);)            /*<Master/slave mode */

pub macro_rules! TIM_SMCR_ETF (() =>                        (0x0F00 as uint16_t);)            /*<ETF[3:0] bits (External trigger filter) */
pub macro_rules! TIM_SMCR_ETF_0 (() =>                      (0x0100 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_SMCR_ETF_1 (() =>                      (0x0200 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_SMCR_ETF_2 (() =>                      (0x0400 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_SMCR_ETF_3 (() =>                      (0x0800 as uint16_t);)            /*<Bit 3 */

pub macro_rules! TIM_SMCR_ETPS (() =>                       (0x3000 as uint16_t);)            /*<ETPS[1:0] bits (External trigger prescaler) */
pub macro_rules! TIM_SMCR_ETPS_0 (() =>                     (0x1000 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_SMCR_ETPS_1 (() =>                     (0x2000 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_SMCR_ECE (() =>                        (0x4000 as uint16_t);)            /*<External clock enable */
pub macro_rules! TIM_SMCR_ETP (() =>                        (0x8000 as uint16_t);)            /*<External trigger polarity */

/*  Bit definition for TIM_DIER register  */
pub macro_rules! TIM_DIER_UIE (() =>                        (0x0001 as uint16_t);)            /*<Update interrupt enable */
pub macro_rules! TIM_DIER_CC1IE (() =>                      (0x0002 as uint16_t);)            /*<Capture/Compare 1 interrupt enable */
pub macro_rules! TIM_DIER_CC2IE (() =>                      (0x0004 as uint16_t);)            /*<Capture/Compare 2 interrupt enable */
pub macro_rules! TIM_DIER_CC3IE (() =>                      (0x0008 as uint16_t);)            /*<Capture/Compare 3 interrupt enable */
pub macro_rules! TIM_DIER_CC4IE (() =>                      (0x0010 as uint16_t);)            /*<Capture/Compare 4 interrupt enable */
pub macro_rules! TIM_DIER_COMIE (() =>                      (0x0020 as uint16_t);)            /*<COM interrupt enable */
pub macro_rules! TIM_DIER_TIE (() =>                        (0x0040 as uint16_t);)            /*<Trigger interrupt enable */
pub macro_rules! TIM_DIER_BIE (() =>                        (0x0080 as uint16_t);)            /*<Break interrupt enable */
pub macro_rules! TIM_DIER_UDE (() =>                        (0x0100 as uint16_t);)            /*<Update DMA request enable */
pub macro_rules! TIM_DIER_CC1DE (() =>                      (0x0200 as uint16_t);)            /*<Capture/Compare 1 DMA request enable */
pub macro_rules! TIM_DIER_CC2DE (() =>                      (0x0400 as uint16_t);)            /*<Capture/Compare 2 DMA request enable */
pub macro_rules! TIM_DIER_CC3DE (() =>                      (0x0800 as uint16_t);)            /*<Capture/Compare 3 DMA request enable */
pub macro_rules! TIM_DIER_CC4DE (() =>                      (0x1000 as uint16_t);)            /*<Capture/Compare 4 DMA request enable */
pub macro_rules! TIM_DIER_COMDE (() =>                      (0x2000 as uint16_t);)            /*<COM DMA request enable */
pub macro_rules! TIM_DIER_TDE (() =>                        (0x4000 as uint16_t);)            /*<Trigger DMA request enable */

/*  Bit definition for TIM_SR register  */
pub macro_rules! TIM_SR_UIF (() =>                          (0x0001 as uint16_t);)            /*<Update interrupt Flag */
pub macro_rules! TIM_SR_CC1IF (() =>                        (0x0002 as uint16_t);)            /*<Capture/Compare 1 interrupt Flag */
pub macro_rules! TIM_SR_CC2IF (() =>                        (0x0004 as uint16_t);)            /*<Capture/Compare 2 interrupt Flag */
pub macro_rules! TIM_SR_CC3IF (() =>                        (0x0008 as uint16_t);)            /*<Capture/Compare 3 interrupt Flag */
pub macro_rules! TIM_SR_CC4IF (() =>                        (0x0010 as uint16_t);)            /*<Capture/Compare 4 interrupt Flag */
pub macro_rules! TIM_SR_COMIF (() =>                        (0x0020 as uint16_t);)            /*<COM interrupt Flag */
pub macro_rules! TIM_SR_TIF (() =>                          (0x0040 as uint16_t);)            /*<Trigger interrupt Flag */
pub macro_rules! TIM_SR_BIF (() =>                          (0x0080 as uint16_t);)            /*<Break interrupt Flag */
pub macro_rules! TIM_SR_CC1OF (() =>                        (0x0200 as uint16_t);)            /*<Capture/Compare 1 Overcapture Flag */
pub macro_rules! TIM_SR_CC2OF (() =>                        (0x0400 as uint16_t);)            /*<Capture/Compare 2 Overcapture Flag */
pub macro_rules! TIM_SR_CC3OF (() =>                        (0x0800 as uint16_t);)            /*<Capture/Compare 3 Overcapture Flag */
pub macro_rules! TIM_SR_CC4OF (() =>                        (0x1000 as uint16_t);)            /*<Capture/Compare 4 Overcapture Flag */

/*  Bit definition for TIM_EGR register  */
pub macro_rules! TIM_EGR_UG (() =>                          (0x01 as uint8_t);)               /*<Update Generation */
pub macro_rules! TIM_EGR_CC1G (() =>                        (0x02 as uint8_t);)               /*<Capture/Compare 1 Generation */
pub macro_rules! TIM_EGR_CC2G (() =>                        (0x04 as uint8_t);)               /*<Capture/Compare 2 Generation */
pub macro_rules! TIM_EGR_CC3G (() =>                        (0x08 as uint8_t);)               /*<Capture/Compare 3 Generation */
pub macro_rules! TIM_EGR_CC4G (() =>                        (0x10 as uint8_t);)               /*<Capture/Compare 4 Generation */
pub macro_rules! TIM_EGR_COMG (() =>                        (0x20 as uint8_t);)               /*<Capture/Compare Control Update Generation */
pub macro_rules! TIM_EGR_TG (() =>                          (0x40 as uint8_t);)               /*<Trigger Generation */
pub macro_rules! TIM_EGR_BG (() =>                          (0x80 as uint8_t);)               /*<Break Generation */

/*  Bit definition for TIM_CCMR1 register  */
pub macro_rules! TIM_CCMR1_CC1S (() =>                      (0x0003 as uint16_t);)            /*<CC1S[1:0] bits (Capture/Compare 1 Selection) */
pub macro_rules! TIM_CCMR1_CC1S_0 (() =>                    (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_CC1S_1 (() =>                    (0x0002 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR1_OC1FE (() =>                     (0x0004 as uint16_t);)            /*<Output Compare 1 Fast enable */
pub macro_rules! TIM_CCMR1_OC1PE (() =>                     (0x0008 as uint16_t);)            /*<Output Compare 1 Preload enable */

pub macro_rules! TIM_CCMR1_OC1M (() =>                      (0x0070 as uint16_t);)            /*<OC1M[2:0] bits (Output Compare 1 Mode) */
pub macro_rules! TIM_CCMR1_OC1M_0 (() =>                    (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_OC1M_1 (() =>                    (0x0020 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR1_OC1M_2 (() =>                    (0x0040 as uint16_t);)            /*<Bit 2 */

pub macro_rules! TIM_CCMR1_OC1CE (() =>                     (0x0080 as uint16_t);)            /*<Output Compare 1Clear Enable */

pub macro_rules! TIM_CCMR1_CC2S (() =>                      (0x0300 as uint16_t);)            /*<CC2S[1:0] bits (Capture/Compare 2 Selection) */
pub macro_rules! TIM_CCMR1_CC2S_0 (() =>                    (0x0100 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_CC2S_1 (() =>                    (0x0200 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR1_OC2FE (() =>                     (0x0400 as uint16_t);)            /*<Output Compare 2 Fast enable */
pub macro_rules! TIM_CCMR1_OC2PE (() =>                     (0x0800 as uint16_t);)            /*<Output Compare 2 Preload enable */

pub macro_rules! TIM_CCMR1_OC2M (() =>                      (0x7000 as uint16_t);)            /*<OC2M[2:0] bits (Output Compare 2 Mode) */
pub macro_rules! TIM_CCMR1_OC2M_0 (() =>                    (0x1000 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_OC2M_1 (() =>                    (0x2000 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR1_OC2M_2 (() =>                    (0x4000 as uint16_t);)            /*<Bit 2 */

pub macro_rules! TIM_CCMR1_OC2CE (() =>                     (0x8000 as uint16_t);)            /*<Output Compare 2 Clear Enable */

/*----------------------------------------------------------------------------*/

pub macro_rules! TIM_CCMR1_IC1PSC (() =>                    (0x000C as uint16_t);)            /*<IC1PSC[1:0] bits (Input Capture 1 Prescaler) */
pub macro_rules! TIM_CCMR1_IC1PSC_0 (() =>                  (0x0004 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_IC1PSC_1 (() =>                  (0x0008 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR1_IC1F (() =>                      (0x00F0 as uint16_t);)            /*<IC1F[3:0] bits (Input Capture 1 Filter) */
pub macro_rules! TIM_CCMR1_IC1F_0 (() =>                    (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_IC1F_1 (() =>                    (0x0020 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR1_IC1F_2 (() =>                    (0x0040 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_CCMR1_IC1F_3 (() =>                    (0x0080 as uint16_t);)            /*<Bit 3 */

pub macro_rules! TIM_CCMR1_IC2PSC (() =>                    (0x0C00 as uint16_t);)            /*<IC2PSC[1:0] bits (Input Capture 2 Prescaler) */
pub macro_rules! TIM_CCMR1_IC2PSC_0 (() =>                  (0x0400 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_IC2PSC_1 (() =>                  (0x0800 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR1_IC2F (() =>                      (0xF000 as uint16_t);)            /*<IC2F[3:0] bits (Input Capture 2 Filter) */
pub macro_rules! TIM_CCMR1_IC2F_0 (() =>                    (0x1000 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR1_IC2F_1 (() =>                    (0x2000 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR1_IC2F_2 (() =>                    (0x4000 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_CCMR1_IC2F_3 (() =>                    (0x8000 as uint16_t);)            /*<Bit 3 */

/*  Bit definition for TIM_CCMR2 register  */
pub macro_rules! TIM_CCMR2_CC3S (() =>                      (0x0003 as uint16_t);)            /*<CC3S[1:0] bits (Capture/Compare 3 Selection) */
pub macro_rules! TIM_CCMR2_CC3S_0 (() =>                    (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_CC3S_1 (() =>                    (0x0002 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR2_OC3FE (() =>                     (0x0004 as uint16_t);)            /*<Output Compare 3 Fast enable */
pub macro_rules! TIM_CCMR2_OC3PE (() =>                     (0x0008 as uint16_t);)            /*<Output Compare 3 Preload enable */

pub macro_rules! TIM_CCMR2_OC3M (() =>                      (0x0070 as uint16_t);)            /*<OC3M[2:0] bits (Output Compare 3 Mode) */
pub macro_rules! TIM_CCMR2_OC3M_0 (() =>                    (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_OC3M_1 (() =>                    (0x0020 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR2_OC3M_2 (() =>                    (0x0040 as uint16_t);)            /*<Bit 2 */

pub macro_rules! TIM_CCMR2_OC3CE (() =>                     (0x0080 as uint16_t);)            /*<Output Compare 3 Clear Enable */

pub macro_rules! TIM_CCMR2_CC4S (() =>                      (0x0300 as uint16_t);)            /*<CC4S[1:0] bits (Capture/Compare 4 Selection) */
pub macro_rules! TIM_CCMR2_CC4S_0 (() =>                    (0x0100 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_CC4S_1 (() =>                    (0x0200 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR2_OC4FE (() =>                     (0x0400 as uint16_t);)            /*<Output Compare 4 Fast enable */
pub macro_rules! TIM_CCMR2_OC4PE (() =>                     (0x0800 as uint16_t);)            /*<Output Compare 4 Preload enable */

pub macro_rules! TIM_CCMR2_OC4M (() =>                      (0x7000 as uint16_t);)            /*<OC4M[2:0] bits (Output Compare 4 Mode) */
pub macro_rules! TIM_CCMR2_OC4M_0 (() =>                    (0x1000 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_OC4M_1 (() =>                    (0x2000 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR2_OC4M_2 (() =>                    (0x4000 as uint16_t);)            /*<Bit 2 */

pub macro_rules! TIM_CCMR2_OC4CE (() =>                     (0x8000 as uint16_t);)            /*<Output Compare 4 Clear Enable */

/*----------------------------------------------------------------------------*/

pub macro_rules! TIM_CCMR2_IC3PSC (() =>                    (0x000C as uint16_t);)            /*<IC3PSC[1:0] bits (Input Capture 3 Prescaler) */
pub macro_rules! TIM_CCMR2_IC3PSC_0 (() =>                  (0x0004 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_IC3PSC_1 (() =>                  (0x0008 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR2_IC3F (() =>                      (0x00F0 as uint16_t);)            /*<IC3F[3:0] bits (Input Capture 3 Filter) */
pub macro_rules! TIM_CCMR2_IC3F_0 (() =>                    (0x0010 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_IC3F_1 (() =>                    (0x0020 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR2_IC3F_2 (() =>                    (0x0040 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_CCMR2_IC3F_3 (() =>                    (0x0080 as uint16_t);)            /*<Bit 3 */

pub macro_rules! TIM_CCMR2_IC4PSC (() =>                    (0x0C00 as uint16_t);)            /*<IC4PSC[1:0] bits (Input Capture 4 Prescaler) */
pub macro_rules! TIM_CCMR2_IC4PSC_0 (() =>                  (0x0400 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_IC4PSC_1 (() =>                  (0x0800 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_CCMR2_IC4F (() =>                      (0xF000 as uint16_t);)            /*<IC4F[3:0] bits (Input Capture 4 Filter) */
pub macro_rules! TIM_CCMR2_IC4F_0 (() =>                    (0x1000 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_CCMR2_IC4F_1 (() =>                    (0x2000 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_CCMR2_IC4F_2 (() =>                    (0x4000 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_CCMR2_IC4F_3 (() =>                    (0x8000 as uint16_t);)            /*<Bit 3 */

/*  Bit definition for TIM_CCER register  */
pub macro_rules! TIM_CCER_CC1E (() =>                       (0x0001 as uint16_t);)            /*<Capture/Compare 1 output enable */
pub macro_rules! TIM_CCER_CC1P (() =>                       (0x0002 as uint16_t);)            /*<Capture/Compare 1 output Polarity */
pub macro_rules! TIM_CCER_CC1NE (() =>                      (0x0004 as uint16_t);)            /*<Capture/Compare 1 Complementary output enable */
pub macro_rules! TIM_CCER_CC1NP (() =>                      (0x0008 as uint16_t);)            /*<Capture/Compare 1 Complementary output Polarity */
pub macro_rules! TIM_CCER_CC2E (() =>                       (0x0010 as uint16_t);)            /*<Capture/Compare 2 output enable */
pub macro_rules! TIM_CCER_CC2P (() =>                       (0x0020 as uint16_t);)            /*<Capture/Compare 2 output Polarity */
pub macro_rules! TIM_CCER_CC2NE (() =>                      (0x0040 as uint16_t);)            /*<Capture/Compare 2 Complementary output enable */
pub macro_rules! TIM_CCER_CC2NP (() =>                      (0x0080 as uint16_t);)            /*<Capture/Compare 2 Complementary output Polarity */
pub macro_rules! TIM_CCER_CC3E (() =>                       (0x0100 as uint16_t);)            /*<Capture/Compare 3 output enable */
pub macro_rules! TIM_CCER_CC3P (() =>                       (0x0200 as uint16_t);)            /*<Capture/Compare 3 output Polarity */
pub macro_rules! TIM_CCER_CC3NE (() =>                      (0x0400 as uint16_t);)            /*<Capture/Compare 3 Complementary output enable */
pub macro_rules! TIM_CCER_CC3NP (() =>                      (0x0800 as uint16_t);)            /*<Capture/Compare 3 Complementary output Polarity */
pub macro_rules! TIM_CCER_CC4E (() =>                       (0x1000 as uint16_t);)            /*<Capture/Compare 4 output enable */
pub macro_rules! TIM_CCER_CC4P (() =>                       (0x2000 as uint16_t);)            /*<Capture/Compare 4 output Polarity */
pub macro_rules! TIM_CCER_CC4NP (() =>                      (0x8000 as uint16_t);)            /*<Capture/Compare 4 Complementary output Polarity */

/*  Bit definition for TIM_CNT register  */
pub macro_rules! TIM_CNT_CNT (() =>                         (0xFFFF as uint16_t);)            /*<Counter Value */

/*  Bit definition for TIM_PSC register  */
pub macro_rules! TIM_PSC_PSC (() =>                         (0xFFFF as uint16_t);)            /*<Prescaler Value */

/*  Bit definition for TIM_ARR register  */
pub macro_rules! TIM_ARR_ARR (() =>                         (0xFFFF as uint16_t);)            /*<actual auto-reload Value */

/*  Bit definition for TIM_RCR register  */
pub macro_rules! TIM_RCR_REP (() =>                         (0xFF as uint8_t);)               /*<Repetition Counter Value */

/*  Bit definition for TIM_CCR1 register  */
pub macro_rules! TIM_CCR1_CCR1 (() =>                       (0xFFFF as uint16_t);)            /*<Capture/Compare 1 Value */

/*  Bit definition for TIM_CCR2 register  */
pub macro_rules! TIM_CCR2_CCR2 (() =>                       (0xFFFF as uint16_t);)            /*<Capture/Compare 2 Value */

/*  Bit definition for TIM_CCR3 register  */
pub macro_rules! TIM_CCR3_CCR3 (() =>                       (0xFFFF as uint16_t);)            /*<Capture/Compare 3 Value */

/*  Bit definition for TIM_CCR4 register  */
pub macro_rules! TIM_CCR4_CCR4 (() =>                       (0xFFFF as uint16_t);)            /*<Capture/Compare 4 Value */

/*  Bit definition for TIM_BDTR register  */
pub macro_rules! TIM_BDTR_DTG (() =>                        (0x00FF as uint16_t);)            /*<DTG[0:7] bits (Dead-Time Generator set-up) */
pub macro_rules! TIM_BDTR_DTG_0 (() =>                      (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_BDTR_DTG_1 (() =>                      (0x0002 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_BDTR_DTG_2 (() =>                      (0x0004 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_BDTR_DTG_3 (() =>                      (0x0008 as uint16_t);)            /*<Bit 3 */
pub macro_rules! TIM_BDTR_DTG_4 (() =>                      (0x0010 as uint16_t);)            /*<Bit 4 */
pub macro_rules! TIM_BDTR_DTG_5 (() =>                      (0x0020 as uint16_t);)            /*<Bit 5 */
pub macro_rules! TIM_BDTR_DTG_6 (() =>                      (0x0040 as uint16_t);)            /*<Bit 6 */
pub macro_rules! TIM_BDTR_DTG_7 (() =>                      (0x0080 as uint16_t);)            /*<Bit 7 */

pub macro_rules! TIM_BDTR_LOCK (() =>                       (0x0300 as uint16_t);)            /*<LOCK[1:0] bits (Lock Configuration) */
pub macro_rules! TIM_BDTR_LOCK_0 (() =>                     (0x0100 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_BDTR_LOCK_1 (() =>                     (0x0200 as uint16_t);)            /*<Bit 1 */

pub macro_rules! TIM_BDTR_OSSI (() =>                       (0x0400 as uint16_t);)            /*<Off-State Selection for Idle mode */
pub macro_rules! TIM_BDTR_OSSR (() =>                       (0x0800 as uint16_t);)            /*<Off-State Selection for Run mode */
pub macro_rules! TIM_BDTR_BKE (() =>                        (0x1000 as uint16_t);)            /*<Break enable */
pub macro_rules! TIM_BDTR_BKP (() =>                        (0x2000 as uint16_t);)            /*<Break Polarity */
pub macro_rules! TIM_BDTR_AOE (() =>                        (0x4000 as uint16_t);)            /*<Automatic Output enable */
pub macro_rules! TIM_BDTR_MOE (() =>                        (0x8000 as uint16_t);)            /*<Main Output enable */

/*  Bit definition for TIM_DCR register  */
pub macro_rules! TIM_DCR_DBA (() =>                         (0x001F as uint16_t);)            /*<DBA[4:0] bits (DMA Base Address) */
pub macro_rules! TIM_DCR_DBA_0 (() =>                       (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_DCR_DBA_1 (() =>                       (0x0002 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_DCR_DBA_2 (() =>                       (0x0004 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_DCR_DBA_3 (() =>                       (0x0008 as uint16_t);)            /*<Bit 3 */
pub macro_rules! TIM_DCR_DBA_4 (() =>                       (0x0010 as uint16_t);)            /*<Bit 4 */

pub macro_rules! TIM_DCR_DBL (() =>                         (0x1F00 as uint16_t);)            /*<DBL[4:0] bits (DMA Burst Length) */
pub macro_rules! TIM_DCR_DBL_0 (() =>                       (0x0100 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_DCR_DBL_1 (() =>                       (0x0200 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_DCR_DBL_2 (() =>                       (0x0400 as uint16_t);)            /*<Bit 2 */
pub macro_rules! TIM_DCR_DBL_3 (() =>                       (0x0800 as uint16_t);)            /*<Bit 3 */
pub macro_rules! TIM_DCR_DBL_4 (() =>                       (0x1000 as uint16_t);)            /*<Bit 4 */

/*  Bit definition for TIM_DMAR register  */
pub macro_rules! TIM_DMAR_DMAB (() =>                       (0xFFFF as uint16_t);)            /*<DMA register for burst accesses */

/*  Bit definition for TIM_OR register  */
pub macro_rules! TIM_OR_TI4_RMP (() =>                       (0x00C0 as uint16_t);)            /*<TI4_RMP[1:0] bits (TIM5 Input 4 remap) */
pub macro_rules! TIM_OR_TI4_RMP_0 (() =>                     (0x0040 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_OR_TI4_RMP_1 (() =>                     (0x0080 as uint16_t);)            /*<Bit 1 */
pub macro_rules! TIM_OR_ITR1_RMP (() =>                      (0x0C00 as uint16_t);)            /*<ITR1_RMP[1:0] bits (TIM2 Internal trigger 1 remap) */
pub macro_rules! TIM_OR_ITR1_RMP_0 (() =>                    (0x0400 as uint16_t);)            /*<Bit 0 */
pub macro_rules! TIM_OR_ITR1_RMP_1 (() =>                    (0x0800 as uint16_t);)            /*<Bit 1 */



/*                                                                            */
/*         Universal Synchronous Asynchronous Receiver Transmitter            */
/*                                                                            */

/*  Bit definition for USART_SR register  */
pub macro_rules! USART_SR_PE (() =>                         (0x0001 as uint16_t);)            /*<Parity Error */
pub macro_rules! USART_SR_FE (() =>                         (0x0002 as uint16_t);)            /*<Framing Error */
pub macro_rules! USART_SR_NE (() =>                         (0x0004 as uint16_t);)            /*<Noise Error Flag */
pub macro_rules! USART_SR_ORE (() =>                        (0x0008 as uint16_t);)            /*<OverRun Error */
pub macro_rules! USART_SR_IDLE (() =>                       (0x0010 as uint16_t);)            /*<IDLE line detected */
pub macro_rules! USART_SR_RXNE (() =>                       (0x0020 as uint16_t);)            /*<Read Data Register Not Empty */
pub macro_rules! USART_SR_TC (() =>                         (0x0040 as uint16_t);)            /*<Transmission Complete */
pub macro_rules! USART_SR_TXE (() =>                        (0x0080 as uint16_t);)            /*<Transmit Data Register Empty */
pub macro_rules! USART_SR_LBD (() =>                        (0x0100 as uint16_t);)            /*<LIN Break Detection Flag */
pub macro_rules! USART_SR_CTS (() =>                        (0x0200 as uint16_t);)            /*<CTS Flag */

/*  Bit definition for USART_DR register  */
pub macro_rules! USART_DR_DR (() =>                         (0x01FF as uint16_t);)            /*<Data value */

/*  Bit definition for USART_BRR register  */
pub macro_rules! USART_BRR_DIV_Fraction (() =>              (0x000F as uint16_t);)            /*<Fraction of USARTDIV */
pub macro_rules! USART_BRR_DIV_Mantissa (() =>              (0xFFF0 as uint16_t);)            /*<Mantissa of USARTDIV */

/*  Bit definition for USART_CR1 register  */
pub macro_rules! USART_CR1_SBK (() =>                       (0x0001 as uint16_t);)            /*<Send Break */
pub macro_rules! USART_CR1_RWU (() =>                       (0x0002 as uint16_t);)            /*<Receiver wakeup */
pub macro_rules! USART_CR1_RE (() =>                        (0x0004 as uint16_t);)            /*<Receiver Enable */
pub macro_rules! USART_CR1_TE (() =>                        (0x0008 as uint16_t);)            /*<Transmitter Enable */
pub macro_rules! USART_CR1_IDLEIE (() =>                    (0x0010 as uint16_t);)            /*<IDLE Interrupt Enable */
pub macro_rules! USART_CR1_RXNEIE (() =>                    (0x0020 as uint16_t);)            /*<RXNE Interrupt Enable */
pub macro_rules! USART_CR1_TCIE (() =>                      (0x0040 as uint16_t);)            /*<Transmission Complete Interrupt Enable */
pub macro_rules! USART_CR1_TXEIE (() =>                     (0x0080 as uint16_t);)            /*<PE Interrupt Enable */
pub macro_rules! USART_CR1_PEIE (() =>                      (0x0100 as uint16_t);)            /*<PE Interrupt Enable */
pub macro_rules! USART_CR1_PS (() =>                        (0x0200 as uint16_t);)            /*<Parity Selection */
pub macro_rules! USART_CR1_PCE (() =>                       (0x0400 as uint16_t);)            /*<Parity Control Enable */
pub macro_rules! USART_CR1_WAKE (() =>                      (0x0800 as uint16_t);)            /*<Wakeup method */
pub macro_rules! USART_CR1_M (() =>                         (0x1000 as uint16_t);)            /*<Word length */
pub macro_rules! USART_CR1_UE (() =>                        (0x2000 as uint16_t);)            /*<USART Enable */
pub macro_rules! USART_CR1_OVER8 (() =>                     (0x8000 as uint16_t);)            /*<USART Oversampling by 8 enable */

/*  Bit definition for USART_CR2 register  */
pub macro_rules! USART_CR2_ADD (() =>                       (0x000F as uint16_t);)            /*<Address of the USART node */
pub macro_rules! USART_CR2_LBDL (() =>                      (0x0020 as uint16_t);)            /*<LIN Break Detection Length */
pub macro_rules! USART_CR2_LBDIE (() =>                     (0x0040 as uint16_t);)            /*<LIN Break Detection Interrupt Enable */
pub macro_rules! USART_CR2_LBCL (() =>                      (0x0100 as uint16_t);)            /*<Last Bit Clock pulse */
pub macro_rules! USART_CR2_CPHA (() =>                      (0x0200 as uint16_t);)            /*<Clock Phase */
pub macro_rules! USART_CR2_CPOL (() =>                      (0x0400 as uint16_t);)            /*<Clock Polarity */
pub macro_rules! USART_CR2_CLKEN (() =>                     (0x0800 as uint16_t);)            /*<Clock Enable */

pub macro_rules! USART_CR2_STOP (() =>                      (0x3000 as uint16_t);)            /*<STOP[1:0] bits (STOP bits) */
pub macro_rules! USART_CR2_STOP_0 (() =>                    (0x1000 as uint16_t);)            /*<Bit 0 */
pub macro_rules! USART_CR2_STOP_1 (() =>                    (0x2000 as uint16_t);)            /*<Bit 1 */

pub macro_rules! USART_CR2_LINEN (() =>                     (0x4000 as uint16_t);)            /*<LIN mode enable */

/*  Bit definition for USART_CR3 register  */
pub macro_rules! USART_CR3_EIE (() =>                       (0x0001 as uint16_t);)            /*<Error Interrupt Enable */
pub macro_rules! USART_CR3_IREN (() =>                      (0x0002 as uint16_t);)            /*<IrDA mode Enable */
pub macro_rules! USART_CR3_IRLP (() =>                      (0x0004 as uint16_t);)            /*<IrDA Low-Power */
pub macro_rules! USART_CR3_HDSEL (() =>                     (0x0008 as uint16_t);)            /*<Half-Duplex Selection */
pub macro_rules! USART_CR3_NACK (() =>                      (0x0010 as uint16_t);)            /*<Smartcard NACK enable */
pub macro_rules! USART_CR3_SCEN (() =>                      (0x0020 as uint16_t);)            /*<Smartcard mode enable */
pub macro_rules! USART_CR3_DMAR (() =>                      (0x0040 as uint16_t);)            /*<DMA Enable Receiver */
pub macro_rules! USART_CR3_DMAT (() =>                      (0x0080 as uint16_t);)            /*<DMA Enable Transmitter */
pub macro_rules! USART_CR3_RTSE (() =>                      (0x0100 as uint16_t);)            /*<RTS Enable */
pub macro_rules! USART_CR3_CTSE (() =>                      (0x0200 as uint16_t);)            /*<CTS Enable */
pub macro_rules! USART_CR3_CTSIE (() =>                     (0x0400 as uint16_t);)            /*<CTS Interrupt Enable */
pub macro_rules! USART_CR3_ONEBIT (() =>                    (0x0800 as uint16_t);)            /*<USART One bit method enable */

/*  Bit definition for USART_GTPR register  */
pub macro_rules! USART_GTPR_PSC (() =>                      (0x00FF as uint16_t);)            /*<PSC[7:0] bits (Prescaler value) */
pub macro_rules! USART_GTPR_PSC_0 (() =>                    (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! USART_GTPR_PSC_1 (() =>                    (0x0002 as uint16_t);)            /*<Bit 1 */
pub macro_rules! USART_GTPR_PSC_2 (() =>                    (0x0004 as uint16_t);)            /*<Bit 2 */
pub macro_rules! USART_GTPR_PSC_3 (() =>                    (0x0008 as uint16_t);)            /*<Bit 3 */
pub macro_rules! USART_GTPR_PSC_4 (() =>                    (0x0010 as uint16_t);)            /*<Bit 4 */
pub macro_rules! USART_GTPR_PSC_5 (() =>                    (0x0020 as uint16_t);)            /*<Bit 5 */
pub macro_rules! USART_GTPR_PSC_6 (() =>                    (0x0040 as uint16_t);)            /*<Bit 6 */
pub macro_rules! USART_GTPR_PSC_7 (() =>                    (0x0080 as uint16_t);)            /*<Bit 7 */

pub macro_rules! USART_GTPR_GT (() =>                       (0xFF00 as uint16_t);)            /*<Guard time value */


/*                                                                            */
/*                            Window WATCHDOG                                 */
/*                                                                            */

/*  Bit definition for WWDG_CR register  */
pub macro_rules! WWDG_CR_T (() =>                           (0x7F as uint8_t);)               /*<T[6:0] bits (7-Bit counter (MSB to LSB)) */
pub macro_rules! WWDG_CR_T0 (() =>                          (0x01 as uint8_t);)               /*<Bit 0 */
pub macro_rules! WWDG_CR_T1 (() =>                          (0x02 as uint8_t);)               /*<Bit 1 */
pub macro_rules! WWDG_CR_T2 (() =>                          (0x04 as uint8_t);)               /*<Bit 2 */
pub macro_rules! WWDG_CR_T3 (() =>                          (0x08 as uint8_t);)               /*<Bit 3 */
pub macro_rules! WWDG_CR_T4 (() =>                          (0x10 as uint8_t);)               /*<Bit 4 */
pub macro_rules! WWDG_CR_T5 (() =>                          (0x20 as uint8_t);)               /*<Bit 5 */
pub macro_rules! WWDG_CR_T6 (() =>                          (0x40 as uint8_t);)               /*<Bit 6 */

pub macro_rules! WWDG_CR_WDGA (() =>                        (0x80 as uint8_t);)               /*<Activation bit */

/*  Bit definition for WWDG_CFR register  */
pub macro_rules! WWDG_CFR_W (() =>                          (0x007F as uint16_t);)            /*<W[6:0] bits (7-bit window value) */
pub macro_rules! WWDG_CFR_W0 (() =>                         (0x0001 as uint16_t);)            /*<Bit 0 */
pub macro_rules! WWDG_CFR_W1 (() =>                         (0x0002 as uint16_t);)            /*<Bit 1 */
pub macro_rules! WWDG_CFR_W2 (() =>                         (0x0004 as uint16_t);)            /*<Bit 2 */
pub macro_rules! WWDG_CFR_W3 (() =>                         (0x0008 as uint16_t);)            /*<Bit 3 */
pub macro_rules! WWDG_CFR_W4 (() =>                         (0x0010 as uint16_t);)            /*<Bit 4 */
pub macro_rules! WWDG_CFR_W5 (() =>                         (0x0020 as uint16_t);)            /*<Bit 5 */
pub macro_rules! WWDG_CFR_W6 (() =>                         (0x0040 as uint16_t);)            /*<Bit 6 */

pub macro_rules! WWDG_CFR_WDGTB (() =>                      (0x0180 as uint16_t);)            /*<WDGTB[1:0] bits (Timer Base) */
pub macro_rules! WWDG_CFR_WDGTB0 (() =>                     (0x0080 as uint16_t);)            /*<Bit 0 */
pub macro_rules! WWDG_CFR_WDGTB1 (() =>                     (0x0100 as uint16_t);)            /*<Bit 1 */

pub macro_rules! WWDG_CFR_EWI (() =>                        (0x0200 as uint16_t);)            /*<Early Wakeup Interrupt */

/*  Bit definition for WWDG_SR register  */
pub macro_rules! WWDG_SR_EWIF (() =>                        (0x01 as uint8_t);)               /*<Early Wakeup Interrupt Flag */



/*                                                                            */
/*                                DBG                                         */
/*                                                                            */

/*  Bit definition for DBGMCU_IDCODE register  */
pub macro_rules! DBGMCU_IDCODE_DEV_ID (() =>                (0x00000FFF as uint32_t);)
pub macro_rules! DBGMCU_IDCODE_REV_ID (() =>                (0xFFFF0000 as uint32_t);)

/*  Bit definition for DBGMCU_CR register  */
pub macro_rules! DBGMCU_CR_DBG_SLEEP (() =>                 (0x00000001 as uint32_t);)
pub macro_rules! DBGMCU_CR_DBG_STOP (() =>                  (0x00000002 as uint32_t);)
pub macro_rules! DBGMCU_CR_DBG_STANDBY (() =>               (0x00000004 as uint32_t);)
pub macro_rules! DBGMCU_CR_TRACE_IOEN (() =>                (0x00000020 as uint32_t);)

pub macro_rules! DBGMCU_CR_TRACE_MODE (() =>                (0x000000C0 as uint32_t);)
pub macro_rules! DBGMCU_CR_TRACE_MODE_0 (() =>              (0x00000040 as uint32_t);)/*<Bit 0 */
pub macro_rules! DBGMCU_CR_TRACE_MODE_1 (() =>              (0x00000080 as uint32_t);)/*<Bit 1 */

/*  Bit definition for DBGMCU_APB1_FZ register  */
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM2_STOP (() =>            (0x00000001 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM3_STOP (() =>            (0x00000002 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM4_STOP (() =>            (0x00000004 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM5_STOP (() =>            (0x00000008 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM6_STOP (() =>            (0x00000010 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM7_STOP (() =>            (0x00000020 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM12_STOP (() =>           (0x00000040 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM13_STOP (() =>           (0x00000080 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM14_STOP (() =>           (0x00000100 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_RTC_STOP (() =>             (0x00000400 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_WWDG_STOP (() =>            (0x00000800 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_IWDG_STOP (() =>            (0x00001000 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_I2C1_SMBUS_TIMEOUT (() =>   (0x00200000 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_I2C2_SMBUS_TIMEOUT (() =>   (0x00400000 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_I2C3_SMBUS_TIMEOUT (() =>   (0x00800000 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_CAN1_STOP (() =>            (0x02000000 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_CAN2_STOP (() =>            (0x04000000 as uint32_t);)
/* Old IWDGSTOP bit definition, maintained for legacy purpose */
pub macro_rules! DBGMCU_APB1_FZ_DBG_IWDEG_STOP (() =>           (DBGMCU_APB1_FZ_DBG_IWDG_STOP!());)

/*  Bit definition for DBGMCU_APB2_FZ register  */
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM1_STOP (() =>        (0x00000001 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM8_STOP (() =>        (0x00000002 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM9_STOP (() =>        (0x00010000 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM10_STOP (() =>       (0x00020000 as uint32_t);)
pub macro_rules! DBGMCU_APB1_FZ_DBG_TIM11_STOP (() =>       (0x00040000 as uint32_t);)


/*                                                                            */
/*                Ethernet MAC Registers bits definitions                     */
/*                                                                            */

/* Bit definition for Ethernet MAC Control Register register */
pub macro_rules! ETH_MACCR_WD (() =>      (0x00800000 as uint32_t);)  /* Watchdog disable */
pub macro_rules! ETH_MACCR_JD (() =>      (0x00400000 as uint32_t);)  /* Jabber disable */
pub macro_rules! ETH_MACCR_IFG (() =>     (0x000E0000 as uint32_t);)  /* Inter-frame gap */
pub macro_rules! ETH_MACCR_IFG_96Bit (() =>     (0x00000000 as uint32_t);)  /* Minimum IFG between frames during transmission is 96Bit */
  pub macro_rules! ETH_MACCR_IFG_88Bit (() =>     (0x00020000 as uint32_t);)  /* Minimum IFG between frames during transmission is 88Bit */
  pub macro_rules! ETH_MACCR_IFG_80Bit (() =>     (0x00040000 as uint32_t);)  /* Minimum IFG between frames during transmission is 80Bit */
  pub macro_rules! ETH_MACCR_IFG_72Bit (() =>     (0x00060000 as uint32_t);)  /* Minimum IFG between frames during transmission is 72Bit */
  pub macro_rules! ETH_MACCR_IFG_64Bit (() =>     (0x00080000 as uint32_t);)  /* Minimum IFG between frames during transmission is 64Bit */        
  pub macro_rules! ETH_MACCR_IFG_56Bit (() =>     (0x000A0000 as uint32_t);)  /* Minimum IFG between frames during transmission is 56Bit */
  pub macro_rules! ETH_MACCR_IFG_48Bit (() =>     (0x000C0000 as uint32_t);)  /* Minimum IFG between frames during transmission is 48Bit */
  pub macro_rules! ETH_MACCR_IFG_40Bit (() =>     (0x000E0000 as uint32_t);)  /* Minimum IFG between frames during transmission is 40Bit */              
pub macro_rules! ETH_MACCR_CSD (() =>     (0x00010000 as uint32_t);)  /* Carrier sense disable (during transmission) */
pub macro_rules! ETH_MACCR_FES (() =>     (0x00004000 as uint32_t);)  /* Fast ethernet speed */
pub macro_rules! ETH_MACCR_ROD (() =>     (0x00002000 as uint32_t);)  /* Receive own disable */
pub macro_rules! ETH_MACCR_LM (() =>      (0x00001000 as uint32_t);)  /* loopback mode */
pub macro_rules! ETH_MACCR_DM (() =>      (0x00000800 as uint32_t);)  /* Duplex mode */
pub macro_rules! ETH_MACCR_IPCO (() =>    (0x00000400 as uint32_t);)  /* IP Checksum offload */
pub macro_rules! ETH_MACCR_RD (() =>      (0x00000200 as uint32_t);)  /* Retry disable */
pub macro_rules! ETH_MACCR_APCS (() =>    (0x00000080 as uint32_t);)  /* Automatic Pad/CRC stripping */
pub macro_rules! ETH_MACCR_BL (() =>      (0x00000060 as uint32_t);)  /* Back-off limit: random integer number (r) of slot time delays before rescheduling
                                                       a transmission attempt during retries after a collision: 0 =< r <2^k */
  pub macro_rules! ETH_MACCR_BL_10 (() =>    (0x00000000 as uint32_t);)  /* k = min (n, 10) */
  pub macro_rules! ETH_MACCR_BL_8 (() =>     (0x00000020 as uint32_t);)  /* k = min (n, 8) */
  pub macro_rules! ETH_MACCR_BL_4 (() =>     (0x00000040 as uint32_t);)  /* k = min (n, 4) */
  pub macro_rules! ETH_MACCR_BL_1 (() =>     (0x00000060 as uint32_t);)  /* k = min (n, 1) */ 
pub macro_rules! ETH_MACCR_DC (() =>      (0x00000010 as uint32_t);)  /* Defferal check */
pub macro_rules! ETH_MACCR_TE (() =>      (0x00000008 as uint32_t);)  /* Transmitter enable */
pub macro_rules! ETH_MACCR_RE (() =>      (0x00000004 as uint32_t);)  /* Receiver enable */

/* Bit definition for Ethernet MAC Frame Filter Register */
pub macro_rules! ETH_MACFFR_RA (() =>     (0x80000000 as uint32_t);)  /* Receive all */ 
pub macro_rules! ETH_MACFFR_HPF (() =>    (0x00000400 as uint32_t);)  /* Hash or perfect filter */ 
pub macro_rules! ETH_MACFFR_SAF (() =>    (0x00000200 as uint32_t);)  /* Source address filter enable */ 
pub macro_rules! ETH_MACFFR_SAIF (() =>   (0x00000100 as uint32_t);)  /* SA inverse filtering */ 
pub macro_rules! ETH_MACFFR_PCF (() =>    (0x000000C0 as uint32_t);)  /* Pass control frames: 3 cases */
  pub macro_rules! ETH_MACFFR_PCF_BlockAll (() =>                (0x00000040 as uint32_t);)  /* MAC filters all control frames from reaching the application */
  pub macro_rules! ETH_MACFFR_PCF_ForwardAll (() =>              (0x00000080 as uint32_t);)  /* MAC forwards all control frames to application even if they fail the Address Filter */
  pub macro_rules! ETH_MACFFR_PCF_ForwardPassedAddrFilter (() => (0x000000C0 as uint32_t);)  /* MAC forwards control frames that pass the Address Filter. */ 
pub macro_rules! ETH_MACFFR_BFD (() =>    (0x00000020 as uint32_t);)  /* Broadcast frame disable */ 
pub macro_rules! ETH_MACFFR_PAM (() =>    (0x00000010 as uint32_t);)  /* Pass all mutlicast */ 
pub macro_rules! ETH_MACFFR_DAIF (() =>   (0x00000008 as uint32_t);)  /* DA Inverse filtering */ 
pub macro_rules! ETH_MACFFR_HM (() =>     (0x00000004 as uint32_t);)  /* Hash multicast */ 
pub macro_rules! ETH_MACFFR_HU (() =>     (0x00000002 as uint32_t);)  /* Hash unicast */
pub macro_rules! ETH_MACFFR_PM (() =>     (0x00000001 as uint32_t);)  /* Promiscuous mode */

/* Bit definition for Ethernet MAC Hash Table High Register */
pub macro_rules! ETH_MACHTHR_HTH (() =>   (0xFFFFFFFF as uint32_t);)  /* Hash table high */

/* Bit definition for Ethernet MAC Hash Table Low Register */
pub macro_rules! ETH_MACHTLR_HTL (() =>   (0xFFFFFFFF as uint32_t);)  /* Hash table low */

/* Bit definition for Ethernet MAC MII Address Register */
pub macro_rules! ETH_MACMIIAR_PA (() =>   (0x0000F800 as uint32_t);)  /* Physical layer address */ 
pub macro_rules! ETH_MACMIIAR_MR (() =>   (0x000007C0 as uint32_t);)  /* MII register in the selected PHY */ 
pub macro_rules! ETH_MACMIIAR_CR (() =>   (0x0000001C as uint32_t);)  /* CR clock range: 6 cases */ 
  pub macro_rules! ETH_MACMIIAR_CR_Div42 (() =>   (0x00000000 as uint32_t);)  /* HCLK:60-100 MHz; MDC clock= HCLK/42 */
  pub macro_rules! ETH_MACMIIAR_CR_Div62 (() =>   (0x00000004 as uint32_t);)  /* HCLK:100-150 MHz; MDC clock= HCLK/62 */
  pub macro_rules! ETH_MACMIIAR_CR_Div16 (() =>   (0x00000008 as uint32_t);)  /* HCLK:20-35 MHz; MDC clock= HCLK/16 */
  pub macro_rules! ETH_MACMIIAR_CR_Div26 (() =>   (0x0000000C as uint32_t);)  /* HCLK:35-60 MHz; MDC clock= HCLK/26 */
  pub macro_rules! ETH_MACMIIAR_CR_Div102 (() =>  (0x00000010 as uint32_t);)  /* HCLK:150-168 MHz; MDC clock= HCLK/102 */  
pub macro_rules! ETH_MACMIIAR_MW (() =>   (0x00000002 as uint32_t);)  /* MII write */ 
pub macro_rules! ETH_MACMIIAR_MB (() =>   (0x00000001 as uint32_t);)  /* MII busy */ 
  
/* Bit definition for Ethernet MAC MII Data Register */
pub macro_rules! ETH_MACMIIDR_MD (() =>   (0x0000FFFF as uint32_t);)  /* MII data: read/write data from/to PHY */

/* Bit definition for Ethernet MAC Flow Control Register */
pub macro_rules! ETH_MACFCR_PT (() =>     (0xFFFF0000 as uint32_t);)  /* Pause time */
pub macro_rules! ETH_MACFCR_ZQPD (() =>   (0x00000080 as uint32_t);)  /* Zero-quanta pause disable */
pub macro_rules! ETH_MACFCR_PLT (() =>    (0x00000030 as uint32_t);)  /* Pause low threshold: 4 cases */
  pub macro_rules! ETH_MACFCR_PLT_Minus4 (() =>   (0x00000000 as uint32_t);)  /* Pause time minus 4 slot times */
  pub macro_rules! ETH_MACFCR_PLT_Minus28 (() =>  (0x00000010 as uint32_t);)  /* Pause time minus 28 slot times */
  pub macro_rules! ETH_MACFCR_PLT_Minus144 (() => (0x00000020 as uint32_t);)  /* Pause time minus 144 slot times */
  pub macro_rules! ETH_MACFCR_PLT_Minus256 (() => (0x00000030 as uint32_t);)  /* Pause time minus 256 slot times */      
pub macro_rules! ETH_MACFCR_UPFD (() =>   (0x00000008 as uint32_t);)  /* Unicast pause frame detect */
pub macro_rules! ETH_MACFCR_RFCE (() =>   (0x00000004 as uint32_t);)  /* Receive flow control enable */
pub macro_rules! ETH_MACFCR_TFCE (() =>   (0x00000002 as uint32_t);)  /* Transmit flow control enable */
pub macro_rules! ETH_MACFCR_FCBBPA (() => (0x00000001 as uint32_t);)  /* Flow control busy/backpressure activate */

/* Bit definition for Ethernet MAC VLAN Tag Register */
pub macro_rules! ETH_MACVLANTR_VLANTC (() => (0x00010000 as uint32_t);)  /* 12-bit VLAN tag comparison */
pub macro_rules! ETH_MACVLANTR_VLANTI (() => (0x0000FFFF as uint32_t);)  /* VLAN tag identifier (for receive frames) */

/* Bit definition for Ethernet MAC Remote Wake-UpFrame Filter Register */ 
pub macro_rules! ETH_MACRWUFFR_D (() =>   (0xFFFFFFFF as uint32_t);)  /* Wake-up frame filter register data */
/* Eight sequential Writes to this address (offset 0x28) will write all Wake-UpFrame Filter Registers.
   Eight sequential Reads from this address (offset 0x28) will read all Wake-UpFrame Filter Registers. */
/* Wake-UpFrame Filter Reg0 : Filter 0 Byte Mask
   Wake-UpFrame Filter Reg1 : Filter 1 Byte Mask
   Wake-UpFrame Filter Reg2 : Filter 2 Byte Mask
   Wake-UpFrame Filter Reg3 : Filter 3 Byte Mask
   Wake-UpFrame Filter Reg4 : RSVD - Filter3 Command - RSVD - Filter2 Command - 
                              RSVD - Filter1 Command - RSVD - Filter0 Command
   Wake-UpFrame Filter Re5 : Filter3 Offset - Filter2 Offset - Filter1 Offset - Filter0 Offset
   Wake-UpFrame Filter Re6 : Filter1 CRC16 - Filter0 CRC16
   Wake-UpFrame Filter Re7 : Filter3 CRC16 - Filter2 CRC16 */

/* Bit definition for Ethernet MAC PMT Control and Status Register */ 
pub macro_rules! ETH_MACPMTCSR_WFFRPR (() => (0x80000000 as uint32_t);)  /* Wake-Up Frame Filter Register Pointer Reset */
pub macro_rules! ETH_MACPMTCSR_GU (() =>     (0x00000200 as uint32_t);)  /* Global Unicast */
pub macro_rules! ETH_MACPMTCSR_WFR (() =>    (0x00000040 as uint32_t);)  /* Wake-Up Frame Received */
pub macro_rules! ETH_MACPMTCSR_MPR (() =>    (0x00000020 as uint32_t);)  /* Magic Packet Received */
pub macro_rules! ETH_MACPMTCSR_WFE (() =>    (0x00000004 as uint32_t);)  /* Wake-Up Frame Enable */
pub macro_rules! ETH_MACPMTCSR_MPE (() =>    (0x00000002 as uint32_t);)  /* Magic Packet Enable */
pub macro_rules! ETH_MACPMTCSR_PD (() =>     (0x00000001 as uint32_t);)  /* Power Down */

/* Bit definition for Ethernet MAC Status Register */
pub macro_rules! ETH_MACSR_TSTS (() =>      (0x00000200 as uint32_t);)  /* Time stamp trigger status */
pub macro_rules! ETH_MACSR_MMCTS (() =>     (0x00000040 as uint32_t);)  /* MMC transmit status */
pub macro_rules! ETH_MACSR_MMMCRS (() =>    (0x00000020 as uint32_t);)  /* MMC receive status */
pub macro_rules! ETH_MACSR_MMCS (() =>      (0x00000010 as uint32_t);)  /* MMC status */
pub macro_rules! ETH_MACSR_PMTS (() =>      (0x00000008 as uint32_t);)  /* PMT status */

/* Bit definition for Ethernet MAC Interrupt Mask Register */
pub macro_rules! ETH_MACIMR_TSTIM (() =>     (0x00000200 as uint32_t);)  /* Time stamp trigger interrupt mask */
pub macro_rules! ETH_MACIMR_PMTIM (() =>     (0x00000008 as uint32_t);)  /* PMT interrupt mask */

/* Bit definition for Ethernet MAC Address0 High Register */
pub macro_rules! ETH_MACA0HR_MACA0H (() =>   (0x0000FFFF as uint32_t);)  /* MAC address0 high */

/* Bit definition for Ethernet MAC Address0 Low Register */
pub macro_rules! ETH_MACA0LR_MACA0L (() =>   (0xFFFFFFFF as uint32_t);)  /* MAC address0 low */

/* Bit definition for Ethernet MAC Address1 High Register */
pub macro_rules! ETH_MACA1HR_AE (() =>       (0x80000000 as uint32_t);)  /* Address enable */
pub macro_rules! ETH_MACA1HR_SA (() =>       (0x40000000 as uint32_t);)  /* Source address */
pub macro_rules! ETH_MACA1HR_MBC (() =>      (0x3F000000 as uint32_t);)  /* Mask byte control: bits to mask for comparison of the MAC Address bytes */
  pub macro_rules! ETH_MACA1HR_MBC_HBits15_8 (() =>    (0x20000000 as uint32_t);)  /* Mask MAC Address high reg bits [15:8] */
  pub macro_rules! ETH_MACA1HR_MBC_HBits7_0 (() =>     (0x10000000 as uint32_t);)  /* Mask MAC Address high reg bits [7:0] */
  pub macro_rules! ETH_MACA1HR_MBC_LBits31_24 (() =>   (0x08000000 as uint32_t);)  /* Mask MAC Address low reg bits [31:24] */
  pub macro_rules! ETH_MACA1HR_MBC_LBits23_16 (() =>   (0x04000000 as uint32_t);)  /* Mask MAC Address low reg bits [23:16] */
  pub macro_rules! ETH_MACA1HR_MBC_LBits15_8 (() =>    (0x02000000 as uint32_t);)  /* Mask MAC Address low reg bits [15:8] */
  pub macro_rules! ETH_MACA1HR_MBC_LBits7_0 (() =>     (0x01000000 as uint32_t);)  /* Mask MAC Address low reg bits [7:0] */ 
pub macro_rules! ETH_MACA1HR_MACA1H (() =>   (0x0000FFFF as uint32_t);)  /* MAC address1 high */

/* Bit definition for Ethernet MAC Address1 Low Register */
pub macro_rules! ETH_MACA1LR_MACA1L (() =>   (0xFFFFFFFF as uint32_t);)  /* MAC address1 low */

/* Bit definition for Ethernet MAC Address2 High Register */
pub macro_rules! ETH_MACA2HR_AE (() =>       (0x80000000 as uint32_t);)  /* Address enable */
pub macro_rules! ETH_MACA2HR_SA (() =>       (0x40000000 as uint32_t);)  /* Source address */
pub macro_rules! ETH_MACA2HR_MBC (() =>      (0x3F000000 as uint32_t);)  /* Mask byte control */
  pub macro_rules! ETH_MACA2HR_MBC_HBits15_8 (() =>    (0x20000000 as uint32_t);)  /* Mask MAC Address high reg bits [15:8] */
  pub macro_rules! ETH_MACA2HR_MBC_HBits7_0 (() =>     (0x10000000 as uint32_t);)  /* Mask MAC Address high reg bits [7:0] */
  pub macro_rules! ETH_MACA2HR_MBC_LBits31_24 (() =>   (0x08000000 as uint32_t);)  /* Mask MAC Address low reg bits [31:24] */
  pub macro_rules! ETH_MACA2HR_MBC_LBits23_16 (() =>   (0x04000000 as uint32_t);)  /* Mask MAC Address low reg bits [23:16] */
  pub macro_rules! ETH_MACA2HR_MBC_LBits15_8 (() =>    (0x02000000 as uint32_t);)  /* Mask MAC Address low reg bits [15:8] */
  pub macro_rules! ETH_MACA2HR_MBC_LBits7_0 (() =>     (0x01000000 as uint32_t);)  /* Mask MAC Address low reg bits [70] */
pub macro_rules! ETH_MACA2HR_MACA2H (() =>   (0x0000FFFF as uint32_t);)  /* MAC address1 high */

/* Bit definition for Ethernet MAC Address2 Low Register */
pub macro_rules! ETH_MACA2LR_MACA2L (() =>   (0xFFFFFFFF as uint32_t);)  /* MAC address2 low */

/* Bit definition for Ethernet MAC Address3 High Register */
pub macro_rules! ETH_MACA3HR_AE (() =>       (0x80000000 as uint32_t);)  /* Address enable */
pub macro_rules! ETH_MACA3HR_SA (() =>       (0x40000000 as uint32_t);)  /* Source address */
pub macro_rules! ETH_MACA3HR_MBC (() =>      (0x3F000000 as uint32_t);)  /* Mask byte control */
  pub macro_rules! ETH_MACA3HR_MBC_HBits15_8 (() =>    (0x20000000 as uint32_t);)  /* Mask MAC Address high reg bits [15:8] */
  pub macro_rules! ETH_MACA3HR_MBC_HBits7_0 (() =>     (0x10000000 as uint32_t);)  /* Mask MAC Address high reg bits [7:0] */
  pub macro_rules! ETH_MACA3HR_MBC_LBits31_24 (() =>   (0x08000000 as uint32_t);)  /* Mask MAC Address low reg bits [31:24] */
  pub macro_rules! ETH_MACA3HR_MBC_LBits23_16 (() =>   (0x04000000 as uint32_t);)  /* Mask MAC Address low reg bits [23:16] */
  pub macro_rules! ETH_MACA3HR_MBC_LBits15_8 (() =>    (0x02000000 as uint32_t);)  /* Mask MAC Address low reg bits [15:8] */
  pub macro_rules! ETH_MACA3HR_MBC_LBits7_0 (() =>     (0x01000000 as uint32_t);)  /* Mask MAC Address low reg bits [70] */
pub macro_rules! ETH_MACA3HR_MACA3H (() =>   (0x0000FFFF as uint32_t);)  /* MAC address3 high */

/* Bit definition for Ethernet MAC Address3 Low Register */
pub macro_rules! ETH_MACA3LR_MACA3L (() =>   (0xFFFFFFFF as uint32_t);)  /* MAC address3 low */


/*                Ethernet MMC Registers bits definition                      */


/* Bit definition for Ethernet MMC Contol Register */
pub macro_rules! ETH_MMCCR_MCFHP (() =>      (0x00000020 as uint32_t);)  /* MMC counter Full-Half preset */
pub macro_rules! ETH_MMCCR_MCP (() =>        (0x00000010 as uint32_t);)  /* MMC counter preset */
pub macro_rules! ETH_MMCCR_MCF (() =>        (0x00000008 as uint32_t);)  /* MMC Counter Freeze */
pub macro_rules! ETH_MMCCR_ROR (() =>        (0x00000004 as uint32_t);)  /* Reset on Read */
pub macro_rules! ETH_MMCCR_CSR (() =>        (0x00000002 as uint32_t);)  /* Counter Stop Rollover */
pub macro_rules! ETH_MMCCR_CR (() =>         (0x00000001 as uint32_t);)  /* Counters Reset */

/* Bit definition for Ethernet MMC Receive Interrupt Register */
pub macro_rules! ETH_MMCRIR_RGUFS (() =>     (0x00020000 as uint32_t);)  /* Set when Rx good unicast frames counter reaches half the maximum value */
pub macro_rules! ETH_MMCRIR_RFAES (() =>     (0x00000040 as uint32_t);)  /* Set when Rx alignment error counter reaches half the maximum value */
pub macro_rules! ETH_MMCRIR_RFCES (() =>     (0x00000020 as uint32_t);)  /* Set when Rx crc error counter reaches half the maximum value */

/* Bit definition for Ethernet MMC Transmit Interrupt Register */
pub macro_rules! ETH_MMCTIR_TGFS (() =>      (0x00200000 as uint32_t);)  /* Set when Tx good frame count counter reaches half the maximum value */
pub macro_rules! ETH_MMCTIR_TGFMSCS (() =>   (0x00008000 as uint32_t);)  /* Set when Tx good multi col counter reaches half the maximum value */
pub macro_rules! ETH_MMCTIR_TGFSCS (() =>    (0x00004000 as uint32_t);)  /* Set when Tx good single col counter reaches half the maximum value */

/* Bit definition for Ethernet MMC Receive Interrupt Mask Register */
pub macro_rules! ETH_MMCRIMR_RGUFM (() =>    (0x00020000 as uint32_t);)  /* Mask the interrupt when Rx good unicast frames counter reaches half the maximum value */
pub macro_rules! ETH_MMCRIMR_RFAEM (() =>    (0x00000040 as uint32_t);)  /* Mask the interrupt when when Rx alignment error counter reaches half the maximum value */
pub macro_rules! ETH_MMCRIMR_RFCEM (() =>    (0x00000020 as uint32_t);)  /* Mask the interrupt when Rx crc error counter reaches half the maximum value */

/* Bit definition for Ethernet MMC Transmit Interrupt Mask Register */
pub macro_rules! ETH_MMCTIMR_TGFM (() =>     (0x00200000 as uint32_t);)  /* Mask the interrupt when Tx good frame count counter reaches half the maximum value */
pub macro_rules! ETH_MMCTIMR_TGFMSCM (() =>  (0x00008000 as uint32_t);)  /* Mask the interrupt when Tx good multi col counter reaches half the maximum value */
pub macro_rules! ETH_MMCTIMR_TGFSCM (() =>   (0x00004000 as uint32_t);)  /* Mask the interrupt when Tx good single col counter reaches half the maximum value */

/* Bit definition for Ethernet MMC Transmitted Good Frames after Single Collision Counter Register */
pub macro_rules! ETH_MMCTGFSCCR_TGFSCC (() =>     (0xFFFFFFFF as uint32_t);)  /* Number of successfully transmitted frames after a single collision in Half-duplex mode. */

/* Bit definition for Ethernet MMC Transmitted Good Frames after More than a Single Collision Counter Register */
pub macro_rules! ETH_MMCTGFMSCCR_TGFMSCC (() =>   (0xFFFFFFFF as uint32_t);)  /* Number of successfully transmitted frames after more than a single collision in Half-duplex mode. */

/* Bit definition for Ethernet MMC Transmitted Good Frames Counter Register */
pub macro_rules! ETH_MMCTGFCR_TGFC (() =>    (0xFFFFFFFF as uint32_t);)  /* Number of good frames transmitted. */

/* Bit definition for Ethernet MMC Received Frames with CRC Error Counter Register */
pub macro_rules! ETH_MMCRFCECR_RFCEC (() =>  (0xFFFFFFFF as uint32_t);)  /* Number of frames received with CRC error. */

/* Bit definition for Ethernet MMC Received Frames with Alignement Error Counter Register */
pub macro_rules! ETH_MMCRFAECR_RFAEC (() =>  (0xFFFFFFFF as uint32_t);)  /* Number of frames received with alignment (dribble) error */

/* Bit definition for Ethernet MMC Received Good Unicast Frames Counter Register */
pub macro_rules! ETH_MMCRGUFCR_RGUFC (() =>  (0xFFFFFFFF as uint32_t);)  /* Number of good unicast frames received. */


/*               Ethernet PTP Registers bits definition                       */


/* Bit definition for Ethernet PTP Time Stamp Contol Register */
pub macro_rules! ETH_PTPTSCR_TSCNT (() =>       (0x00030000 as uint32_t);)  /* Time stamp clock node type */
pub macro_rules! ETH_PTPTSSR_TSSMRME (() =>     (0x00008000 as uint32_t);)  /* Time stamp snapshot for message relevant to master enable */
pub macro_rules! ETH_PTPTSSR_TSSEME (() =>      (0x00004000 as uint32_t);)  /* Time stamp snapshot for event message enable */
pub macro_rules! ETH_PTPTSSR_TSSIPV4FE (() =>   (0x00002000 as uint32_t);)  /* Time stamp snapshot for IPv4 frames enable */
pub macro_rules! ETH_PTPTSSR_TSSIPV6FE (() =>   (0x00001000 as uint32_t);)  /* Time stamp snapshot for IPv6 frames enable */
pub macro_rules! ETH_PTPTSSR_TSSPTPOEFE (() =>  (0x00000800 as uint32_t);)  /* Time stamp snapshot for PTP over ethernet frames enable */
pub macro_rules! ETH_PTPTSSR_TSPTPPSV2E (() =>  (0x00000400 as uint32_t);)  /* Time stamp PTP packet snooping for version2 format enable */
pub macro_rules! ETH_PTPTSSR_TSSSR (() =>       (0x00000200 as uint32_t);)  /* Time stamp Sub-seconds rollover */
pub macro_rules! ETH_PTPTSSR_TSSARFE (() =>     (0x00000100 as uint32_t);)  /* Time stamp snapshot for all received frames enable */

pub macro_rules! ETH_PTPTSCR_TSARU (() =>    (0x00000020 as uint32_t);)  /* Addend register update */
pub macro_rules! ETH_PTPTSCR_TSITE (() =>    (0x00000010 as uint32_t);)  /* Time stamp interrupt trigger enable */
pub macro_rules! ETH_PTPTSCR_TSSTU (() =>    (0x00000008 as uint32_t);)  /* Time stamp update */
pub macro_rules! ETH_PTPTSCR_TSSTI (() =>    (0x00000004 as uint32_t);)  /* Time stamp initialize */
pub macro_rules! ETH_PTPTSCR_TSFCU (() =>    (0x00000002 as uint32_t);)  /* Time stamp fine or coarse update */
pub macro_rules! ETH_PTPTSCR_TSE (() =>      (0x00000001 as uint32_t);)  /* Time stamp enable */

/* Bit definition for Ethernet PTP Sub-Second Increment Register */
pub macro_rules! ETH_PTPSSIR_STSSI (() =>    (0x000000FF as uint32_t);)  /* System time Sub-second increment value */

/* Bit definition for Ethernet PTP Time Stamp High Register */
pub macro_rules! ETH_PTPTSHR_STS (() =>      (0xFFFFFFFF as uint32_t);)  /* System Time second */

/* Bit definition for Ethernet PTP Time Stamp Low Register */
pub macro_rules! ETH_PTPTSLR_STPNS (() =>    (0x80000000 as uint32_t);)  /* System Time Positive or negative time */
pub macro_rules! ETH_PTPTSLR_STSS (() =>     (0x7FFFFFFF as uint32_t);)  /* System Time sub-seconds */

/* Bit definition for Ethernet PTP Time Stamp High Update Register */
pub macro_rules! ETH_PTPTSHUR_TSUS (() =>    (0xFFFFFFFF as uint32_t);)  /* Time stamp update seconds */

/* Bit definition for Ethernet PTP Time Stamp Low Update Register */
pub macro_rules! ETH_PTPTSLUR_TSUPNS (() =>  (0x80000000 as uint32_t);)  /* Time stamp update Positive or negative time */
pub macro_rules! ETH_PTPTSLUR_TSUSS (() =>   (0x7FFFFFFF as uint32_t);)  /* Time stamp update sub-seconds */

/* Bit definition for Ethernet PTP Time Stamp Addend Register */
pub macro_rules! ETH_PTPTSAR_TSA (() =>      (0xFFFFFFFF as uint32_t);)  /* Time stamp addend */

/* Bit definition for Ethernet PTP Target Time High Register */
pub macro_rules! ETH_PTPTTHR_TTSH (() =>     (0xFFFFFFFF as uint32_t);)  /* Target time stamp high */

/* Bit definition for Ethernet PTP Target Time Low Register */
pub macro_rules! ETH_PTPTTLR_TTSL (() =>     (0xFFFFFFFF as uint32_t);)  /* Target time stamp low */

/* Bit definition for Ethernet PTP Time Stamp Status Register */
pub macro_rules! ETH_PTPTSSR_TSTTR (() =>    (0x00000020 as uint32_t);)  /* Time stamp target time reached */
pub macro_rules! ETH_PTPTSSR_TSSO (() =>     (0x00000010 as uint32_t);)  /* Time stamp seconds overflow */


/*                 Ethernet DMA Registers bits definition                     */


/* Bit definition for Ethernet DMA Bus Mode Register */
pub macro_rules! ETH_DMABMR_AAB (() =>       (0x02000000 as uint32_t);)  /* Address-Aligned beats */
pub macro_rules! ETH_DMABMR_FPM (() =>        (0x01000000 as uint32_t);)  /* 4xPBL mode */
pub macro_rules! ETH_DMABMR_USP (() =>       (0x00800000 as uint32_t);)  /* Use separate PBL */
pub macro_rules! ETH_DMABMR_RDP (() =>       (0x007E0000 as uint32_t);)  /* RxDMA PBL */
  pub macro_rules! ETH_DMABMR_RDP_1Beat (() =>    (0x00020000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 1 */
  pub macro_rules! ETH_DMABMR_RDP_2Beat (() =>    (0x00040000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 2 */
  pub macro_rules! ETH_DMABMR_RDP_4Beat (() =>    (0x00080000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 4 */
  pub macro_rules! ETH_DMABMR_RDP_8Beat (() =>    (0x00100000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 8 */
  pub macro_rules! ETH_DMABMR_RDP_16Beat (() =>   (0x00200000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 16 */
  pub macro_rules! ETH_DMABMR_RDP_32Beat (() =>   (0x00400000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 32 */                
  pub macro_rules! ETH_DMABMR_RDP_4xPBL_4Beat (() =>   (0x01020000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 4 */
  pub macro_rules! ETH_DMABMR_RDP_4xPBL_8Beat (() =>   (0x01040000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 8 */
  pub macro_rules! ETH_DMABMR_RDP_4xPBL_16Beat (() =>  (0x01080000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 16 */
  pub macro_rules! ETH_DMABMR_RDP_4xPBL_32Beat (() =>  (0x01100000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 32 */
  pub macro_rules! ETH_DMABMR_RDP_4xPBL_64Beat (() =>  (0x01200000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 64 */
  pub macro_rules! ETH_DMABMR_RDP_4xPBL_128Beat (() => (0x01400000 as uint32_t);)  /* maximum number of beats to be transferred in one RxDMA transaction is 128 */  
pub macro_rules! ETH_DMABMR_FB (() =>        (0x00010000 as uint32_t);)  /* Fixed Burst */
pub macro_rules! ETH_DMABMR_RTPR (() =>      (0x0000C000 as uint32_t);)  /* Rx Tx priority ratio */
  pub macro_rules! ETH_DMABMR_RTPR_1_1 (() =>     (0x00000000 as uint32_t);)  /* Rx Tx priority ratio */
  pub macro_rules! ETH_DMABMR_RTPR_2_1 (() =>     (0x00004000 as uint32_t);)  /* Rx Tx priority ratio */
  pub macro_rules! ETH_DMABMR_RTPR_3_1 (() =>     (0x00008000 as uint32_t);)  /* Rx Tx priority ratio */
  pub macro_rules! ETH_DMABMR_RTPR_4_1 (() =>     (0x0000C000 as uint32_t);)  /* Rx Tx priority ratio */  
pub macro_rules! ETH_DMABMR_PBL (() =>    (0x00003F00 as uint32_t);)  /* Programmable burst length */
  pub macro_rules! ETH_DMABMR_PBL_1Beat (() =>    (0x00000100 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 1 */
  pub macro_rules! ETH_DMABMR_PBL_2Beat (() =>    (0x00000200 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 2 */
  pub macro_rules! ETH_DMABMR_PBL_4Beat (() =>    (0x00000400 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 4 */
  pub macro_rules! ETH_DMABMR_PBL_8Beat (() =>    (0x00000800 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 8 */
  pub macro_rules! ETH_DMABMR_PBL_16Beat (() =>   (0x00001000 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 16 */
  pub macro_rules! ETH_DMABMR_PBL_32Beat (() =>   (0x00002000 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 32 */                
  pub macro_rules! ETH_DMABMR_PBL_4xPBL_4Beat (() =>   (0x01000100 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 4 */
  pub macro_rules! ETH_DMABMR_PBL_4xPBL_8Beat (() =>   (0x01000200 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 8 */
  pub macro_rules! ETH_DMABMR_PBL_4xPBL_16Beat (() =>  (0x01000400 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 16 */
  pub macro_rules! ETH_DMABMR_PBL_4xPBL_32Beat (() =>  (0x01000800 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 32 */
  pub macro_rules! ETH_DMABMR_PBL_4xPBL_64Beat (() =>  (0x01001000 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 64 */
  pub macro_rules! ETH_DMABMR_PBL_4xPBL_128Beat (() => (0x01002000 as uint32_t);)  /* maximum number of beats to be transferred in one TxDMA (or both) transaction is 128 */
pub macro_rules! ETH_DMABMR_EDE (() =>       (0x00000080 as uint32_t);)  /* Enhanced Descriptor Enable */
pub macro_rules! ETH_DMABMR_DSL (() =>       (0x0000007C as uint32_t);)  /* Descriptor Skip Length */
pub macro_rules! ETH_DMABMR_DA (() =>        (0x00000002 as uint32_t);)  /* DMA arbitration scheme */
pub macro_rules! ETH_DMABMR_SR (() =>        (0x00000001 as uint32_t);)  /* Software reset */

/* Bit definition for Ethernet DMA Transmit Poll Demand Register */
pub macro_rules! ETH_DMATPDR_TPD (() =>      (0xFFFFFFFF as uint32_t);)  /* Transmit poll demand */

/* Bit definition for Ethernet DMA Receive Poll Demand Register */
pub macro_rules! ETH_DMARPDR_RPD (() =>      (0xFFFFFFFF as uint32_t);)  /* Receive poll demand  */

/* Bit definition for Ethernet DMA Receive Descriptor List Address Register */
pub macro_rules! ETH_DMARDLAR_SRL (() =>     (0xFFFFFFFF as uint32_t);)  /* Start of receive list */

/* Bit definition for Ethernet DMA Transmit Descriptor List Address Register */
pub macro_rules! ETH_DMATDLAR_STL (() =>     (0xFFFFFFFF as uint32_t);)  /* Start of transmit list */

/* Bit definition for Ethernet DMA Status Register */
pub macro_rules! ETH_DMASR_TSTS (() =>       (0x20000000 as uint32_t);)  /* Time-stamp trigger status */
pub macro_rules! ETH_DMASR_PMTS (() =>       (0x10000000 as uint32_t);)  /* PMT status */
pub macro_rules! ETH_DMASR_MMCS (() =>       (0x08000000 as uint32_t);)  /* MMC status */
pub macro_rules! ETH_DMASR_EBS (() =>        (0x03800000 as uint32_t);)  /* Error bits status */
  /* combination with EBS[2:0] for GetFlagStatus function */
  pub macro_rules! ETH_DMASR_EBS_DescAccess (() =>      (0x02000000 as uint32_t);)  /* Error bits 0-data buffer, 1-desc. access */
  pub macro_rules! ETH_DMASR_EBS_ReadTransf (() =>      (0x01000000 as uint32_t);)  /* Error bits 0-write trnsf, 1-read transfr */
  pub macro_rules! ETH_DMASR_EBS_DataTransfTx (() =>    (0x00800000 as uint32_t);)  /* Error bits 0-Rx DMA, 1-Tx DMA */
pub macro_rules! ETH_DMASR_TPS (() =>         (0x00700000 as uint32_t);)  /* Transmit process state */
  pub macro_rules! ETH_DMASR_TPS_Stopped (() =>         (0x00000000 as uint32_t);)  /* Stopped - Reset or Stop Tx Command issued  */
  pub macro_rules! ETH_DMASR_TPS_Fetching (() =>        (0x00100000 as uint32_t);)  /* Running - fetching the Tx descriptor */
  pub macro_rules! ETH_DMASR_TPS_Waiting (() =>         (0x00200000 as uint32_t);)  /* Running - waiting for status */
  pub macro_rules! ETH_DMASR_TPS_Reading (() =>         (0x00300000 as uint32_t);)  /* Running - reading the data from host memory */
  pub macro_rules! ETH_DMASR_TPS_Suspended (() =>       (0x00600000 as uint32_t);)  /* Suspended - Tx Descriptor unavailabe */
  pub macro_rules! ETH_DMASR_TPS_Closing (() =>         (0x00700000 as uint32_t);)  /* Running - closing Rx descriptor */
pub macro_rules! ETH_DMASR_RPS (() =>         (0x000E0000 as uint32_t);)  /* Receive process state */
  pub macro_rules! ETH_DMASR_RPS_Stopped (() =>         (0x00000000 as uint32_t);)  /* Stopped - Reset or Stop Rx Command issued */
  pub macro_rules! ETH_DMASR_RPS_Fetching (() =>        (0x00020000 as uint32_t);)  /* Running - fetching the Rx descriptor */
  pub macro_rules! ETH_DMASR_RPS_Waiting (() =>         (0x00060000 as uint32_t);)  /* Running - waiting for packet */
  pub macro_rules! ETH_DMASR_RPS_Suspended (() =>       (0x00080000 as uint32_t);)  /* Suspended - Rx Descriptor unavailable */
  pub macro_rules! ETH_DMASR_RPS_Closing (() =>         (0x000A0000 as uint32_t);)  /* Running - closing descriptor */
  pub macro_rules! ETH_DMASR_RPS_Queuing (() =>         (0x000E0000 as uint32_t);)  /* Running - queuing the recieve frame into host memory */
pub macro_rules! ETH_DMASR_NIS (() =>        (0x00010000 as uint32_t);)  /* Normal interrupt summary */
pub macro_rules! ETH_DMASR_AIS (() =>        (0x00008000 as uint32_t);)  /* Abnormal interrupt summary */
pub macro_rules! ETH_DMASR_ERS (() =>        (0x00004000 as uint32_t);)  /* Early receive status */
pub macro_rules! ETH_DMASR_FBES (() =>       (0x00002000 as uint32_t);)  /* Fatal bus error status */
pub macro_rules! ETH_DMASR_ETS (() =>        (0x00000400 as uint32_t);)  /* Early transmit status */
pub macro_rules! ETH_DMASR_RWTS (() =>       (0x00000200 as uint32_t);)  /* Receive watchdog timeout status */
pub macro_rules! ETH_DMASR_RPSS (() =>       (0x00000100 as uint32_t);)  /* Receive process stopped status */
pub macro_rules! ETH_DMASR_RBUS (() =>       (0x00000080 as uint32_t);)  /* Receive buffer unavailable status */
pub macro_rules! ETH_DMASR_RS (() =>         (0x00000040 as uint32_t);)  /* Receive status */
pub macro_rules! ETH_DMASR_TUS (() =>        (0x00000020 as uint32_t);)  /* Transmit underflow status */
pub macro_rules! ETH_DMASR_ROS (() =>        (0x00000010 as uint32_t);)  /* Receive overflow status */
pub macro_rules! ETH_DMASR_TJTS (() =>       (0x00000008 as uint32_t);)  /* Transmit jabber timeout status */
pub macro_rules! ETH_DMASR_TBUS (() =>       (0x00000004 as uint32_t);)  /* Transmit buffer unavailable status */
pub macro_rules! ETH_DMASR_TPSS (() =>       (0x00000002 as uint32_t);)  /* Transmit process stopped status */
pub macro_rules! ETH_DMASR_TS (() =>         (0x00000001 as uint32_t);)  /* Transmit status */

/* Bit definition for Ethernet DMA Operation Mode Register */
pub macro_rules! ETH_DMAOMR_DTCEFD (() =>    (0x04000000 as uint32_t);)  /* Disable Dropping of TCP/IP checksum error frames */
pub macro_rules! ETH_DMAOMR_RSF (() =>       (0x02000000 as uint32_t);)  /* Receive store and forward */
pub macro_rules! ETH_DMAOMR_DFRF (() =>      (0x01000000 as uint32_t);)  /* Disable flushing of received frames */
pub macro_rules! ETH_DMAOMR_TSF (() =>       (0x00200000 as uint32_t);)  /* Transmit store and forward */
pub macro_rules! ETH_DMAOMR_FTF (() =>       (0x00100000 as uint32_t);)  /* Flush transmit FIFO */
pub macro_rules! ETH_DMAOMR_TTC (() =>       (0x0001C000 as uint32_t);)  /* Transmit threshold control */
  pub macro_rules! ETH_DMAOMR_TTC_64Bytes (() =>       (0x00000000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 64 Bytes */
  pub macro_rules! ETH_DMAOMR_TTC_128Bytes (() =>      (0x00004000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 128 Bytes */
  pub macro_rules! ETH_DMAOMR_TTC_192Bytes (() =>      (0x00008000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 192 Bytes */
  pub macro_rules! ETH_DMAOMR_TTC_256Bytes (() =>      (0x0000C000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 256 Bytes */
  pub macro_rules! ETH_DMAOMR_TTC_40Bytes (() =>       (0x00010000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 40 Bytes */
  pub macro_rules! ETH_DMAOMR_TTC_32Bytes (() =>       (0x00014000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 32 Bytes */
  pub macro_rules! ETH_DMAOMR_TTC_24Bytes (() =>       (0x00018000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 24 Bytes */
  pub macro_rules! ETH_DMAOMR_TTC_16Bytes (() =>       (0x0001C000 as uint32_t);)  /* threshold level of the MTL Transmit FIFO is 16 Bytes */
pub macro_rules! ETH_DMAOMR_ST (() =>        (0x00002000 as uint32_t);)  /* Start/stop transmission command */
pub macro_rules! ETH_DMAOMR_FEF (() =>       (0x00000080 as uint32_t);)  /* Forward error frames */
pub macro_rules! ETH_DMAOMR_FUGF (() =>      (0x00000040 as uint32_t);)  /* Forward undersized good frames */
pub macro_rules! ETH_DMAOMR_RTC (() =>       (0x00000018 as uint32_t);)  /* receive threshold control */
  pub macro_rules! ETH_DMAOMR_RTC_64Bytes (() =>       (0x00000000 as uint32_t);)  /* threshold level of the MTL Receive FIFO is 64 Bytes */
  pub macro_rules! ETH_DMAOMR_RTC_32Bytes (() =>       (0x00000008 as uint32_t);)  /* threshold level of the MTL Receive FIFO is 32 Bytes */
  pub macro_rules! ETH_DMAOMR_RTC_96Bytes (() =>       (0x00000010 as uint32_t);)  /* threshold level of the MTL Receive FIFO is 96 Bytes */
  pub macro_rules! ETH_DMAOMR_RTC_128Bytes (() =>      (0x00000018 as uint32_t);)  /* threshold level of the MTL Receive FIFO is 128 Bytes */
pub macro_rules! ETH_DMAOMR_OSF (() =>       (0x00000004 as uint32_t);)  /* operate on second frame */
pub macro_rules! ETH_DMAOMR_SR (() =>        (0x00000002 as uint32_t);)  /* Start/stop receive */

/* Bit definition for Ethernet DMA Interrupt Enable Register */
pub macro_rules! ETH_DMAIER_NISE (() =>      (0x00010000 as uint32_t);)  /* Normal interrupt summary enable */
pub macro_rules! ETH_DMAIER_AISE (() =>      (0x00008000 as uint32_t);)  /* Abnormal interrupt summary enable */
pub macro_rules! ETH_DMAIER_ERIE (() =>      (0x00004000 as uint32_t);)  /* Early receive interrupt enable */
pub macro_rules! ETH_DMAIER_FBEIE (() =>     (0x00002000 as uint32_t);)  /* Fatal bus error interrupt enable */
pub macro_rules! ETH_DMAIER_ETIE (() =>      (0x00000400 as uint32_t);)  /* Early transmit interrupt enable */
pub macro_rules! ETH_DMAIER_RWTIE (() =>     (0x00000200 as uint32_t);)  /* Receive watchdog timeout interrupt enable */
pub macro_rules! ETH_DMAIER_RPSIE (() =>     (0x00000100 as uint32_t);)  /* Receive process stopped interrupt enable */
pub macro_rules! ETH_DMAIER_RBUIE (() =>     (0x00000080 as uint32_t);)  /* Receive buffer unavailable interrupt enable */
pub macro_rules! ETH_DMAIER_RIE (() =>       (0x00000040 as uint32_t);)  /* Receive interrupt enable */
pub macro_rules! ETH_DMAIER_TUIE (() =>      (0x00000020 as uint32_t);)  /* Transmit Underflow interrupt enable */
pub macro_rules! ETH_DMAIER_ROIE (() =>      (0x00000010 as uint32_t);)  /* Receive Overflow interrupt enable */
pub macro_rules! ETH_DMAIER_TJTIE (() =>     (0x00000008 as uint32_t);)  /* Transmit jabber timeout interrupt enable */
pub macro_rules! ETH_DMAIER_TBUIE (() =>     (0x00000004 as uint32_t);)  /* Transmit buffer unavailable interrupt enable */
pub macro_rules! ETH_DMAIER_TPSIE (() =>     (0x00000002 as uint32_t);)  /* Transmit process stopped interrupt enable */
pub macro_rules! ETH_DMAIER_TIE (() =>       (0x00000001 as uint32_t);)  /* Transmit interrupt enable */

/* Bit definition for Ethernet DMA Missed Frame and Buffer Overflow Counter Register */
pub macro_rules! ETH_DMAMFBOCR_OFOC (() =>   (0x10000000 as uint32_t);)  /* Overflow bit for FIFO overflow counter */
pub macro_rules! ETH_DMAMFBOCR_MFA (() =>    (0x0FFE0000 as uint32_t);)  /* Number of frames missed by the application */
pub macro_rules! ETH_DMAMFBOCR_OMFC (() =>   (0x00010000 as uint32_t);)  /* Overflow bit for missed frame counter */
pub macro_rules! ETH_DMAMFBOCR_MFC (() =>    (0x0000FFFF as uint32_t);)  /* Number of frames missed by the controller */

/* Bit definition for Ethernet DMA Current Host Transmit Descriptor Register */
pub macro_rules! ETH_DMACHTDR_HTDAP (() =>   (0xFFFFFFFF as uint32_t);)  /* Host transmit descriptor address pointer */

/* Bit definition for Ethernet DMA Current Host Receive Descriptor Register */
pub macro_rules! ETH_DMACHRDR_HRDAP (() =>   (0xFFFFFFFF as uint32_t);)  /* Host receive descriptor address pointer */

/* Bit definition for Ethernet DMA Current Host Transmit Buffer Address Register */
pub macro_rules! ETH_DMACHTBAR_HTBAP (() =>  (0xFFFFFFFF as uint32_t);)  /* Host transmit buffer address pointer */

/* Bit definition for Ethernet DMA Current Host Receive Buffer Address Register */
pub macro_rules! ETH_DMACHRBAR_HRBAP (() =>  (0xFFFFFFFF as uint32_t);)  /* Host receive buffer address pointer */

pub macro_rules! SCS_BASE (() =>  (0xE000E000 as uint32_t);)                            /*< System Control Space Base Address */
pub macro_rules! NVIC_BASE (() =>  (SCS_BASE!() +  0x0100);)                    /*< NVIC Base Address                 */


/* file core_cm0.h  */
/* Structure type to access the Nested Vectored Interrupt Controller (NVIC).
 */
pub struct NVICType {
  ISER: [u32, ..1],                 /*< Offset: 0x000 (R/W)  Interrupt Set Enable Register           */
  RESERVED0: [u32, ..31],
  ICER: [u32, ..1],                 /*< Offset: 0x080 (R/W)  Interrupt Clear Enable Register          */
  RSERVED1: [u32, ..31],
  ISPR: [u32, ..1],                 /*< Offset: 0x100 (R/W)  Interrupt Set Pending Register           */
  RESERVED2: [u32, ..31],
  ICPR: [u32, ..1],                 /*< Offset: 0x180 (R/W)  Interrupt Clear Pending Register         */
  RESERVED3: [u32, ..31],
  RESERVED4: [u32, ..64],
  IP: [u32, ..8]                   /*< Offset: 0x300 (R/W)  Interrupt Priority Register              */
}

/*  end core_cm0.h */
 
#[inline(always)]
pub fn hw_ptr<T>(raw :u32) -> &mut T{
	unsafe {
		&mut *(raw as *mut T)
	}
}

/* Peripheral_declaration

  */  

#[inline(always)]
pub fn TIM2() -> &mut TIMType {unsafe {&mut *(TIM2_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM3() -> &mut TIMType {unsafe {&mut *(TIM3_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM4() -> &mut TIMType {unsafe {&mut *(TIM4_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM5() -> &mut TIMType {unsafe {&mut *(TIM5_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM6() -> &mut TIMType {unsafe {&mut *(TIM6_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM7() -> &mut TIMType {unsafe {&mut *(TIM7_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM12() -> &mut TIMType {unsafe {&mut *(TIM12_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM13() -> &mut TIMType {unsafe {&mut *(TIM13_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM14() -> &mut TIMType {unsafe {&mut *(TIM14_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn RTC() -> &mut RTCType {unsafe {&mut *(RTC_BASE!() as *mut RTCType)}}
#[inline(always)]
pub fn WWDG() -> &mut WWDGType {unsafe {&mut *(WWDG_BASE!() as *mut WWDGType)}}
#[inline(always)]
pub fn IWDG() -> &mut IWDGType {unsafe {&mut *(IWDG_BASE!() as *mut IWDGType)}}
#[inline(always)]
pub fn I2S2ext() -> &mut SPIType {unsafe {&mut *(I2S2ext_BASE!() as *mut SPIType)}}
#[inline(always)]
pub fn SPI2() -> &mut SPIType {unsafe {&mut *(SPI2_BASE!() as *mut SPIType)}}
#[inline(always)]
pub fn SPI3() -> &mut SPIType {unsafe {&mut *(SPI3_BASE!() as *mut SPIType)}}
#[inline(always)]
pub fn I2S3ext() -> &mut SPIType {unsafe {&mut *(I2S3ext_BASE!() as *mut SPIType)}}
#[inline(always)]
pub fn USART2() -> &mut USARTType {unsafe {&mut *(USART2_BASE!() as *mut USARTType)}}
#[inline(always)]
pub fn USART3() -> &mut USARTType {unsafe {&mut *(USART3_BASE!() as *mut USARTType)}}
#[inline(always)]
pub fn UART4() -> &mut USARTType {unsafe {&mut *(UART4_BASE!() as *mut USARTType)}}
#[inline(always)]
pub fn UART5() -> &mut USARTType {unsafe {&mut *(UART5_BASE!() as *mut USARTType)}}
#[inline(always)]
pub fn I2C1() -> &mut I2CType {unsafe {&mut *(I2C1_BASE!() as *mut I2CType)}}
#[inline(always)]
pub fn I2C2() -> &mut I2CType {unsafe {&mut *(I2C2_BASE!() as *mut I2CType)}}
#[inline(always)]
pub fn I2C3() -> &mut I2CType {unsafe {&mut *(I2C3_BASE!() as *mut I2CType)}}
#[inline(always)]
pub fn CAN1() -> &mut CANType {unsafe {&mut *(CAN1_BASE!() as *mut CANType)}}
#[inline(always)]
pub fn CAN2() -> &mut CANType {unsafe {&mut *(CAN2_BASE!() as *mut CANType)}}
#[inline(always)]
pub fn PWR() -> &mut PWRType {unsafe {&mut *(PWR_BASE!() as *mut PWRType)}}
#[inline(always)]
pub fn DAC() -> &mut DACType {unsafe {&mut *(DAC_BASE!() as *mut DACType)}}
#[inline(always)]
pub fn TIM1() -> &mut TIMType {unsafe {&mut *(TIM1_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM8() -> &mut TIMType {unsafe {&mut *(TIM8_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn USART1() -> &mut USARTType {unsafe {&mut *(USART1_BASE!() as *mut USARTType)}}
#[inline(always)]
pub fn USART6() -> &mut USARTType {unsafe {&mut *(USART6_BASE!() as *mut USARTType)}}
#[inline(always)]
pub fn ADC() -> &mut ADC_CommonType {unsafe {&mut *(ADC_BASE!() as *mut ADC_CommonType)}}
#[inline(always)]
pub fn ADC1() -> &mut ADCType {unsafe {&mut *(ADC1_BASE!() as *mut ADCType)}}
#[inline(always)]
pub fn ADC2() -> &mut ADCType {unsafe {&mut *(ADC2_BASE!() as *mut ADCType)}}
#[inline(always)]
pub fn ADC3() -> &mut ADCType {unsafe {&mut *(ADC3_BASE!() as *mut ADCType)}}
#[inline(always)]
pub fn SDIO() -> &mut SDIOType {unsafe {&mut *(SDIO_BASE!() as *mut SDIOType)}}
#[inline(always)]
pub fn SPI1() -> &mut SPIType {unsafe {&mut *(SPI1_BASE!() as *mut SPIType)}}
#[inline(always)]
pub fn SYSCFG() -> &mut SYSCFGType {unsafe {&mut *(SYSCFG_BASE!() as *mut SYSCFGType)}}
#[inline(always)]
pub fn EXTI() -> &mut EXTIType {unsafe {&mut *(EXTI_BASE!() as *mut EXTIType)}}
#[inline(always)]
pub fn TIM9() -> &mut TIMType {unsafe {&mut *(TIM9_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM10() -> &mut TIMType {unsafe {&mut *(TIM10_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn TIM11() -> &mut TIMType {unsafe {&mut *(TIM11_BASE!() as *mut TIMType)}}
#[inline(always)]
pub fn GPIOA() -> &mut GPIOType {unsafe {&mut *(GPIOA_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOB() -> &mut GPIOType {unsafe {&mut *(GPIOB_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOC() -> &mut GPIOType {unsafe {&mut *(GPIOC_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOD() -> &mut GPIOType {unsafe {&mut *(GPIOD_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOE() -> &mut GPIOType {unsafe {&mut *(GPIOE_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOF() -> &mut GPIOType {unsafe {&mut *(GPIOF_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOG() -> &mut GPIOType {unsafe {&mut *(GPIOG_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOH() -> &mut GPIOType {unsafe {&mut *(GPIOH_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn GPIOI() -> &mut GPIOType {unsafe {&mut *(GPIOI_BASE!() as *mut GPIOType)}}
#[inline(always)]
pub fn CRC() -> &mut CRCType {unsafe {&mut *(CRC_BASE!() as *mut CRCType)}}
#[inline(always)]
pub fn RCC() -> &mut RCCType {unsafe {&mut *(RCC_BASE!() as *mut RCCType)}}
#[inline(always)]
pub fn FLASH() -> &mut FLASHType {unsafe {&mut *(FLASH_R_BASE!() as *mut FLASHType)}}
#[inline(always)]
pub fn DMA1() -> &mut DMAType {unsafe {&mut *(DMA1_BASE!() as *mut DMAType)}}
#[inline(always)]
pub fn DMA1_Stream0() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream0_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA1_Stream1() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream1_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA1_Stream2() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream2_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA1_Stream3() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream3_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA1_Stream4() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream4_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA1_Stream5() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream5_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA1_Stream6() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream6_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA1_Stream7() -> &mut DMA_StreamType {unsafe {&mut *(DMA1_Stream7_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2() -> &mut DMAType {unsafe {&mut *(DMA2_BASE!() as *mut DMAType)}}
#[inline(always)]
pub fn DMA2_Stream0() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream0_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2_Stream1() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream1_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2_Stream2() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream2_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2_Stream3() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream3_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2_Stream4() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream4_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2_Stream5() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream5_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2_Stream6() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream6_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn DMA2_Stream7() -> &mut DMA_StreamType {unsafe {&mut *(DMA2_Stream7_BASE!() as *mut DMA_StreamType)}}
#[inline(always)]
pub fn ETH() -> &mut ETHType {unsafe {&mut *(ETH_BASE!() as *mut ETHType)}}  
#[inline(always)]
pub fn DCMI() -> &mut DCMIType {unsafe {&mut *(DCMI_BASE!() as *mut DCMIType)}}
#[inline(always)]
pub fn CRYP() -> &mut CRYPType {unsafe {&mut *(CRYP_BASE!() as *mut CRYPType)}}
#[inline(always)]
pub fn HASH() -> &mut HASHType {unsafe {&mut *(HASH_BASE!() as *mut HASHType)}}
#[inline(always)]
pub fn RNG() -> &mut RNGType {unsafe {&mut *(RNG_BASE!() as *mut RNGType)}}
#[inline(always)]
pub fn FSMC_Bank1() -> &mut FSMC_Bank1Type {unsafe {&mut *(FSMC_Bank1_R_BASE!() as *mut FSMC_Bank1Type)}}
#[inline(always)]
pub fn FSMC_Bank1E() -> &mut FSMC_Bank1EType {unsafe {&mut *(FSMC_Bank1E_R_BASE!() as *mut FSMC_Bank1EType)}}
#[inline(always)]
pub fn FSMC_Bank2() -> &mut FSMC_Bank2Type {unsafe {&mut *(FSMC_Bank2_R_BASE!() as *mut FSMC_Bank2Type)}}
#[inline(always)]
pub fn FSMC_Bank3() -> &mut FSMC_Bank3Type {unsafe {&mut *(FSMC_Bank3_R_BASE!() as *mut FSMC_Bank3Type)}}
#[inline(always)]
pub fn FSMC_Bank4() -> &mut FSMC_Bank4Type {unsafe {&mut *(FSMC_Bank4_R_BASE!() as *mut FSMC_Bank4Type)}}
#[inline(always)]
pub fn DBGMCU() -> &mut DBGMCUType {unsafe {&mut *(DBGMCU_BASE!() as *mut DBGMCUType)}}

#[inline(always)]
pub fn NVIC() -> &mut NVICType {unsafe {&mut *(NVIC_BASE!() as *mut NVICType)}}

