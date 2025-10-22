extern crate alloc;
use crate::settings::*;
use alloc::boxed::Box;
use cty::c_void;
//
use rust_esprit as rn;
//
use rn::rn_fast_event_group::rnFastEventGroup;
use rn::rn_gpio::{digital_write, pinMode, rnPin};
use rn::rn_os_helper::delay_ms;
use rn::rn_timing_adc::rnTimingAdc;

use crate::app::slave_task::{PeripheralEvent, i2c_task};
type Display<'a> = rs_psu_gfx::gfx::display2::lnDisplay2<'a>;

use ili9341::ili9341_init_sequence::{DSO_RESET, DSO_WAKEUP};
use lnspi_ili9341::SpiIli9341;
use rn::rn_spi::{rnSPI, rnSPISettings, rnSpiBitOrder::SPI_MSBFIRST};
use rn::{lnLogger, lnLogger_init};

mod slave_task;
mod utils;

lnLogger_init!();
/**
 *  A => shut volt / 10.2 => amp
 *  102 mOhm
 *
 *
 */
struct main_loop<'a> {
    event_group: rnFastEventGroup,
    adc: rnTimingAdc,
    output: [u16; ADC_SAMPLE * 2],
    pins: [rnPin; 2],
    outputEnabled: bool,
    display: Display<'a>,
    control: i2c_task,
}
//---------------------------------
impl<'a> main_loop<'a> {
    fn notify(&mut self, event: PeripheralEvent) {
        self.event_group.set_events(event as u32);
    }
    fn callback_trampoline(cookie: *mut c_void, event: PeripheralEvent) {
        unsafe {
            let p = (cookie as *mut main_loop).as_mut();
            p.expect("oops").notify(event);
        }
    }

    //---- init----
    fn init(&mut self) {
        self.display.init();
        self.event_group.take_ownership();

        let me = self as *mut main_loop as *mut c_void;
        self.control.set_observer(Self::callback_trampoline, me);
        self.control.start_slave_task();

        // create adc
        self.adc.set_source(3, 3, 1000, &self.pins);
        //
        // Check we are not in low battery mode from the start
        //---
        {
            let (sbat, _maxCurrent) = self.run_adc();
            if sbat < PS_MIN_VBAT {
                self.stop_due_to_low_voltage()
            }
        }
        // ok enable the DC/DC and make sure the relay is off
        self.control.set_output_enable(false);
        self.control.set_dcdc_enable(true);
        // turn off the front button
        rn::rn_gpio::digital_write(PIN_LED, true);
    }
    //
    fn is_set(data: u32, ev: PeripheralEvent) -> bool {
        if (data & (ev as u32)) != 0 {
            return true;
        }
        false
    }
    fn abs_diff(a: usize, b: usize) -> usize {
        if a > b {
            return a - b;
        }
        b - a
    }
    /**
     *  main loop
     */
    fn run(&mut self) {
        let mut lastMaxCurrent = 0;
        loop {
            let mut gauge_update: bool = false;
            let ev: u32 = self.event_group.wait_events(0xff, 100);
            let current: usize = self.control.current_ma();
            let cc: bool = self.control.cc_limited();
            let mut voltage: f32 = self.control.voltage();

            if Self::is_set(ev, PeripheralEvent::ButtonEvent) {
                delay_ms(40); // dumb anti bounce
                let state = !rn::rn_gpio::digital_read(PIN_SWITCH);
                if state {
                    self.outputEnabled = true;
                //
                } else {
                    self.outputEnabled = false;
                }
                lnLogger!("Output Enabled = {}\n", self.outputEnabled);
                self.control.set_output_enable(self.outputEnabled);
                rn::rn_gpio::digital_write(PIN_LED, !self.outputEnabled); // active low

                delay_ms(40); // dumb anti bounce
                rn::rn_exti::enable_interrupt(PIN_SWITCH);
            }

            // Display voltage & current
            if Self::is_set(ev, PeripheralEvent::VoltageChangeEvent) {
                let mut correction: f32 = WIRE_RESISTANCE_MOHM as f32;
                correction *= current as f32;
                correction /= 1000000.;
                voltage -= correction;
                self.display.display_voltage(cc, voltage);
                let mut power: f32 = voltage * (current as f32);
                power /= 1000.;
                self.display.display_power(cc, power);
            }
            if Self::is_set(ev, PeripheralEvent::CurrentChangeEvent) {
                self.display.display_current(current);
                gauge_update = true;
            }

            //  Read ADC to get maxCurrent and vbat
            let (sbat, maxCurrent) = self.run_adc();
            if sbat < PS_MIN_VBAT_CRIT {
                self.stop_due_to_low_voltage()
            }
            self.display.display_Vbat(sbat);

            // manage current Limiting
            // Read the ADC to get the input and program new max current
            // if it changed
            if Self::abs_diff(lastMaxCurrent, maxCurrent) > 10 {
                lastMaxCurrent = maxCurrent;
                let mut d: f32 = maxCurrent as f32;
                d /= 1.5;
                d -= 25.;
                if d < 0. {
                    d = 0.;
                }
                self.control.set_max_current(d as usize); // convert maxCurrent to the mcp voltage to control max current
                self.display.display_max_current(maxCurrent);
                gauge_update = true;
            }
            if gauge_update {
                self.display.display_current_percent(current, maxCurrent);
            }
        }
    }

