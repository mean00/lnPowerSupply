#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnGpio::rnPin as rnPin;
use rn::rnExti as rnExti;
use rn::rnFastEventGroup::rnFastEventGroup;
use rn::rnTimingAdc::rnTimingAdc;

use rn::rnOsHelper::rnLogger as rnLogger;

use crate::settings;
use crate::i2c_rs_task;
use crate::gfx;
use crate::taist::*;

//use ina219;
type Display <'a> =  crate::gfx::display2::lnDisplay2 <'a>;
/**
 * \brief runTime
 */
struct runTime  <'a>
{
   eventGroup     : rnFastEventGroup,
   adc            : rnTimingAdc,
   output         : [u16; settings::ADC_SAMPLE*2],
   pins           : [rnPin; 2] ,   
   outputEnabled  : bool,
   display        : Display <'a>, 
}
/**
 * 
 * 
 */
impl <'a> runTime <'a>
{
   /**
    *     
    */
   fn new() ->  runTime  <'a>
   {
      let mut t : runTime = runTime
         {
            eventGroup  :  rnFastEventGroup::new(),
            adc         :  rnTimingAdc::new(0),
            output      :  [0;16],
            pins        :  [settings::PS_PIN_VBAT , settings::PS_PIN_MAX_CURRENT] , // PA0 + PA1
            outputEnabled: false,
            display      : Display::new(),
         };         
         t.display.init();
         t      
   }
   /**
    * 
    */
   pub extern "C" fn cb( signal : u32 ,  cookie: *const cty::c_void) -> ()
   {
      
         let p: &mut runTime =  unsafe{  &mut *(cookie as *mut runTime) };
         p.eventGroup.setEvents(signal);         
   }
   /**
    * 
    */
   fn pushed(&mut self)
   {
      self.outputEnabled =!self.outputEnabled;
      rnExti::disableInterrupt(settings::PIN_SWITCH);
      self.eventGroup.setEvents(settings::EnableButtonEvent);
   }
   /**
    * 
    */
   pub extern "C" fn onOffCallback(pin: rnPin, cookie: *mut cty::c_void)  -> ()
   {
      let p: &mut runTime ;
      p= unsafe { &mut *(cookie as *mut runTime) };
      p.pushed();
   }
   /**
    * 
    */
   fn run(&mut self) -> ()
   {
      self.eventGroup.takeOwnership();             
      self.adc.setSource(3,3,1000,2,self.pins.as_ptr() );
      
      //---
      // Check we are not in low battery mode from the start
      //---
      let mut lastMaxCurrent : i32 = -11;
      {
         let mut sbat  : f32=0.;
         let mut maxCurrent : i32=0;
         self.run_adc( &mut sbat, &mut maxCurrent);
         if(sbat < settings::PS_MIN_VBAT)
         {
            self.stopLowVoltage()
         }
      }
      
      self.setDCEnable(true);
      self.setOutputEnable(false); 

      rnGpio::digitalWrite(settings::PIN_LED,true);
  

      loop
      {   
         let ev : u32 ;
         
         ev = self.eventGroup.waitEvents( 0xff , 100);         
         rnGpio::digitalToggle(rnPin::PC13 );             

         let   current: i32;
         let   cc  : bool;
         let mut voltage : f32;
         current=self.getCurrent();
         cc=self.getCCLimited();
         if((ev & settings::EnableButtonEvent)!=0)
         {            
            rnGpio::digitalWrite(settings::PIN_LED,!self.outputEnabled);
            self.setOutputEnable(self.outputEnabled);            

            rn::rnOsHelper::rnDelay(150); // dumb anti bounce
            rnExti::enableInterrupt(settings::PIN_SWITCH);
            rn::rnOsHelper::rnDelay(150); // dumb anti bounce
         }
         
         voltage=self.getVoltage();
         // Display voltage & current
        // const msk : u32 =  (i2c_rs_task::lni2c_rs_task_SignalChange::VoltageChangeEvent as u32) +  (i2c_rs_task::lni2c_rs_task_SignalChange::CCChangeEvent as u32) + ( i2c_rs_task::lni2c_rs_task_SignalChange::CurrentChangeEvent    as u32);
        const msk : u32 =0;
         if( (ev & msk)!=0)
         {
             let mut correction: f32 =settings::WIRE_RESISTANCE_MOHM as f32;
             correction=correction*(current as f32);
             correction/=1000000.;
             voltage-=correction;
             self.display.display_voltage( cc,  voltage);
             
             let mut power : f32 =voltage*(current as f32);
             power/=1000.;
             self.display.display_power( cc,  power);
             
         }
         if false
         //if((ev & (i2c_rs_task::lni2c_rs_task_SignalChange::CurrentChangeEvent as u32) ) !=0 )//(lni2c_rs_task::CurrentChangeEvent)) != 0)
         {         
            self.display.display_current(current as usize);         
         }
 
         let mut sbat  : f32=0.;
         let mut maxCurrent : i32=0;
         self.run_adc( &mut sbat, &mut maxCurrent);
         if(sbat < settings::PS_MIN_VBAT_CRIT)
         {
            self.stopLowVoltage()
         }
         self.display.display_Vbat( sbat);
         
         // manage current Limiting
         let mut delta: i32 =lastMaxCurrent- maxCurrent;
         if(delta<0) 
         {
            delta=-delta;
         }
         if(delta>10)
         {
             lastMaxCurrent=maxCurrent;
             let mut d: f32 = maxCurrent as f32;             
             d/=1.5;
             d-=25.;
             if(d<0.)
             {
                d=0.;
             }
            self.setMaxCurrent(d as i32);
            self.display.display_max_current(maxCurrent as usize);
         }
      }
   }
   /**
    * 
   */
   fn run_adc(&mut self, fvbat : &mut f32 ,maxCurrentSlopped :  &mut i32 )
   {
      
      self.adc.multiRead(settings::ADC_SAMPLE as i32 ,self.output.as_mut_ptr() ); 
      let mut max0 : isize =0;
      let mut max1 : isize =0;
      
      for i in 0..settings::ADC_SAMPLE   {
         max0+=self.output[i+i] as isize;   
         max1+=self.output[i+i+1] as isize;
      }
   
      let mut vbat : isize ;
      let mut maxCurrent : isize ;
      
      vbat = max0 + ((settings::ADC_SAMPLE-1)/2) as isize;
      vbat = vbat /(settings::ADC_SAMPLE as isize);

      maxCurrent = (max1 as isize) + (((settings::ADC_SAMPLE as isize)-1)/2);
      maxCurrent = maxCurrent/(settings::ADC_SAMPLE as isize);
   
      *fvbat = vbat as f32;    
      *fvbat=*fvbat*9.;
      *fvbat/=1000.;
      
      maxCurrent=maxCurrent*maxCurrent;
      // 0..4095 -> 0.. 4A amp=val*1.5+50
      // so max=4000/1.5=2660
      maxCurrent/=6300; // 0..4000
      maxCurrent-=maxCurrent&7;
      maxCurrent+=50;
      *maxCurrentSlopped=maxCurrent as i32;
   }
   
 
   ///
   /// 
   /// 
   fn stopLowVoltage(&mut self) -> !
   {    
      self.setOutputEnable(false);    
      self.setDCEnable(false);    
      
      //self.display.banner("LOW BATTERY");    
      
      loop
      {
         rnGpio::digitalToggle(rnPin::PC13 );    
         rnGpio::digitalToggle(settings::PIN_LED);
         rn::rnOsHelper::rnDelay(20);
      }   
   }
   ///
   /// 
   /// 
   /// 
   fn getCurrent(&mut self, ) -> i32
   {
      unsafe{
  //    return i2c_rs_task::lni2c_rs_taskShim::getCurrent() as i32;
      }
      0
   }
   ///
   /// 
   ///    
   fn getCCLimited(&mut self) -> bool 
   {
      unsafe {
  //       i2c_rs_task::lni2c_rs_taskShim::getCCLimited()
      }
      false
   }
     ///
   /// 
   /// 
   fn setOutputEnable(&mut self, enable: bool) -> ()
   {
      unsafe {
   //      i2c_rs_task::lni2c_rs_taskShim::setOutputEnable(enable);
      }
   }
   ///
   /// 
   /// 
   /// 
   fn setDCEnable(&mut self, enable: bool) -> ()
   {
      unsafe {
   //      i2c_rs_task::lni2c_rs_taskShim::setDCEnable(enable);
      }      
   }
   ///
   /// 
   /// 
   fn getVoltage(&mut self) -> f32
   {
      unsafe {
    //     return i2c_rs_task::lni2c_rs_taskShim::getVoltage();
      }
      1.
   }
   ///
   /// 
   /// 
   fn setMaxCurrent(&mut self, max : i32) -> ()
   {
      unsafe{
    //  i2c_rs_task::lni2c_rs_taskShim::setMaxCurrent(max);
      }
   }
}

