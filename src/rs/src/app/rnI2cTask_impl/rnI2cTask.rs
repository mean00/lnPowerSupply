
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]


pub trait I2CObserver
{
    fn callback(&self, signal : u32) -> ();
}



pub trait I2CTaskInterface  {
    fn    getVoltage(&self) -> f32 ;
    fn    getCurrent(&self) -> i32;
    fn    setMaxCurrent(&mut self, ma : i32) -> () ;
    fn    getCCLimited(&self) -> bool;
    fn    setDCEnable(&mut self,  enable: bool)  ->();
    fn    setOutputEnable(&mut self, enable: bool) -> () ;
    fn    setCb(&mut self, cb : &'static dyn I2CObserver) -> () ;
}

//pub fn createI2CTask() -> &I2CTask;