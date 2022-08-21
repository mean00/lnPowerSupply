

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
            virtual void    setCb(signalCb *c) {_cb=c;};
            virtual float   getVoltage()=0;
            virtual int     getCurrent()=0;
            virtual void    setMaxCurrent(int mA)=0;
            virtual bool    getCCLimited()=0;
            virtual void    setDCEnable(bool enable)=0;
            virtual void    setOutputEnable(bool enable)=0;
public:
            signalCb        *_cb;

            


};