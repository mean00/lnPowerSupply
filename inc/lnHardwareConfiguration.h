
#pragma once

#define PS_I2C_INSTANCE     1

// INA
#define INA_SHUNT_VALUE     106 // 106 mOhm

// screen 
#define ILI_SPI_INSTANCE    0
#define ILI_PIN_DC          PA4
#define ILI_PIN_CS          PB1
#define ILI_PIN_RESET       PB0

#define PIN_LED             PA3
#define PIN_SWITCH          PB9

#define PS_PIN_VBAT         PA0
#define PS_PIN_MAX_CURRENT  PA1
#define PS_INTERNAL_RESISTANCE_MILLIOHM 300 // error on voltage is  ~ 50 mV

// IO Expander
#define IO_EXPANDER_ADDRESS         32
#define IO_EXPANDER_DC_ENABLE       0
#define IO_EXPANDER_RELAY_ENABLE    1
#define IO_EXPANDER_CC_MODE         2

// PCF8574
#define PCF8574_ADDRESS         32
#define PCF8574_DCDC_ENABLE     1 // out
#define PCF8574_RELAY_ENABLE    2 // out
#define PCF8574_CC_MODE         4 // in

// Tasks 
#define I2C_TASK_PRIORITY 2
#define I2C_STACK_SIZE    1024


// VBAT limit
#define PS_MIN_VBAT         15. // at start
#define PS_MIN_VBAT_CRIT    14. // anytime

//
#define WIRE_RESISTANCE_MOHM  220 // wire resistance