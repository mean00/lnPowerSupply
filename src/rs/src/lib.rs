#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

//use lazy_static::lazy_static;
use rnarduino as rn;


//extern "C" {
  // static mut eventGroup: *mut rn::lnFastEventGroup;
//}

//static  eventGrp : & 'static   rn::lnFastEventGroup = unsafe { &mut *eventGroup as &mut  rn::lnFastEventGroup  };

extern "C" {
   static mut eventGroup: &'static mut rn::lnFastEventGroup;
}



#[no_mangle]
pub extern "C" fn rnLoop() -> ()
{
   unsafe{
      eventGroup.takeOwnership();
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
