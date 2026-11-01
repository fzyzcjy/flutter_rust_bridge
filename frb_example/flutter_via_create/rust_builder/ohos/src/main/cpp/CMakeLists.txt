# the minimum version of CMake.
cmake_minimum_required(VERSION 3.10)

# Project-level configuration.
set(PROJECT_NAME "rust_lib_flutter_via_create")
set(TARGET_LIB_DIR ${CMAKE_CURRENT_SOURCE_DIR}/../../../libs/${OHOS_ARCH})
project(${PROJECT_NAME} LANGUAGES CXX)
if (NOT DEFINED FLUTTER_TARGET_PLATFORM)
    if (OHOS_ARCH STREQUAL "arm64-v8a")
        set(FLUTTER_TARGET_PLATFORM "ohos-arm64")
    elseif(OHOS_ARCH STREQUAL "x86_64")
        set(FLUTTER_TARGET_PLATFORM "ohos-x64")
    elseif(OHOS_ARCH STREQUAL "armeabi-v7a")
        message(FATAL_ERROR
            "The current HarmonyOS Flutter/Hvigor toolchain does not support "
            "the armeabi-v7a ABI. Use arm64-v8a or x86_64.")
    else()
        message(FATAL_ERROR
            "Unsupported OHOS_ARCH: ${OHOS_ARCH}. Use arm64-v8a or x86_64.")
    endif()
    message(STATUS "Detected OpenHarmony platform: ${FLUTTER_TARGET_PLATFORM}")
endif()
include("../../../../cargokit/cmake/cargokit.cmake")
apply_cargokit(${PROJECT_NAME} ../../../../../rust rust_lib_flutter_via_create "")
