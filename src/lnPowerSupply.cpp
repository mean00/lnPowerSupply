/**
 *  Massively over engineered power supply
 * 
 
 * 
 */
#include "lnArduino.h"
#include "lnADC.h"
#include "lnHardwareConfiguration.h"
#include "lnI2CTask.h"
#include "lnDisplay.h"

#define INA_MA_FLOOR 24
#define ADC_SAMPLE 8

extern lnI2cTask *createI2cTask();
void stopLowVoltage();
void onOffCallback(lnPin pin, void *cookie);

lnTimingAdc         *adc;
bool outputEnabled=false;
bool newOutputEnabled=false;
lnI2cTask *tsk;
float lastVoltage=-1;
int   lastCurrent=-1;
int   lastMaxCurrent=-1;
int   lastCC=-1;
bool  relayEnable=false;
lnPin pins[2]={PS_PIN_VBAT, PS_PIN_MAX_CURRENT};

/**
 * 
 */
void setup()
{
    Logger("Setuping up Power Supply...\n");
    lnPinMode(LN_SYSTEM_LED,lnOUTPUT);
    lnPinMode(PS_PIN_VBAT,lnADC_MODE);
    lnPinMode(PS_PIN_MAX_CURRENT,lnADC_MODE);
    lnPinMode(PIN_LED,lnOUTPUT);
    lnPinMode(PIN_SWITCH,lnINPUT_PULLUP);
    lnExtiAttachInterrupt(PIN_SWITCH, LN_EDGE_FALLING, onOffCallback, NULL);
    lnExtiEnableInterrupt(PIN_SWITCH);
  
    lnDisplay::init();
    adc=new lnTimingAdc(0);
    adc->setSource(3,3,1000,2,pins);
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
            lnDisplay::displayVoltage( cc,  voltage);
        }
        if(lastCurrent!=current)
        {
            lastCurrent=current;
            lnDisplay::displayCurrent(current);
        }

        int ivbat, imaxAmp;
        runAdc(ivbat, imaxAmp);


        float vbat=(float)ivbat;
        vbat=vbat*9;
        vbat/=1000.;
        lnDisplay::displayVbat( vbat);
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
            lnDisplay::displayMaxCurrent(maxAmp);
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
    lnDisplay::banner("LOW BATTERY");
    while(1)
    {
        lnDigitalToggle(LN_SYSTEM_LED);    
        lnDigitalToggle(PIN_LED);
        xDelay(20);
    }
}

// EOF
