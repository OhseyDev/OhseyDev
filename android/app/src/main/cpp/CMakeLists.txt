cmake_minimum_required(VERSION 3.4.1)

set(${CMAKE_C_FLAGS}, "${CMAKE_C_FLAGS}")
add_library(android-glue STATIC ${ANDROID_NDK}/sources/android/native_app_glue/android_native_app_glue.c)
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -std=gnu++20 -Wall -Werror")
set(CMAKE_SHARED_LINKER_FLAGS "${CMAKE_SHARED_LINKER_FLAGS} -u ANativeActivity_onCreate")
add_library(ocme-android SHARED main.cpp)
target_include_directories(ocme-android PRIVATE ${ANDROID_NDK}/sources/android/native_app_glue)
target_link_libraries(ocme-android android android-glue EGL GLESv1_CM log)
