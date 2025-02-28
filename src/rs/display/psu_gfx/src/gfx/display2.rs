#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use core::str;
use heapless::String;
use ufmt::{derive::uDebug, uDebug, uWrite, uwrite, uwriteln, Formatter};

const ORANGE: u32 = 0xFCAE1E;

const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 240;
const MAIN_COLUMN: usize = 76;
const LIMIT_COLUMN: usize = 318;
const SMALL_LIMIT_COLUMN: usize = 188;

const V_LINE: usize = 80;
const A_LINE: usize = 130;
const PW_LINE: usize = 182;
const MAX_C_LINE: usize = 210;
const VBAT_LINE: usize = 238;
const UNITS_OFFSET: usize = 100;

const SMALL_UNITS_OFFSET: usize = 48;

const SMALL_UNIT_START: usize = SMALL_LIMIT_COLUMN - SMALL_UNITS_OFFSET;

// Gauge
const G_Y: usize = 188;
const R_I: usize = 26;
const R_E: usize = 42;
const G_Y2: usize = 236;
const G_X: usize = (SCREEN_WIDTH as usize - R_E) - 4;
const G_OFFSET: usize = 16;

use ili9341::access::Ili9341Access;
use ili9341::colors::{rgb, BLACK, BLUE, GREEN, RED, WHITE};
use ili9341::ili9341::gauge_meter::Gauge;
use ili9341::ili9341::Ili9341;

const YELLOW: u16 = (0x1f << 11) + (0x3f << 5) + (0 << 0);

use ili9341::ili9341::FontFamily;
use ili9341::ili9341::FontFamily::{BigFont, MediumFont, SmallFont};
extern crate alloc;

use alloc::boxed::Box;
// fonts
use crate::gfx::robotoLight28::Roboto_Light28pt7b as med_font;
use crate::gfx::robotoslab48::RobotoSlab_SemiBold48pt7b as big_font;
use crate::gfx::waree12::Waree12pt7b as small_font;

//const BITMAP_BATS : [&[u8];3]=[ &crate::gfx::bat1::BITMAP_HS,  &crate::gfx::bat2::BITMAP_HS,  &crate::gfx::bat3::BITMAP_HS];

const FAST: u32 = 1;

pub struct lnDisplay2<'a> {
    ili: Box<Ili9341<'a>>,
    buffer: [u8; 128],
    string_buf: heapless::String<64>,
    gaugeCurrent: Gauge<'a>,
    gaugeBattery: Gauge<'a>,
}

