/**
 *  This file does all the display related stuff
 *
 */

#include "lnDisplay.h"
#include "ili_lnSpi.h"
#include "lnHardwareConfiguration.h"
#include "simpler9341_priv.h"

#include "roboto12.h"
#include "robotoLight.h"
#include "robotoslab.h"
#include "waree12.h"

// #define FONT Waree36pt7b
#define FONT RobotoSlab_SemiBold48pt7b
// #define SMALLFONT Waree12pt7b
#define SMALLFONT Roboto_Light12pt7b
#define MEDFONT Roboto_Light28pt7b
#define FAST 1

#define MAIN_COLUMN 76
#define LIMIT_COLUMN 318

#define V_LINE 80
#define A_LINE 130
#define PW_LINE 180
#define MAX_C_LINE 210
#define VBAT_LINE 238
#define UNITS_OFFSET 100

extern const uint8_t dso_resetOff[];
extern const uint8_t dso_wakeOn[];

lnSpi9341 *ili;
hwlnSPIClass *spi;

static char buffer[64];
;

static void lcdPrint(lnSpi9341::FontSize size, int colum, int maxColumn, int line, const char *txt)
{
    ili->setFontSize(size);
    ili->setCursor(colum, line);
    ili->printUpTo(txt, maxColumn);
}

/**
 *
 */
void lnDisplay::init()
{
    spi = new hwlnSPIClass(ILI_SPI_INSTANCE, -1);
    lnSPISettings transaction(FAST * 36 * 1000 * 1000 + (1 - FAST) * 10 * 1000, SPI_MSBFIRST, SPI_MODE0, -1);
    spi->begin();

    ili = new lnSpi9341(240, 320, spi,
                        ILI_PIN_DC,     // DC/RS
                        ILI_PIN_CS,     // CS
                        ILI_PIN_RESET); // Reset
    spi->beginTransaction(transaction);
    ili->init(dso_resetOff, dso_wakeOn);
    ili->setRotation(1);
    ili->enableCache(512);
    ili->fillScreen(WHITE);

    ili->setFontFamily(&SMALLFONT, &MEDFONT, &FONT);

    ili->setFontSize(ili9341::BigFont);

    ili->fillScreen(BLACK);

    ili->setTextColor(YELLOW, BLACK);
    lcdPrint(lnSpi9341::BigFont, 0, MAIN_COLUMN, V_LINE, "V");
    lcdPrint(lnSpi9341::MediumFont, 18, MAIN_COLUMN, A_LINE, "A");
    lcdPrint(lnSpi9341::SmallFont, 18, MAIN_COLUMN, MAX_C_LINE, "Max");
    lcdPrint(lnSpi9341::MediumFont, 18, MAIN_COLUMN, PW_LINE, "P");
    ili->setTextColor(WHITE, BLACK);
    lcdPrint(lnSpi9341::MediumFont, LIMIT_COLUMN - UNITS_OFFSET, LIMIT_COLUMN, A_LINE, "mA");
    lcdPrint(lnSpi9341::MediumFont, LIMIT_COLUMN - UNITS_OFFSET, LIMIT_COLUMN, PW_LINE, "W");

    // benchmark();
}
/**
 *
 *
 */
void lnDisplay::displayMaxCurrent(int maxAmp)
{
    float f = maxAmp;
    f /= 1000.;
    sprintf(buffer, "%2.1fA", f);
    lcdPrint(ili9341::SmallFont, MAIN_COLUMN, LIMIT_COLUMN, MAX_C_LINE, buffer);
}

void lnDisplay::banner(const char *msg)
{
    ili->setFontSize(ili9341::SmallFont);
    ili->setCursor(180, MAX_C_LINE);
    ili->print(msg);
}
/**
 *
 */
void lnDisplay::displayVbat(float vbat)
{
    sprintf(buffer, "Bat%2.1f", vbat);
    lcdPrint(ili9341::SmallFont, 200, 318, VBAT_LINE, buffer);
}
/**
 *
 */
void lnDisplay::displayCurrent(int ma)
{
    sprintf(buffer, "%d", ma);
    lcdPrint(ili9341::MediumFont, MAIN_COLUMN, LIMIT_COLUMN - UNITS_OFFSET - MAIN_COLUMN, A_LINE, buffer);
}

/**
 *
 */
static void lnDisplay_float(bool cc, float value, int line)
{
    sprintf(buffer, "%2.2f", value);
    if (cc)
        ili->setTextColor(RED, BLACK);
    else
        ili->setTextColor(GREEN, BLACK);
    lcdPrint(ili9341::BigFont, MAIN_COLUMN, LIMIT_COLUMN, line, buffer);
    ili->setTextColor(WHITE, BLACK);
}

/**
 *
 */
void lnDisplay::displayVoltage(bool cc, float voltage)
{
    lnDisplay_float(cc, voltage, V_LINE);
}
/**
 *
 */
void lnDisplay::displayPower(bool cc, float powr)
{
    sprintf(buffer, "%2.1f", powr);
    lcdPrint(ili9341::MediumFont, MAIN_COLUMN, LIMIT_COLUMN - UNITS_OFFSET - MAIN_COLUMN, PW_LINE, buffer);
}
/**
 *
 */
void lnDisplay::benchmark()
{
    int rnd = 0;
    int total = 0;
    while (1)
    {
        int before = lnGetUs();
        lcdPrint(ili9341::BigFont, 20, 240, 120, "VVVVV");
        int after = lnGetUs();
        Logger("Rnd %d : %d us \n", rnd, after - before);
        rnd++;
    }
}

// EOF
