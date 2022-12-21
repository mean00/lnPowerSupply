mod utils;
mod slave_task;
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
use pcf8574::PC8754;
use ina219::INA219;
use mcp4725::MCP4725;



type Display <'a> =  crate::gfx::display2::lnDisplay2 <'a>;

/**
 *  A => shut volt / 10.2 => amp
 *  102 mOhm
 *
 *
 */
enum PeripheralEvent
{
    CCChangeEvent=1,
    VoltageChangeEvent=2,
    CurrentChangeEvent=4,
}

struct main_loop <'a>
{
    eventGroup              : rnFastEventGroup,
    adc                     : rnTimingAdc,
    output                  : [u16; ADC_SAMPLE*2],
    pins                    : [rnPin; 2] ,
    outputEnabled           : bool,
    display                 : Display <'a>,

    pc8574                  : PC8754,
    ina219                  : INA219,
    mcp4725                 : MCP4725,


    // slave task part
    current_volt            : f32,
    current_ma              : usize,
    current_max_current     : usize,
    current_dc_enabled      : bool,
    current_relay_enabled   : bool,
    current_cc              : bool,


    updated_max_current     : usize,
    updated_dc_enabled      : bool,
    updated_relay_enabled   : bool,

}

//---------------------------------
impl  <'a> main_loop  <'a>
{
    //---- notify----
    fn notify(&mut self, event :  PeripheralEvent)
    {
        self.eventGroup.setEvents(event as u32);
    }
    //---- init----
    fn init(&mut self)
    {
        self.display.init();
        self.eventGroup.takeOwnership();
        // start the i2c task
        self.start_slave_task();
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
        // ok enable the DC/DC and shutdown the Relay
        self.set_output_enable(false);
        self.set_dcdc_enable(true);
        rnGpio::digital_write(PIN_LED,true);
    }

    /**
     *  main loop
     */
    fn run(&mut self)
    {
        let mut lastMaxCurrent = -1;
        loop
        {
           let ev : u32 = self.eventGroup.waitEvents( 0xff , 100);

           let   current: usize=   self.current_ma();
           let   cc     : bool =   self.cc_limited();
           let mut voltage : f32 ;

           if (ev & EnableButtonEvent)!=0
           {
              rnGpio::digital_write(PIN_LED,!self.outputEnabled); // active low
              self.set_output_enable(self.outputEnabled);

              rn::rnOsHelper::rnDelay(150); // dumb anti bounce
              rnExti::enableInterrupt(PIN_SWITCH);
              rn::rnOsHelper::rnDelay(150); // dumb anti bounce
           }
           // Display voltage & current
           voltage=self.voltage();                  
           
           if  (ev & (PeripheralEvent::VoltageChangeEvent as u32))!=0
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
              self.set_max_current(d as usize); // convert maxCurrent to the mcp voltage to control max current
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


                //-- slave thread --
                pc8574      : PC8754::new(PS_I2C_INSTANCE, IO_EXPANDER_ADDRESS as u8),
                ina219      : INA219::new(PS_I2C_INSTANCE as usize, INA219_ADDRESS as u8,  100*1000, INA219_SHUNT_VALUE ,3),
                mcp4725     : MCP4725::new( PS_I2C_INSTANCE as usize, MCP4725_ADDRESS , 100*1000),
                current_volt            : 0.,
                current_ma              : 0,
                current_max_current     : 200,
                current_dc_enabled      : false,
                current_relay_enabled   : false,
                current_cc              : false,

                updated_max_current     : 200,
                updated_dc_enabled      : false,
                updated_relay_enabled   : false,
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
