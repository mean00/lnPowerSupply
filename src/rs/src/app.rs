mod utils;
mod slave_task;
extern crate alloc;
use alloc::boxed::Box;
use cty::c_void;
use rnarduino as rn;
use rn::rnGpio::rnPin;
use rn::rnGpio;
use rn::rnOsHelper::*;
use rn::rnExti as rnExti;
use rn::rnFastEventGroup::rnFastEventGroup;
use rn::rnTimingAdc::rnTimingAdc;
use crate::settings::*;

use crate::app::slave_task::{i2c_task,PeripheralEvent};
type Display <'a> =  crate::gfx::display2::lnDisplay2 <'a>;

/**
 *  A => shut volt / 10.2 => amp
 *  102 mOhm
 *
 *
 */
struct main_loop <'a>
{
    eventGroup              : rnFastEventGroup,
    adc                     : rnTimingAdc,
    output                  : [u16; ADC_SAMPLE*2],
    pins                    : [rnPin; 2] ,
    outputEnabled           : bool,
    display                 : Display <'a>,
    control                 : i2c_task ,

}
//---------------------------------
impl  <'a> main_loop  <'a>
{
    fn notify(&mut self, event : PeripheralEvent)
    {
        self.eventGroup.setEvents(event as u32);
    }
    fn callback_trampoline( cookie: *mut c_void, event : PeripheralEvent)
    {
        unsafe
        {
            let p  = ( cookie as *mut main_loop).as_mut();
            p.expect("oops").notify(event);
        }        
    }

    //---- init----
    fn init(&mut self)
    {
        self.display.init();
        self.eventGroup.takeOwnership();    

        let me = self as *mut  main_loop as *mut c_void;
        self.control.set_observer(Self::callback_trampoline, me);        
        self.control.start_slave_task();
        
        // create adc
        self.adc.setSource(3,3,1000,2,self.pins.as_ptr() );
        //
        // Check we are not in low battery mode from the start
        //---
        {
            let (sbat, _maxCurrent) = self.run_adc();
            if sbat < PS_MIN_VBAT
            {
                self.stop_due_to_low_voltage()
            }
        }
        // ok enable the DC/DC and make sure the relay is off
        self.control.set_output_enable(false);
        self.control.set_dcdc_enable(true);
        // turn off the front button
        rnGpio::digital_write(PIN_LED,true);
    }
    ///
    fn is_set(data: u32, ev : PeripheralEvent) -> bool
    {
        if (data &  (ev as u32))!=0
        {
            return true;
        }
        false
    }
    fn abs_diff( a: usize, b: usize )-> usize
    {
        if a>b
        {
            return a-b;
        }
        b-a
    }
    /**
     *  main loop
     */
    fn run(&mut self)
    {
        let mut lastMaxCurrent = 0;
        loop
        {
           let   ev : u32 = self.eventGroup.waitEvents( 0xff , 100);
           let   current: usize=   self.control.current_ma();
           let   cc     : bool =   self.control.cc_limited();
           let mut voltage : f32 = self.control.voltage();

           if Self::is_set(ev, PeripheralEvent::EnableButtonEvent)
           {
              rnGpio::digital_write(PIN_LED,!self.outputEnabled); // active low
              self.control.set_output_enable(self.outputEnabled);

              rn::rnOsHelper::rnDelay(150); // dumb anti bounce
              rnExti::enableInterrupt(PIN_SWITCH);
              rn::rnOsHelper::rnDelay(150); // dumb anti bounce
           }
           
           // Display voltage & current
           if Self::is_set(ev, PeripheralEvent::VoltageChangeEvent)
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
           if Self::is_set(ev, PeripheralEvent::CurrentChangeEvent)
           {
              self.display.display_current(current as usize);
           }

           //  Read ADC to get maxCurrent and vbat
           let (sbat, maxCurrent) = self.run_adc( );
           if sbat < PS_MIN_VBAT_CRIT
           {
              self.stop_due_to_low_voltage()
           }
           self.display.display_Vbat( sbat);

           // manage current Limiting
           // Read the ADC to get the input and program new max current 
           // if it changed
           if Self::abs_diff( lastMaxCurrent, maxCurrent) > 10
           {
               lastMaxCurrent=maxCurrent;
               let mut d: f32 = maxCurrent as f32;
               d/=1.5;
               d-=25.;
               if d<0.
               {
                  d=0.;
               }
              self.control.set_max_current(d as usize); // convert maxCurrent to the mcp voltage to control max current
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
      self.eventGroup.setEvents(PeripheralEvent::EnableButtonEvent as u32);
    }
    /*
        Trampoline from C to class
    */
    pub extern "C" fn onOffCallback(_pin: rnPin, cookie: *mut cty::c_void)  -> ()
    {
      let p: &mut main_loop ;
      p= unsafe { &mut *(cookie as *mut main_loop) };
      p.pushed();
    }
    /*
     */
    pub fn new()-> Self
    {
        main_loop
        {
                eventGroup  :  rnFastEventGroup::new(),
                adc         :  rnTimingAdc::new(0),
                output      :  [0;16],
                pins        :  [PS_PIN_VBAT , PS_PIN_MAX_CURRENT] , // PA0 + PA1
                outputEnabled: false,
                display     : Display::new(),
                control     : i2c_task::new(),

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
        let r  = main_loop::new() ; //&mut sl);
        let boxed : Box<main_loop> = Box::new(r);
        let mut boxed2 : Box<main_loop>;

        let ptr = Box::into_raw(boxed);
        rnExti::attachInterrupt(PIN_SWITCH , rnExti::rnEdge::LN_EDGE_FALLING, Some(main_loop::onOffCallback) ,    ptr as  *mut   cty::c_void) ;
        rnExti::enableInterrupt(PIN_SWITCH);
        unsafe {
            boxed2 = Box::from_raw(ptr);
         }
        boxed2.init();
        boxed2.run();
}
// EOF
