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
use rn::rnOsHelper::*;
use crate::settings::*;

pub fn test_me()
{

    let mut pcf : PC8754 = PC8754::new(PS_I2C_INSTANCE, IO_EXPANDER_ADDRESS as u8);

    pcf.write(IO_EXPANDER_DC_ENABLE, false); // DC/DC is active low
    pcf.write(IO_EXPANDER_RELAY_ENABLE,false);

    while true
    {
        rnDelay(10);
    }

}