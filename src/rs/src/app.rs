#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use cty::c_char;
use rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnGpio::rnPin as rnPin;
use rn::rnExti as rnExti;
use rn::rnFastEventGroup::rnFastEventGroup as rnFastEventGroup;
use rn::rnTimingAdc::rnTimingAdc as rnTimingAdc;

use rn::rnOsHelper::rnLogger as rnLogger;

use crate::settings;
use crate::i2cTask;
use crate::display;


/**
 * \brief runTime
 */
struct runTime
{
   eventGroup     : rnFastEventGroup,
   adc            : rnTimingAdc,
   output         : [u16; settings::ADC_SAMPLE*2],
   pins           : [rnPin; 2] ,   
   outputEnabled  : bool,
}
/**
 * 
 * 
 */
impl runTime
{
   /**
    *     
    */
   fn new() -> runTime
   {
      let t : runTime = runTime
         {
            eventGroup  :  rnFastEventGroup::new(),
            adc         :  rnTimingAdc::new(0),
            output      :  [0,0,0,0, 0,0,0,0, 0,0,0,0 ,0,0,0,0],
            pins        :  [settings::PS_PIN_VBAT , settings::PS_PIN_MAX_CURRENT] , // PA0 + PA1
            outputEnabled: false,
         };         
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
      
      let mut lastMaxCurrent : i32 = -11;
      {
         let mut sbat  : f32=0.;
         let mut maxCurrent : i32=0;
         self.runAdc( &mut sbat, &mut maxCurrent);
         if(sbat < settings::PS_MIN_VBAT)
         {
            stopLowVoltage()
         }
      }
      
      setDCEnable(true);
      setOutputEnable(false); 

      rnGpio::digitalWrite(settings::PIN_LED,true);
  

      loop
      {   
         let ev : u32 ;
         
         ev = self.eventGroup.waitEvents( 0xff , 100);         
         rnGpio::digitalToggle(rnPin::PC13 );             

         let   current: i32;
         let   cc  : bool;
         let mut voltage : f32;
         current=getCurrent();
         cc=getCCLimited();
         if((ev & settings::EnableButtonEvent)!=0)
         {
            
            rnGpio::digitalWrite(settings::PIN_LED,!self.outputEnabled);
            setOutputEnable(self.outputEnabled);            

            rn::rnOsHelper::rnDelay(50);
            rnExti::enableInterrupt(settings::PIN_SWITCH);
            
         }
         
         voltage=getVoltage();
         
         const msk : u32 =  (i2cTask::lnI2cTask_SignalChange::VoltageChangeEvent as u32) +  (i2cTask::lnI2cTask_SignalChange::CCChangeEvent as u32) + ( i2cTask::lnI2cTask_SignalChange::CurrentChangeEvent    as u32);
         if( (ev & msk)!=0)
         {
             let mut correction: f32 =settings::WIRE_RESISTANCE_MOHM as f32;
             correction=correction*(current as f32);
             correction/=1000000.;
             voltage-=correction;
             unsafe{
               display::lnDisplay::displayVoltage( cc,  voltage);
             }
             let mut power : f32 =voltage*(current as f32);
             power/=1000.;
             unsafe{
               display::lnDisplay::displayPower( cc,  power);
             }
         }
         if((ev & (i2cTask::lnI2cTask_SignalChange::CurrentChangeEvent as u32) ) !=0 )//(lnI2cTask::CurrentChangeEvent)) != 0)
         {
            unsafe {
             display::lnDisplay::displayCurrent(current);
            }
         }
 
         let mut sbat  : f32=0.;
         let mut maxCurrent : i32=0;
         self.runAdc( &mut sbat, &mut maxCurrent);
         if(sbat < settings::PS_MIN_VBAT_CRIT)
         {
            stopLowVoltage()
         }
         unsafe {
         display::lnDisplay::displayVbat( sbat);
         }

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
             setMaxCurrent(d as i32);
             unsafe{
               display::lnDisplay::displayMaxCurrent(maxCurrent);
             }
         }
      }  
   }
   /**
    * 
    */
   fn runAdc(&mut self, fvbat : &mut f32 ,maxCurrentSlopped :  &mut i32 )
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

  
}
///
/// 
/// 
fn setOutputEnable(enable: bool) -> ()
{
   unsafe {
      i2cTask::lnI2cTaskShim::setOutputEnable(enable);
   }
}
///
/// 
/// 
/// 
fn setDCEnable(enable: bool) -> ()
{
   unsafe {
      i2cTask::lnI2cTaskShim::setDCEnable(enable);
   }
}
///
/// 
/// 
fn stopLowVoltage() -> !
{    
   setOutputEnable(false);    
   setDCEnable(false);    
   unsafe {
      display::lnDisplay::banner("LOW BATTERY" .as_ptr() as *const c_char);    
   }
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
fn getCurrent() -> i32
{
   unsafe{
   return i2cTask::lnI2cTaskShim::getCurrent() as i32;
   }
}
///
/// 
///    
fn getCCLimited() -> bool 
{
   unsafe {
      i2cTask::lnI2cTaskShim::getCCLimited()
   }
}
///
/// 
/// 
fn getVoltage() -> f32
{
   unsafe {
       return i2cTask::lnI2cTaskShim::getVoltage();
   }
}
///
/// 
/// 
fn setMaxCurrent(max : i32) -> ()
{
   unsafe{
   i2cTask::lnI2cTaskShim::setMaxCurrent(max);
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
      let r : runTime = runTime::new();
      let boxed : Box<runTime> = Box::new(r);
      let mut boxed2 : Box<runTime>;
        
      let ptr = Box::into_raw(boxed);
      rnExti::attachInterrupt(settings::PIN_SWITCH , rnExti::rnEdge::LN_EDGE_FALLING,
         Some(runTime::onOffCallback) , 
         ptr as  *mut   cty::c_void) ;
      rnExti::enableInterrupt(settings::PIN_SWITCH);   
      unsafe {    
            i2cTask::shimCreateI2CTask(Some(runTime::cb),ptr as *const cty::c_void);

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

   unsafe {
      display::lnDisplay::init();
   }
}
// EOF
