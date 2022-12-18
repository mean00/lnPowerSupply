#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use rnarduino as rn;
use rn::rnGpio::rnPin;
use rn::rnGpio;
use pcf8574::PC8754;
use ina219::INA219;
use rn::rnOsHelper::*;
use crate::settings::*;
use heapless::String;

use ufmt::{derive::uDebug, uDebug, uWrite, uwrite, uwriteln, Formatter};
use core::str;
use mcp4725::MCP4725;

use crate::i2c_rs_task::power_supply_peripheral;
use crate::i2c_rs_task::peripheral_notify;
use crate::i2c_rs_task::PeripheralEvent;
/**
 *  A => shut volt / 10.2 => amp
 *  102 mOhm
 * 
 * 
 */
pub fn test_me()
{
    let mut x : main_loop = main_loop {} ;
    x.run();
}
struct main_loop
{

}
impl peripheral_notify for main_loop
{
    fn notify(&self, event :  PeripheralEvent)
    {

    }
}
impl main_loop
{
    pub fn run(&mut self)
    {
    // create the i2c fake task
    let tsk : power_supply_peripheral = power_supply_peripheral::new(self);
    let mut led : rnPin  = rnPin::PC13;
    rnGpio::pin_mode(led, rnGpio::rnGpioMode::lnOUTPUT);
    loop
        {
            rnGpio::digital_toggle(led);
            rnDelay(1000);
        }
    }
}
// EOF