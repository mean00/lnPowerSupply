#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use ufmt::{derive::uDebug, uDebug, uWrite, uwrite, uwriteln, Formatter};
use core::str;
use heapless::String;

const SCREEN_WIDTH      : u32 = 320;
const SCREEN_HEIGHT     : u32 = 240;
const ILI_PIN_DC        : rnPin =  rnPin::PA4 ;
const ILI_PIN_CS        : rnPin =  rnPin::PB1 ;
const ILI_PIN_RESET     : rnPin =  rnPin::PB0 ;
const MAIN_COLUMN       : usize = 76;
const LIMIT_COLUMN      : usize  = 318;

const V_LINE            : usize  = 80;
const A_LINE            : usize  = 130;
const PW_LINE           : usize  = 180;
const MAX_C_LINE        : usize  = 210;
const VBAT_LINE         : usize  = 238;
const UNITS_OFFSET      : usize  = 100;

use rnarduino as rn;

use rn::rnGpio as rnGpio;
use rn::rnGpio::rnPin as rnPin;
use rn::rnExti as rnExti;
use rn::rnOsHelper::rnLogger as rnLogger;
use rnarduino::rnSpi::rnPin::{NoPin};
use rnarduino::rnSpi::rnSpiBitOrder::*;
use rnarduino::rnSpi::rnSPISettings;

use ili9341::ili9341::Ili9341 as Ili9341;
use lnspi_ili9341::spi_ili9341 as spi_ili9341;
use ili9341::colors::{WHITE,BLACK,RED,GREEN,BLUE};

use ili9341::ili9341::FontFamily as FontFamily;
use ili9341::ili9341::FontFamily::{BigFont,SmallFont,MediumFont};
extern crate alloc;

use alloc::boxed::Box;
use ili9341::ili9341_init_sequence::{DSO_RESET,DSO_WAKEUP};
// fonts
use crate::waree12::Waree12pt7b                     as small_font;
use crate::robotoLight28::Roboto_Light28pt7b        as med_font;
use crate::robotoslab48::RobotoSlab_SemiBold48pt7b  as big_font;



const FAST : u32 = 1;

pub struct  lnDisplay2 <'a>
{
    ili         :    Box<Ili9341<'a>> ,
    buffer      : [u8;128], 
    string_buf  : heapless::String<64>,
}

impl  lnDisplay2  <'_>
{
    
    pub fn init(&mut self)
    {
        self.ili.set_rotation(1);
    }
    pub  fn new() -> Self {
    
        let transaction : rnSPISettings  = rnSPISettings{
            speed: FAST*36*1000*1000+(1-FAST)*10*1000, 
            bOrder : SPI_MSBFIRST, 
            dMode : 0, 
            pinCS : rnPin::NoPin};
   
         let mut spi = rn::rnSpi::rnSPI::new(0,36*1000*1000);
         spi.begin();
         spi.begin_transaction(&transaction);
         let mut ili_access = Box::new(spi_ili9341::new(spi, ILI_PIN_CS, ILI_PIN_DC,ILI_PIN_RESET));
         // init low level
         ili_access.reset();
         ili_access.send_init_sequence(DSO_RESET);
         ili_access.send_init_sequence(DSO_WAKEUP);

         lnDisplay2
         {
            // Roboto_Light12pt7b
            // Roboto_Light28pt7b
            // RobotoSlab_SemiBold48pt7b
            
            ili         :  Ili9341::new(240,320, 
                                    ili_access, 
                                    &small_font, &med_font ,&big_font),
            buffer      : [0;128],
            string_buf  :  String::new(),
         }         
    }
    
    pub  fn display_max_current(&mut self,currentMa: usize) {
        
    }
    fn sprintf_f(&mut self, fmt : &str, value : f32) -> &str
    {
        //return core::str::from_utf8(&self.buffer).unwrap();
        let s : &str = &"foo";
        s
    }
    pub  fn banner(&mut self,msg: &str) {
        self.ili.set_font_size(SmallFont);  
        self.ili.print(180,MAX_C_LINE,msg);    
    
    }
    
    pub  fn display_Vbat(&mut self,vbat: f32) {
        
        
        let up = vbat as usize;
        let down = ((vbat - (up as f32))*10.) as usize;
        uwrite!(&mut self.string_buf, "Bat:{}.{}",up,down).unwrap();

        self.lcd_print(SmallFont, 200,318, VBAT_LINE, self.string_buf.as_str());
    
    }
    
    pub  fn display_current(&mut self,ma: usize) {
        //sprintf(buffer,"%d",ma);
        let mut s : heapless::String<64> = String::new();
        uwrite!(&mut s, "{}",ma).unwrap();
        self.lcd_print(MediumFont, MAIN_COLUMN,LIMIT_COLUMN-UNITS_OFFSET-MAIN_COLUMN, A_LINE, s.as_str());
    
    }
    
    pub  fn display_voltage(&mut self, cc: bool, voltage: f32) {
        self.display_float(cc,voltage,V_LINE);

    }
    
    pub  fn display_power(&mut self, cc: bool, pw: f32) {
        //sprintf(buffer,"%2.1f",powr);
        let mut s : heapless::String<64> = String::new();
        let up = pw as usize;
        let down = ((pw - (up as f32))*10.) as usize;
        uwrite!(&mut s, "{}.{}",up,down).unwrap();
        self.lcd_print(MediumFont, MAIN_COLUMN,LIMIT_COLUMN-UNITS_OFFSET-MAIN_COLUMN, PW_LINE, s.as_str());
    
    }
    fn lcd_print(&mut self, size : FontFamily,  column : usize ,  maxColumn : usize , line : usize , txt : &str)
    {
        self.ili.set_font_size(size);
        self.ili.print(column,line,txt);
    }
    fn display_float(&mut self, cc : bool ,  value: f32,  line : usize)
    {
        let mut s : heapless::String<64> = String::new();
        let up = value as usize;
        let down = ((value - (up as f32))*100.) as usize;
        uwrite!(&mut s, "{}.{}",up,down).unwrap();

      //  sprintf(self.buffer,"%2.2f",value);    
        if cc
        {
            self.ili.set_text_color(RED,BLACK);
        }
        else
        {
            self.ili.set_text_color(GREEN,BLACK); 
        }
        let str = "ABCD";
        self.lcd_print(BigFont, MAIN_COLUMN,LIMIT_COLUMN, line, s.as_str()); //self.buffer);
        self.ili.set_text_color(WHITE,BLACK);    
    }
}
