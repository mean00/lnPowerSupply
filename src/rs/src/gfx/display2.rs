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
use ili9341::colors::{WHITE,BLACK,RED,GREEN,BLUE,rgb};

const   YELLOW : u16 = (0x1f<<11)+(0x3f<<5)+(0<<0);

use ili9341::ili9341::FontFamily as FontFamily;
use ili9341::ili9341::FontFamily::{BigFont,SmallFont,MediumFont};
extern crate alloc;

use alloc::boxed::Box;
use ili9341::ili9341_init_sequence::{DSO_RESET,DSO_WAKEUP};
// fonts
use crate::gfx::waree12::Waree12pt7b                     as small_font;
use crate::gfx::robotoLight28::Roboto_Light28pt7b        as med_font;
use crate::gfx::robotoslab48::RobotoSlab_SemiBold48pt7b  as big_font;

const BITMAP_BATS : [&[u8];3]=[ &crate::gfx::bat1::BITMAP_HS,  &crate::gfx::bat2::BITMAP_HS,  &crate::gfx::bat3::BITMAP_HS];

const FAST : u32 = 1;

pub struct  lnDisplay2 <'a>
{
    ili         : Box<Ili9341<'a>> ,
    buffer      : [u8;128], 
    string_buf  : heapless::String<64>,
}

impl  lnDisplay2  <'_>
{
    
    pub fn init(&mut self)
    {
        self.ili.set_rotation(1);
        self.ili.fill_screen(0);

        self.ili.set_text_color(YELLOW,BLACK);
        self.lcd_print2( BigFont,    0,  MAIN_COLUMN, V_LINE, "V");
        self.lcd_print2( MediumFont, 18, MAIN_COLUMN, A_LINE, "A");
        self.lcd_print2( SmallFont,  18, MAIN_COLUMN, MAX_C_LINE, "Max");
        self.lcd_print2( MediumFont, 18, MAIN_COLUMN, PW_LINE, "P");        
        self.ili.set_text_color(WHITE,BLACK);
        self.lcd_print2( MediumFont, LIMIT_COLUMN-UNITS_OFFSET, LIMIT_COLUMN, A_LINE, "mA");
        self.lcd_print2( MediumFont, LIMIT_COLUMN-UNITS_OFFSET, LIMIT_COLUMN, PW_LINE,"W");      

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
            ili         :  Ili9341::new(240,320, 
                                    ili_access, 
                                    &small_font, &med_font ,&big_font),
            buffer      : [0;128],
            string_buf  :  String::new(),
         }         
    }

    pub fn print1f(&mut self, prefix: &str, value : f32)
    {
        let up = value as usize;
        let down = ((value - (up as f32))*10.) as usize;
        self.string_buf.clear();
        uwrite!(&mut self.string_buf, "{}{}.{}",prefix,up,down).unwrap();
    }

    pub fn print2f(&mut self, prefix: &str, value : f32)
    {
        let up = value as usize;
        let down = ((value - (up as f32))*100.) as usize;

        let d1 = down / 10;
        let d2 = down -(d1*10);

        self.string_buf.clear();
        uwrite!(&mut  self.string_buf, "{}{}.{}{}",prefix,up,d1,d2 ).unwrap();
    }


    pub  fn display_max_current(&mut self,currentMa: usize) {
        //sprintf(buffer,"%2.1fA",f);
        
        let mut currentMa = currentMa as f32;
        currentMa /= 1000.; 
        self.print1f("", currentMa);
        self.lcd_print(SmallFont, MAIN_COLUMN,LIMIT_COLUMN, MAX_C_LINE);     
    }
   
    pub  fn banner(&mut self,msg: &str) {
        self.ili.set_font_size(SmallFont);  
        self.ili.print(180,MAX_C_LINE,msg);        
    }
    
   

    pub  fn display_Vbat(&mut self,vbat: f32) {
        self.print1f("Bat:",vbat);
        self.lcd_print(SmallFont, 200,318, VBAT_LINE);    
        
        let index;
        let color : u16;
        //let mut vbat : f32 = 17.;
        if vbat > 19.5
        {
            index = 2;
            color = GREEN;
        }else
        {
            if vbat > 18.0
            {
                index = 1;
                color = YELLOW;
            }else {
                index = 0;
                color = RED;
            }
        }
        self.ili.draw_bitmap_hs(crate::gfx::bat1::WIDTH , crate::gfx::bat1::HEIGHT ,
            MAIN_COLUMN, 220, 
             color, 0, BITMAP_BATS[index]); //&crate::gfx::bat1::BITMAP_HS);
    }
    
    pub  fn display_current(&mut self,ma: usize) {
        self.string_buf.clear();
        uwrite!(&mut self.string_buf, "{}",ma).unwrap();
        self.lcd_print(MediumFont, MAIN_COLUMN,LIMIT_COLUMN-UNITS_OFFSET-MAIN_COLUMN, A_LINE);
    
    }
    
    pub  fn display_voltage(&mut self, cc: bool, voltage: f32) {
        self.display_float(cc,voltage,V_LINE);
    }
    
    pub  fn display_power(&mut self, _cc: bool, pw: f32) {
        self.print1f("",pw);
        self.lcd_print(MediumFont, MAIN_COLUMN,LIMIT_COLUMN-UNITS_OFFSET-MAIN_COLUMN, PW_LINE);    
    }

    fn lcd_print2(&mut self, size : FontFamily,  column : usize ,  max_column : usize , line : usize, s: &str)
    {
        self.ili.set_font_size(size);
        self.ili.set_cursor(column,line);
        self.ili.print_up_to(s, max_column);
    }

    fn lcd_print(&mut self, size : FontFamily,  column : usize ,  max_column : usize , line : usize)
    {
        self.ili.set_font_size(size);
        self.ili.set_cursor(column,line);
        self.ili.print_up_to(self.string_buf.as_str(), max_column);
    }

    fn display_float(&mut self, cc : bool ,  value: f32,  line : usize)
    {        
        self.print2f("",value);        
        if cc
        {
            self.ili.set_text_color(RED,BLACK);
        }
        else
        {
            self.ili.set_text_color(GREEN,BLACK); 
        }
        self.lcd_print(BigFont, MAIN_COLUMN,LIMIT_COLUMN, line);
        self.ili.set_text_color(WHITE,BLACK);    
    }
}
// EOF