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

enum otherEvent
{
    EnableButtonEvent=128,
};

//----------

extern lnI2cTask *createI2cTask(lnI2cTask::signalCb *c);
void stopLowVoltage();
void onOffCallback(lnPin pin, void *cookie);

xFastEventGroup *eventGroup;

//----------

lnTimingAdc         *adc;
lnI2cTask           *tsk;

bool                outputEnabled=false;

const lnPin pins[2]={PS_PIN_VBAT, PS_PIN_MAX_CURRENT};

/**
 * 
 */
void setup()
{
    Logger("Setuping up Power Supply...\n");
   
    lnPinMode(PS_PIN_VBAT,lnADC_MODE);
    lnPinMode(PS_PIN_MAX_CURRENT,lnADC_MODE);

    lnPinMode(LN_SYSTEM_LED,lnOUTPUT);
    lnPinMode(PIN_LED,lnOUTPUT);
    lnPinMode(PIN_SWITCH,lnINPUT_PULLUP);

    lnExtiAttachInterrupt(PIN_SWITCH, LN_EDGE_FALLING, onOffCallback, NULL);
    lnExtiEnableInterrupt(PIN_SWITCH);
  
    lnDisplay::init();
    adc=new lnTimingAdc(0);
    adc->setSource(3,3,1000,2,pins);

    eventGroup = new xFastEventGroup;
}

/**
 * 
 */
void onOffCallback(lnPin pin, void *cookie)
{
    outputEnabled^=1;    
    lnExtiDisableInterrupt(PIN_SWITCH);
    eventGroup->setEvents(EnableButtonEvent);
}
/**
 * \fn runAdc
 * \brief Average ADC reading over ADC_SAMPLE samples
 * Convert ADC reading to vbat and maxCurrent (in mA)
 */
void runAdc(float &fvbat, int &maxCurrentSlopped)
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
    int vbat, maxCurrent;
    vbat = (max0 + ((ADC_SAMPLE-1)/2))/ADC_SAMPLE;
    maxCurrent = (max1 + ((ADC_SAMPLE-1)/2))/ADC_SAMPLE;

    fvbat=(float)vbat;
    fvbat=fvbat*9.;
    fvbat/=1000.;
    
    maxCurrent=maxCurrent*maxCurrent;
    // 0..4095 -> 0.. 4A amp=val*1.5+50
    // so max=4000/1.5=2660
    maxCurrent/=6300; // 0..4000
    maxCurrent-=(maxCurrent&7);
    maxCurrent+=50;
    maxCurrentSlopped=maxCurrent;
}
/**
 * 
 */
void i2cCb(uint32_t signal)
{
    eventGroup->setEvents(signal);
}
/**
 * 
 */
void loop()
{
    eventGroup->takeOwnership();
    tsk=createI2cTask(i2cCb);
    xDelay(20); // let it start    
    float vbat;
    int imaxAmp, lastMaxCurrent=-1;;
    
    runAdc(vbat, imaxAmp);
    if(vbat<PS_MIN_VBAT)
    {
        stopLowVoltage();
    }    

    tsk->setDCEnable(true);
    tsk->setOutputEnable(false); 
    lnDigitalWrite(PIN_LED,true);

   while(1)
   {
        int event=eventGroup->waitEvents(0xfff,100);
        lnDigitalToggle(LN_SYSTEM_LED);            

        int   current=tsk->getCurrent();
        bool  cc=tsk->getCCLimited();


        if(event & EnableButtonEvent)
        {
            lnDigitalWrite(PIN_LED,!outputEnabled);
            tsk->setOutputEnable(outputEnabled);
            xDelay(20);
            lnExtiEnableInterrupt(PIN_SWITCH);
        }

        float voltage=tsk->getVoltage();
        if(event & (lnI2cTask::VoltageChangeEvent | lnI2cTask::CCChangeEvent | lnI2cTask::CurrentChangeEvent))
        {
            float correction=WIRE_RESISTANCE_MOHM;
            correction=correction*current;
            correction/=1000000.;
            voltage-=correction;
            lnDisplay::displayVoltage( cc,  voltage);
            float power=voltage*(float)current;
            power/=1000.;
            lnDisplay::displayPower( cc,  power);
        }
        if(event & (lnI2cTask::CurrentChangeEvent))
        {
            lnDisplay::displayCurrent(current);
        }

       
        runAdc(vbat, imaxAmp);
        lnDisplay::displayVbat( vbat);

        if(vbat<PS_MIN_VBAT_CRIT)
        {
            stopLowVoltage();
        }

        int delta=lastMaxCurrent- imaxAmp;
        if(delta<0) delta=-delta;
        if(delta>10)
        {
            lastMaxCurrent=imaxAmp;
            float d=imaxAmp;
            d/=1.5;
            d-=25;
            if(d<0) d=0.;
            
            tsk->setMaxCurrent(d);
            lnDisplay::displayMaxCurrent(imaxAmp);
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
