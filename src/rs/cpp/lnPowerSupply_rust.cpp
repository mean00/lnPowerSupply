/**
 *  Massively over engineered power supply
 *  This is the startup code that will then give control to the rust part

 *
 */
#include "esprit.h"
//
#include "lnDebug.h"
extern "C" void rnLoop(void);
extern "C" void rnInit(void);
extern void rttLoggerFunction(int n, const char *data);
extern void LN_RTT_Init();
/**
 *
 */
void setup()
{
    LN_RTT_Init();
    setLogger(rttLoggerFunction); // REDIRECT LOGGING TO RTT
    Logger("\nJumping in rust part..\n");
    rnInit();
    return;
}

/**
 *
 */
void loop()
{
    rnLoop();
    deadEnd(4);
}

extern "C"
{
    void __aeabi_unwind_cpp_pr0()
    {
        xAssert(0);
    }
    void lnp_c_hook()
    {
    }
}
// EOF
