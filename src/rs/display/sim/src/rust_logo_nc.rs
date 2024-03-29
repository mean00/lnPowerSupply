// Bitmap image converter  
// https://github.com/mean00/simpler_gfx 
// from rust_logo.gif 
pub const WIDTH : usize = 64;
pub const HEIGHT : usize = 64;
pub const BITMAP : [u8;512] = [
 0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x61,  0x86,  0x00,  0x00,  0x00, 
 0x00,  0x00,  0x00,  0x73,  0xCE,  0x00,  0x00,  0x00,  0x00,  0x00,  0x1C,  0xFF,  0xFF,  0x38,  0x00,  0x00, 
 0x00,  0x00,  0x1E,  0xFF,  0xFF,  0x78,  0x00,  0x00,  0x00,  0x00,  0x1F,  0xFF,  0xFF,  0xF8,  0x00,  0x00, 
 0x00,  0x03,  0x9F,  0xFE,  0x7F,  0xF9,  0xC0,  0x00,  0x00,  0x03,  0xFF,  0xFC,  0x3F,  0xFF,  0xC0,  0x00, 
 0x00,  0x03,  0xFF,  0xFC,  0x3F,  0xFF,  0xC0,  0x00,  0x00,  0x23,  0xFF,  0xCE,  0x73,  0xFF,  0xC4,  0x00, 
 0x00,  0x7F,  0xFE,  0x07,  0xE0,  0x7F,  0xFE,  0x00,  0x00,  0x3F,  0xF8,  0x03,  0xC0,  0x1F,  0xFC,  0x00, 
 0x00,  0x3F,  0xE0,  0x01,  0x80,  0x07,  0xFC,  0x00,  0x00,  0x3F,  0xC0,  0x00,  0x00,  0x03,  0xFC,  0x00, 
 0x03,  0xFF,  0x80,  0x00,  0x00,  0x01,  0xFF,  0xC0,  0x03,  0xFF,  0x00,  0x00,  0x00,  0x00,  0xFF,  0xC0, 
 0x03,  0xFF,  0xFF,  0xFF,  0xFF,  0x80,  0x7F,  0xC0,  0x01,  0xFF,  0xFF,  0xFF,  0xFF,  0xF0,  0x3F,  0x80, 
 0x01,  0xFF,  0xFF,  0xFF,  0xFF,  0xF8,  0x1F,  0x80,  0x1F,  0xFF,  0xFF,  0xFF,  0xFF,  0xFC,  0x0F,  0xF8, 
 0x1F,  0xFF,  0xFF,  0xFF,  0xFF,  0xFE,  0x0F,  0xF8,  0x1F,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0x0F,  0xF8, 
 0x0F,  0xFF,  0xFF,  0xFF,  0xFF,  0xFF,  0x0F,  0xF0,  0x07,  0x3F,  0xFF,  0xFF,  0xFF,  0xFF,  0x1C,  0xE0, 
 0x1E,  0x18,  0x7F,  0xE0,  0x07,  0xFF,  0x18,  0x78,  0x7F,  0x1C,  0x7F,  0xE0,  0x03,  0xFF,  0x38,  0xFE, 
 0x7F,  0xF8,  0x7F,  0xE0,  0x03,  0xFF,  0x1F,  0xFE,  0x3F,  0xF0,  0x7F,  0xE0,  0x0F,  0xFE,  0x0F,  0xFC, 
 0x1F,  0xC0,  0x7F,  0xFF,  0xFF,  0xFE,  0x03,  0xF8,  0x1F,  0x80,  0x7F,  0xFF,  0xFF,  0xFC,  0x01,  0xF8, 
 0x3F,  0x80,  0x7F,  0xFF,  0xFF,  0xF8,  0x01,  0xFC,  0x7F,  0x80,  0x7F,  0xFF,  0xFF,  0xF0,  0x01,  0xFE, 
 0x7F,  0x80,  0x7F,  0xFF,  0xFF,  0xF8,  0x01,  0xFE,  0x3F,  0x80,  0x7F,  0xFF,  0xFF,  0xFC,  0x01,  0xFC, 
 0x1F,  0x80,  0x7F,  0xFF,  0xFF,  0xFC,  0x0F,  0xF8,  0x1F,  0x80,  0x7F,  0xE0,  0x0F,  0xFE,  0x0F,  0xF8, 
 0x3F,  0x80,  0x7F,  0xE0,  0x0F,  0xFE,  0x0F,  0xFC,  0x7F,  0x80,  0x7F,  0xE0,  0x07,  0xFF,  0x0F,  0xFE, 
 0x7F,  0xC0,  0x7F,  0xE0,  0x07,  0xFF,  0xFF,  0xFE,  0x1F,  0xFF,  0xFF,  0xFF,  0x87,  0xFF,  0xFF,  0xF8, 
 0x07,  0xFF,  0xFF,  0xFF,  0x87,  0xFF,  0xFF,  0xE0,  0x0F,  0xFF,  0xFF,  0xFF,  0x83,  0xFF,  0xFF,  0xF0, 
 0x1F,  0xFF,  0xFF,  0xFF,  0x83,  0xFF,  0xFF,  0xF8,  0x1F,  0xFF,  0xFF,  0xFF,  0x83,  0xFF,  0xFF,  0xF8, 
 0x1F,  0xFF,  0xFF,  0xFF,  0x81,  0xFF,  0xFF,  0xF8,  0x01,  0xFF,  0xFF,  0xFF,  0x00,  0xFF,  0xFF,  0x80, 
 0x01,  0xFC,  0x00,  0x00,  0x00,  0x00,  0x3F,  0x80,  0x03,  0xFE,  0x30,  0x00,  0x00,  0x0C,  0x7F,  0xC0, 
 0x03,  0xFF,  0xF8,  0x00,  0x00,  0x1F,  0xFF,  0xC0,  0x03,  0xFF,  0xFC,  0x00,  0x00,  0x3F,  0xFF,  0xC0, 
 0x00,  0x3F,  0x1C,  0x00,  0x00,  0x38,  0xFC,  0x00,  0x00,  0x3F,  0x0C,  0x00,  0x00,  0x30,  0xFC,  0x00, 
 0x00,  0x3F,  0x1C,  0x00,  0x00,  0x38,  0xFC,  0x00,  0x00,  0x7F,  0xFE,  0x00,  0x00,  0x7F,  0xFE,  0x00, 
 0x00,  0x23,  0xFF,  0xC0,  0x03,  0xFF,  0xC4,  0x00,  0x00,  0x03,  0xFF,  0xFF,  0xFF,  0xFF,  0xC0,  0x00, 
 0x00,  0x03,  0xFF,  0xFF,  0xFF,  0xFF,  0xC0,  0x00,  0x00,  0x03,  0x9F,  0xFF,  0xFF,  0xF9,  0xC0,  0x00, 
 0x00,  0x00,  0x1F,  0xFF,  0xFF,  0xF8,  0x00,  0x00,  0x00,  0x00,  0x1E,  0xFF,  0xFF,  0x78,  0x00,  0x00, 
 0x00,  0x00,  0x1C,  0xFF,  0xFF,  0x38,  0x00,  0x00,  0x00,  0x00,  0x00,  0x73,  0xCE,  0x00,  0x00,  0x00, 
 0x00,  0x00,  0x00,  0x61,  0x86,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00,  0x00, 
];
