/**
 *  Demo code for ILI9341 320x240 screen with touch capability
 * 
 
 * 
 */
#include "lnArduino.h"
#include "simpler_INA219.h"
#include "simplerMCP4725.h"
#include "lnHardwareConfiguration.h"
#include "lnI2CTask.h"

static myMCP4725           *currentLimiter;
static lnI2C               *i2c;
static simpler_INA219      *ina;

#define INA_MA_FLOOR 24

/**
 * 
 * 
 */

class i2cTask: public lnI2cTask, public xTask 
{
public:
                            i2cTask();
                    virtual ~i2cTask();
                    void    run();
            virtual float   getVoltage()                {return _voltage;}
            virtual int     getCurrent()                {return _currentMa;}
            virtual void    setMaxCurrent(int mA)       {_maxCurrentMa=mA;}
            virtual bool    getCCLimited()              {return _currentLimited;}


protected:
                bool    _currentLimited;
                int     _currentMa;
                int     _maxCurrentMa;
                float   _voltage;
                uint16_t output[2];
};

i2cTask *t;

/**
 * 
 */
lnI2cTask * createI2cTask()
{
    t=new i2cTask();
    return t;
}
/**
 * 
 */
i2cTask::~i2cTask()
{

}
/**
 * 
 */
i2cTask::i2cTask() : xTask("I2C",I2C_TASK_PRIORITY,256)
{
    Logger("Setuping up I2C Task...\n");
    
    _currentLimited=false;
    _currentMa=0;
    _maxCurrentMa=0;
    _voltage=0;


     // arbitrer must be created with screen already set up
    // ili must be first
    //
    i2c=new lnI2C(PS_I2C_INSTANCE,100*1000); // I2C2
    i2c->begin();

  //  i2cScanner();


    // 105->1050
    // 107->1024

    currentLimiter = new myMCP4725(*i2c,MCP4725_DEFAULT_I2C_ADDRESS);

    // 1024 -> 1259 mA
    // 512-> 619 mA
    // 256 -> 286
    // 128 -> 133
    // 64 ->62
    // 32 -> 25
    // 16 -> 12
    // Limit = 1.25*value-25
    currentLimiter->setVoltage(200);
    ina=new simpler_INA219(i2c,3,INA219_ADDRESS,INA_SHUNT_VALUE); // 17 mOhm shunt
    ina->setZero(21); // 21 mA offset, accuracy is ~ 25 mA
    
    float lastRaw=0,lastVoltage=0,lastVbat=0;
    int lastMaxAmp;

    // enable DCD/C
    // BIT 0 = DC/DC
    // BIT 1 = Enable

    uint8_t data=0xff & ~(PCF8574_DCDC_ENABLE + PCF8574_RELAY_ENABLE);
    i2c->write(PCF8574_ADDRESS,1,&data);
    _currentLimited=false;
    start();
}



/**
 * 
 */
void i2cTask::run()
{
    float f,lastVbat;
    int lastRaw,lastMaxAmp,lastVoltage;
    while(1)
    {
       
       xDelay(50);

        int ma=ina->getCurrent_mA();
        _currentMa=ma;

        uint8_t ovr=0xff;
        i2c->read(PCF8574_ADDRESS,1,&ovr);
        _currentLimited=!(ovr & PCF8574_CC_MODE);

        float  f=ina->getVoltage_V();
        if(f<0.) f=0.;
        _voltage=f;

        currentLimiter->setVoltage(_maxCurrentMa);
       
    }

}



/**
 * 
 */
void i2cScanner()
{
 while(1)
    {
        for(int i=020;i<40;i++)
        {
            xDelay(100);
            Logger("Scanning %d\n",i);
            
            i2c->setAddress(i);
        //   while(1)
            {
                if(!i2c->write(0,NULL))
                {
                    xDelay(5);
                    continue;
                }else
                {
                    Logger(">>>found something at address %d (0x%0x)\n",i,i);
                }
            }
            
        }
    }
}

//
