

#pragma once

class lnI2cTask
{
public:
                            lnI2cTask() {}
            virtual         ~lnI2cTask() {} 
            virtual float   getVoltage()=0;
            virtual int     getCurrent()=0;
            virtual void    setMaxCurrent(int mA)=0;
            virtual bool    getCCLimited()=0;
            virtual void    setDCEnable(bool enable) =0;
            virtual void    setOutputEnable(bool enable)=0;

            


};