#![no_std]

use rust_esprit::rn_i2c::rnI2C;
pub const PC8754_DEFAULT_ADDRESS: u8 = 0x20;

/**
 * The PC875x are very simple.
 * They are output with very weak pullup
 * What you read is actually the "actual" value of the output
 * So if it is pulled down, even if it is configured as "1"  you'll read 0
 *
 *
 */

pub struct PC8754 {
    i2c: rnI2C,
    address: u8,
    conf: u8,
    last_read: u8,
}
impl PC8754 {
    pub fn new(instance: usize, address: u8) -> Self {
        let mut r = PC8754 {
            i2c: rnI2C::new(instance as u32, 100 * 1000), // no need to be fast..
            address: address,
            conf: 0xff,
            last_read: 0xff,
        };
        r.default_configuration();
        r
    }
    fn default_configuration(&mut self) {
        self.i2c.begin(self.address);
        self.i2c.write_to(self.address, &[self.conf]);
    }
    pub fn refresh(&mut self) {
        let mut data: [u8; 1] = [0];
        self.i2c.read_from(self.address, &mut data);
        self.last_read = data[0];
    }
    pub fn read(&mut self, pin: usize) -> bool {
        if (self.last_read & (1 << pin)) != 0 {
            return true;
        }
        false
    }
    pub fn write(&mut self, pin: usize, value: bool) {
        match value {
            true => self.conf |= 1 << pin,
            false => self.conf &= !(1 << pin),
        }
        self.i2c.write_to(self.address, &[self.conf]);
    }
}
