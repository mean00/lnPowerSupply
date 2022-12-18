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
use mcp4725::MCP4725;
/**
 *  A => shut volt / 10.2 => amp
 *  102 mOhm
 * 
 * 
 */

pub fn test_me()
{
    // PC84
    let mut pcf : PC8754 = PC8754::new(PS_I2C_INSTANCE, IO_EXPANDER_ADDRESS as u8);

    pcf.write(IO_EXPANDER_DC_ENABLE, false); // DC/DC is active low
    pcf.write(IO_EXPANDER_RELAY_ENABLE,false);

   

    // INA
    let mut string_buf: String<64>  =  String::new();
    let mut ina : INA219 = INA219::new(PS_I2C_INSTANCE as usize, INA219_ADDRESS as u8,  100*1000, INA219_SHUNT_VALUE ,3);

    let mut mcp475 : MCP4725 = MCP4725::new( PS_I2C_INSTANCE as usize, MCP4725_ADDRESS , 100*1000);

    mcp475.set_voltage(1024);


    loop
    {
        let mut a = ina.get_shunt_voltage_raw();
        let mut b = ina.get_current_ma();
        if b> INA219_OFFSET
        {
            b-= INA219_OFFSET;
        }else
        {
            b=0;
        }
        if(b<INA219_MIN)
        {
            b=0;
        }
        let mut v = (ina.get_voltage_v()*1000.) as usize;
        string_buf.clear();
        uwrite!(&mut string_buf, "V:{}  Araw:{}  B:{}\n",v,a,b).unwrap();
        rnLogger(&string_buf);

    }

    loop
    {
        let volt = ina.get_voltage_v();
        string_buf.clear();
        //uwrite!(&mut string_buf, "Volt:{}\n",(1000.*volt) as usize).unwrap();
       // rnLogger(&string_buf);

        let mut milli_amp=ina.get_current_ma();
        if(milli_amp>INA219_OFFSET)
        {
            milli_amp=milli_amp-INA219_OFFSET;
        }
        if(milli_amp<INA219_MIN)
        {
            milli_amp=0;
        }
        uwrite!(&mut string_buf, "Amp:{}\n",milli_amp).unwrap();
        rnLogger(&string_buf);
        
    }

}