#![no_std]
#![allow(dead_code)]

use rnarduino::rnI2C::rnI2C as rnI2C;
mod ina219_regs;
pub const INA219_ADDRESS  : u8 = 0x40;
use crate::ina219_regs::*;
use rnarduino::rnOsHelper::rnDelay;

#[derive(Copy, Clone)]
pub enum  PGA
{
    PGA1= 0,
    PGA2= 1,
    PGA4= 2,
    PGA8= 3,
}

pub struct INA219
{
    i2c         : rnI2C,
    address     : u8,
    shunt       : usize,
    multiplier  : f32,
    scale       : PGA,
    high_voltage: bool, 
}
//-------------------------------------------
fn value_to_volt(value: usize ) -> f32
{
    let f = ((value>>3)*4) as f32;   
    f/1000.
}

impl INA219
{
    //-------------------------------------------
    pub fn new( instance : usize, address : u8 , speed : usize, shunt_in_mohm : usize, max_current_ampere : usize) -> Self
    {
        let multi : f32 = (40./(shunt_in_mohm as f32))/4.096;
        let max_shunt_voltage = shunt_in_mohm*max_current_ampere;
        let divider : PGA ;

        if max_shunt_voltage>320
        {
            panic!("oops");// wrong configuration
        } 
        else
        {
            divider = match max_shunt_voltage
            {
                160..=319 => PGA::PGA8,
                80..=159  => PGA::PGA4,
                40..=79   => PGA::PGA2,
                _         => PGA::PGA1,
            };
        }
        //  compute cal  
        let mut calf : f32 =0.04096;
        calf/=(shunt_in_mohm as f32)/1000.;
        calf/=(max_current_ampere as f32)/32768.; // max 4A
        let cal=calf as u16;
  

        let mut r=INA219
        {
            i2c         : rnI2C::new(instance as u32,speed as u32), // we can create new ones, they are alisased in the C part
            address     : address ,
            shunt       : shunt_in_mohm,
            multiplier  : multi,
            scale       : divider,
            high_voltage: false,
        };
        r.calibrate(cal);
        r.reconfigure();
        r
    }
    //-------------------------------------------    
    pub fn calibrate(&mut self, cal : u16)
    {
        self.write_register(INA219_REG_CALIBRATION,cal); // 4A max, 20 Ohm
    }
    //-------------------------------------------    
    pub fn get_current_scaler(& self)  -> PGA
    {
        return self.scale;
    }
    //-------------------------------------------
    pub fn  get_shunt_voltage_raw(&mut self)-> u16 // actually u16
    {
       return self.read_register(INA219_REG_BUSVOLTAGE);
    }
    //-------------------------------------------
    pub fn  get_shunt_voltage_mv(&self)-> isize 
    {
        return 0;
    }
    //-------------------------------------------
    pub fn  get_voltage_v(&mut self)-> f32
    {
        let mut value = self.get_bus_voltage_raw() as usize;
  
        let mut flat=value_to_volt(value);
        let mut redo : bool =false;
      
        if !self.high_voltage && flat > 15. // switch to high voltage
        {
              self.high_voltage=true;
              redo=true;   
        }
        if !redo && self.high_voltage && flat < 13.
        {
            self.high_voltage=false;
            redo=true;
        }
        if redo 
        {
              self.reconfigure();    
              rnDelay(5);
              value = self.get_bus_voltage_raw() as usize;
              flat=value_to_volt(value);
        }
        flat      
    }
    //-------------------------------------------
    pub fn  get_current_ma(&mut self)-> usize
    {
        let  current = self.read_register(INA219_REG_CURRENT);
        if (current & (1<<15))!=0
        {
           return 0; // we dont support negative
        }
        (current as usize+5)/10
    }
    //-------------------------------------------
    fn     reconfigure(&mut self)
    {
        // Set Config register to take into account the settings above
        let  config :u16 = 0+
            match self.high_voltage
            {
                true => INA219_CONFIG_BVOLTAGERANGE_32V,
                false => 0,
            }+      INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS |
            (10<<3) | // both I & V = 12 bits, 4 samples
            (10<<7) + 
            ((self.scale as u16)<<11)
            ;              
        self.write_register(INA219_REG_CONFIG, config);
    }
    //-------------------------------------------
    fn    write_register(&mut self, reg : u8, value : u16)
    {
        let datas : [u8;3]= [ reg, (value>>8) as u8, (value&0xff) as u8 ];
        self.i2c.write_to(self.address,&datas);    
    }
    //-------------------------------------------
    fn read_register(&mut self, reg : u8) -> u16
    {
        let mut datas : [u8;2]=[0,0];
        let regs  : [u8;1]=[reg];
        self.i2c.write_to(self.address,&regs);
        self.i2c.read_from(self.address, &mut datas);
        let v =  ((datas[0] as u16)<<8) as u16+datas[1] as u16;
        v
    }
    //-------------------------------------------
    fn get_bus_voltage_raw(&mut self)->u16
    {
        self.read_register(INA219_REG_BUSVOLTAGE)
    }
    //-------------------------------------------
    fn set_scaler(&mut self,  scale : PGA)
    {
       self.scale = scale;
    }
}