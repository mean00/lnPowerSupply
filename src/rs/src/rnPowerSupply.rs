#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use rnarduino as rn;
/**
 * 
 * 
 */
extern "C" {
   static mut eventGroup: &'static mut rn::lnFastEventGroup;
}

/**
 * 
 * 
 * 
 */
#[no_mangle]
pub extern "C" fn rnLoop() -> ()
{
   unsafe{
      eventGroup.takeOwnership();
   }
   let mut ev : u32 = 0;
   unsafe{
   ev = eventGroup.waitEvents( 0xff , 100);
   }
   //tsk=createI2cTask(i2cCb);
   //xDelay(20); // let it start    
   //float vbat;
   //int imaxAmp, lastMaxCurrent=-1;;
   
   //runAdc(vbat, imaxAmp);
   //if(vbat<PS_MIN_VBAT)
   //{
   //    stopLowVoltage();
   //}    

  
}
