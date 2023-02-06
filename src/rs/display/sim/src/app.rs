#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
use std::{thread, time};
use macroquad::prelude::*;

//const SCREEN_WIDTH: u32 = 320;
//const SCREEN_HEIGHT: u32 = 240;

use rs_psu_gfx::gfx::display2::lnDisplay2 as display;
use ili9341::access::Ili9341Access;

struct quadAccess 
{    
    x1 : usize,
    x2 : usize,
    y1 : usize,
    y2 : usize,
    x  : usize,
    y  : usize,
}
impl quadAccess 
{
    fn next(&mut self)
    {
        self.x+=1;
        if self.x > self.x2
        {
            self.x = self.x1;
            self.y +=1;
        }
        if self.y > self.y2
        {
            self.y = self.y1;        
        }
    }
    fn flush(&mut self)
    {
        
    }
}
//-------
fn full_to_unit( c : u16 , shift: usize, range : usize) -> f32
{
    let mut f= c;
    f=f>>shift;
    if range==6
    {
        f&=0x3f;
    }else {
        f&=0x1f;
    }
    f<<= 8-range;
    let mut m=f as f32;
    m=m/255.;
    if m>1.0
    {
        m=1.0;
    }
    m
}
//-------
impl Ili9341Access for quadAccess 
{
    fn send_byte(&mut self,  b : u8)
    {

    }
   
    fn send_word(&mut self,  color : u16)
    {
        let   r : f32=  full_to_unit( color, 11,5);
        let   g : f32 = full_to_unit( color, 5, 6);
        let   b : f32 = full_to_unit( color, 0, 5);

        let ix= (self.x as i32)*2;
        let iy= (self.y as i32)*2;

        let color = Color::new(r,g,b,1.0);
        draw_rectangle(ix as f32, iy as f32, 2.,2.,color);
        self.next();
    }
    fn update_hw_rotation(&mut self, rotation  : usize )
    {
        self.flush();
    }
    fn set_address(&mut self,  x: usize, y : usize, w: usize, h:usize)
    {
        self.x1=x;
        self.x2=x+w-1;
        self.y1=y;
        self.y2=y+h-1;
        self.x=self.x1;
        self.y=self.y1;
        self.flush();
    }
    fn data_end(&mut self, )
    {
       
    }
    fn data_begin(&mut self, )
    {
       
    }   
}

//---
#[macroquad::main("BasicShapes")]
async fn main() {
    let mut loops = 0;
    let bitmap_width = 96;
    let bitmap_height = 96;
    let bitmap = include_bytes!("test_bitmap.bin");
    loop {
    loops+=1;
    if loops > 50000
    {
        break;
    }
    clear_background(macroquad::color::BLACK);

    let access = Box::new(quadAccess{  x1: 0, x2: 0, y1: 0, y2  :0 , x : 0, y:0 });

    let mut display =  display::new(access);
   
    //loop 
    let ten_millis = time::Duration::from_millis(10);
    let mut percent: isize=0;
    let mut inc : isize =2;
    let mut bat : isize = 20;
    let mut bat_inc : isize =2;
    loop
    {
        display.init();   
        display.display_max_current(3500);
        display.display_Vbat(22.5);
        display.display_current(500);
        display.display_voltage(false, 22.);        
        display.display_power(false,4.56);
        display.display_Vbat(bat as f32);

        display.display_current_percent(percent as usize,101 as usize);        
        percent=percent+inc;
        if percent>150 && inc>0
        {
            inc=-inc;
        }
        if percent<3 && inc<0
        {
            inc=-inc;
        }
        bat+=bat_inc;
        if bat>20 && bat_inc>0
        {
            bat_inc=-bat_inc;
        }
        if bat<3 && bat_inc<0
        {
            bat_inc=-bat_inc;
        }
        next_frame().await;
        std::thread::sleep(ten_millis);
    }
    }
    std::println!("Exiting....");
}
