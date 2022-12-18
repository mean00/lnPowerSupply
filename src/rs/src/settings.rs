#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]

use rnarduino as rn;
use rn::rnGpio::rnPin as rnPin;

pub const PS_I2C_INSTANCE    : usize =  1;

// INA
pub const INA219_SHUNT_VALUE    : usize =    109;// 106 mOhm;
pub const INA219_ADDRESS        : u8 =       0x40; //addr: u8,
pub const INA219_OFFSET         : usize =    25;
pub const INA219_MIN            : usize =    5;

// screen 
pub const ILI_SPI_INSTANCE   : usize =  0;
pub const ILI_PIN_DC         : rnPin =  rnPin::PA4 ;
pub const ILI_PIN_CS         : rnPin =  rnPin::PB1 ;
pub const ILI_PIN_RESET      : rnPin =  rnPin::PB0 ;

pub const PIN_LED            : rnPin =  rnPin::PA3 ;
pub const PIN_SWITCH         : rnPin =  rnPin::PB9 ;

pub const PS_PIN_VBAT        : rnPin =  rnPin::PA0 ;
pub const PS_PIN_MAX_CURRENT : rnPin =  rnPin::PA1 ;
pub const PS_INTERNAL_RESISTANCE_MILLIOHM : usize = 300; // error on voltage is  ~ 50 mV

// IO Expander - PCF8574
pub const IO_EXPANDER_ADDRESS        : usize =  32;
pub const IO_EXPANDER_DC_ENABLE      : usize =  0; // bit 0 - output
pub const IO_EXPANDER_RELAY_ENABLE   : usize =  1; // bit 1 - output
pub const IO_EXPANDER_CC_MODE        : usize =  2; // bit 2 - input

// MCP4725
pub const MCP4725_ADDRESS             : u8 = 0x60;

// Tasks 
pub const I2C_TASK_PRIORITY : usize = 2;


// VBAT limit
pub const PS_MIN_VBAT         : f32 = 16.; // at start
pub const PS_MIN_VBAT_CRIT    : f32 = 15.; // anytime

//
pub const WIRE_RESISTANCE_MOHM  : usize = 220; // wire resistance

pub const ADC_SAMPLE : usize = 8;

pub const   EnableButtonEvent : u32 = 128;


// EOF
