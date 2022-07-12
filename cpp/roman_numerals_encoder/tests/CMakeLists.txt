
#
add_executable(rne_test roman_num_enc_test.cc)

target_link_libraries(rne_test gtest_main rne_lib)

include(GoogleTest)
gtest_discover_tests(rne_test)
