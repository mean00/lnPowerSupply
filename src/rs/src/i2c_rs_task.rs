
#![allow(unused_variables)]
extern crate alloc;
use cty::c_void;
use crate::settings::{*};
use rnarduino::rnOsHelper::{rnCreateTask,rnTaskEntry,rnDelay};
use pcf8574::PC8754;
use ina219::INA219;
use mcp4725::MCP4725;


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
pub trait   peripheral_interface <'a>
{
            fn  start(&mut self,  cb : &'a dyn peripheral_notify);
            fn  voltage(&mut self) -> f32;
            fn  current_ma(&mut self) -> usize;
            fn  set_max_current(&mut self, milliamp : usize) ;
            fn  cc_limited(&mut self) -> bool;
            fn  set_dcdc_enable(&mut self, enable : bool) ;
            fn  set_output_enable(&mut self, enable: bool) ;            
}
//-----------------------------------------------------------
pub struct  power_supply_peripheral <'a>
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

    callback                : Option<&'a dyn peripheral_notify>,

}
//-----------------------------------------------------------
impl <'a> power_supply_peripheral  <'a>
{
    pub fn new() -> Self
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
        
            callback                : None,
        
            updated_max_current     : 200,
            updated_dc_enabled      : false, 
            updated_relay_enabled   : false,
        };      
        r
    }
    pub fn trampoline( param : *mut c_void)
    {
        unsafe {
            let me:  &mut power_supply_peripheral = &mut (*(param as *mut power_supply_peripheral));
            me.run();
        }
      
    }
  


    fn notify(&mut self, event : PeripheralEvent)
    {
        match self.callback
        {
            Some(x) => x.notify(event),
            None => panic!("no callback"),
        }
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
impl <'a> peripheral_interface <'a> for power_supply_peripheral <'a>
{
    fn start(&mut self,  cb : &'a dyn peripheral_notify)
    {        
        self.callback = Some(cb);
        let myself =  self as *mut _ as *mut c_void;
        rnCreateTask( &(Self::trampoline as rnTaskEntry) , "i2c",I2C_TASK_PRIORITY, 1024, myself);
    }
    fn  voltage(&mut self) -> f32
    {
        self.current_volt
    }
    fn  current_ma(&mut self) -> usize
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
    fn  set_dcdc_enable(&mut self, enable : bool)
    {
        self.updated_dc_enabled = enable;         
    }
    fn  set_output_enable(&mut self, enable: bool)
    {
        self.updated_relay_enabled = enable;
    }    
}