    /*
     *
     */
    fn pushed(&mut self) {
        rn::rn_exti::disable_interrupt(PIN_SWITCH);
        self.event_group
            .set_events(PeripheralEvent::ButtonEvent as u32);
    }
    /*
        Trampoline from C to class
    */
    pub extern "C" fn onOffCallback(_pin: rnPin, cookie: *mut cty::c_void) {
        let p: &mut main_loop = unsafe { &mut *(cookie as *mut main_loop) };
        p.pushed();
    }
    /*
     */
    pub fn new() -> Self {
        let transaction: rnSPISettings = rnSPISettings {
            speed: 36 * 1000 * 1000,
            bOrder: SPI_MSBFIRST,
            dMode: 0,
            pinCS: rnPin::NoPin,
        };

        let mut spi = rnSPI::new(0, 36 * 1000 * 1000);
        spi.set(&transaction);

        let mut ili_access = SpiIli9341::new(spi, ILI_PIN_CS, ILI_PIN_DC, ILI_PIN_RESET);
        // init low level
        ili_access.reset();
        //ili_access.send_init_sequence(ST7735);
        ili_access.send_init_sequence(DSO_RESET);
        ili_access.send_init_sequence(DSO_WAKEUP);
        ili_access.set_chip_id(0x9341);

        main_loop {
            event_group: rnFastEventGroup::new(),
            adc: rnTimingAdc::new(0),
            output: [0; 16],
            pins: [PS_PIN_VBAT, PS_PIN_MAX_CURRENT], // PA0 + PA1
            outputEnabled: false,
            display: Display::new(Box::new(ili_access)),
            control: i2c_task::new(),
        }
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn rnInit() {
    lnLogger!("Setuping up Power Supply...\n");

    pinMode(PS_PIN_VBAT, rn::rn_gpio::rnGpioMode::lnADC_MODE);
    pinMode(PS_PIN_MAX_CURRENT, rn::rn_gpio::rnGpioMode::lnADC_MODE);
    pinMode(rnPin::PC13, rn::rn_gpio::rnGpioMode::lnOUTPUT);
    digital_write(PIN_LED, true);
    pinMode(PIN_LED, rn::rn_gpio::rnGpioMode::lnOUTPUT);
    pinMode(PIN_SWITCH, rn::rn_gpio::rnGpioMode::lnINPUT_PULLDOWN);
}

/**
 * \fn rnLoop
 *
 *
 */
#[unsafe(no_mangle)]
pub extern "C" fn rnLoop() {
    let r = main_loop::new(); //&mut sl);
    let boxed: Box<main_loop> = Box::new(r);
    let mut boxed2: Box<main_loop>;

    let ptr = Box::into_raw(boxed);
    //-----------------------------------------------
    rn::rn_exti::attach_interrupt(
        PIN_SWITCH,
        rn::rn_exti::rnEdge::LN_EDGE_BOTH,
        Some(main_loop::onOffCallback),
        ptr as *mut cty::c_void,
    );
    //-----------------------------------------------
    rn::rn_exti::enable_interrupt(PIN_SWITCH);
    unsafe {
        boxed2 = Box::from_raw(ptr);
    }
    boxed2.init();
    boxed2.run();
}
// EOF
