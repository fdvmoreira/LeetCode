include(FetchContent)
FetchContent_Declare(
        googletest
        GIT_REPOSITORY git@github.com:fdvmoreira/googletest.git
        GIT_TAG        release-1.12.1
)

FetchContent_MakeAvailable(googletest)
add_library(GTest::GTest INTERFACE IMPORTED)
target_link_libraries(GTest::GTest INTERFACE gtest_main)

add_executable(rot13_test rot13_test.cpp)
target_link_libraries(rot13_test PRIVATE GTest::GTest)

add_test(rot13_gtest rot13_test)