#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_code)]
extern crate alloc;

use alloc::boxed::Box;
use rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnGpio::rnPin as rnPin;
use rn::rnExti as rnExti;
use rn::rnFastEventGroup::rnFastEventGroup;
use rn::rnTimingAdc::rnTimingAdc;
use rn::rnOsHelper::rnLogger as rnLogger;
use cty::c_void;
use crate::settings::{*};
use rnarduino::rnOsHelper::{rnCreateTask,rnTaskEntry,rnDelay};
//-----------------------------------------------------------
pub trait peripheral_interface
{

            fn  get_voltage() -> f32;
            fn  get_current_ma() -> usize;
            fn  set_max_current(milliamp : usize) ;
            fn  cc_limited() -> bool;
            fn  dcdc_enable(enable : bool) ;
            fn  output_enable(enable: bool) ;
            //fn  setCb(lnI2cTask::signalCb *c) ;
}
//-----------------------------------------------------------
pub struct power_supply_peripheral
{

}
//-----------------------------------------------------------
impl power_supply_peripheral
{
    pub fn new() -> Self
    {
        //pub fn rnCreateTask(entry : &rnTaskEntry, name: &str, priority : usize, taskSize : u32)
        let mut r = power_supply_peripheral
        {

        };
        r.start();
        r
    }
    pub fn trampoline( param : *mut c_void)
    {
        unsafe {
            let mut me:  &mut power_supply_peripheral = &mut (*(param as *mut power_supply_peripheral));
            me.run();
        }
      
    }
    pub fn run(&mut self)
    {
        loop
        {
            rnLogger("Hey!!\n");
            rnDelay(1000);
        }
    }
    pub fn start(&mut self)
    {        
        let mut myself =  self as *mut _ as *mut c_void;
        rnCreateTask( &(Self::trampoline as rnTaskEntry) , "i2c",I2C_TASK_PRIORITY, 1024, myself);
    }

}
//-----------------------------------------------------------
impl peripheral_interface for power_supply_peripheral
{
    fn  get_voltage() -> f32
    {
        5.
    }
    fn  get_current_ma() -> usize
    {
        120
    }
    fn  set_max_current(milliamp : usize)
    {

    }
    fn  cc_limited() -> bool
    {
        false
    }
    fn  dcdc_enable(enable : bool)
    {
        
    }
    fn  output_enable(enable: bool)
    {
        
    }
}