
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]


pub mod rnI2cTask;

extern crate alloc;
use alloc::boxed::Box;
use crate::settings;

use rnI2cTask::I2CObserver;
mod simpler_INA219;
use rnarduino as rn;
use rn::lnI2C as lnI2C;


/* 
const PS_I2C_INSTANCE : usize = 1;
const PCF8574_ADDRESS : usize = 32;
const INA219_ADDRESS  : u8 = 0x40; //addr: u8,
const INA_SHUNT_VALUE : i32 = 106;//shut
*/

struct i2c_task_impl <'a>
{      
    _currentMa          : i32,
    _maxCurrentMa       : i32,
    _newMaxCurrentMa    : i32,
    _voltage            : f32,
    _currentLimited     : bool,
    _newDCEnabled       : bool,
    _DCEnabled          : bool,
    _newOutputEnabled   : bool ,
    _OutputEnabled      : bool,
    _accumulated        : u32,    
    _i2c                : Box< rn::lnI2C >,
    _cb                 : Option<& 'a dyn I2CObserver>,
}
/**
 * 
 * 
 */
impl  <'a> rnI2cTask::I2CTaskInterface for i2c_task_impl <'a>
{

    fn    getVoltage(&self) -> f32 
    {
            return self._voltage;
    }
    fn    getCurrent(&self) -> i32
    {
            return self._currentMa
    }
    fn    setMaxCurrent(&mut self,ma : i32) -> ()
    {
             self._currentMa=ma;
    }
    fn    getCCLimited(&self) -> bool
    {
            return self._currentLimited;
    }
    fn    setDCEnable(&mut self,  enable: bool)  ->()
    {
        self._newDCEnabled = enable;
    }
    fn    setOutputEnable(&mut self, enable: bool) -> () 
    {
        self._newOutputEnabled = enable;
    }
    fn  setCb(&mut self, cb : &'static dyn I2CObserver) 
    {
        self._cb = Some(cb);
    }
}
/**
 * 
 */
impl i2c_task_impl <'_>
{
    //-------
    fn init(&mut self) -> ()
    {
        unsafe {
            
                   
            let data : &[u8;1] = &[0xff];            
            self._i2c.begin(0);
            self._i2c.write1(settings::PCF8574_ADDRESS as i32 ,1 as i32 ,data.as_ptr() as *mut u8);
        }
        // create ina and adc
    }
    //-------
    extern "C" fn tramp(  param: *mut cty::c_void) -> ()
    {
        let  me : &mut i2c_task_impl;
        unsafe {
        me =&mut *(param as *mut i2c_task_impl );
        }
        me.task();
    }
    //-------
    fn new() -> Self
    {
        let mut instance : i2c_task_impl = i2c_task_impl
          {
            _currentMa          : -1,
            _maxCurrentMa       : 201,
            _newMaxCurrentMa    : 200,
            _voltage            : 0.0,
            _currentLimited     : false,
            _newDCEnabled       : false,
            _DCEnabled          : false,
            _newOutputEnabled   : false ,
            _OutputEnabled      : false,
            _accumulated        : 0,                   
            _i2c                : unsafe { Box::new( rn::lnI2C::new(settings::PS_I2C_INSTANCE as i32,100*1000 as i32)) },
            _cb                 : None,      
          };       
        //
        instance.init();
        let &mut handle: &mut rn::TaskHandle_t;
        /*
        unsafe {
        const name : &str = "i2cTask";
        let &mut pinstance =&mut &instance;
        
        rn::xTaskCreate(
                    Some(i2c_task_impl::tramp ), 
                    name.as_bytes().as_ptr() as *const u8 ,
                     300,
                     pinstance as *mut _ as *mut cty::c_void,
                     2,
                     handle as *mut _ as *mut  rn::TaskHandle_t);

        }
        */
        instance           
    }
    //-------
    fn task(&mut self) -> ()
    {
            let ina : simpler_INA219::simpler_INA219;
            
            unsafe {
                let ptr = Box::into_raw(self._i2c );
                ina= simpler_INA219::simpler_INA219::new(
                    ptr as *mut rn::lnI2C, //self._i2c as *mut rn::lnI2C, //i2c: *mut lnI2C,
                    3, // 3amps maxCurrentinA: cty::c_int,
                    settings::INA219_ADDRESS, //addr: u8,
                    settings::INA_SHUNT_VALUE as i32 //shutResistorMilliOhm: cty::c_int,
            ); }

    }
}
/**
 * 
 * 
 */
pub fn spawnI2c() -> Box<dyn rnI2cTask::I2CTaskInterface>
{
    return Box::new(i2c_task_impl::new());
}

// EOF