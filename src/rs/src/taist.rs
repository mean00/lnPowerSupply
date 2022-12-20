



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

use crate::i2c_rs_task::{power_supply_peripheral,peripheral_notify , PeripheralEvent};

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
}
//--------------------------------
impl   <'a> peripheral_notify for main_loop  <'a>
{
    fn notify(&self, _event :  PeripheralEvent)
    {

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
            let mut sbat  : f32=0.;
            let mut maxCurrent : i32=0;
            self.run_adc( &mut sbat, &mut maxCurrent);
            if sbat < PS_MIN_VBAT
            {
                self.stop_due_to_low_voltage()
            }
        }
        loop
            {
                rnGpio::digital_toggle(led);
                rnDelay(1000);
            }
    }

    fn run_adc(&mut self, fvbat : &mut f32 ,maxCurrentSlopped :  &mut i32 ) -> (f32, i32) // vbat, maxCurrent
    {
       
       self.adc.multiRead(ADC_SAMPLE as i32 ,self.output.as_mut_ptr() ); 
       let mut max0 : isize =0;
       let mut max1 : isize =0;
       
       for i in 0..ADC_SAMPLE   {
          max0+=self.output[i+i] as isize;   
          max1+=self.output[i+i+1] as isize;
       }
    
       let mut vbat : isize ;
       let mut maxCurrent : isize ;
       
       vbat = max0 + ((ADC_SAMPLE-1)/2) as isize;
       vbat = vbat /(ADC_SAMPLE as isize);
 
       maxCurrent = (max1 as isize) + (((ADC_SAMPLE as isize)-1)/2);
       maxCurrent = maxCurrent/(ADC_SAMPLE as isize);
    
       let mut fvbat = vbat as f32;    
       fvbat=fvbat*9.;
       fvbat/=1000.;
       
       maxCurrent=maxCurrent*maxCurrent;
       // 0..4095 -> 0.. 4A amp=val*1.5+50
       // so max=4000/1.5=2660
       maxCurrent/=6300; // 0..4000
       maxCurrent-=maxCurrent&7;
       maxCurrent+=50;
       let maxCurrentSlopped=maxCurrent as i32;
       (fvbat, maxCurrentSlopped)
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