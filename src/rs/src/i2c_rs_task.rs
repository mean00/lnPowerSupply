#![allow(non_upper_case_globals)]

extern crate alloc;

use alloc::boxed::Box;
use rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnGpio::rnPin as rnPin;
use rn::rnExti as rnExti;
use rn::rnFastEventGroup::rnFastEventGroup;
use rn::rnTimingAdc::rnTimingAdc;
use rn::rnOsHelper::rnLogger as rnLogger;


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

struct power_supply_peripheral
{

}

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