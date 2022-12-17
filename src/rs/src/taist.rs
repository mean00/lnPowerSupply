#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use rnarduino as rn;
use pcf8574::PC8754;
use ina219::INA219;
use rn::rnOsHelper::*;
use crate::settings::*;
use heapless::String;

use ufmt::{derive::uDebug, uDebug, uWrite, uwrite, uwriteln, Formatter};
use core::str;


pub fn test_me()
{
    // PC84
    let mut pcf : PC8754 = PC8754::new(PS_I2C_INSTANCE, IO_EXPANDER_ADDRESS as u8);

    pcf.write(IO_EXPANDER_DC_ENABLE, false); // DC/DC is active low
    pcf.write(IO_EXPANDER_RELAY_ENABLE,false);

   

    // INA
    let mut string_buf: String<64>  =  String::new();
    let mut ina : INA219 = INA219::new(PS_I2C_INSTANCE as usize, INA219_ADDRESS as u8,  100*1000, 10 ,3);
    loop
    {
        let volt = ina.get_voltage_v();
        string_buf.clear();
        uwrite!(&mut string_buf, "Volt:{}\n",(1000.*volt) as usize).unwrap();
        rnLogger(&string_buf);
    }

}