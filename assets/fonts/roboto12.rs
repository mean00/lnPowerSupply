// Generated by flatconvert-rs  https://github.com/mean00/flatconvert-rs.git 
// a modified version of adafruit fontconvert 
// RUST HEADER
#![allow(non_upper_case_globals)]
use ili9341::glyph::PFXglyph;
use ili9341::glyph::PFXfont;
const Roboto_Light12pt7b_bitmap :  [u8;3513] = [
		 0x9C, 0x78, 0xF1, 0xC0, 0x43, 0x4D, 0x9A, 0x74, 0xE9, 0xB0, 0x08, 0x04,
		 0x3A, 0x70, 0xB9, 0x77, 0x36, 0xF2, 0xDE, 0x55, 0xCB, 0x00, 0x80, 0x4D,
		 0x21, 0xB0, 0x08, 0x1D, 0x03, 0x81, 0x00, 0xB8, 0x47, 0x20, 0x10, 0x78,
		 0x10, 0x30, 0xEC, 0x25, 0x10, 0x3F, 0xFF, 0xFF, 0xFB, 0x11, 0xE8, 0xDC,
		 0x60, 0x10, 0xD8, 0x3C, 0x02, 0x03, 0xC0, 0xD8, 0x40, 0x25, 0x90, 0xA1,
		 0xE1, 0xC0, 0x74, 0x21, 0xE2, 0x83, 0xF4, 0x89, 0xD1, 0x78, 0x84, 0xA3,
		 0x2C, 0x80, 0x42, 0xE0, 0x58, 0x08, 0x07, 0x82, 0x68, 0x3C, 0x26, 0x83,
		 0x00, 0x80, 0x4E, 0x20, 0x10, 0x0F, 0x04, 0x02, 0x07, 0x80, 0x80, 0x6F,
		 0xFC, 0x10, 0xFC, 0xD7, 0xC3, 0xE1, 0x02, 0xE9, 0xE0, 0x20, 0x7C, 0xE8,
		 0x04, 0x2C, 0x50, 0x62, 0x9B, 0x41, 0x71, 0xDF, 0x01, 0x0D, 0xFF, 0x20,
		 0x18, 0x23, 0xC3, 0x02, 0xE8, 0x90, 0x8F, 0xB0, 0x20, 0x76, 0x10, 0x09,
		 0x67, 0xC2, 0x03, 0xE1, 0xFC, 0xDB, 0xE8, 0x25, 0x62, 0x6C, 0xA0, 0x8F,
		 0xF8, 0x20, 0x10, 0x09, 0xC5, 0xE0, 0x0C, 0x76, 0x11, 0xC8, 0x1D, 0x03,
		 0xC1, 0x0C, 0x85, 0xC0, 0x00, 0xC6, 0x3A, 0x16, 0x53, 0x81, 0x32, 0xB8,
		 0x40, 0x23, 0xFD, 0x01, 0xC2, 0x81, 0x70, 0x00, 0xC6, 0x0F, 0x0B, 0xF8,
		 0x40, 0x21, 0xD2, 0xD9, 0x64, 0x02, 0x3B, 0x30, 0x87, 0x40, 0x26, 0x97,
		 0x08, 0x51, 0x00, 0x83, 0x8C, 0x07, 0x81, 0x34, 0x86, 0xC0, 0xB0, 0x11,
		 0xD9, 0xE0, 0xE0, 0x8D, 0x88, 0x81, 0x7F, 0xA0, 0x10, 0x0D, 0xCE, 0xF2,
		 0x01, 0x0E, 0x80, 0xF4, 0x00, 0x46, 0x1A, 0x0C, 0x5C, 0x08, 0x07, 0xC2,
		 0xF1, 0x00, 0x83, 0xDB, 0x85, 0x06, 0x5F, 0x40, 0x06, 0x1F, 0x84, 0x02,
		 0x03, 0xE3, 0xE8, 0x45, 0x27, 0x90, 0xF8, 0x1E, 0x2E, 0x01, 0x78, 0x9C,
		 0x06, 0x0F, 0xAE, 0x1F, 0x00, 0x81, 0x8E, 0x94, 0xF6, 0x05, 0xF5, 0xBF,
		 0xED, 0x20, 0xFF, 0xFC, 0x17, 0x40, 0xDD, 0xEE, 0xB4, 0x00, 0x80, 0x4C,
		 0x20, 0x7C, 0x09, 0x64, 0x03, 0xA1, 0x0B, 0x80, 0x4D, 0x20, 0x5C, 0x08,
		 0x74, 0x02, 0x71, 0x00, 0xF0, 0x41, 0xE0, 0x11, 0xD8, 0x05, 0xD2, 0x03,
		 0xD0, 0x83, 0xD0, 0x03, 0x50, 0x50, 0x60, 0x61, 0x05, 0x1C, 0x80, 0x11,
		 0x0C, 0xF0, 0xA4, 0x40, 0xC8, 0x40, 0x7A, 0x10, 0x1E, 0x04, 0x0B, 0x81,
		 0x03, 0xA0, 0x42, 0xE0, 0x11, 0xC8, 0x04, 0xE2, 0x01, 0xE0, 0x0A, 0x10,
		 0x81, 0x96, 0x40, 0x36, 0x00, 0xCB, 0xE0, 0x81, 0xE0, 0x05, 0x07, 0x60,
		 0x36, 0x32, 0xC2, 0x41, 0x8E, 0xC0, 0x32, 0x10, 0x00, 0x80, 0x6C, 0x20,
		 0x10, 0xB8, 0x00, 0x20, 0xC2, 0xE9, 0x52, 0xBB, 0x7F, 0xFF, 0xA1, 0x1F,
		 0xC0, 0x40, 0xBF, 0x30, 0x0B, 0xA7, 0x82, 0x1D, 0x07, 0xA0, 0x60, 0x22,
		 0x80, 0x80, 0x42, 0x60, 0x10, 0x08, 0x1E, 0x00, 0x0C, 0x7C, 0x00, 0x43,
		 0x38, 0x02, 0x18, 0x70, 0x61, 0xC0, 0xFF, 0xFF, 0xFF, 0xE5, 0x96, 0xFB,
		 0x65, 0x0C, 0x51, 0x9A, 0x82, 0x47, 0x00, 0x06, 0x71, 0x34, 0x80, 0x00,
		 0xDF, 0xFC, 0x2A, 0x94, 0x00, 0x9C, 0xF8, 0x00, 0x80, 0x40, 0xF0, 0x10,
		 0x0E, 0x84, 0x03, 0x61, 0x00, 0x9A, 0x40, 0x23, 0x90, 0x08, 0x70, 0x60,
		 0xD0, 0x20, 0x5C, 0x08, 0x07, 0x82, 0x01, 0x70, 0x80, 0x4E, 0x20, 0x10,
		 0xD8, 0x04, 0x2C, 0x6C, 0xA0, 0x21, 0xC6, 0x36, 0x32, 0xC8, 0x04, 0x00,
		 0x81, 0xFF, 0xA8, 0x13, 0xEA, 0xFD, 0x0F, 0x81, 0x0B, 0xA7, 0x40, 0x21,
		 0xD7, 0x48, 0x04, 0xEB, 0x81, 0x00, 0xDD, 0xC0, 0x20, 0x5E, 0xC8, 0x04,
		 0x2F, 0x61, 0x00, 0x97, 0x05, 0x38, 0xA8, 0xCE, 0x3C, 0x10, 0x0F, 0x19,
		 0x00, 0xD0, 0xFC, 0xBF, 0x48, 0x17, 0xFE, 0x80, 0x80, 0xEC, 0x6F, 0xFD,
		 0xFC, 0x3D, 0x42, 0x39, 0x00, 0xBA, 0x40, 0x7A, 0x10, 0x7A, 0x04, 0x74,
		 0x1B, 0xC0, 0x83, 0x7F, 0xE8, 0x11, 0xFA, 0xDF, 0x42, 0x79, 0x00, 0xDA,
		 0x6C, 0x20, 0x17, 0x4F, 0x00, 0x10, 0xD0, 0x20, 0x13, 0x48, 0x04, 0x00,
		 0x28, 0x60, 0x01, 0x43, 0x03, 0xE0, 0x40, 0x21, 0x61, 0xA3, 0x2D, 0x0A,
		 0x1D, 0xA4, 0x02, 0x02, 0x1A, 0x30, 0x7C, 0x04, 0x02, 0x3C, 0x14, 0x33,
		 0xDA, 0x8D, 0x47, 0x7F, 0xFF, 0xFE, 0x80, 0x83, 0x7F, 0xE8, 0x17, 0xEA,
		 0xFD, 0x0F, 0x01, 0x0F, 0x96, 0x40, 0x21, 0xD6, 0x08, 0x04, 0xE2, 0x01,
		 0x00, 0xF0, 0x40, 0x21, 0x70, 0x08, 0x0E, 0xD2, 0x02, 0x2A, 0x30, 0x1A,
		 0xFD, 0x02, 0x01, 0x07, 0x02, 0x18, 0x7E, 0x82, 0x01, 0x2F, 0xE0, 0x40,
		 0x36, 0xF8, 0x08, 0x1E, 0x2F, 0xAB, 0xBC, 0x2B, 0x1C, 0x00, 0x80, 0x40,
		 0x3E, 0x00, 0x43, 0x7D, 0x02, 0x19, 0xF8, 0x10, 0xCB, 0x7A, 0x10, 0x08,
		 0xEC, 0xD2, 0x01, 0x0E, 0x8E, 0xC0, 0x21, 0x70, 0x7A, 0x04, 0x1E, 0x83,
		 0xC0, 0x81, 0xF0, 0x26, 0x90, 0x1E, 0x80, 0x60, 0xF8, 0x03, 0x04, 0x84,
		 0x7A, 0x17, 0xFF, 0xFF, 0xFF, 0x8B, 0x15, 0x96, 0xD4, 0x20, 0x01, 0x83,
		 0x00, 0x14, 0x12, 0xA1, 0xBA, 0x40, 0x00, 0x87, 0xFF, 0xFF, 0x90, 0xFA,
		 0xAE, 0x52, 0x1D, 0x00, 0x80, 0x43, 0x40, 0x92, 0x38, 0x04, 0x31, 0xDF,
		 0xFE, 0x02, 0x3F, 0xA6, 0xFA, 0x42, 0x20, 0x17, 0x80, 0xA1, 0x96, 0x06,
		 0x90, 0x11, 0x49, 0x02, 0x06, 0x71, 0x00, 0x96, 0x86, 0x0D, 0xE2, 0x1F,
		 0x95, 0xFA, 0x40, 0x87, 0x44, 0x80, 0x4B, 0xF8, 0x10, 0x1F, 0xD5, 0x02,
		 0x0F, 0x90, 0x80, 0x43, 0xA0, 0x10, 0x09, 0x60, 0x10, 0xCE, 0x00, 0x86,
		 0xE5, 0xBF, 0xC0, 0x5F, 0xF2, 0xDF, 0x0D, 0xE4, 0x02, 0xF1, 0xB0, 0x80,
		 0x4F, 0x00, 0x86, 0x38, 0x04, 0x57, 0x40, 0x21, 0x17, 0x19, 0xE4, 0xB6,
		 0x01, 0x74, 0x87, 0xE5, 0xBC, 0x10, 0x2F, 0xFD, 0x00, 0x9F, 0xFF, 0xFF,
		 0xF4, 0x58, 0xAD, 0x4F, 0x81, 0x00, 0x80, 0x78, 0x00, 0x87, 0x60, 0x04,
		 0x33, 0x80, 0x21, 0x8E, 0x81, 0x0C, 0x38, 0x08, 0x60, 0xF4, 0x08, 0x04,
		 0x0C, 0x54, 0x60, 0x3D, 0x00, 0x91, 0xF8, 0x01, 0x0D, 0xD0, 0x08, 0x65,
		 0x82, 0xA9, 0x0B, 0x02, 0x18, 0x1E, 0x02, 0x01, 0x02, 0x2E, 0x20, 0x83,
		 0x7F, 0xE8, 0x11, 0xFA, 0xDF, 0x82, 0x79, 0x00, 0xDA, 0x5D, 0x20, 0x13,
		 0x80, 0x25, 0x0B, 0x1B, 0xA4, 0x7A, 0x8D, 0xE0, 0x83, 0x8B, 0x95, 0x6B,
		 0xC0, 0x1C, 0x37, 0x8D, 0x84, 0x02, 0x59, 0xE0, 0x02, 0x1F, 0x84, 0x02,
		 0x38, 0x16, 0x52, 0xFA, 0xB7, 0xD0, 0x5C, 0x70, 0x00, 0x83, 0x7F, 0xE0,
		 0x17, 0xDB, 0x7C, 0x0F, 0x81, 0x0F, 0x96, 0x40, 0x23, 0xBE, 0x08, 0x04,
		 0xFF, 0x01, 0x00, 0xFF, 0x40, 0x20, 0x7E, 0xC8, 0x04, 0x3E, 0x79, 0x00,
		 0xDE, 0x6E, 0x6D, 0xFB, 0x0B, 0xFF, 0x4F, 0x81, 0x00, 0x87, 0x40, 0x20,
		 0x1B, 0x01, 0x21, 0x07, 0x1B, 0xE4, 0x02, 0xDF, 0xC0, 0x85, 0xFA, 0x20,
		 0x00, 0xBA, 0xF8, 0x20, 0x00, 0x09, 0x1D, 0xBC, 0x00, 0x8E, 0xCB, 0x60,
		 0x00, 0x0D, 0x04, 0x8E, 0x00, 0x0C, 0xE2, 0x69, 0x00, 0x80, 0x40, 0x25,
		 0x10, 0x08, 0xDF, 0x42, 0x0F, 0xF4, 0x80, 0xFF, 0x60, 0x13, 0xEA, 0x04,
		 0x03, 0xF2, 0x1C, 0x37, 0xF0, 0x51, 0x8F, 0xF4, 0x20, 0x10, 0x6F, 0xA0,
		 0x48, 0xE8, 0x00, 0xBF, 0xFF, 0xFF, 0x15, 0x5A, 0xAD, 0x42, 0x00, 0x00,
		 0xB0, 0x1F, 0xFF, 0xFF, 0xC2, 0x1A, 0x34, 0x00, 0xD0, 0x40, 0x20, 0x3F,
		 0xD8, 0x04, 0x02, 0xFF, 0x40, 0x80, 0x46, 0xFC, 0x10, 0x08, 0x17, 0xD2,
		 0x01, 0x0B, 0xE8, 0x41, 0xBF, 0x02, 0x04, 0x1E, 0x30, 0x2C, 0x00, 0xA8,
		 0x80, 0x81, 0xFF, 0xA8, 0x1F, 0x9D, 0xE4, 0xF2, 0x01, 0xE3, 0xC0, 0x43,
		 0xE2, 0x10, 0x0F, 0x84, 0x02, 0x1F, 0x00, 0x80, 0xF4, 0x20, 0x17, 0x88,
		 0x04, 0x7A, 0x01, 0x03, 0xE0, 0x40, 0x36, 0x10, 0x08, 0x5C, 0x00, 0x01,
		 0xC2, 0x4B, 0x69, 0x00, 0x80, 0x41, 0xBF, 0xFE, 0x48, 0x04, 0x02, 0x7D,
		 0x84, 0xD7, 0x82, 0x0F, 0xD2, 0x01, 0x02, 0xE8, 0x40, 0xF8, 0x00, 0xE3,
		 0x74, 0x85, 0xD0, 0x01, 0x06, 0x51, 0x1C, 0x80, 0x5F, 0xF8, 0x11, 0xC9,
		 0x64, 0x07, 0xC9, 0xF0, 0x86, 0xCD, 0x20, 0xF4, 0x0D, 0x84, 0x2A, 0xE1,
		 0x0B, 0x80, 0x02, 0x0E, 0xC2, 0x1A, 0x08, 0xBA, 0x08, 0x76, 0xC0, 0x78,
		 0x7C, 0x01, 0x22, 0x08, 0x50, 0xD0, 0x41, 0x80, 0xF4, 0x21, 0x97, 0x08,
		 0xEC, 0x0B, 0xC1, 0x2C, 0x9A, 0x43, 0xEB, 0xDB, 0x4D, 0x84, 0xE2, 0x0D,
		 0xF8, 0xB7, 0xF0, 0x23, 0xA7, 0xE2, 0x00, 0x30, 0xF0, 0x43, 0x81, 0xF4,
		 0x01, 0x4F, 0xF3, 0x45, 0xD8, 0x02, 0x0C, 0x6F, 0xFF, 0x90, 0x80, 0x00,
		 0x80, 0x42, 0xE8, 0x10, 0x08, 0x04, 0xFC, 0x08, 0x60, 0x3D, 0xD0, 0x31,
		 0x85, 0xCE, 0x00, 0xC6, 0x71, 0xB0, 0x0A, 0x1E, 0x87, 0x42, 0x01, 0x0F,
		 0x81, 0xE0, 0x02, 0x86, 0x0F, 0x00, 0x81, 0x70, 0x21, 0xD0, 0x00, 0x11,
		 0x9E, 0x40, 0x2E, 0x90, 0x0B, 0x84, 0x0B, 0xFF, 0xFF, 0xF0, 0x43, 0xAA,
		 0xB5, 0x6E, 0x80, 0x50, 0xC0, 0xF0, 0x7C, 0x01, 0xF1, 0x15, 0x18, 0x04,
		 0xB0, 0xA8, 0x48, 0xC4, 0xFF, 0xFF, 0xE8, 0x1F, 0x5A, 0xB7, 0xC3, 0xC1,
		 0x00, 0xDA, 0x01, 0x0C, 0xF0, 0x09, 0xEF, 0x1E, 0x08, 0x37, 0x80, 0x5C,
		 0x72, 0x1F, 0x41, 0x73, 0x0E, 0x28, 0xE8, 0x13, 0xDF, 0x04, 0x47, 0xE8,
		 0x2E, 0x40, 0x80, 0x6F, 0xFC, 0x90, 0x0B, 0xF5, 0x5F, 0xC1, 0x0F, 0x80,
		 0x43, 0xE0, 0x5D, 0x08, 0x04, 0xF2, 0x71, 0x00, 0x81, 0x71, 0xE8, 0x10,
		 0x08, 0x65, 0xD2, 0x01, 0x00, 0x85, 0x81, 0x0C, 0x03, 0x60, 0x0D, 0x10,
		 0x71, 0x42, 0x1B, 0x2E, 0x11, 0x11, 0x14, 0x8C, 0x2E, 0x80, 0x60, 0x46,
		 0xE3, 0xF0, 0x80, 0x00, 0xFF, 0xFF, 0xA8, 0x13, 0xDA, 0xAF, 0xE2, 0x1D,
		 0x00, 0x85, 0xE0, 0xF0, 0x10, 0x0D, 0xA7, 0x82, 0x01, 0x0F, 0x9C, 0x40,
		 0x20, 0x5C, 0xE8, 0x04, 0x03, 0x66, 0x18, 0x31, 0xD0, 0xC1, 0x83, 0xDE,
		 0x20, 0x10, 0x1E, 0xF0, 0x62, 0x4B, 0x06, 0x52, 0x17, 0x4E, 0x26, 0x2E,
		 0x8D, 0xBF, 0x00, 0x66, 0x50, 0x00, 0xFF, 0x80, 0x0F, 0x56, 0x2B, 0x5E,
		 0x80, 0x40, 0x27, 0x00, 0x43, 0xE0, 0x80, 0x40, 0xF0, 0x10, 0x08, 0x70,
		 0x52, 0x0A, 0x0F, 0x8F, 0xA3, 0x55, 0x0A, 0x51, 0x57, 0x9E, 0xC5, 0x6A,
		 0x82, 0xA3, 0xFC, 0xFF, 0x80, 0x0F, 0x56, 0x2B, 0x5E, 0x80, 0x40, 0x27,
		 0x00, 0x43, 0xE0, 0x80, 0x40, 0xF0, 0x10, 0x08, 0x70, 0x38, 0xCF, 0xFF,
		 0xFF, 0x8F, 0xAD, 0x16, 0xA6, 0x14, 0x82, 0xB4, 0x14, 0x90, 0x00, 0x80,
		 0x6F, 0xFE, 0x90, 0x09, 0xFD, 0x6B, 0xE1, 0x0F, 0xA0, 0x41, 0xF0, 0x5C,
		 0x08, 0x04, 0xB6, 0x79, 0x00, 0x80, 0x74, 0xF0, 0x00, 0x63, 0x74, 0x02,
		 0x18, 0x3D, 0x00, 0x19, 0xA5, 0xFF, 0xE0, 0x61, 0xAA, 0xF8, 0xC5, 0x06,
		 0x1C, 0x36, 0x5E, 0x1E, 0x80, 0x30, 0x94, 0x8C, 0x07, 0xA1, 0x3E, 0xCB,
		 0x7E, 0x20, 0x17, 0xFF, 0xCC, 0x00, 0xF0, 0x40, 0x20, 0x7F, 0x88, 0x04,
		 0x03, 0xFC, 0x04, 0x33, 0xFC, 0x04, 0x02, 0x1E, 0x19, 0x9F, 0xF0, 0x01,
		 0xFE, 0xC5, 0x6A, 0xA3, 0x3E, 0x18, 0xDC, 0x00, 0xDD, 0x80, 0x36, 0xC0,
		 0x80, 0x40, 0x23, 0xA0, 0x5E, 0x05, 0xF1, 0xDB, 0xA0, 0x10, 0xCE, 0x20,
		 0x13, 0xC9, 0xE4, 0x02, 0xF1, 0x0F, 0xCD, 0x7C, 0x20, 0x5F, 0xFC, 0x00,
		 0xF0, 0x40, 0x23, 0xF3, 0x80, 0x21, 0x87, 0x01, 0x0C, 0x0F, 0x01, 0x1F,
		 0x80, 0x78, 0x00, 0x86, 0x70, 0x04, 0x30, 0xE0, 0x21, 0x81, 0xE7, 0xE0,
		 0x10, 0x0F, 0x7F, 0xC2, 0x01, 0x3F, 0xAF, 0x40, 0x21, 0xF4, 0x1F, 0x00,
		 0xC8, 0xCF, 0x46, 0x46, 0x07, 0xD0, 0x26, 0x1B, 0xE1, 0x30, 0xC1, 0xF8,
		 0x78, 0x08, 0x06, 0xD0, 0xCC, 0x61, 0x74, 0x00, 0xF0, 0x40, 0x20, 0x78,
		 0x08, 0x04, 0x3A, 0x01, 0x00, 0x9C, 0x01, 0x08, 0x57, 0x85, 0x6C, 0xF6,
		 0x8B, 0x54, 0xFF, 0xFF, 0xFE, 0x00, 0xFA, 0x40, 0x20, 0x10, 0xBF, 0xF8,
		 0x18, 0xC0, 0xFD, 0xFC, 0x08, 0x04, 0x0B, 0xF7, 0xB8, 0x08, 0x1F, 0xF7,
		 0xAC, 0x0C, 0x6E, 0xFB, 0x8F, 0x04, 0x02, 0x79, 0xEE, 0x9E, 0x40, 0x21,
		 0xD3, 0xBC, 0x3D, 0x02, 0x17, 0x0E, 0xF0, 0x7C, 0x20, 0x78, 0x3D, 0xC4,
		 0xB2, 0x03, 0xD0, 0xF7, 0x40, 0xF0, 0x1F, 0x09, 0xDE, 0x03, 0x61, 0x74,
		 0x87, 0xF8, 0x23, 0xB2, 0xC8, 0x1F, 0xE2, 0x07, 0xCE, 0x80, 0x7B, 0xA0,
		 0x17, 0xF8, 0x00, 0x80, 0xC3, 0xF8, 0x04, 0x23, 0x02, 0xE8, 0x0D, 0x88,
		 0xF8, 0x40, 0x20, 0x5F, 0xE8, 0x04, 0x03, 0xFF, 0xD0, 0x80, 0x4B, 0xF7,
		 0x90, 0x08, 0x7F, 0x9F, 0x01, 0x03, 0xFC, 0x6D, 0x20, 0x1B, 0xE8, 0x7C,
		 0x02, 0x7F, 0x81, 0xF0, 0x43, 0xFC, 0x12, 0xD8, 0x17, 0xE2, 0x07, 0xC0,
		 0x1C, 0x1B, 0xC4, 0xFF, 0x01, 0x0B, 0xA5, 0xF8, 0x20, 0x1F, 0x3F, 0xC4,
		 0x02, 0x3F, 0xBE, 0x2C, 0x1F, 0xFE, 0x00, 0x70, 0x43, 0x06, 0x0F, 0xC0,
		 0x80, 0x6F, 0xFC, 0x90, 0x0B, 0xF5, 0x6F, 0xA1, 0x0F, 0x80, 0x43, 0xE8,
		 0x5D, 0x08, 0x04, 0xB2, 0x71, 0x00, 0x80, 0xF5, 0xE8, 0x10, 0x08, 0x76,
		 0xC2, 0x01, 0x00, 0xF5, 0x81, 0x0C, 0x3C, 0x18, 0xBF, 0x60, 0xCB, 0xE7,
		 0x14, 0x21, 0xB2, 0xE9, 0xF0, 0x20, 0x12, 0xD8, 0x7D, 0x02, 0x17, 0x40,
		 0x30, 0x23, 0x73, 0xFF, 0xFF, 0xF2, 0x1F, 0x5A, 0x4F, 0xD3, 0xC1, 0x00,
		 0x9E, 0x81, 0x0C, 0x78, 0x08, 0x61, 0xE0, 0x4F, 0x2D, 0xF0, 0x41, 0x3F,
		 0x03, 0x43, 0xE0, 0xFA, 0x55, 0x20, 0x01, 0x62, 0x05, 0xE0, 0x80, 0x6F,
		 0xFC, 0x90, 0x0B, 0xF5, 0x6F, 0xA1, 0x0F, 0x80, 0x43, 0xE0, 0x7C, 0x08,
		 0x04, 0xF2, 0x71, 0x00, 0x80, 0xF5, 0xE8, 0x10, 0x08, 0x76, 0xC2, 0x01,
		 0x00, 0xF5, 0x81, 0x0C, 0x3C, 0x18, 0xBF, 0x60, 0xCB, 0xE7, 0x14, 0x21,
		 0xB2, 0xE9, 0x91, 0x11, 0x49, 0x18, 0x19, 0xB8, 0xFF, 0x05, 0x86, 0x03,
		 0xF0, 0x03, 0x18, 0x18, 0x50, 0xC0, 0x21, 0x20, 0xFF, 0xFF, 0xF0, 0x13,
		 0xDA, 0x96, 0xF2, 0x1D, 0x00, 0x83, 0xF0, 0xF0, 0x10, 0x0D, 0x87, 0x82,
		 0x01, 0x0E, 0x9C, 0x40, 0x20, 0x78, 0xE8, 0x04, 0x07, 0xA6, 0x18, 0x3F,
		 0x0F, 0x04, 0x07, 0xE9, 0x3F, 0xFF, 0xFC, 0x21, 0xF5, 0x4D, 0xA4, 0x0F,
		 0x01, 0x0F, 0x80, 0x06, 0x0F, 0x80, 0x64, 0x65, 0xA4, 0xC3, 0x03, 0x26,
		 0x2B, 0xC1, 0x30, 0x88, 0x08, 0x81, 0x7F, 0xFA, 0x10, 0x7F, 0x2E, 0xBE,
		 0x07, 0xC0, 0x80, 0xF8, 0x7A, 0x10, 0x09, 0x66, 0xC2, 0x01, 0x07, 0x8E,
		 0xC0, 0x20, 0x10, 0x30, 0x91, 0x80, 0x4B, 0xC0, 0xCB, 0x7E, 0x06, 0x60,
		 0x43, 0x01, 0xF8, 0x01, 0x8C, 0xB3, 0xC0, 0x32, 0x37, 0xC0, 0x91, 0xEB,
		 0xE0, 0x20, 0x1F, 0x0B, 0xF6, 0x5B, 0xE8, 0x66, 0x21, 0xC2, 0xDF, 0xFF,
		 0xFF, 0xFE, 0x1A, 0xAF, 0xD6, 0x2B, 0x00, 0x85, 0xC0, 0x00, 0x07, 0x60,
		 0x04, 0x20, 0xDE, 0x0D, 0xE0, 0xD8, 0x9C, 0x40, 0x20, 0x5C, 0xE8, 0x04,
		 0x03, 0x67, 0x80, 0x80, 0x4B, 0x3C, 0x10, 0x08, 0x58, 0x67, 0x86, 0x5C,
		 0x0F, 0x9E, 0x18, 0x3E, 0x10, 0x91, 0xF8, 0x4B, 0xEA, 0xDF, 0x48, 0x0E,
		 0xFF, 0xC9, 0x00, 0xD8, 0x40, 0x20, 0x1B, 0x4F, 0x04, 0x02, 0x07, 0xC3,
		 0xE0, 0x40, 0x21, 0xD0, 0x7A, 0x04, 0x03, 0x61, 0x0E, 0x0A, 0x1E, 0x04,
		 0xB2, 0x01, 0x1D, 0x80, 0x5D, 0x02, 0x86, 0x01, 0xF0, 0x81, 0xE0, 0x20,
		 0x5C, 0x00, 0xA1, 0x05, 0x10, 0xA1, 0x87, 0x82, 0x08, 0xA0, 0xC7, 0x0A,
		 0x86, 0x70, 0x5C, 0x5B, 0x72, 0x61, 0x80, 0xFB, 0x01, 0x86, 0x07, 0xF0,
		 0x03, 0x18, 0x58, 0x50, 0x80, 0xBA, 0x40, 0x21, 0xD0, 0x08, 0x1F, 0x3A,
		 0x01, 0x07, 0xE0, 0x40, 0x3C, 0x7C, 0x08, 0x17, 0xD2, 0x01, 0x34, 0xD8,
		 0x40, 0x3D, 0xD0, 0x08, 0xEC, 0x76, 0x01, 0x36, 0xE0, 0x43, 0xE0, 0x78,
		 0x09, 0x65, 0xC0, 0x0C, 0x76, 0x10, 0xF8, 0x6C, 0x07, 0xA1, 0x1D, 0x81,
		 0xD0, 0xC2, 0x06, 0x07, 0x81, 0xE0, 0x5D, 0x25, 0x90, 0x0F, 0x07, 0xC2,
		 0x1A, 0x60, 0x32, 0xC9, 0xC4, 0x0F, 0x0F, 0x40, 0x83, 0xD7, 0x20, 0x1B,
		 0x13, 0x51, 0xF5, 0xC0, 0x21, 0xA6, 0x43, 0x3B, 0xC0, 0x40, 0xF6, 0x90,
		 0x08, 0x5F, 0x42, 0x01, 0xBD, 0x80, 0x40, 0x7E, 0x10, 0x08, 0xFC, 0x02,
		 0x01, 0x3C, 0x52, 0x1C, 0x04, 0x00, 0x9E, 0x40, 0x20, 0x3E, 0x0B, 0xC4,
		 0x02, 0x1F, 0x00, 0xDA, 0x00, 0x97, 0xC2, 0x0F, 0x80, 0x0C, 0x11, 0x01,
		 0x81, 0x74, 0xBA, 0x10, 0x08, 0x1F, 0x7A, 0x01, 0x00, 0x83, 0xFC, 0x00,
		 0xC6, 0x1F, 0x40, 0x03, 0x1B, 0xF8, 0x10, 0x87, 0x1C, 0xF2, 0x3C, 0x54,
		 0x43, 0xA2, 0x54, 0x25, 0xC2, 0x30, 0x37, 0x82, 0xA1, 0x05, 0x13, 0xB3,
		 0xDA, 0x40, 0x20, 0x1F, 0x0F, 0x84, 0x02, 0x17, 0x41, 0xF0, 0x40, 0x27,
		 0x10, 0x3E, 0x04, 0x0B, 0xA1, 0x07, 0xC0, 0x43, 0x83, 0x06, 0x81, 0xB0,
		 0x80, 0x43, 0xE0, 0xF4, 0x08, 0x04, 0x76, 0x78, 0x36, 0x33, 0xEE, 0x00,
		 0x18, 0xDF, 0x80, 0x84, 0x8C, 0x40, 0x86, 0x04, 0x14, 0x50, 0xB0, 0xA2,
		 0x96, 0x01, 0x91, 0x00, 0x87, 0x38, 0xFF, 0x80, 0x0C, 0x52, 0xAB, 0x57,
		 0xE0, 0x40, 0x20, 0x60, 0x43, 0x01, 0xF0, 0x40, 0x20, 0x1F, 0x00, 0x24,
		 0xDA, 0x01, 0x0D, 0xE0, 0x08, 0x67, 0x80, 0x43, 0x2D, 0x02, 0x18, 0xF0,
		 0x10, 0xC3, 0xC0, 0x86, 0x17, 0x40, 0x80, 0x41, 0xF0, 0x04, 0x87, 0x03,
		 0xEA, 0xC5, 0x6A, 0x86, 0x65, 0x40, 0x9F, 0xF3, 0xAA, 0x78, 0x0F, 0x04,
		 0xE2, 0x1D, 0x03, 0x04, 0xF0, 0x43, 0xCA, 0x7F, 0xC0, 0xBA, 0x40, 0x20,
		 0x78, 0x08, 0x04, 0xB2, 0x01, 0x00, 0xF0, 0x40, 0x21, 0xC0, 0x82, 0x19,
		 0xE1, 0x9E, 0xFF, 0xD6, 0xE1, 0x60, 0x1E, 0x00, 0x55, 0xBF, 0xF8, 0x80,
		 0x7C, 0x20, 0x17, 0xD8, 0x04, 0xEF, 0x01, 0x1C, 0xD8, 0x42, 0xE1, 0xD0,
		 0x3C, 0x0F, 0x0B, 0x81, 0x34, 0xF0, 0x42, 0xC0, 0xFF, 0xFF, 0xFE, 0xB1,
		 0x58, 0xAC, 0x40, 0xBC, 0x42, 0xE8, 0x1B, 0x00, 0x83, 0xFF, 0xA8, 0x1B,
		 0x9B, 0x7D, 0x1E, 0x81, 0x1E, 0x96, 0x40, 0x00, 0x46, 0x01, 0xB0, 0x80,
		 0x40, 0xB8, 0x1A, 0xFF, 0xFC, 0x3F, 0x29, 0x2C, 0xDA, 0x40, 0x2E, 0x78,
		 0x08, 0x1F, 0x3E, 0x01, 0x1F, 0x8F, 0xD9, 0xFD, 0xB1, 0xBF, 0xE5, 0xD0,
		 0xBA, 0x40, 0x20, 0x00, 0x57, 0x75, 0xDF, 0xE0, 0x2F, 0x79, 0x6F, 0x05,
		 0xF2, 0x01, 0xB4, 0x0B, 0x19, 0xE0, 0x10, 0xCB, 0x00, 0x86, 0x38, 0x04,
		 0xE1, 0x65, 0x38, 0x17, 0x2B, 0xFD, 0x9B, 0xC1, 0x72, 0xFF, 0xD0, 0x00,
		 0x83, 0x7F, 0xA8, 0x17, 0xDB, 0x7D, 0x0F, 0x81, 0x0F, 0x96, 0x40, 0x23,
		 0xBF, 0x08, 0x04, 0x5F, 0x01, 0x00, 0x87, 0x40, 0x20, 0x13, 0x80, 0x21,
		 0x0A, 0x18, 0x17, 0x02, 0x01, 0xD3, 0xE0, 0x42, 0xE0, 0xFD, 0x5F, 0xA4,
		 0x1B, 0xFF, 0x00, 0x80, 0x40, 0x27, 0x00, 0x43, 0xE0, 0x80, 0x40, 0xF0,
		 0x10, 0x08, 0x70, 0x51, 0x46, 0xFF, 0x1E, 0x1F, 0xAE, 0xEF, 0x1F, 0x00,
		 0x9F, 0x5D, 0x20, 0x13, 0xAE, 0x04, 0x03, 0xDE, 0x2A, 0x3B, 0xC1, 0x51,
		 0xF8, 0x05, 0x1F, 0x9E, 0x81, 0x03, 0x15, 0x19, 0xEC, 0x7E, 0xB7, 0xF8,
		 0x8D, 0xFE, 0x7A, 0x00, 0x81, 0x7F, 0xE0, 0x17, 0xDA, 0xFD, 0x0F, 0x81,
		 0x0F, 0x8E, 0xC0, 0x21, 0xBE, 0x08, 0x04, 0xEF, 0xFF, 0xFF, 0xF7, 0x55,
		 0x62, 0xB3, 0x88, 0x04, 0x03, 0xE1, 0x00, 0x81, 0x70, 0x20, 0x10, 0x3E,
		 0x04, 0x1E, 0x0F, 0xD5, 0xDC, 0x10, 0x0E, 0x00, 0x80, 0x6F, 0xE0, 0x1B,
		 0x9C, 0x04, 0xE2, 0x01, 0x0E, 0x80, 0x41, 0xE8, 0x11, 0xFF, 0xFD, 0x06,
		 0xEB, 0x40, 0x8E, 0x84, 0x14, 0x07, 0xA1, 0x00, 0xBA, 0x40, 0x02, 0x3C,
		 0x83, 0x7F, 0xA7, 0x17, 0xDB, 0x77, 0x8F, 0x81, 0x0F, 0xCE, 0xC0, 0x21,
		 0xDB, 0x08, 0x04, 0xEF, 0x81, 0x00, 0xF7, 0xC0, 0x20, 0x7B, 0xC0, 0xA3,
		 0x9C, 0xF4, 0x20, 0x02, 0xA5, 0x87, 0xEB, 0x7F, 0x48, 0x0A, 0x01, 0x00,
		 0xF0, 0x40, 0x20, 0xF4, 0xD8, 0x04, 0xF2, 0x3F, 0x57, 0xE8, 0x46, 0xFF,
		 0x90, 0x00, 0xBA, 0x40, 0x20, 0x3D, 0x08, 0x04, 0x1E, 0x81, 0x00, 0x8E,
		 0xC0, 0x20, 0x01, 0x45, 0xD2, 0xFF, 0x41, 0xEF, 0xB6, 0xF8, 0xFD, 0x02,
		 0x59, 0x78, 0x80, 0x5D, 0x45, 0x47, 0xAF, 0x40, 0x81, 0xD7, 0x60, 0x10,
		 0xD1, 0x51, 0x9A, 0x85, 0x67, 0x40, 0x9C, 0x78, 0x20, 0x10, 0x0F, 0x1E,
		 0x3A, 0x70, 0x04, 0xBE, 0x00, 0x80, 0x78, 0x23, 0xB0, 0x00, 0x03, 0x87,
		 0x04, 0x08, 0x17, 0x81, 0x14, 0x07, 0xA3, 0xBC, 0xBF, 0x50, 0x00, 0xBA,
		 0x40, 0x20, 0x3D, 0x08, 0x04, 0x1E, 0x81, 0x00, 0x8E, 0xC0, 0x20, 0x01,
		 0x47, 0x03, 0xE3, 0xD0, 0x27, 0x91, 0xD8, 0x1F, 0x02, 0xE9, 0x3C, 0x80,
		 0xF4, 0xF8, 0x10, 0x7B, 0xE4, 0x02, 0x3F, 0xF8, 0x80, 0x5F, 0x2E, 0x82,
		 0xA3, 0xB4, 0x20, 0x1F, 0x81, 0x00, 0xFC, 0x02, 0x47, 0xA3, 0xD0, 0x0C,
		 0x10, 0xF7, 0x00, 0x38, 0xB8, 0xFF, 0xE8, 0x3B, 0xFC, 0x8F, 0x7A, 0xD7,
		 0x9F, 0xAD, 0xF1, 0xFA, 0x10, 0xB0, 0x11, 0x8F, 0x40, 0x00, 0x46, 0x1D,
		 0x74, 0x80, 0x4E, 0x20, 0x13, 0x9E, 0x84, 0x00, 0x04, 0x7C, 0xF4, 0x08,
		 0x18, 0x08, 0xE7, 0x60, 0x10, 0xD1, 0x23, 0x9A, 0x09, 0x78, 0x91, 0xCE,
		 0x00, 0xB8, 0xEF, 0xF0, 0x3D, 0xEC, 0xDE, 0x1F, 0xA1, 0x0B, 0x8F, 0x40,
		 0x23, 0xB7, 0x48, 0x04, 0xD7, 0xA1, 0x00, 0xEB, 0xD0, 0x20, 0x75, 0xD8,
		 0x04, 0x34, 0x2B, 0x26, 0x80, 0x81, 0x7F, 0xE8, 0x11, 0xFA, 0xBF, 0x82,
		 0x79, 0x00, 0xDA, 0x6C, 0x20, 0x13, 0xCF, 0x84, 0x02, 0x3B, 0xE0, 0x02,
		 0x38, 0x68, 0x10, 0xC3, 0x82, 0xC6, 0x38, 0x22, 0x42, 0xE4, 0x3A, 0x50,
		 0x3F, 0xF5, 0x00, 0xB8, 0xFF, 0xF0, 0x17, 0xBC, 0xBF, 0x82, 0xF9, 0x00,
		 0xDA, 0x5D, 0x20, 0x13, 0xC0, 0x21, 0x96, 0x01, 0x0C, 0x70, 0x09, 0xC2,
		 0xCA, 0x70, 0x2E, 0x57, 0xBC, 0xB7, 0x82, 0xEB, 0xFF, 0xA0, 0x05, 0x8C,
		 0x00, 0x0A, 0xC0, 0x83, 0x7F, 0xA7, 0x17, 0xDB, 0x77, 0x8F, 0x81, 0x0F,
		 0xCE, 0xC0, 0x21, 0xDB, 0x08, 0x04, 0xEF, 0x81, 0x00, 0xF7, 0xC0, 0x20,
		 0x7B, 0xC0, 0xA3, 0x9C, 0xF4, 0x20, 0x02, 0xA3, 0x0B, 0xC3, 0xF5, 0x7F,
		 0xC4, 0x05, 0x00, 0x80, 0x78, 0x05, 0x47, 0x00, 0x2A, 0x20, 0xE2, 0x14,
		 0x60, 0xB9, 0x7E, 0x7D, 0xD9, 0x7E, 0x04, 0x7A, 0x01, 0x74, 0x80, 0xF4,
		 0x20, 0xF4, 0x08, 0xE8, 0x34, 0x40, 0x00, 0x83, 0xFF, 0xA8, 0x7E, 0x5D,
		 0xE5, 0xD2, 0x03, 0xE7, 0xA0, 0x42, 0xAE, 0x90, 0x08, 0x17, 0xE2, 0x01,
		 0x06, 0xFF, 0x40, 0x20, 0x1B, 0xC0, 0x21, 0xFD, 0x40, 0x21, 0xFB, 0x48,
		 0x07, 0x8C, 0x60, 0x61, 0x7F, 0xEA, 0x00, 0x85, 0xC0, 0x25, 0x90, 0x0D,
		 0x84, 0xFF, 0xFD, 0x1B, 0xA2, 0x03, 0x14, 0x0B, 0x80, 0x1A, 0x80, 0xC7,
		 0x1D, 0x80, 0x4F, 0x68, 0x17, 0xF0, 0xBA, 0x40, 0x26, 0xBD, 0x08, 0x07,
		 0x5E, 0x81, 0x03, 0xAE, 0xC0, 0x21, 0xA1, 0x59, 0x34, 0xF0, 0x40, 0x3C,
		 0x7C, 0x08, 0x7D, 0x1F, 0xAD, 0xFE, 0x83, 0xFF, 0xA7, 0x00, 0xBA, 0x40,
		 0x25, 0x93, 0x88, 0x04, 0xE2, 0x79, 0x00, 0x9A, 0x47, 0x60, 0x1B, 0x08,
		 0x74, 0x03, 0xC1, 0x0B, 0x80, 0xF4, 0x20, 0xF0, 0x3E, 0x04, 0x0F, 0x07,
		 0x80, 0x81, 0x71, 0xE0, 0x10, 0x0E, 0xF8, 0x10, 0xFB, 0x40, 0x86, 0xF8,
		 0x04, 0x24, 0x22, 0x9C, 0x40, 0x3E, 0x10, 0x1E, 0x1E, 0x02, 0x3F, 0x00,
		 0x96, 0x4B, 0x20, 0x5F, 0x48, 0x1E, 0x07, 0xA1, 0x2B, 0xE0, 0x4D, 0x21,
		 0xD0, 0x3A, 0xE4, 0x2E, 0x01, 0xE0, 0xD8, 0x74, 0x36, 0x10, 0xB0, 0x01,
		 0x86, 0xC0, 0x2E, 0x1E, 0x0B, 0x8F, 0x02, 0x07, 0x9C, 0x12, 0x18, 0x04,
		 0xBB, 0x81, 0x3B, 0xC0, 0x40, 0x7F, 0x10, 0x1F, 0xA4, 0x02, 0x1F, 0x00,
		 0x2D, 0x13, 0x41, 0x01, 0x20, 0x9E, 0x40, 0x2F, 0x11, 0xD8, 0x07, 0xD2,
		 0x17, 0x41, 0xE8, 0x40, 0xF8, 0x78, 0x08, 0x0F, 0x7E, 0x01, 0x00, 0xDE,
		 0x81, 0x0D, 0xF0, 0x08, 0x7F, 0xB0, 0x08, 0x17, 0x2E, 0x80, 0x22, 0x50,
		 0xF8, 0x0F, 0x42, 0x5B, 0x00, 0xD8, 0x5E, 0x20, 0x13, 0xC0, 0xD8, 0x40,
		 0x27, 0x93, 0x88, 0x04, 0xE2, 0x79, 0x00, 0xBA, 0x47, 0x60, 0x1F, 0x08,
		 0x74, 0x03, 0xC1, 0x0B, 0x81, 0x70, 0x20, 0xF4, 0x3C, 0x04, 0x0F, 0x87,
		 0x40, 0x81, 0x71, 0xE8, 0x10, 0x0F, 0x74, 0x02, 0x01, 0xFD, 0x02, 0x1B,
		 0xE0, 0x10, 0x90, 0x8C, 0x02, 0x68, 0x04, 0x25, 0x42, 0x3E, 0x30, 0x08,
		 0xBF, 0x42, 0x01, 0x3F, 0x07, 0x10, 0xFF, 0xFF, 0xFC, 0x95, 0x59, 0xEC,
		 0x02, 0x0F, 0x80, 0x80, 0x7C, 0x20, 0x13, 0xC0, 0x62, 0x80, 0xF8, 0x01,
		 0x8A, 0x17, 0x40, 0x06, 0x48, 0x7D, 0x16, 0x8A, 0x3A, 0x3F, 0x00, 0x80,
		 0x43, 0xA0, 0x17, 0x88, 0x07, 0xC2, 0x03, 0xD0, 0x81, 0x70, 0x00, 0x5E,
		 0x07, 0xC0, 0x87, 0xC0, 0x2F, 0xB0, 0x09, 0x7C, 0x02, 0x07, 0x80, 0x13,
		 0x91, 0xF1, 0x01, 0x12, 0x51, 0x80, 0x4F, 0x60, 0x10, 0xB0, 0xEE, 0x80,
		 0x40, 0xF0, 0x40, 0x27, 0xB0, 0x08, 0x7C, 0x02, 0x0F, 0x40, 0x81, 0xE0,
		 0x00, 0x63, 0x81, 0x01, 0x14, 0x1B, 0xE4, 0x03, 0xF1, 0x02, 0xE0, 0x09,
		 0xC8, 0xF8, 0xC2, 0xC9, 0x48, 0x50, 0x46, 0x04, 0x85, 0xFD, 0x20, 0x1A,
		 0x2F, 0x3F, 0x82, 0x1D, 0x34, 0x85, 0xE4, 0xBA, 0xF0, 0x08, 0xF8, 0x58,
		 0xC0, 0x22, 0x30, 0x00, 0xFF, 0xFF, 0xF7, 0x10, 0x0D, 0x74, 0x02, 0xD7,
		 0x20, 0x93, 0x72, 0x22, 0xBB, 0x1A, 0x4C, 0xB2, 0x4D, 0x0B, 0x83, 0xD0,
		 0xB8, 0x1C, 0x1D, 0x85, 0xD0, 0x20, 0xA2, 0x99, 0x21, 0x11, 0xD9, 0x40,
		 0xAD, 0xD0, 0x0B, 0x3E, 0x02, 0x14, 0x46, 0x38, 0x00,
];
const Roboto_Light12pt7b_glyphs :  [PFXglyph;96] = [
			{ PFXglyph{offset : 0 , 	width : 0 , 	height : 0 , 	x_advance : 0 , 	x_offset : 0 , 	y_offset : 0 } },   // 0x20 ' ' 
			{ PFXglyph{offset : 0 , 	width : 3 , 	height : 17 , 	x_advance : 5 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x21 '!' 
			{ PFXglyph{offset : 14 , 	width : 5 , 	height : 5 , 	x_advance : 7 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x22 '"' 
			{ PFXglyph{offset : 22 , 	width : 14 , 	height : 17 , 	x_advance : 14 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x23 '#' 
			{ PFXglyph{offset : 85 , 	width : 11 , 	height : 22 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -19 } },   // 0x24 '$' 
			{ PFXglyph{offset : 143 , 	width : 16 , 	height : 17 , 	x_advance : 18 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x25 '%' 
			{ PFXglyph{offset : 209 , 	width : 14 , 	height : 17 , 	x_advance : 15 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x26 '&' 
			{ PFXglyph{offset : 270 , 	width : 2 , 	height : 5 , 	x_advance : 4 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x27 ''' 
			{ PFXglyph{offset : 274 , 	width : 7 , 	height : 25 , 	x_advance : 8 , 	x_offset : 1 , 	y_offset : -18 } },   // 0x28 '(' 
			{ PFXglyph{offset : 316 , 	width : 7 , 	height : 25 , 	x_advance : 8 , 	x_offset : 0 , 	y_offset : -18 } },   // 0x29 ')' 
			{ PFXglyph{offset : 357 , 	width : 10 , 	height : 10 , 	x_advance : 10 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x2a '*' 
			{ PFXglyph{offset : 385 , 	width : 13 , 	height : 13 , 	x_advance : 14 , 	x_offset : 0 , 	y_offset : -14 } },   // 0x2b '+' 
			{ PFXglyph{offset : 412 , 	width : 4 , 	height : 7 , 	x_advance : 5 , 	x_offset : 0 , 	y_offset : -2 } },   // 0x2c ',' 
			{ PFXglyph{offset : 420 , 	width : 7 , 	height : 2 , 	x_advance : 7 , 	x_offset : 0 , 	y_offset : -7 } },   // 0x2d '-' 
			{ PFXglyph{offset : 425 , 	width : 3 , 	height : 2 , 	x_advance : 6 , 	x_offset : 1 , 	y_offset : -1 } },   // 0x2e '.' 
			{ PFXglyph{offset : 428 , 	width : 9 , 	height : 18 , 	x_advance : 10 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x2f '/' 
			{ PFXglyph{offset : 468 , 	width : 11 , 	height : 17 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x30 '0' 
			{ PFXglyph{offset : 512 , 	width : 7 , 	height : 17 , 	x_advance : 13 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x31 '1' 
			{ PFXglyph{offset : 530 , 	width : 12 , 	height : 17 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x32 '2' 
			{ PFXglyph{offset : 583 , 	width : 11 , 	height : 17 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x33 '3' 
			{ PFXglyph{offset : 634 , 	width : 13 , 	height : 17 , 	x_advance : 13 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x34 '4' 
			{ PFXglyph{offset : 691 , 	width : 12 , 	height : 17 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x35 '5' 
			{ PFXglyph{offset : 737 , 	width : 12 , 	height : 17 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x36 '6' 
			{ PFXglyph{offset : 789 , 	width : 13 , 	height : 17 , 	x_advance : 13 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x37 '7' 
			{ PFXglyph{offset : 839 , 	width : 12 , 	height : 17 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x38 '8' 
			{ PFXglyph{offset : 885 , 	width : 11 , 	height : 17 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x39 '9' 
			{ PFXglyph{offset : 937 , 	width : 3 , 	height : 13 , 	x_advance : 5 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x3a ':' 
			{ PFXglyph{offset : 945 , 	width : 4 , 	height : 17 , 	x_advance : 5 , 	x_offset : 0 , 	y_offset : -12 } },   // 0x3b ';' 
			{ PFXglyph{offset : 957 , 	width : 11 , 	height : 10 , 	x_advance : 12 , 	x_offset : 0 , 	y_offset : -12 } },   // 0x3c '<' 
			{ PFXglyph{offset : 987 , 	width : 11 , 	height : 7 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -10 } },   // 0x3d '=' 
			{ PFXglyph{offset : 1004 , 	width : 11 , 	height : 10 , 	x_advance : 12 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x3e '>' 
			{ PFXglyph{offset : 1033 , 	width : 10 , 	height : 17 , 	x_advance : 11 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x3f '?' 
			{ PFXglyph{offset : 1072 , 	width : 20 , 	height : 22 , 	x_advance : 22 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x40 '@' 
			{ PFXglyph{offset : 1176 , 	width : 15 , 	height : 17 , 	x_advance : 15 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x41 'A' 
			{ PFXglyph{offset : 1240 , 	width : 12 , 	height : 17 , 	x_advance : 15 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x42 'B' 
			{ PFXglyph{offset : 1274 , 	width : 14 , 	height : 17 , 	x_advance : 16 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x43 'C' 
			{ PFXglyph{offset : 1324 , 	width : 13 , 	height : 17 , 	x_advance : 16 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x44 'D' 
			{ PFXglyph{offset : 1374 , 	width : 11 , 	height : 17 , 	x_advance : 14 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x45 'E' 
			{ PFXglyph{offset : 1407 , 	width : 11 , 	height : 17 , 	x_advance : 14 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x46 'F' 
			{ PFXglyph{offset : 1439 , 	width : 14 , 	height : 17 , 	x_advance : 16 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x47 'G' 
			{ PFXglyph{offset : 1494 , 	width : 13 , 	height : 17 , 	x_advance : 17 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x48 'H' 
			{ PFXglyph{offset : 1520 , 	width : 2 , 	height : 17 , 	x_advance : 6 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x49 'I' 
			{ PFXglyph{offset : 1524 , 	width : 12 , 	height : 17 , 	x_advance : 13 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x4a 'J' 
			{ PFXglyph{offset : 1548 , 	width : 13 , 	height : 17 , 	x_advance : 15 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x4b 'K' 
			{ PFXglyph{offset : 1604 , 	width : 11 , 	height : 17 , 	x_advance : 13 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x4c 'L' 
			{ PFXglyph{offset : 1626 , 	width : 17 , 	height : 17 , 	x_advance : 21 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x4d 'M' 
			{ PFXglyph{offset : 1704 , 	width : 13 , 	height : 17 , 	x_advance : 17 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x4e 'N' 
			{ PFXglyph{offset : 1764 , 	width : 14 , 	height : 17 , 	x_advance : 16 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x4f 'O' 
			{ PFXglyph{offset : 1815 , 	width : 12 , 	height : 17 , 	x_advance : 15 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x50 'P' 
			{ PFXglyph{offset : 1846 , 	width : 14 , 	height : 20 , 	x_advance : 16 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x51 'Q' 
			{ PFXglyph{offset : 1904 , 	width : 13 , 	height : 17 , 	x_advance : 15 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x52 'R' 
			{ PFXglyph{offset : 1961 , 	width : 13 , 	height : 17 , 	x_advance : 14 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x53 'S' 
			{ PFXglyph{offset : 2014 , 	width : 14 , 	height : 17 , 	x_advance : 14 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x54 'T' 
			{ PFXglyph{offset : 2034 , 	width : 13 , 	height : 17 , 	x_advance : 16 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x55 'U' 
			{ PFXglyph{offset : 2067 , 	width : 15 , 	height : 17 , 	x_advance : 15 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x56 'V' 
			{ PFXglyph{offset : 2129 , 	width : 21 , 	height : 17 , 	x_advance : 22 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x57 'W' 
			{ PFXglyph{offset : 2226 , 	width : 15 , 	height : 17 , 	x_advance : 15 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x58 'X' 
			{ PFXglyph{offset : 2280 , 	width : 15 , 	height : 17 , 	x_advance : 14 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x59 'Y' 
			{ PFXglyph{offset : 2334 , 	width : 13 , 	height : 17 , 	x_advance : 14 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x5a 'Z' 
			{ PFXglyph{offset : 2382 , 	width : 5 , 	height : 24 , 	x_advance : 6 , 	x_offset : 1 , 	y_offset : -19 } },   // 0x5b '[' 
			{ PFXglyph{offset : 2397 , 	width : 10 , 	height : 18 , 	x_advance : 9 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x5c '\' 
			{ PFXglyph{offset : 2414 , 	width : 4 , 	height : 24 , 	x_advance : 6 , 	x_offset : 0 , 	y_offset : -19 } },   // 0x5d ']' 
			{ PFXglyph{offset : 2423 , 	width : 9 , 	height : 8 , 	x_advance : 10 , 	x_offset : 0 , 	y_offset : -16 } },   // 0x5e '^' 
			{ PFXglyph{offset : 2444 , 	width : 11 , 	height : 2 , 	x_advance : 10 , 	x_offset : 0 , 	y_offset : 1 } },   // 0x5f '_' 
			{ PFXglyph{offset : 2451 , 	width : 5 , 	height : 3 , 	x_advance : 7 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x60 '`' 
			{ PFXglyph{offset : 2456 , 	width : 11 , 	height : 13 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x61 'a' 
			{ PFXglyph{offset : 2496 , 	width : 12 , 	height : 18 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x62 'b' 
			{ PFXglyph{offset : 2532 , 	width : 11 , 	height : 13 , 	x_advance : 12 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x63 'c' 
			{ PFXglyph{offset : 2571 , 	width : 11 , 	height : 18 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x64 'd' 
			{ PFXglyph{offset : 2620 , 	width : 11 , 	height : 13 , 	x_advance : 12 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x65 'e' 
			{ PFXglyph{offset : 2660 , 	width : 9 , 	height : 18 , 	x_advance : 8 , 	x_offset : 0 , 	y_offset : -17 } },   // 0x66 'f' 
			{ PFXglyph{offset : 2688 , 	width : 11 , 	height : 18 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x67 'g' 
			{ PFXglyph{offset : 2738 , 	width : 11 , 	height : 18 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x68 'h' 
			{ PFXglyph{offset : 2778 , 	width : 3 , 	height : 18 , 	x_advance : 5 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x69 'i' 
			{ PFXglyph{offset : 2789 , 	width : 6 , 	height : 23 , 	x_advance : 5 , 	x_offset : -2 , 	y_offset : -17 } },   // 0x6a 'j' 
			{ PFXglyph{offset : 2807 , 	width : 11 , 	height : 18 , 	x_advance : 12 , 	x_offset : 1 , 	y_offset : -17 } },   // 0x6b 'k' 
			{ PFXglyph{offset : 2857 , 	width : 2 , 	height : 18 , 	x_advance : 5 , 	x_offset : 2 , 	y_offset : -17 } },   // 0x6c 'l' 
			{ PFXglyph{offset : 2860 , 	width : 19 , 	height : 13 , 	x_advance : 21 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x6d 'm' 
			{ PFXglyph{offset : 2905 , 	width : 11 , 	height : 13 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x6e 'n' 
			{ PFXglyph{offset : 2933 , 	width : 12 , 	height : 13 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x6f 'o' 
			{ PFXglyph{offset : 2967 , 	width : 12 , 	height : 18 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x70 'p' 
			{ PFXglyph{offset : 3003 , 	width : 11 , 	height : 18 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x71 'q' 
			{ PFXglyph{offset : 3049 , 	width : 7 , 	height : 13 , 	x_advance : 8 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x72 'r' 
			{ PFXglyph{offset : 3067 , 	width : 10 , 	height : 13 , 	x_advance : 12 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x73 's' 
			{ PFXglyph{offset : 3103 , 	width : 7 , 	height : 16 , 	x_advance : 8 , 	x_offset : 0 , 	y_offset : -15 } },   // 0x74 't' 
			{ PFXglyph{offset : 3126 , 	width : 11 , 	height : 13 , 	x_advance : 13 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x75 'u' 
			{ PFXglyph{offset : 3154 , 	width : 12 , 	height : 13 , 	x_advance : 12 , 	x_offset : 0 , 	y_offset : -12 } },   // 0x76 'v' 
			{ PFXglyph{offset : 3195 , 	width : 18 , 	height : 13 , 	x_advance : 18 , 	x_offset : 0 , 	y_offset : -12 } },   // 0x77 'w' 
			{ PFXglyph{offset : 3257 , 	width : 12 , 	height : 13 , 	x_advance : 12 , 	x_offset : 0 , 	y_offset : -12 } },   // 0x78 'x' 
			{ PFXglyph{offset : 3298 , 	width : 12 , 	height : 18 , 	x_advance : 11 , 	x_offset : 0 , 	y_offset : -12 } },   // 0x79 'y' 
			{ PFXglyph{offset : 3354 , 	width : 10 , 	height : 13 , 	x_advance : 12 , 	x_offset : 1 , 	y_offset : -12 } },   // 0x7a 'z' 
			{ PFXglyph{offset : 3383 , 	width : 8 , 	height : 24 , 	x_advance : 8 , 	x_offset : 0 , 	y_offset : -18 } },   // 0x7b '{' 
			{ PFXglyph{offset : 3418 , 	width : 2 , 	height : 20 , 	x_advance : 5 , 	x_offset : 2 , 	y_offset : -16 } },   // 0x7c '|' 
			{ PFXglyph{offset : 3421 , 	width : 8 , 	height : 24 , 	x_advance : 8 , 	x_offset : 0 , 	y_offset : -18 } },   // 0x7d '}' 
			{ PFXglyph{offset : 3452 , 	width : 14 , 	height : 5 , 	x_advance : 16 , 	x_offset : 1 , 	y_offset : -8 } },   // 0x7e '~' 
			{ PFXglyph{offset : 3472 , 	width : 9 , 	height : 17 , 	x_advance : 11 , 	x_offset : 1 , 	y_offset : -16 } },   // 0x7f '' 
];
pub const Roboto_Light12pt7b :  PFXfont = PFXfont {
	bitmap : &Roboto_Light12pt7b_bitmap,
	glyphs : &Roboto_Light12pt7b_glyphs,
	first    :  0x20 ,
	last     :  0x7f ,
	y_advance :  0 ,
	bpp      :  2 ,
	shrinked :  1 ,
	hs_conf  :  0x74 ,
};
// Bitmap uncompressed  : about 4041 bytes (4 kBytes)
// Bitmap output size   : about 3513 bytes (4 kBytes)
// Bitmap compression   : 86%
// Header : about 768 bytes (1 kBytes)
//--------------------------------------
// Total : about 4297 bytes (5 kBytes)
