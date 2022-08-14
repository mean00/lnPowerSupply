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
#include "robotoslab.h"


extern lnI2cTask *createI2cTask();

lnSpi9341           *ili;
hwlnSPIClass        *spi;
lnTimingAdc         *adc;


#define WW  280
#define HH  240
#define INA_MA_FLOOR 24

extern const uint8_t dso_resetOff[];
extern const uint8_t dso_wakeOn[];
void i2cScanner();

//#define FONT Waree36pt7b
#define FONT RobotoSlab_SemiBold48pt7b
#define SMALLFONT Waree12pt7b


#define ENABLE_CC

#define MAIN_COLUMN  76
#define LIMIT_COLUMN 318
#define V_LINE      88
#define A_LINE      176
#define MAX_C_LINE  236

#define ADC_SAMPLE 8


bool outputEnabled=false;
bool newOutputEnabled=false;
lnI2cTask *tsk;

float lastVoltage=-1;
int   lastCurrent=-1;
int   lastMaxCurrent=-1;
int   lastCC=-1;
bool  relayEnable=false;


/**
 * 
 */
lnPin pins[2]={PS_PIN_VBAT, PS_PIN_MAX_CURRENT};
void stopLowVoltage();
void onOffCallback(lnPin pin, void *cookie);
void setup()
{
    Logger("Setuping up Power Supply...\n");
    lnPinMode(LN_SYSTEM_LED,lnOUTPUT);
    lnPinMode(PS_PIN_VBAT,lnADC_MODE);
    lnPinMode(PS_PIN_MAX_CURRENT,lnADC_MODE);
    adc=new lnTimingAdc(0);
    adc->setSource(3,3,1000,2,pins);

    lnPinMode(PIN_LED,lnOUTPUT);
    lnPinMode(PIN_SWITCH,lnINPUT_PULLUP);
    lnExtiAttachInterrupt(PIN_SWITCH, LN_EDGE_FALLING, onOffCallback, NULL);
    lnExtiEnableInterrupt(PIN_SWITCH);
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

/**
 * 
 */
void onOffCallback(lnPin pin, void *cookie)
{
    newOutputEnabled^=1;
    lnExtiDisableInterrupt(PIN_SWITCH);
}
/**
 * 
 */
void runAdc(int &vbat, int &maxCurrent)
{
    static uint16_t output[ADC_SAMPLE*2];
    adc->multiRead(ADC_SAMPLE,output); 
    uint16_t *p=output;
    int max0=0,max1=0;
    for(int i=0;i<ADC_SAMPLE;i++)
    {
            max0+=*p++;
            max1+=*p++;
    }
    vbat = (max0 + ((ADC_SAMPLE-1)/2))/ADC_SAMPLE;
    maxCurrent = (max1 + ((ADC_SAMPLE-1)/2))/ADC_SAMPLE;
}
/**
 * 
 */
void loop()
{

    tsk=createI2cTask();
    xDelay(20); // let it start
    char buffer[64];
    
    
    ili->fillScreen(BLACK);
    ili->setFontSize(ili9341::BigFont);            
    ili->setCursor(0,V_LINE);
    ili->printUpTo("V",MAIN_COLUMN);
    ili->setCursor(0,A_LINE);
    ili->printUpTo("A",MAIN_COLUMN);

    {
        int ivbat, imaxAmp;
        runAdc(ivbat, imaxAmp);
        float vbat=(float)ivbat;
        vbat=vbat*9;
        vbat/=1000.;      
        if(vbat<PS_MIN_VBAT)
        {
            stopLowVoltage();
        }
    }

    tsk->setDCEnable(true);
    tsk->setOutputEnable(false); 
    lnDigitalWrite(PIN_LED,true);



   while(1)
   {
        xDelay(10);
        lnDigitalToggle(LN_SYSTEM_LED);            

        float voltage=tsk->getVoltage();
        int   current=tsk->getCurrent();
        bool  cc=tsk->getCCLimited();


        if(outputEnabled!=newOutputEnabled)
        {
            outputEnabled=newOutputEnabled;
            lnDigitalWrite(PIN_LED,!outputEnabled);
            tsk->setOutputEnable(outputEnabled);
            xDelay(20);
            lnExtiEnableInterrupt(PIN_SWITCH);
        }

        float correction=WIRE_RESISTANCE_MOHM;
        correction=correction*current;
        correction/=1000000.;
        voltage-=correction;

        if(voltage!=lastVoltage)
        {
            lastVoltage=voltage;
            ili->setFontSize(ili9341::BigFont);        
            sprintf(buffer,"%2.2f",voltage);
            ili->setCursor(MAIN_COLUMN,V_LINE);
            if(cc)
                ili->setTextColor(RED,BLACK);
            ili->printUpTo(buffer,LIMIT_COLUMN);
            ili->setTextColor(WHITE,BLACK);
        }
        if(lastCurrent!=current)
        {
            lastCurrent=current;
            ili->setFontSize(ili9341::BigFont);
            sprintf(buffer,"%d",current);
            ili->setCursor(MAIN_COLUMN,A_LINE);
            ili->printUpTo(buffer,LIMIT_COLUMN);
        }
        
        ili->setFontSize(ili9341::SmallFont);  


        int ivbat, imaxAmp;
        runAdc(ivbat, imaxAmp);


        float vbat=(float)ivbat;
        vbat=vbat*9;
        vbat/=1000.;      
        sprintf(buffer,"vbat:%2.1f",vbat);
        ili->setCursor(36,235);
        ili->printUpTo(buffer,140);
        if(vbat<PS_MIN_VBAT_CRIT)
        {
            stopLowVoltage();
        }



        int maxAmp=imaxAmp;
        maxAmp=maxAmp*maxAmp;
        // 0..4095 -> 0.. 4A amp=val*1.5+50
        // so max=4000/1.5=2660
        maxAmp/=6300; // 0..4000
        maxAmp-=(maxAmp&7);
        maxAmp+=50;

        if(lastMaxCurrent != maxAmp)
        {
            lastMaxCurrent=maxAmp;
            float d=maxAmp;
            d/=1.5;
            d-=25;
            if(d<0) d=0.;
            
            tsk->setMaxCurrent(d);

            ili->setFontSize(ili9341::SmallFont);
            sprintf(buffer,"maxC:%d",maxAmp);
            ili->setCursor(180,MAX_C_LINE);
            ili->printUpTo(buffer,260);
        }
    }
}
/**
 * 
 */
void stopLowVoltage()
{
    tsk->setOutputEnable(false);
    tsk->setDCEnable(false);
    ili->setFontSize(ili9341::SmallFont);  
    ili->setCursor(180,MAX_C_LINE);

    ili->print("LOW VOLTAGE");
    while(1)
    {
        lnDigitalToggle(LN_SYSTEM_LED);    
        lnDigitalToggle(PIN_LED);
        xDelay(20);
    }
}

//