/**
 * \fn rnLoop
 * 
 * 
 */
#[no_mangle]
pub extern "C" fn rnLoop() -> ()
{
      crate::taist::test_me();
      let r : runTime = runTime::new();
      let boxed : Box<runTime> = Box::new(r);
      let mut boxed2 : Box<runTime>;
        
      let ptr = Box::into_raw(boxed);
      rnExti::attachInterrupt(settings::PIN_SWITCH , rnExti::rnEdge::LN_EDGE_FALLING,
         Some(runTime::onOffCallback) , 
         ptr as  *mut   cty::c_void) ;
      rnExti::enableInterrupt(settings::PIN_SWITCH);   
      unsafe {    
          //  i2c_rs_task::shimCreatei2c_rs_task(Some(runTime::cb),ptr as *const cty::c_void);

            boxed2 = Box::from_raw(ptr);
         }
    
      boxed2.run();
}

//Box::into_raw(r) as  * mut  cty::c_void); //Box::into_raw(r) as * mut  cty::c_void);
//Box::into_raw(boxed) as  *const   cty::c_void); //Box::into_raw(r) as * mut  cty::c_void);

#[no_mangle]
pub extern "C" fn rnInit() -> ()
{
   rnLogger("Setuping up Power Supply...\n");
   
   rnGpio::pinMode(settings::PS_PIN_VBAT          ,rnGpio::rnGpioMode::lnADC_MODE);
   rnGpio::pinMode(settings::PS_PIN_MAX_CURRENT   ,rnGpio::rnGpioMode::lnADC_MODE);
   rnGpio::pinMode(rnPin::PC13                    ,rnGpio::rnGpioMode::lnOUTPUT);
   rnGpio::pinMode(settings::PIN_LED              ,rnGpio::rnGpioMode::lnOUTPUT);
   rnGpio::pinMode(settings::PIN_SWITCH           ,rnGpio::rnGpioMode::lnINPUT_PULLDOWN);  
}
// EOF
