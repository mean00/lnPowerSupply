#-----------------------------------------------------------------------------
#
# CMakeLists for lnDSO
# An opensource firmware for the DSO SHELL 150
# With optional hardware mods :
#   - Alternative rotary encoder wiring : MUCH Better, almost mandatory
#
#-----------------------------------------------------------------------------

cmake_minimum_required(VERSION 3.0)
MESSAGE(STATUS "======================")
MESSAGE(STATUS "Starting lnPowerSupply")
MESSAGE(STATUS "======================")
#____________________
# Options
#____________________

SET(LN_ENABLE_I2C  ON CACHE INTERNAL "")
SET(LN_ENABLE_SPI  ON CACHE INTERNAL "")

# USB SETUP

SET(LN_USB_NB_CDC  0 CACHE INTERNAL "")
SET(LN_USB_NB_HID  0 CACHE INTERNAL "")
SET(LN_USB_DFU_RUNTIME 1 CACHE INTERNAL "")

OPTION(USE_ROTARY_FULL_STEP     "Use full step for rotary encoder, else half step"  FALSE)
OPTION(USE_USB                  "Enable USB support"                                FALSE)
OPTION(USE_SMALLFOOTPRINT       "Build for 64k flash targets"                       FALSE)
OPTION(USE_GD32F3               "Build for GD32F303"                                FALSE)


SET(AF_FOLDER  ${CMAKE_SOURCE_DIR}/lnArduino/)
include(./mcuSelect.cmake)
SET(CMAKE_TOOLCHAIN_FILE ${AF_FOLDER}/lnArduino.cmake)

SET(LN_ENABLE_RUST ON CACHE INTERNAL "")

PROJECT(lnPowerSupply NONE) # the NONE is very important !

enable_language(C CXX ASM) # this is important too!

#

include_directories(${CMAKE_SOURCE_DIR})
include_directories(${CMAKE_SOURCE_DIR}/include)
add_subdirectory(${AF_FOLDER})

include(${AF_FOLDER}/setup.cmake)

SET(EXTENSION "_${LN_MCU}")


IF(USE_ROTARY_FULL_STEP)
    SET(EXTENSION "${EXTENSION}_fstep")
ELSE(USE_ROTARY_FULL_STEP)
    ADD_DEFINITIONS("-DDSO_ROTARY_FULL_STEP")
    SET(EXTENSION "${EXTENSION}_hstep")
ENDIF(USE_ROTARY_FULL_STEP)


math(EXPR MCU_SPEED_M "${LN_MCU_SPEED}/1000000")
SET(EXTENSION "${EXTENSION}_${MCU_SPEED_M}M")


include_directories(src)

#_______________________
# FPU
#_______________________
IF("${LN_ARCH}" STREQUAL "RISCV")
    SET(FP_LIB rvfp)
ELSEIF("${LN_ARCH}" STREQUAL "ARM")
    IF("${LN_MCU}" STREQUAL "M3")
        SET(FP_LIB qfpm3)
    ENDIF("${LN_MCU}" STREQUAL "M3")
ENDIF("${LN_ARCH}" STREQUAL "RISCV")

#_______________________
#  Display
#_______________________
IF(USE_SMALLFOOTPRINT)
    SET(EXTENSION "${EXTENSION}_small") 
ELSE()
  SET(DSO_EXTRA_LIB simplerILI9341HS)
  SET(SIMPLER_ILI9341_FONT_COMPRESSION ON CACHE INTERNAL "")
  SET(SIMPLER_ILI9341_2BPP_SUPPORT     ON CACHE INTERNAL "")
  SET(SIMPLER_ILI9341_4BPP_SUPPORT     ON CACHE INTERNAL "")
ENDIF()
include_directories(externals/simplerILI9341/platformExample/lnArduino)
SET(ILI9341_DRIVER  platformExample/lnArduino/ili_lnSpi.cpp)
ADD_SUBDIRECTORY(externals/simplerILI9341)
include_directories(externals/simplerILI9341)
#

ADD_SUBDIRECTORY(externals/simpler_INA219)
include_directories(externals/simpler_INA219)
#

USE_LIBRARY(NeVerMind)
USE_LIBRARY(simplerMCP4725)

#ADD_SUBDIRECTORY(testFunc)
#
IF(LN_ENABLE_USBD)
    include(${AF_FOLDER}/setup.cmake)
    USE_LIBRARY(tinyUsb)
    SET(EXTENSION "${EXTENSION}_USB")
ENDIF(LN_ENABLE_USBD)
#____________________
# Build
#____________________
include_directories( ${CMAKE_CURRENT_SOURCE_DIR}/inc)
include_directories( ${CMAKE_CURRENT_SOURCE_DIR}/assets/fonts)
add_subdirectory( ${CMAKE_CURRENT_SOURCE_DIR}/src/rs)
GENERATE_GD32_FIRMWARE(lnPowerSupply src/lnPowerSupply.cpp src/lnI2CTask.cpp src/lnDisplay.cpp)
IF(LN_ENABLE_USBD)
    TARGET_LINK_LIBRARIES( lnPowerSupply  gd32_usb_usbd)
ENDIF()
TARGET_LINK_LIBRARIES( lnPowerSupply  simplerILI9341 simpler_INA219 simplerMCP4725 ${DSO_EXTRA_LIB} rsPowerSupply )


IF(FALSE)
  add_custom_command(TARGET lnPowerSupply
                   POST_BUILD
                   COMMAND ${CMAKE_OBJCOPY} -Obinary $<TARGET_FILE:lnDSO> $<TARGET_FILE:lnDSO>.tmp
                   COMMAND python3 ${CMAKE_SOURCE_DIR}/lnDsoChecksum.py  $<TARGET_FILE:lnDSO>.tmp $<TARGET_FILE:lnDSO>.checksumed.bin
                   WORKING_DIRECTORY ${CMAKE_BINARY_DIR}
                   COMMENT "Generating checksumed bin file"
    )
ENDIF(FALSE)


# Summary
MESSAGE(STATUS "Configuration:")
MESSAGE(STATUS "\tBuild folder ${CMAKE_BINARY_DIR}")
MESSAGE(STATUS "\tUsing ${LN_MCU} based MCU at ${LN_MCU_SPEED} Hz")
MESSAGE(STATUS "\tUsing ${LN_MCU_FLASH_SIZE} kB of flash (small = ${USE_SMALLFOOTPRINT})")
MESSAGE(STATUS "\tUSB control enabled : ${LN_ENABLE_USBD}")

IF(USE_ROTARY_FULL_STEP)
     MESSAGE(STATUS "\tUsing rotary encoder Full step ")
ELSE()
     MESSAGE(STATUS "\tUsing rotary encoder Half step ")
ENDIF()

#

set_target_properties(lnPowerSupply PROPERTIES OUTPUT_NAME lnPowerSupply${EXTENSION})

MESSAGE(STATUS "\tFinal name is ${EXTENSION}")
