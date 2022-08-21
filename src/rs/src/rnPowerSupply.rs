#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_doc_comments)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(dead_code)]


use cty::c_char;
use rnarduino as rn;

mod display;
mod i2cTask;
mod settings;

extern "C" {
   //static mut eventGroup: &'static mut rn::lnFastEventGroup;
   //static mut tsk       : &'static mut i2cTask::lnI2cTask;
}
extern "C" {
static mut runTimeInstance : &'static mut runTime;
}


/**
 * \brief runTime
 */
struct runTime
{
   eventGroup     : rn::lnFastEventGroup,
   adc            : rn::lnTimingAdc,
   output         : [u16; settings::ADC_SAMPLE*2],
   pins           : [rn::lnPin; 2] ,   
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
      unsafe {
      let t : runTime = runTime
         {
            eventGroup  :  rn::lnFastEventGroup::new(),
            adc         :  rn::lnTimingAdc::new(0),
            output      :  [0,0,0,0, 0,0,0,0, 0,0,0,0 ,0,0,0,0],
            pins        :  [settings::PS_PIN_VBAT , settings::PS_PIN_MAX_CURRENT] , // PA0 + PA1
            outputEnabled: false,
         };
         t      
      }
   }
   /**
    * 
    */
   pub extern "C" fn cb( signal : u32 ) -> ()
   {
      unsafe{
      runTimeInstance.eventGroup.setEvents(signal);
      }
   }
   /**
    * 
    */
   fn onOffCallback(pin: rn::lnPin, cookie: *mut cty::c_void)  -> ()
   {
      
   }
   /**
    * 
    */
   fn run(&mut self) -> ()
   {      
      unsafe{
      self.eventGroup.takeOwnership();    
   

      i2cTask::shimCreateI2CTask(Some(runTime::cb));
      self.adc.setSource(3,3,1000,2,self.pins.as_ptr() );
      }
      let mut lastMaxCurrent : i32 = -1;
      {
         let mut sbat  : f32=0.;
         let mut maxCurrent : i32=0;
         self.runAdc( &mut sbat, &mut maxCurrent);
         if(sbat < settings::PS_MIN_VBAT)
         {
            runTime::stopLowVoltage()
         }
      }

      unsafe {      
      i2cTask::lnI2cTaskShim::setDCEnable(true);
      i2cTask::lnI2cTaskShim::setOutputEnable(false); 
      }
      unsafe{
      rn::lnDigitalWrite(settings::PIN_LED,true);
      };
  

      loop
      {   
         let ev : u32 ;
         unsafe{
            ev = self.eventGroup.waitEvents( 0xff , 100);         
            rn::lnDigitalToggle(rn::lnPin_PC13 );    
         }

         let   current: i32;
         let   cc  : bool;
         let mut voltage : f32;
         unsafe {
            current=i2cTask::lnI2cTaskShim::getCurrent();
            cc=i2cTask::lnI2cTaskShim::getCCLimited();
         }         

         if((ev & settings::EnableButtonEvent)!=0)
         {
            unsafe {
             rn::lnDigitalWrite(settings::PIN_LED,!self.outputEnabled);
             i2cTask::lnI2cTaskShim::setOutputEnable(self.outputEnabled);            
             rn::lnDelay(20);
             rn::lnExtiEnableInterrupt(settings::PIN_SWITCH);
            }
         }
         
         unsafe {
         voltage=i2cTask::lnI2cTaskShim::getVoltage();
         }
         
         if((ev & (i2cTask::lnI2cTask_SignalChange_VoltageChangeEvent |  i2cTask::lnI2cTask_SignalChange_CCChangeEvent |  i2cTask::lnI2cTask_SignalChange_CurrentChangeEvent  ))!=0)
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
         if((ev & i2cTask::lnI2cTask_SignalChange_CurrentChangeEvent ) !=0 )//(lnI2cTask::CurrentChangeEvent)) != 0)
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
            runTime::stopLowVoltage()
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
             unsafe 
             {
               i2cTask::lnI2cTaskShim::setMaxCurrent(d as i32);             
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
      
      unsafe {
      self.adc.multiRead(settings::ADC_SAMPLE as i32 ,self.output.as_mut_ptr() ); 
      }
       
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
   /**
    *     
    */
   fn stopLowVoltage() -> !
   {
    unsafe {
    i2cTask::lnI2cTaskShim::setOutputEnable(false);
    i2cTask::lnI2cTaskShim::setDCEnable(false);    
    display::lnDisplay::banner("LOW BATTERY" .as_ptr() as *const c_char);    
    loop
    {
        rn::lnDigitalToggle(rn::lnPin_PC13 );    
        rn::lnDigitalToggle(settings::PIN_LED);
        rn::lnDelay(20);
    }
   }
   }

}
/**
 * \fn Logger
 * \brief simple logger version
 * 
 */
fn Logger(st : &str) -> ()
{
   unsafe{
      rn::Logger(st.as_ptr() as *const c_char);
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
   let mut r : runTime = runTime::new();
   r.run();
}
#[no_mangle]
pub extern "C" fn rnInit() -> ()
{
   Logger("Setuping up Power Supply...\n");
   unsafe {
   rn::lnPinMode(settings::PS_PIN_VBAT          ,rn::GpioMode_lnADC_MODE);
   rn::lnPinMode(settings::PS_PIN_MAX_CURRENT   ,rn::GpioMode_lnADC_MODE);

   rn::lnPinMode(rn::lnPin_PC13                 ,rn::GpioMode_lnOUTPUT);
   rn::lnPinMode(settings::PIN_LED                  ,rn::GpioMode_lnOUTPUT);
   rn::lnPinMode(settings::PIN_SWITCH               ,rn::GpioMode_lnINPUT_PULLUP);

  // !! rn::lnExtiAttachInterrupt(settings::PIN_SWITCH as rn::lnPin, rn::lnEdge_LN_EDGE_FALLING, runTime::onOffCallback, 0 );
   rn::lnExtiEnableInterrupt(settings::PIN_SWITCH);
   display::lnDisplay::init();
   }
  
}
// EOF