impl lnDisplay2<'_> {
    pub fn new(ili_access: Box<dyn Ili9341Access>) -> Self {
        lnDisplay2 {
            ili: Ili9341::new(240, 320, ili_access, &small_font, &med_font, &big_font),
            buffer: [0; 128],
            string_buf: String::new(),
            gaugeCurrent: Gauge::new(R_I, R_E),
            gaugeBattery: Gauge::new(R_I, R_E),
        }
    }
    pub fn init(&mut self) {
        self.ili.set_rotation(1);
        self.ili.fill_screen(0);

        self.ili.set_text_color(YELLOW, BLACK);
        self.lcd_print2(BigFont, 0, MAIN_COLUMN, V_LINE, "V");
        self.lcd_print2(MediumFont, 18, MAIN_COLUMN, A_LINE, "A");
        self.lcd_print2(
            MediumFont,
            LIMIT_COLUMN - UNITS_OFFSET,
            LIMIT_COLUMN,
            A_LINE,
            "mA",
        );

        self.ili
            .set_text_color(ili9341::colors::rgb32(ORANGE), BLACK);

        self.lcd_print2(SmallFont, 18, MAIN_COLUMN, MAX_C_LINE, "Max");
        self.lcd_print2(SmallFont, 18, MAIN_COLUMN, PW_LINE, "Pow");
        self.lcd_print2(SmallFont, 18, MAIN_COLUMN, VBAT_LINE, "Bat");

        self.lcd_print2(SmallFont, 180 + 8, SCREEN_WIDTH as usize, G_Y, "Cur");
        self.lcd_print2(SmallFont, 180 + 8, SCREEN_WIDTH as usize, G_Y2, "Bat");

        self.lcd_print2(
            SmallFont,
            SMALL_UNIT_START,
            SMALL_UNITS_OFFSET,
            PW_LINE,
            "W",
        );
        self.lcd_print2(
            SmallFont,
            SMALL_UNIT_START,
            SMALL_UNITS_OFFSET,
            MAX_C_LINE,
            "A",
        );
        self.lcd_print2(
            SmallFont,
            SMALL_UNIT_START,
            SMALL_UNITS_OFFSET,
            VBAT_LINE,
            "V",
        );

        self.ili.set_text_color(WHITE, BLACK);
    }

    pub fn set_rotation(&mut self, rotation: usize) {
        self.ili.set_rotation(rotation);
    }
    pub fn print1u(&mut self, prefix: &str, value: usize) {
        self.string_buf.clear();
        uwrite!(&mut self.string_buf, "{}{}", prefix, value).unwrap();
    }
    pub fn print1f(&mut self, prefix: &str, value: f32) {
        let up = value as usize;
        let down = ((value - (up as f32)) * 10.) as usize;
        self.string_buf.clear();
        uwrite!(&mut self.string_buf, "{}{}.{}", prefix, up, down).unwrap();
    }

    pub fn print2f(&mut self, prefix: &str, value: f32) {
        let up = value as usize;
        let down = ((value - (up as f32)) * 100.) as usize;

        let d1 = down / 10;
        let d2 = down - (d1 * 10);

        self.string_buf.clear();
        uwrite!(&mut self.string_buf, "{}{}.{}{}", prefix, up, d1, d2).unwrap();
    }
    pub fn display_current_percent(&mut self, ma: usize, max_ma: usize) {
        let mut max_ma: usize = max_ma;
        if max_ma < 10 {
            max_ma = 10;
        }
        let percent = (100 * ma) / (max_ma + 1);
        let color: u16 = match percent {
            90.. => RED,
            60..=89 => ili9341::colors::rgb32(ORANGE),
            _ => GREEN,
        };

        self.gaugeCurrent
            .draw(percent, &mut self.ili, G_X, G_Y, color);
        //self.gauge.draw(100-per, &mut self.ili, G_X,G_Y, ili9341::colors::rgb32(ORANGE));
        //self.print1u("", percent );
        //self.lcd_print(SmallFont, G_X-G_OFFSET, 2*G_OFFSET, G_Y);
    }

    pub fn display_max_current(&mut self, currentMa: usize) {
        //sprintf(buffer,"%2.1fA",f);

        let mut currentMa = currentMa as f32;
        currentMa /= 1000.;
        self.print1f("", currentMa);
        self.lcd_print(
            SmallFont,
            MAIN_COLUMN,
            SMALL_LIMIT_COLUMN - SMALL_UNITS_OFFSET - MAIN_COLUMN,
            MAX_C_LINE,
        );
    }

    pub fn display_power(&mut self, _cc: bool, pw: f32) {
        self.print1f("", pw);
        self.lcd_print(
            SmallFont,
            MAIN_COLUMN,
            SMALL_LIMIT_COLUMN - SMALL_UNITS_OFFSET - MAIN_COLUMN,
            PW_LINE,
        );
    }
    pub fn banner(&mut self, msg: &str) {
        self.ili.set_font_size(SmallFont);
        self.ili.print(180, MAX_C_LINE, msg);
    }

    pub fn display_Vbat(&mut self, vbat: f32) {
        self.print1f("", vbat);
        self.lcd_print(
            SmallFont,
            MAIN_COLUMN,
            SMALL_LIMIT_COLUMN - SMALL_UNITS_OFFSET - MAIN_COLUMN,
            VBAT_LINE,
        );
        let mut per = ((100. * vbat) / 22.) as usize;
        if per > 100 {
            per = 100;
        }
        let color: u16 = match per {
            0..=20 => RED,
            21..=60 => YELLOW,
            _ => GREEN,
        };
        self.gaugeBattery.draw(per, &mut self.ili, G_X, G_Y2, color);
    }

    pub fn display_current(&mut self, ma: usize) {
        self.string_buf.clear();
        uwrite!(&mut self.string_buf, "{}", ma).unwrap();
        self.lcd_print(
            MediumFont,
            MAIN_COLUMN,
            LIMIT_COLUMN - UNITS_OFFSET - MAIN_COLUMN,
            A_LINE,
        );
    }

    pub fn display_voltage(&mut self, cc: bool, voltage: f32) {
        self.display_float(cc, voltage, V_LINE);
    }

    fn lcd_print2(
        &mut self,
        size: FontFamily,
        column: usize,
        max_column: usize,
        line: usize,
        s: &str,
    ) {
        self.ili.set_font_size(size);
        self.ili.set_cursor(column, line);
        self.ili.print_up_to(s, max_column);
    }

    fn lcd_print(&mut self, size: FontFamily, column: usize, max_column: usize, line: usize) {
        self.ili.set_font_size(size);
        self.ili.set_cursor(column, line);
        self.ili.print_up_to(self.string_buf.as_str(), max_column);
    }

    fn display_float(&mut self, cc: bool, value: f32, line: usize) {
        self.print2f("", value);
        if cc {
            self.ili.set_text_color(RED, BLACK);
        } else {
            self.ili.set_text_color(GREEN, BLACK);
        }
        self.lcd_print(BigFont, MAIN_COLUMN, LIMIT_COLUMN, line);
        self.ili.set_text_color(WHITE, BLACK);
    }
}
// EOF
