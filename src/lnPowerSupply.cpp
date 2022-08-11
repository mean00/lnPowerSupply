/**
 *  Massively over engineered power supply
 * 
 
 * 
 */
#include "lnArduino.h"
#include "ili_lnSpi.h"
#include "simpler9341_priv.h"
#include "lnADC.h"
#include "lnHardwareConfiguration.h"
#include "lnI2CTask.h"

#include "waree36.h"
#include "waree12.h"


extern lnI2cTask *createI2cTask();

lnSpi9341           *ili;
hwlnSPIClass        *spi;
lnSimpleADC         *adc;


#define WW  280
#define HH  240
#define INA_MA_FLOOR 24

extern const uint8_t dso_resetOff[];
extern const uint8_t dso_wakeOn[];
void i2cScanner();

#define FONT Waree36pt7b
#define SMALLFONT Waree12pt7b


#define ENABLE_CC


/**
 * 
 */
void setup()
{
    Logger("Setuping up Power Supply...\n");
    lnPinMode(LN_SYSTEM_LED,lnOUTPUT);
    lnPinMode(PS_PIN_VBAT,lnADC_MODE);
    lnPinMode(PA1,lnADC_MODE);
    adc=new lnSimpleADC(0,PS_PIN_VBAT);

    lnPinMode(PIN_LED,lnOUTPUT);
    lnPinMode(PIN_SWITCH,lnINPUT_PULLUP);


  //  i2cScanner();

#define FAST 1
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

   }
int xround=0;
int outputEnabled=false;
lnI2cTask *tsk;

void loop()
{

   tsk=createI2cTask();
   char buffer[64];
   uint16_t output[2];
   int pins[2]={PA0,PA1};
   ili->fillScreen(BLACK);
   while(1)
   {
        lnDigitalToggle(LN_SYSTEM_LED);        
        float voltage=tsk->getVoltage();
        int   current=tsk->getCurrent();
        bool  cc=tsk->getCCLimited();

        
        ili->setFontSize(ili9341::BigFont);
        sprintf(buffer,"%d",current);
        ili->setCursor(36,180);
        ili->printUpTo(buffer,260);
        
        ili->setFontSize(ili9341::BigFont);        
        sprintf(buffer,"%2.2f",voltage);
        ili->setCursor(36,110);
        ili->printUpTo(buffer,160);
        
        ili->setFontSize(ili9341::SmallFont);        
        float vbat=(float)output[0];
        vbat=vbat*9;
        vbat/=1000.;      
        sprintf(buffer,"%2.2f",vbat);
        ili->setCursor(36,235);
        ili->printUpTo(buffer,160);

        int maxAmp=output[1];
        maxAmp=maxAmp*maxAmp;
        maxAmp/=4095;
        tsk->setMaxCurrent(maxAmp);

        ili->setFontSize(ili9341::SmallFont);
        sprintf(buffer,"%d",maxAmp);
        ili->setCursor(36,30);
        ili->printUpTo(buffer,260);


        ili->setFontSize(ili9341::SmallFont);
        ili->setCursor(180,30);
        if(cc)
            ili->printUpTo("--",260);
        else            
            ili->printUpTo("CC",260);


        lnDigitalToggle(PIN_LED);

        if(!(xround & 3))
        {
            Logger("Inp:%d\n",lnDigitalRead(PIN_SWITCH));
        }

        adc->pollingMultiRead(2,pins,output);   

        xDelay(100);

    }
}

//
