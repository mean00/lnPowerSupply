#![allow(unused_parens)]
/**************************************************************************/
/*! 
    @file     Adafruit_INA219.h
    @author   K. Townsend : u16 = (Adafruit Industries)
    @author   modified by mean00@gmail.com
	@license  BSD : u16 = (see license.txt)
	
	This is a library for the Adafruit INA219 breakout board
	----> https://www.adafruit.com/products/???
	
	Adafruit invests time and resources providing this open source code, 
	please support Adafruit and open-source hardware by purchasing 
	products from Adafruit!

	@section  HISTORY

    v1.0  - First release
*/
/**************************************************************************/


    pub const  INA219_READ                            : u16 = (0x01);
/*=========================================================================*/

/*=========================================================================
    CONFIG REGISTER : u16 = (R/W)
    -----------------------------------------------------------------------*/
    pub const  INA219_REG_CONFIG                      : u8 = (0x00);
    /*---------------------------------------------------------------------*/
    pub const  INA219_CONFIG_RESET                    : u16 = (1<<15);  // Reset Bit
	
    pub const  INA219_CONFIG_BVOLTAGERANGE_MASK       : u16 = (1<<13);  // Bus Voltage Range Mask
    pub const  INA219_CONFIG_BVOLTAGERANGE_16V        : u16 = (0<<13);  // 0-16V Range
    pub const  INA219_CONFIG_BVOLTAGERANGE_32V        : u16 = (1<<13);  // 0-32V Range
	
    pub const  INA219_CONFIG_GAIN_MASK                : u16 = (3<<11);  // Gain Mask
    pub const  INA219_CONFIG_GAIN_1_40MV              : u16 = (0<<11);  // Gain 1, 40mV Range
    pub const  INA219_CONFIG_GAIN_2_80MV              : u16 = (1<<11);  // Gain 2, 80mV Range
    pub const  INA219_CONFIG_GAIN_4_160MV             : u16 = (2<<11);  // Gain 4, 160mV Range
    pub const  INA219_CONFIG_GAIN_8_320MV             : u16 = (3<<11);  // Gain 8, 320mV Range
	
    pub const  INA219_CONFIG_BADCRES_MASK             : u16 = (0x0780);  // Bus ADC Resolution Mask
    pub const  INA219_CONFIG_BADCRES_12BIT            : u16 = (0x0400);  // 12-bit bus res = 0..4097
    
    pub const  INA219_CONFIG_SADCRES_MASK             : u16 = (0x0078);  // Shunt ADC Resolution and Averaging Mask
    pub const  INA219_CONFIG_SADCRES_9BIT_1S_84US     : u16 = (0x0000);  // 1 x 9-bit shunt sample
    pub const  INA219_CONFIG_SADCRES_10BIT_1S_148US   : u16 = (0x0008);  // 1 x 10-bit shunt sample
    pub const  INA219_CONFIG_SADCRES_11BIT_1S_276US   : u16 = (0x0010);  // 1 x 11-bit shunt sample
    pub const  INA219_CONFIG_SADCRES_12BIT_1S_532US   : u16 = (0x0018);  // 1 x 12-bit shunt sample
    pub const  INA219_CONFIG_SADCRES_12BIT_2S_1060US  : u16 = (0x0048);	 // 2 x 12-bit shunt samples averaged together
    pub const  INA219_CONFIG_SADCRES_12BIT_4S_2130US  : u16 = (0x0050);  // 4 x 12-bit shunt samples averaged together
    pub const  INA219_CONFIG_SADCRES_12BIT_8S_4260US  : u16 = (0x0058);  // 8 x 12-bit shunt samples averaged together
    pub const  INA219_CONFIG_SADCRES_12BIT_16S_8510US : u16 = (0x0060);  // 16 x 12-bit shunt samples averaged together
    pub const  INA219_CONFIG_SADCRES_12BIT_32S_17MS   : u16 = (0x0068);  // 32 x 12-bit shunt samples averaged together
    pub const  INA219_CONFIG_SADCRES_12BIT_64S_34MS   : u16 = (0x0070);  // 64 x 12-bit shunt samples averaged together
    pub const  INA219_CONFIG_SADCRES_12BIT_128S_69MS  : u16 = (0x0078);  // 128 x 12-bit shunt samples averaged together
	
    pub const  INA219_CONFIG_MODE_MASK                : u16 = (7<<0);  // Operating Mode Mask
    pub const  INA219_CONFIG_MODE_POWERDOWN           : u16 = (0<<0);
    pub const  INA219_CONFIG_MODE_SVOLT_TRIGGERED     : u16 = (1<<0);
    pub const  INA219_CONFIG_MODE_BVOLT_TRIGGERED     : u16 = (2<<0);
    pub const  INA219_CONFIG_MODE_SANDBVOLT_TRIGGERED : u16 = (3<<0);
    pub const  INA219_CONFIG_MODE_ADCOFF              : u16 = (4<<0);
    pub const  INA219_CONFIG_MODE_SVOLT_CONTINUOUS    : u16 = (5<<0);
    pub const  INA219_CONFIG_MODE_BVOLT_CONTINUOUS    : u16 = (6<<0);
    pub const  INA219_CONFIG_MODE_SANDBVOLT_CONTINUOUS: u16 = (7<<0);
/*=========================================================================*/
    //pub const  INA_PGA: u16 = (x)                     : u16 = (x<<11);
/*=========================================================================
    SHUNT VOLTAGE REGISTER : u16 = (R)
    -----------------------------------------------------------------------*/
    pub const  INA219_REG_SHUNTVOLTAGE                : u8 = (0x01);
/*=========================================================================*/

/*=========================================================================
    BUS VOLTAGE REGISTER : u16 = (R)
    -----------------------------------------------------------------------*/
    pub const  INA219_REG_BUSVOLTAGE                  : u8 = (0x02);
/*=========================================================================*/

/*=========================================================================
    POWER REGISTER : u16 = (R)
    -----------------------------------------------------------------------*/
    pub const  INA219_REG_POWER                       : u8 = (0x03);
/*=========================================================================*/

/*=========================================================================
    CURRENT REGISTER : u16 = (R)
    -----------------------------------------------------------------------*/
    pub const  INA219_REG_CURRENT                     : u8 = (0x04);
/*=========================================================================*/

/*=========================================================================
    CALIBRATION REGISTER : u16 = (R/W)
    -----------------------------------------------------------------------*/
    pub const  INA219_REG_CALIBRATION                 : u8 = (0x05);
/*=========================================================================*/