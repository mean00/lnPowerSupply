
use super::main_loop;
use crate::settings::*;

use rnarduino::rnOsHelper::{rnCreateTask,rnTaskEntry,rnDelay};
use cty::c_void;
use crate::app::PeripheralEvent;

//-----------------------------------------------------------
impl  <'a> main_loop  <'a>
{    
    pub fn trampoline( param : *mut c_void)
    {
        unsafe {
            let me:  &mut main_loop = &mut (*(param as *mut main_loop));
            me.run_slave_task();
        }
      
    }
  


    fn internal_notify(&mut self, event : PeripheralEvent)
    {        
        self.notify(event);
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

