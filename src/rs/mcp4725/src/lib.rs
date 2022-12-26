#![no_std]
use rnarduino::rn_i2c::rnI2C as rnI2C;


const MCP4725_CMD_WRITEDACEEPROM  : u8 = 3<<5;  // Writes data to the DAC and the EEPROM (persisting the assigned value after reset)
pub const MCP4725_DEFAULT_ADDRESS : u8 = 0x60;

pub struct MCP4725
{
    i2c     : rnI2C,
    address : u8,
}

impl MCP4725
{
    pub fn new( instance : usize, address : u8 , speed : usize) -> Self
    {
        MCP4725
        {
            i2c     : rnI2C::new(instance as u32,speed as u32), // we can create new ones, they are alisased in the C part
            address : address ,
        }
    }

    pub fn set_voltage( &mut self, output : usize )
    {
        let data : [u8;2]=[((output>>8)&0xf) as u8,(output&0xff) as u8];
        self.i2c.write_to(self.address,&data);
    }

    pub fn set_default_value( &mut self, output : usize )
    {
        let data : [u8;3]=[MCP4725_CMD_WRITEDACEEPROM, (output>>4) as u8,((output&0xf)<<4) as u8];
        self.i2c.write_to(self.address,&data);
    }
}
// EOF
