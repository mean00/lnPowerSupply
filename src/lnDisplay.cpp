/**
 *  This file does all the display related stuff
 * 
 */

#include "ili_lnSpi.h"
#include "simpler9341_priv.h"
#include "lnHardwareConfiguration.h"
#include "lnDisplay.h"


#include "waree36.h"
#include "waree12.h"
#include "robotoslab.h"

//#define FONT Waree36pt7b
#define FONT RobotoSlab_SemiBold48pt7b
#define SMALLFONT Waree12pt7b
#define FAST 1

#define MAIN_COLUMN  76
#define LIMIT_COLUMN 318
#define V_LINE       88
#define A_LINE       176
#define MAX_C_LINE   236

extern const uint8_t dso_resetOff[];
extern const uint8_t dso_wakeOn[];

lnSpi9341           *ili;
hwlnSPIClass        *spi;

static char buffer[64];;

/**
 * 
 */
void lnDisplay::init()
{
    spi=new hwlnSPIClass(ILI_SPI_INSTANCE,-1);    
    lnSPISettings transaction(FAST*36*1000*1000+(1-FAST)*10*1000, SPI_MSBFIRST, SPI_MODE0,-1);
    spi->begin();    
    
    ili=new lnSpi9341( 240, 320,
                                    spi,        
                                    ILI_PIN_DC,       // DC/RS
                                    ILI_PIN_CS,       // CS 
                                    ILI_PIN_RESET);   // Reset
    spi->beginTransaction(transaction);    
    ili->init(dso_resetOff,dso_wakeOn);     
    ili->setRotation(1);
    ili->fillScreen(WHITE);   

    ili->setFontFamily(&SMALLFONT,&FONT,&FONT) ;
    ili->setFontSize(ili9341::BigFont);
    ili->setTextColor(WHITE,BLACK);
    ili->fillScreen(BLACK);
    
    ili->setCursor(0,V_LINE);
    ili->printUpTo("V",MAIN_COLUMN);
    ili->setCursor(0,A_LINE);
    ili->printUpTo("A",MAIN_COLUMN);

}
/**
 * 
 * 
 */
void lnDisplay::displayMaxCurrent(int maxAmp)
{
    ili->setFontSize(ili9341::SmallFont);
    sprintf(buffer,"maxC:%d",maxAmp);
    ili->setCursor(180,MAX_C_LINE);
    ili->printUpTo(buffer,260);
}

void lnDisplay::banner(const char *msg)
{
    ili->setFontSize(ili9341::SmallFont);  
    ili->setCursor(180,MAX_C_LINE);
    ili->print(msg);
}
/**
 * 
 */
void lnDisplay::displayVbat(float vbat)
{
    ili->setFontSize(ili9341::SmallFont);  
    sprintf(buffer,"vbat:%2.1f",vbat);
    ili->setCursor(36,235);
    ili->printUpTo(buffer,140);
}
/**
 * 
 */
void lnDisplay::displayCurrent(int ma)
{
    ili->setFontSize(ili9341::BigFont);
    sprintf(buffer,"%d",ma);
    ili->setCursor(MAIN_COLUMN,A_LINE);
    ili->printUpTo(buffer,LIMIT_COLUMN);
}

/**
 * 
 */
void lnDisplay::displayVoltage(bool cc, float voltage)
{
    ili->setFontSize(ili9341::BigFont);        
    sprintf(buffer,"%2.2f",voltage);
    ili->setCursor(MAIN_COLUMN,V_LINE);
    if(cc)
        ili->setTextColor(RED,BLACK);
    ili->printUpTo(buffer,LIMIT_COLUMN);
    ili->setTextColor(WHITE,BLACK);
}

// EOF