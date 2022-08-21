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
                            i2cTask(signalCb *c);
                    virtual ~i2cTask();
                    void    run();
            virtual float   getVoltage()                {return _voltage;}
            virtual int     getCurrent()                {return _currentMa;}
            virtual bool    getCCLimited()              {return _currentLimited;}

            virtual void    setMaxCurrent(int mA)       {_newMaxCurrentMa=mA;}
            virtual void    setDCEnable(bool enable)    {_newDCEnabled=enable;}
            virtual void    setOutputEnable(bool enable)    {_newOutputEnabled=enable;}
                    void    notify(lnI2cTask::SignalChange sign)  { _accumulated|=sign;}


protected:
                
                int     _currentMa;
                int     _maxCurrentMa,_newMaxCurrentMa;
                float   _voltage;
                bool    _currentLimited;

                bool    _newDCEnabled,_DCEnabled;
                bool    _newOutputEnabled, _OutputEnabled;
                int     _accumulated;
};

i2cTask *t;

/**
 * 
 */
lnI2cTask * createI2cTask(lnI2cTask::signalCb *c)
{
    t=new i2cTask(c);
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
i2cTask::i2cTask(signalCb *c) : lnI2cTask(c) , xTask("I2C",I2C_TASK_PRIORITY,256) 
{
    Logger("Setuping up I2C Task...\n");
    
    _currentLimited=false;
    _currentMa=-1;
    _maxCurrentMa=200;    
    _voltage=0;
    _DCEnabled=false;
    _newDCEnabled=false;
    _newOutputEnabled=false;
    _OutputEnabled=false;
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
    

    // enable DCD/C
    // BIT 0 = DC/DC
    // BIT 1 = Enable

    uint8_t data=0xff ; // the enables are active low
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
       
         xDelay(50); //   The whole sequence lasts ~ 5 ms
        _accumulated=0;

        // --- Read current....
        int ma=ina->getCurrent_mA();        
        if(ma!=_currentMa) 
        {
            _currentMa=ma;
            notify(CurrentChangeEvent );
        }
        // --- Read voltage...
        float  f=ina->getVoltage_V();
        if(f<0.) f=0.;
        if(_voltage!=f)
        {
            _voltage=f;
            notify(VoltageChangeEvent);
        }
        // --- read CC
        uint8_t ovr=0xff;
        i2c->read(PCF8574_ADDRESS,1,&ovr);
        bool cc=!!(ovr & PCF8574_CC_MODE);
        if(_currentLimited!=cc)
        {
            _currentLimited=cc;
            notify(CCChangeEvent);
        }
        // write DC enable or relay enable
        int dc=_newDCEnabled;
        int relay=_newOutputEnabled;

        if(_OutputEnabled!=relay || dc!=_DCEnabled)
        {
            _OutputEnabled  = relay;
            _DCEnabled = dc;
             uint8_t ovr=0xff;
            if(_OutputEnabled) 
                    ovr &= ~PCF8574_RELAY_ENABLE;
            if(_DCEnabled) 
                    ovr &= ~PCF8574_DCDC_ENABLE; // active low!
             i2c->write(PCF8574_ADDRESS,1,&ovr);
        }

       
        // update DAC if it has changed
        int update =_newMaxCurrentMa;
        if( _maxCurrentMa != update)
        {
            _maxCurrentMa = update;
            currentLimiter->setVoltage(_maxCurrentMa);
        }
        if(_accumulated)
        {
            xAssert(_cb);
            _cb(_accumulated);
        }
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

#include "i2cTask_shim.h"
extern i2cTask *tsk;
float   lnI2cTaskShim::getVoltage() {return tsk->getVoltage();};
int     lnI2cTaskShim::getCurrent() {return tsk->getCurrent();};
void    lnI2cTaskShim::setMaxCurrent(int mA) {tsk->setMaxCurrent(mA);};
bool    lnI2cTaskShim::getCCLimited() {return tsk->getCCLimited();};
void    lnI2cTaskShim::setDCEnable(bool enable) {tsk->setDCEnable(enable);};
void    lnI2cTaskShim::setOutputEnable(bool enable) {tsk->setOutputEnable(enable);};
void    lnI2cTaskShim::setCb(lnI2cTask::signalCb *c) {return tsk->setCb(c);};
lnI2cTask *shimCreateI2CTask(lnI2cTask::signalCb *c)
{
    t=new i2cTask(c);
    tsk=t;
    return t;
}
//
