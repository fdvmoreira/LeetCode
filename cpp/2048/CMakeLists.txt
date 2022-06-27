cmake_minimum_required(VERSION 3.22)
project(challenges)

set(CMAKE_CXX_STANDARD 17)

option(Dev ON)

add_executable(2048
        2048.cpp)
