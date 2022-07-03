#include <gtest/gtest.h>
#include <string>
#include "../src/rot13.h"

TEST(Rot13Test, TestEmptyString){
    ASSERT_EQ(rot13(" ")," ");
}

TEST(Rot13Test,TestSpecialCharacters){
    ASSERT_EQ(rot13("#"),"#") <<"# should return #";
    ASSERT_EQ(rot13(","),",") <<", should return ,";
    ASSERT_EQ(rot13("$"),"$") <<"$ should return $";
    ASSERT_EQ(rot13("^"),"^") <<"^ should return ^";
}

TEST(Rot13Test, TestSingleWords){
    ASSERT_EQ(rot13("test"), "grfg") << "test Should return grfg";
    ASSERT_EQ(rot13("Test"), "Grfg") << "Test Should return Grfg";
    ASSERT_EQ(rot13("AbCd"), "NoPq") << "AbCd Should return NoPq";
}