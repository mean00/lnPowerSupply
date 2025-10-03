/**
 *  Demo code for ILI9341 320x240 screen with touch capability
 *

 *
 */
#include "esprit.h"
//
#include "../../inc/lnI2CTask.h"
//

class lnI2cTaskShim
{
  public:
    static float getVoltage();
    static int getCurrent();
    static void setMaxCurrent(int mA);
    static bool getCCLimited();
    static void setDCEnable(bool enable);
    static void setOutputEnable(bool enable);
    static void setCb(lnI2cTask::signalCb *c);
};

lnI2cTask *shimCreateI2CTask(lnI2cTask::signalCb *c, const void *cookie);

// EOF
