#include(FetchContent)

#FetchContent_Declare(
#        googletest
#        GIT_REPOSITORY git@github:/fdvmoreira/googletest.git
#        GIT_HASH       release-1.12.1
#)

#FetchContent_MakeAvailable(googletest)

#add_library(GTest::GTest INTERFACE IMPORTED)
#target_link_libraries(GTest::GTest INTERFACE gtest_main)

#add_library(string_pair_lib ./../src/string_pair.cc ./../src/include/string_pair.h )

#add_executable(string_pair_test string_pair_test.cc)

#target_link_libraries(string_pair_test GTest::GTest string_pair_lib)

#add_test(string_pair_gtest string_pair_test)

# NEW APPROACH

include_directories(string_pair_test PUBLIC "${CMAKE_CURRENT_SOURCE_DIR}/../src/include/")

add_executable(string_pair_test string_pair_test.cc)

target_link_libraries(string_pair_test gtest_main)

include(GoogleTest)
gtest_discover_tests(string_pair_test)