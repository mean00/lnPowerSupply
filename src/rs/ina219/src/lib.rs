#![no_std]
use rnarduino::rnI2C::rnI2C as rnI2C;
mod ina219_regs;
pub const INA219_ADDRESS  : u8 = 0x40;
use crate::ina219_regs::*;

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
    i2c     : rnI2C,
    address : u8,
    shunt   : usize,
    multiplier : f32,
    scale   : PGA,
}

impl INA219
{
    pub fn get_current_scaler(& self)  -> PGA
    {
        return self.scale;
    }

    pub fn  get_shunt_voltage_raw(&self)-> u16 // actually u16
    {
        return 0;
    }
    pub fn  get_shunt_voltage_mv(&self)-> isize 
    {
        return 0;
    }

    pub fn  get_voltage_v(&self)-> f32
    {
        return 0.;
    }
    pub fn  get_current_ma(&mut self)-> usize
    {
        let  current = self.read_register(INA219_REG_CURRENT);
        if (current & (1<<15))!=0
        {
           return 0; // we dont support negative
        }
        (current as usize+5)/10
    }
    fn calibrate(&mut self)
    {

    }
  

    pub fn new( instance : usize, address : u8 , speed : usize, shunt_in_mohm : usize, max_current_A : usize) -> Self
    {
        INA219
        {
            i2c         : rnI2C::new(instance as u32,speed as u32), // we can create new ones, they are alisased in the C part
            address     : address ,
            shunt       : shunt_in_mohm,
            multiplier  : 0.0,
            scale       : PGA::PGA1,
        }
    }

    fn     reconfigure(&mut self)
    {

    }
    fn    write_register(&mut self, reg : u8, value : u16)
    {
        let datas : [u8;3]= [ reg, (value>>8) as u8, (value&0xff) as u8 ];
        self.i2c.write_to(self.address,&datas);    
    }
    fn read_register(&mut self, reg : u8) -> u16
    {
        let mut datas : [u8;2]=[0,0];
        let regs  : [u8;1]=[reg];
        self.i2c.write_to(self.address,&regs);
        self.i2c.read_from(self.address, &mut datas);
        let v =  ((datas[0] as u16)<<8) as u16+datas[1] as u16;
        v
    }
    fn get_bus_voltage_raw(&mut self)->u16
    {
        return 0;
    }
    fn get_single_current_ma(&mut self) -> isize
    {
        return 0;
    }
    fn set_scaler(&mut self,  scale : PGA)
    {
       //  {_currentIScale=nw; }
    }
}