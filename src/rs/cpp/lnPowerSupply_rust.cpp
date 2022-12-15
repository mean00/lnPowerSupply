/**
 *  Massively over engineered power supply
 *  This is the startup code that will then give control to the rust part
 
 * 
 */
#include "lnArduino.h"
#include "lnADC.h"
#include "lnHardwareConfiguration.h"
#include "lnI2CTask.h"
#include "lnDisplay.h"

extern "C" void rnLoop(void);
extern "C" void rnInit(void);
extern lnI2cTask *createI2cTask(lnI2cTask::signalCb *c, const void *cookie);
void i2cCb(uint32_t signal, const void *cookie);
//----------

lnI2cTask           *tsk;
//----------

/**
 * 
 */
void setup()
{
    rnInit();
    return;
}
/**
 * 
 */
 void rsTampoline(void *a)
 {
    rnLoop();
    deadEnd(4);
 }
 /**
  * 
  */
void loop()
{
    xTaskCreate(rsTampoline,"rs",1024,NULL,2,NULL);    
    while(1)
    {
        Logger("*R*\n");
        lnDelay(1000);
    }

}
/**
 * 
 */
/**
 * 
 */
void i2cCb(uint32_t signal, const void *cookie)
{
    // this should be overriden by the rust version
}
extern "C" 
{
void __aeabi_unwind_cpp_pr0()
{
        xAssert(0);
}
}
// EOF
