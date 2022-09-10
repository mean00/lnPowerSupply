#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]

use rnarduino as rn;


pub const PS_I2C_INSTANCE    : usize =  1;

// INA
pub const INA_SHUNT_VALUE  : usize =    106;// 106 mOhm;

// screen 
pub const ILI_SPI_INSTANCE   : usize =  0;
pub const ILI_PIN_DC         : rn::rnGPIO::rnPin =  rn::rnGPIO::rnPin::PA4 ;
pub const ILI_PIN_CS         : rn::rnGPIO::rnPin =  rn::rnGPIO::rnPin::PB1 ;
pub const ILI_PIN_RESET      : rn::rnGPIO::rnPin =  rn::rnGPIO::rnPin::PB0 ;

pub const PIN_LED            : rn::rnGPIO::rnPin =  rn::rnGPIO::rnPin::PA3 ;
pub const PIN_SWITCH         : rn::rnGPIO::rnPin =  rn::rnGPIO::rnPin::PB9 ;

pub const PS_PIN_VBAT        : rn::rnGPIO::rnPin =  rn::rnGPIO::rnPin::PA0 ;
pub const PS_PIN_MAX_CURRENT : rn::rnGPIO::rnPin =  rn::rnGPIO::rnPin::PA1 ;
pub const PS_INTERNAL_RESISTANCE_MILLIOHM : usize = 300; // error on voltage is  ~ 50 mV

// IO Expander
pub const IO_EXPANDER_ADDRESS        : usize =  32;
pub const IO_EXPANDER_DC_ENABLE      : usize =  0;
pub const IO_EXPANDER_RELAY_ENABLE   : usize =  1;
pub const IO_EXPANDER_CC_MODE        : usize =  2;

// PCF8574
pub const PCF8574_ADDRESS         : usize = 32;
pub const PCF8574_DCDC_ENABLE     : usize = 1;// out
pub const PCF8574_RELAY_ENABLE    : usize = 2;// out
pub const PCF8574_CC_MODE         : usize = 4;// in

// Tasks 
pub const I2C_TASK_PRIORITY : usize = 2;


// VBAT limit
pub const PS_MIN_VBAT         : f32 = 16.; // at start
pub const PS_MIN_VBAT_CRIT    : f32 = 15.; // anytime

//
pub const WIRE_RESISTANCE_MOHM  : usize = 220; // wire resistance

pub const INA219_ADDRESS  : u8 = 0x40; //addr: u8,
pub const INA_MA_FLOOR : usize = 24;
pub const ADC_SAMPLE : usize = 8;

pub const   EnableButtonEvent : u32 = 128;


// EOF