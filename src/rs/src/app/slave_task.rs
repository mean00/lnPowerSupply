/**
 *  This is the slave task that periodically polls the peripherals on the i2c bus 
 * to get voltage, current, are we in cc mode
 *  It also accepts command that are sent trough the updated_vars
 *  the task enforces these commands in its own context a little bit later
 * 
 */

use crate::settings::*;
extern crate alloc;
use rnarduino::rnOsHelper::{rnCreateTask,rnTaskEntry,rnDelay};
use cty::c_void;
use pcf8574::PC8754;
use ina219::INA219;
use mcp4725::MCP4725;

pub type peripheral_callback =  fn( cookie: *mut c_void, event : PeripheralEvent);

pub enum PeripheralEvent
{
    CCChangeEvent     = 1,
    VoltageChangeEvent= 2,
    CurrentChangeEvent= 4,
    EnableButtonEvent = 8,
}



pub struct i2c_task 
{
    // slave task part
    current_volt            : f32,
    current_ma              : usize,
    current_max_current     : usize,
    current_dc_enabled      : bool,
    current_relay_enabled   : bool,
    current_cc              : bool,


    updated_max_current     : usize,
    updated_dc_enabled      : bool,
    updated_relay_enabled   : bool,

    pc8574                  : PC8754,
    ina219                  : INA219,
    mcp4725                 : MCP4725,

    obs                     : Option<peripheral_callback>,
    obs_data                : Option<*mut  c_void>,
}
//-----------------------------------------------------------
impl   <'a> i2c_task  
{    
    pub fn trampoline( param : *mut c_void)
    {
        unsafe {
            let me:  &mut i2c_task = &mut (*(param as *mut i2c_task));
            me.run_slave_task();
        }
      
    }
  
    pub fn new() -> Self
    {
        i2c_task{
                //-- slave thread --
                pc8574      : PC8754::new(PS_I2C_INSTANCE, IO_EXPANDER_ADDRESS as u8),
                ina219      : INA219::new(PS_I2C_INSTANCE as usize, INA219_ADDRESS as u8,  100*1000, INA219_SHUNT_VALUE ,3),
                mcp4725     : MCP4725::new( PS_I2C_INSTANCE as usize, MCP4725_ADDRESS , 100*1000),
                current_volt            : 0.,
                current_ma              : 0,
                current_max_current     : 200,
                current_dc_enabled      : false,
                current_relay_enabled   : false,
                current_cc              : false,

                updated_max_current     : 200,
                updated_dc_enabled      : false,
                updated_relay_enabled   : false,
                obs                     : None, 
                obs_data                : None, 
        }
    }
    //
    //
    //
    pub fn set_observer(&mut self,  func : peripheral_callback,  data : *mut  c_void)
    {        
        self.obs = Some(func);
        self.obs_data = Some(data);
    }
    fn internal_notify(&mut self, event : PeripheralEvent)
    {        
        if let Some(x) = self.obs
        {
            (x)( self.obs_data.unwrap(), event);
        }      
    }
    //----------------------
    // This will actually look after sensors
    // and enforce the changes
    //----------------------
    pub fn run_slave_task(&mut self)
    {
        self.pc8574.write(IO_EXPANDER_DC_ENABLE, false);    // DC/DC is active low
        self.pc8574.write(IO_EXPANDER_RELAY_ENABLE,false);  //
        self.mcp4725.set_voltage(200); // default low value
        loop
        {
            rnDelay(10); 
            let voltage =  self.ina219.get_voltage_v();
            if voltage != self.current_volt
            {
                self.current_volt = voltage;
                self.internal_notify(PeripheralEvent::VoltageChangeEvent);
            }
            let current = self.ina219.get_current_ma();
            if current!=self.current_ma
            {
                self.current_ma = current;
                self.internal_notify(PeripheralEvent::CurrentChangeEvent);
            }
            // CC ?
            self.pc8574.refresh();
            let cc = self.pc8574.read(IO_EXPANDER_CC_MODE);
            if cc != self.current_cc
            {
                self.current_cc = cc;
                self.internal_notify(PeripheralEvent::CCChangeEvent);
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

    pub fn start_slave_task(&mut self) //,  cb : &'a dyn peripheral_notify)
    {             
        let myself =  self as *mut _ as *mut c_void;
        rnCreateTask( &(Self::trampoline as rnTaskEntry) , "i2c",I2C_TASK_PRIORITY, 1024, myself);
    }
    pub fn  voltage(&mut self) -> f32
    {
        self.current_volt
    }
    pub fn  current_ma(&mut self) -> usize
    {
        self.current_ma
    }
    pub fn  set_max_current(&mut self, milliamp : usize)
    {
        self.updated_max_current=milliamp;
    }
    pub fn  cc_limited(&mut self) -> bool
    {
        self.current_cc
    }
    //--
    pub fn  set_dcdc_enable(&mut self, enable : bool)
    {
        self.updated_dc_enabled = enable;         
    }
    pub fn  set_output_enable(&mut self, enable: bool)
    {
        self.updated_relay_enabled = enable;
    }    
}

