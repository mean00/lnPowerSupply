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
    SET(LN_LTO_RUST_FLAGS ="-Clinker-plugin-lto=yes -Clto=thin")
ELSE()
  #SET(LN_LTO_RUST_FLAGS ="-Clinker-plugin-lto=no ")
ENDIF()
#
SET(LN_USB_NB_CDC  0 CACHE INTERNAL "")
SET(LN_USB_NB_HID  0 CACHE INTERNAL "")
SET(LN_USB_DFU_RUNTIME 1 CACHE INTERNAL "")
SET(LN_ENABLE_USBD FALSE CACHE INTERNAL "")
SET(LN_DEBUG_UART 1 CACHE INTERNAL "")

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
IF("${LN_ARCH}" STREQUAL "ARM")
    IF("${LN_MCU}" STREQUAL "M3")
        SET(FP_LIB qfpm3)
    ENDIF()
ENDIF()

#_______________________
#  Rust vs C++
#_______________________
include_directories( ${CMAKE_CURRENT_SOURCE_DIR}/src/rs/cpp)
SET(START_CODE  ${CMAKE_CURRENT_SOURCE_DIR}/src/rs/cpp/lnPowerSupply_rust.cpp )
#SET(CPP_CODE    ${CMAKE_CURRENT_SOURCE_DIR}/src/rs/cpp/lnI2CTask.cpp )
#

USE_LIBRARY(NeVerMind)

GENERATE_GD32_FIRMWARE(lnPowerSupply ${START_CODE}  )
HASH_GD32_FIRMWARE( lnPowerSupply )
add_subdirectory( ${CMAKE_CURRENT_SOURCE_DIR}/src/rs)
TARGET_LINK_LIBRARIES( lnPowerSupply lnarduino_c_bindings )
#TARGET_LINK_LIBRARIES( lnPowerSupply lnPowerSupply  )
TARGET_LINK_LIBRARIES( lnPowerSupply rsPowerSupply )

# Summary
MESSAGE(STATUS "Configuration:")
MESSAGE(STATUS "\tBuild folder ${CMAKE_BINARY_DIR}")
MESSAGE(STATUS "\tUsing ${LN_MCU} based MCU at ${LN_MCU_SPEED} Hz")
MESSAGE(STATUS "\tUsing ${LN_MCU_FLASH_SIZE} kB of flash (small = ${USE_SMALLFOOTPRINT})")
MESSAGE(STATUS "\tUSB control enabled : ${LN_ENABLE_USBD}")


#
SET(EXTENSION ${EXTENSION}_RUST)
set_target_properties(lnPowerSupply PROPERTIES OUTPUT_NAME lnPowerSupply${EXTENSION})

MESSAGE(STATUS "\tFinal name is ${EXTENSION}")
