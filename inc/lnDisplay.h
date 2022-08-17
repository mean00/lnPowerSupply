
#pragma once

class lnDisplay
{
public:
        static void init ();
        static void displayMaxCurrent(int currentMa);
        static void banner(const char *msg);
        static void displayVbat(float vbat);
        static void displayCurrent(int ma);
        static void displayVoltage(bool cc, float voltage);
        static void displayPower(bool cc,float pw);


        static void benchmark();
};