cmake_minimum_required(VERSION 3.20)

project(CodingChallenges LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)

option(GDB_Debug ON)

#compiler flags
set(GCC_Compiler_Flags -Wall -Wextra -pedantic)

# add debug flags to the list
if(GBD_Debug)
    list(APPEND GCC_Compiler_Flags -g -ggdb3)
endif()

if(MSVC)
    # warning level 4 and all warnings as error
    add_compile_options(/W4 /WX)
else()
    #lots of warnings
    add_compile_options("${GCC_Compiler_Flags}")
endif()

include(FetchContent)
FetchContent_Declare(
        googletest
        URL https://github.com/google/googletest/archive/609281088cfefc76f9d0ce82e1ff6c30cc3591e5.zip
)
FetchContent_MakeAvailable(googletest)

enable_testing()

add_subdirectory(cpp)
