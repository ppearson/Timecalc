cmake_minimum_required (VERSION 2.8)
project (timecalc)

enable_language(CXX)

SET(CMAKE_CXX_FLAGS_RELEASE "-O2 -mfpmath=sse -fPIC")
SET(CMAKE_CXX_FLAGS_DEBUG "-g -mfpmath=sse -fPIC")

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug CACHE STRING
	  "Choose the type of build, options are: None Debug Release"
	  FORCE)
endif(NOT CMAKE_BUILD_TYPE)
if(CMAKE_FIRST_RUN)
	message(STATUS "Setting Build Type to: ${CMAKE_BUILD_TYPE}")
endif()

#include_directories(src)
FILE(GLOB_RECURSE timecalc_SOURCES "${CMAKE_SOURCE_DIR}/src/*.cpp")

#ADD_EXECUTABLE(timecalc MACOSX_BUNDLE ${timecalc_SOURCES})
ADD_EXECUTABLE(timecalc ${timecalc_SOURCES})

