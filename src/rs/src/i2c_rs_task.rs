#![allow(non_upper_case_globals)]
#![allow(unused_variables)]
#![allow(unused_code)]
extern crate alloc;

use alloc::boxed::Box;
use rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnGpio::rnPin as rnPin;
use rn::rnOsHelper::rnLogger as rnLogger;
use cty::c_void;
use crate::settings::{*};
use rnarduino::rnOsHelper::{rnCreateTask,rnTaskEntry,rnDelay};
use pcf8574::PC8754;
use ina219::INA219;
use mcp4725::MCP4725;

use core::str;

pub enum PeripheralEvent
{
    CCChangeEvent=1,
    VoltageChangeEvent=2,
}
//------------------------------------------------------------
pub trait peripheral_notify
{
    fn notify(&self, event :  PeripheralEvent);
}
//-----------------------------------------------------------
pub trait peripheral_interface
{

            fn  get_voltage(&mut self) -> f32;
            fn  get_current_ma(&mut self) -> usize;
            fn  set_max_current(&mut self, milliamp : usize) ;
            fn  cc_limited(&mut self) -> bool;
            fn  dcdc_enable(&mut self, enable : bool) ;
            fn  output_enable(&mut self, enable: bool) ;            
}
//-----------------------------------------------------------
pub struct power_supply_peripheral
{
    pc8574   : PC8754,
    ina219   : INA219,
    mcp4725  : MCP4725,

    current_volt            : f32,
    current_ma              : usize, 
    current_max_current     : usize,
    current_dc_enabled      : bool,
    current_relay_enabled   : bool, 
    current_cc              : bool,


    updated_max_current     : usize,
    updated_dc_enabled      : bool, 
    updated_relay_enabled   : bool,

    callback                : &'static dyn peripheral_notify, 

}
//-----------------------------------------------------------
impl power_supply_peripheral
{
    pub fn new( cb : &'static dyn peripheral_notify) -> Self
    {        
        let mut r = power_supply_peripheral
        {
            pc8574   : PC8754::new(PS_I2C_INSTANCE, IO_EXPANDER_ADDRESS as u8),
            ina219   : INA219::new(PS_I2C_INSTANCE as usize, INA219_ADDRESS as u8,  100*1000, INA219_SHUNT_VALUE ,3),
            mcp4725  : MCP4725::new( PS_I2C_INSTANCE as usize, MCP4725_ADDRESS , 100*1000),
            current_volt            : 0.,
            current_ma              : 0, 
            current_max_current     : 200,
            current_dc_enabled      : false,
            current_relay_enabled   : false, 
            current_cc              : false,
        
            callback                : cb,
        
            updated_max_current     : 200,
            updated_dc_enabled      : false, 
            updated_relay_enabled   : false,
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
  
    pub fn start(&mut self)
    {        
        let mut myself =  self as *mut _ as *mut c_void;
        rnCreateTask( &(Self::trampoline as rnTaskEntry) , "i2c",I2C_TASK_PRIORITY, 1024, myself);
    }

    fn notify(&mut self, event : PeripheralEvent)
    {
        self.callback.notify(event);
    }

    //----------------------
    // This will actually look after sensors
    // and enforce the changes
    //----------------------
    pub fn run(&mut self)
    {
        self.pc8574.write(IO_EXPANDER_DC_ENABLE, false);    // DC/DC is active low
        self.pc8574.write(IO_EXPANDER_RELAY_ENABLE,false);  //
        self.mcp4725.set_voltage(200); // default low value
        loop
        {
            rnDelay(10); 
            let current = self.ina219.get_current_ma();
            if current!=self.current_ma
            {
                self.current_ma = current;
                self.notify(PeripheralEvent::VoltageChangeEvent);
            }
            // CC ?
            self.pc8574.refresh();
            let cc = self.pc8574.read(IO_EXPANDER_CC_MODE);
            if cc != self.current_cc
            {
                self.current_cc = cc;
                self.notify(PeripheralEvent::CCChangeEvent);
            }
            // limit ?
            let max_current = self.updated_max_current;
            if max_current != self.current_max_current
            {
                    self.current_max_current = max_current;
                    self.mcp4725.set_voltage(max_current);
            }
            let dc_enabled = self.updated_dc_enabled;
            let relay_enabled = self.updated_relay_enabled;

            // something changed ?
            if self.current_dc_enabled!=dc_enabled ||  self.current_relay_enabled!=relay_enabled
            {
                self.current_dc_enabled  = dc_enabled;
                self.current_relay_enabled  = relay_enabled;
                
                self.pc8574.write(IO_EXPANDER_DC_ENABLE, !self.current_dc_enabled); // active low
                self.pc8574.write(IO_EXPANDER_RELAY_ENABLE, !self.current_relay_enabled); // active low
            }
        }
    }
}
//-----------------------------------------------------------
impl peripheral_interface for power_supply_peripheral
{
    fn  get_voltage(&mut self) -> f32
    {
        self.current_volt
    }
    fn  get_current_ma(&mut self) -> usize
    {
        self.current_ma
    }
    fn  set_max_current(&mut self, milliamp : usize)
    {
        self.updated_max_current=milliamp;
    }
    fn  cc_limited(&mut self) -> bool
    {
        self.current_cc
    }
    //--
    fn  dcdc_enable(&mut self, enable : bool)
    {
        self.updated_dc_enabled = enable;         
    }
    fn  output_enable(&mut self, enable: bool)
    {
        self.updated_relay_enabled = enable;
    }    
}

