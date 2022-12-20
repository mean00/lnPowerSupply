


mod utils;
extern crate alloc;
use alloc::boxed::Box;

use rnarduino as rn;
use rn::rnGpio::rnPin;
use rn::rnGpio;
use rn::rnOsHelper::*;
use rn::rnExti as rnExti;
use rn::rnFastEventGroup::rnFastEventGroup;
use rn::rnTimingAdc::rnTimingAdc;
use crate::settings::*;

use crate::i2c_rs_task::{power_supply_peripheral,peripheral_notify , PeripheralEvent,peripheral_interface};

type Display <'a> =  crate::gfx::display2::lnDisplay2 <'a>;

/**
 *  A => shut volt / 10.2 => amp
 *  102 mOhm
 * 
 * 
 */

struct main_loop <'a>
{
    eventGroup     : rnFastEventGroup,
    adc            : rnTimingAdc,
    output         : [u16; ADC_SAMPLE*2],
    pins           : [rnPin; 2] ,   
    outputEnabled  : bool,
    display        : Display <'a>, 
    slave          : &'a dyn peripheral_interface,
}
//--------------------------------
impl   <'a> peripheral_notify for main_loop  <'a>
{
    fn notify(&self, _event :  PeripheralEvent)
    {
        self.eventGroup.setEvents(MeasureChangeEvent);
    }
}

//---------------------------------
impl  <'a> main_loop  <'a>
{
    /**
     * 
     */
    pub fn run(&mut self)
    {
        self.eventGroup.takeOwnership();     
        // create adc        
        self.adc.setSource(3,3,1000,2,self.pins.as_ptr() );
        // create the i2c fake task
        let _tsk : power_supply_peripheral = power_supply_peripheral::new(self);
        let led : rnPin  = rnPin::PC13;
        rnGpio::pin_mode(led, rnGpio::rnGpioMode::lnOUTPUT);

        //
        // Check we are not in low battery mode from the start
        //---
        let mut lastMaxCurrent : i32 = -11;
        {
            let (sbat, maxCurrent) = self.run_adc();
            if sbat < PS_MIN_VBAT
            {
                self.stop_due_to_low_voltage()
            }
        }
        // ok enable the DC/DC and shutdown the Relay
        self.slave.set_output_enable(false);         
        self.slave.set_dcdc_enable(true);  
        rnGpio::digital_write(PIN_LED,true);
    
        loop
        {   
           let ev : u32 ;
           
           ev = self.eventGroup.waitEvents( 0xff , 100);         
           rnGpio::digitalToggle(rnPin::PC13 );             
  
           let   current: usize;
           let   cc  : bool;
           let mut voltage : f32;
           current=self.slave.current_ma();
           cc=self.slave.cc_limited();
           if (ev & EnableButtonEvent)!=0
           {            
              rnGpio::digital_write(PIN_LED,!self.outputEnabled);
              self.slave.set_output_enable(self.outputEnabled);            
  
              rn::rnOsHelper::rnDelay(150); // dumb anti bounce
              rnExti::enableInterrupt(PIN_SWITCH);
              rn::rnOsHelper::rnDelay(150); // dumb anti bounce
           }
           
           voltage=self.slave.voltage();
           // Display voltage & current
          // const msk : u32 =  (i2c_rs_task::lni2c_rs_task_SignalChange::VoltageChangeEvent as u32) +  (i2c_rs_task::lni2c_rs_task_SignalChange::CCChangeEvent as u32) + ( i2c_rs_task::lni2c_rs_task_SignalChange::CurrentChangeEvent    as u32);
           const msk : u32 =0;
           if  (ev & msk)!=0
           {
               let mut correction: f32 =WIRE_RESISTANCE_MOHM as f32;
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
   
           let (sbat, maxCurrent) = self.run_adc( );
           if sbat < PS_MIN_VBAT_CRIT
           {
              self.stop_due_to_low_voltage()
           }
           self.display.display_Vbat( sbat);
           
           // manage current Limiting
           let mut delta: i32 =lastMaxCurrent- maxCurrent;
           if delta<0
           {
              delta=-delta;
           }
           if delta>10
           {
               lastMaxCurrent=maxCurrent;
               let mut d: f32 = maxCurrent as f32;             
               d/=1.5;
               d-=25.;
               if d<0.
               {
                  d=0.;
               }
              self.slave.set_max_current(d as usize); // convert maxCurrent to the mcp voltage to control max current
              self.display.display_max_current(maxCurrent as usize);
           }
        }
    }
   

    /**
     * 
     */
    fn pushed(&mut self)
    {
      self.outputEnabled =!self.outputEnabled;
      rnExti::disableInterrupt(PIN_SWITCH);
      self.eventGroup.setEvents(EnableButtonEvent);
    }
    /*
    
    */
    pub extern "C" fn onOffCallback(_pin: rnPin, cookie: *mut cty::c_void)  -> ()
    {
      let p: &mut main_loop ;
      p= unsafe { &mut *(cookie as *mut main_loop) };
      p.pushed();
    }
    /*
     */
    pub fn new() -> Self
    {
        main_loop
        {
                eventGroup  :  rnFastEventGroup::new(),
                adc         :  rnTimingAdc::new(0),
                output      :  [0;16],
                pins        :  [PS_PIN_VBAT , PS_PIN_MAX_CURRENT] , // PA0 + PA1
                outputEnabled: false,
                display      : Display::new(),
        }
    }
}

#[no_mangle]
pub extern "C" fn rnInit() -> ()
{
   rnLogger("Setuping up Power Supply...\n");
   
   rnGpio::pinMode(PS_PIN_VBAT          ,rnGpio::rnGpioMode::lnADC_MODE);
   rnGpio::pinMode(PS_PIN_MAX_CURRENT   ,rnGpio::rnGpioMode::lnADC_MODE);
   rnGpio::pinMode(rnPin::PC13          ,rnGpio::rnGpioMode::lnOUTPUT);
   rnGpio::pinMode(PIN_LED                  ,rnGpio::rnGpioMode::lnOUTPUT);
   rnGpio::pinMode(PIN_SWITCH               ,rnGpio::rnGpioMode::lnINPUT_PULLDOWN);  
}

/**
 * \fn rnLoop
 * 
 * 
 */
#[no_mangle]
pub extern "C" fn rnLoop() -> ()
{        
        let r  = main_loop::new();
        let boxed : Box<main_loop> = Box::new(r);
        let mut boxed2 : Box<main_loop>;
        
        let ptr = Box::into_raw(boxed);
        rnExti::attachInterrupt(PIN_SWITCH , rnExti::rnEdge::LN_EDGE_FALLING, Some(main_loop::onOffCallback) ,    ptr as  *mut   cty::c_void) ;
        rnExti::enableInterrupt(PIN_SWITCH);   
        unsafe {    
            boxed2 = Box::from_raw(ptr);
         }
    
        boxed2.run();
}


// EOF