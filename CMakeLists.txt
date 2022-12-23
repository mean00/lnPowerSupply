#-----------------------------------------------------------------------------
# CMakeLists for lnPowerSupply
#-----------------------------------------------------------------------------

cmake_minimum_required(VERSION 3.0)
SET(CMAKE_EXPORT_COMPILE_COMMANDS ON)
MESSAGE(STATUS "======================")
MESSAGE(STATUS "Starting lnPowerSupply")
MESSAGE(STATUS "======================")
#____________________
# Options
#____________________

SET(USE_CLANG      ON CACHE INTERNAL "")
SET(LN_ENABLE_RUST ON CACHE INTERNAL "")
SET(LN_ENABLE_I2C  ON CACHE INTERNAL "")
SET(LN_ENABLE_SPI  ON CACHE INTERNAL "")


IF(FALSE)
    SET(LN_LTO "-flto")
    SET(LN_LTO_RUST_FLAGS ="-Clinker-plugin-lto=yes ")
ELSE()
    SET(LN_LTO_RUST_FLAGS ="-Clinker-plugin-lto=no ")
ENDIF()
#
SET(LN_USB_NB_CDC  0 CACHE INTERNAL "")
SET(LN_USB_NB_HID  0 CACHE INTERNAL "")
SET(LN_USB_DFU_RUNTIME 1 CACHE INTERNAL "")

OPTION(USE_USB                  "Enable USB support"                                FALSE)
OPTION(USE_GD32F3               "Build for GD32F303"                                FALSE)


SET(AF_FOLDER  ${CMAKE_SOURCE_DIR}/lnArduino/)
include(./mcuSelect.cmake)
SET(CMAKE_TOOLCHAIN_FILE ${AF_FOLDER}/lnArduino.cmake)



PROJECT(lnPowerSupply NONE) # the NONE is very important !

enable_language(C CXX ASM) # this is important too!

#

include_directories(${CMAKE_SOURCE_DIR})
include_directories(${CMAKE_SOURCE_DIR}/include)
add_subdirectory(${AF_FOLDER})

include(${AF_FOLDER}/setup.cmake)

SET(EXTENSION "_${LN_MCU}")

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
#  Rust vs C++
#_______________________
IF(LN_ENABLE_RUST)
    add_subdirectory( ${CMAKE_CURRENT_SOURCE_DIR}/src/rs)
    include_directories( ${CMAKE_CURRENT_SOURCE_DIR}/src/rs/cpp)
    SET(START_CODE  ${CMAKE_CURRENT_SOURCE_DIR}/src/rs/cpp/lnPowerSupply_rust.cpp )
    #SET(CPP_CODE    ${CMAKE_CURRENT_SOURCE_DIR}/src/rs/cpp/lnI2CTask.cpp )
ELSE() # CPP version
  SET(DSO_EXTRA_LIB simplerILI9341HS)
  SET(SIMPLER_ILI9341_FONT_COMPRESSION ON CACHE INTERNAL "")
  SET(SIMPLER_ILI9341_2BPP_SUPPORT     ON CACHE INTERNAL "")
  SET(SIMPLER_ILI9341_4BPP_SUPPORT     ON CACHE INTERNAL "")
  include_directories(externals/simplerILI9341/cpp/platformExample/lnArduino)
  SET(ILI9341_DRIVER  platformExample/lnArduino/ili_lnSpi.cpp)
  ADD_SUBDIRECTORY(externals/simplerILI9341/cpp)
  include_directories(externals/simplerILI9341/cpp)

  ADD_SUBDIRECTORY(externals/simpler_INA219)
  include_directories(externals/simpler_INA219)
  USE_LIBRARY(simplerMCP4725)
  IF(LN_ENABLE_USBD)
    include(${AF_FOLDER}/setup.cmake)
    USE_LIBRARY(tinyUsb)
    SET(EXTENSION "${EXTENSION}_USB")
    include_directories( ${CMAKE_CURRENT_SOURCE_DIR}/inc)
    include_directories( ${CMAKE_CURRENT_SOURCE_DIR}/assets/fonts)
    SET(START_CODE  src/cpp/lnPowerSupply.cpp )
    SET(CPP_CODE    ${CMAKE_CURRENT_SOURCE_DIR}/src/cpp/lnI2CTask.cpp ${CMAKE_CURRENT_SOURCE_DIR}/src/cpp/lnDisplay.cpp  )
   ENDIF(LN_ENABLE_USBD)
ENDIF()  
#

USE_LIBRARY(NeVerMind)



GENERATE_GD32_FIRMWARE(lnPowerSupply ${START_CODE} ${CPP_CODE}  lnArduino/src/lnFreeRTOSDebug.cpp )
IF(LN_ENABLE_USBD)
    TARGET_LINK_LIBRARIES( lnPowerSupply  gd32_usb_usbd)
ENDIF()

IF(LN_ENABLE_RUST)
    TARGET_LINK_LIBRARIES( lnPowerSupply  rsPowerSupply lnarduino_c_bindings )
ELSE(LN_ENABLE_RUST)    
    TARGET_LINK_LIBRARIES( lnPowerSupply  simplerILI9341 simpler_INA219 simplerMCP4725 ${DSO_EXTRA_LIB} )
ENDIF(LN_ENABLE_RUST)


# Summary
MESSAGE(STATUS "Configuration:")
MESSAGE(STATUS "\tBuild folder ${CMAKE_BINARY_DIR}")
MESSAGE(STATUS "\tUsing ${LN_MCU} based MCU at ${LN_MCU_SPEED} Hz")
MESSAGE(STATUS "\tUsing ${LN_MCU_FLASH_SIZE} kB of flash (small = ${USE_SMALLFOOTPRINT})")
MESSAGE(STATUS "\tUSB control enabled : ${LN_ENABLE_USBD}")


#

set_target_properties(lnPowerSupply PROPERTIES OUTPUT_NAME lnPowerSupply${EXTENSION})

MESSAGE(STATUS "\tFinal name is ${EXTENSION}")
