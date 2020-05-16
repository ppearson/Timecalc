cmake_minimum_required (VERSION 2.8)
project (timecalc)

enable_language(CXX)
set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED on)

set(QMAKE_CXXFLAGS "-std=c++11")

set(CMAKE_CXX_FLAGS_RELEASE "-O3 -std=c++11")
set(CMAKE_CXX_FLAGS_DEBUG "-g -std=c++11")

if(NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Debug CACHE STRING
	  "Choose the type of build, options are: None Debug Release"
	  FORCE)
endif(NOT CMAKE_BUILD_TYPE)
if(CMAKE_FIRST_RUN)
	message(STATUS "Setting Build Type to: ${CMAKE_BUILD_TYPE}")
endif()

#include_directories(src)
file(GLOB_RECURSE timecalc_SOURCES "${CMAKE_SOURCE_DIR}/src/*.cpp")

#add_executable(timecalc MACOSX_BUNDLE ${timecalc_SOURCES})
add_executable(timecalc ${timecalc_SOURCES})

