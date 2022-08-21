

#pragma once
#include "lnArduino.h"
class lnI2cTask
{
public:
            enum SignalChange
            {
                VoltageChangeEvent=1,
                CurrentChangeEvent=2,
                CCChangeEvent     =4,
            };
public:
            typedef void (signalCb)(uint32_t signal);

                            lnI2cTask(signalCb *c) {_cb=c;}
            virtual         ~lnI2cTask() {} 
            virtual float   getVoltage() {xAssert(0);return 0;};
            virtual int     getCurrent(){xAssert(0);return 0;};
            virtual void    setMaxCurrent(int mA){xAssert(0);return ;};
            virtual bool    getCCLimited(){xAssert(0);return 0;};
            virtual void    setDCEnable(bool enable) {xAssert(0);return ;};
            virtual void    setOutputEnable(bool enable){xAssert(0);return ;};;
public:
            signalCb        *_cb;

            


};