extern crate alloc;

use rnarduino as rn;
use rn::rnGpio::rnPin;
use rn::rnGpio;
use rn::rnOsHelper::*;

use crate::i2c_rs_task::power_supply_peripheral;
use crate::i2c_rs_task::peripheral_notify;
use crate::i2c_rs_task::PeripheralEvent;
/**
 *  A => shut volt / 10.2 => amp
 *  102 mOhm
 * 
 * 
 */
pub fn test_me()
{
    let mut x : main_loop = main_loop {} ;
    x.run();
}
struct main_loop
{

}
//--------------------------------
impl peripheral_notify for main_loop
{
    fn notify(&self, _event :  PeripheralEvent)
    {

    }
}

//---------------------------------
impl main_loop
{
    pub fn run(&mut self)
    {
    // create the i2c fake task
    let _tsk : power_supply_peripheral = power_supply_peripheral::new(self);
    let led : rnPin  = rnPin::PC13;
    rnGpio::pin_mode(led, rnGpio::rnGpioMode::lnOUTPUT);
    loop
        {
            rnGpio::digital_toggle(led);
            rnDelay(1000);
        }
    }
}
// EOF