mod utils;
mod slave_task;
extern crate alloc;
use alloc::boxed::Box;
use cty::c_void;
use rnarduino as rn;
use rn::rn_gpio::{rnPin,digital_write, pinMode};
use rn::rn_exti as rnExti;
use rn::rn_fast_event_group::rnFastEventGroup;
use rn::rn_adc_timer::rnTimingAdc;
use rn::rn_os_helper::delay_ms;
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
    event_group              : rnFastEventGroup,
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
        self.event_group.set_events(event as u32);
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
        self.event_group.take_ownership();    

        let me = self as *mut  main_loop as *mut c_void;
        self.control.set_observer(Self::callback_trampoline, me);        
        self.control.start_slave_task();
        
        // create adc
        self.adc.set_source(3,3,1000,2,self.pins.as_ptr() );
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
        rn::rn_gpio::digital_write(PIN_LED,true);
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
           let   ev : u32 = self.event_group.wait_events( 0xff , 100);
           let   current: usize=   self.control.current_ma();
           let   cc     : bool =   self.control.cc_limited();
           let mut voltage : f32 = self.control.voltage();

           if Self::is_set(ev, PeripheralEvent::EnableButtonEvent)
           {
              rn::rn_gpio::digital_write(PIN_LED,!self.outputEnabled); // active low
              self.control.set_output_enable(self.outputEnabled);

              delay_ms(150); // dumb anti bounce
              rn::rn_exti::enable_interrupt(PIN_SWITCH);
              delay_ms(150); // dumb anti bounce
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
      self.outputEnabled = !self.outputEnabled;
      rn::rn_exti::disable_interrupt(PIN_SWITCH);
      self.event_group.set_events(PeripheralEvent::EnableButtonEvent as u32);
    }
    /*
        Trampoline from C to class
    */
    pub extern "C" fn onOffCallback(_pin: rnPin, cookie: *mut cty::c_void)
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
                event_group :  rnFastEventGroup::new(),
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
pub extern "C" fn rnInit()
{
   rn::rn_os_helper::log("Setuping up Power Supply...\n");

   pinMode(PS_PIN_VBAT          ,rn::rn_gpio::rnGpioMode::lnADC_MODE);
   pinMode(PS_PIN_MAX_CURRENT   ,rn::rn_gpio::rnGpioMode::lnADC_MODE);
   pinMode(rnPin::PC13          ,rn::rn_gpio::rnGpioMode::lnOUTPUT);
   digital_write(PIN_LED            ,true);
   pinMode(PIN_LED                  ,rn::rn_gpio::rnGpioMode::lnOUTPUT);
   pinMode(PIN_SWITCH               ,rn::rn_gpio::rnGpioMode::lnINPUT_PULLDOWN);
}

/**
 * \fn rnLoop
 *
 *
 */
#[no_mangle]
pub extern "C" fn rnLoop()
{
        let r  = main_loop::new() ; //&mut sl);
        let boxed : Box<main_loop> = Box::new(r);
        let mut boxed2 : Box<main_loop>;

        let ptr = Box::into_raw(boxed);
        rn::rn_exti::attach_interrupt(PIN_SWITCH , rnExti::rnEdge::LN_EDGE_FALLING, Some(main_loop::onOffCallback) ,    ptr as  *mut   cty::c_void) ;
        rn::rn_exti::enable_interrupt(PIN_SWITCH);
        unsafe {
            boxed2 = Box::from_raw(ptr);
         }
        boxed2.init();
        boxed2.run();
}
// EOF
