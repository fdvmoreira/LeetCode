# Test

include_directories(PUBLIC ./../src/include)

add_library(2048lib STATIC ./../src/2048.cpp)

add_executable(2048_test 2048_test.cc ./../src/include/2048.h)

target_link_libraries(2048_test gtest_main 2048lib)

include(GoogleTest)
gtest_discover_tests(2048_test)
