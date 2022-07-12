//
// Created by Flavio Moreira on 12/07/22.
//

#include <gtest/gtest.h>
#include "../src/include/roman_numerals_encoder.h"

TEST(RomanNumeralsEncoder, SingleDigitNumbers){
    ASSERT_EQ(solution(1),"I") <<" 1 should be I";
    ASSERT_EQ(solution(2),"II") <<" 2 should be II";
    ASSERT_EQ(solution(3),"III") <<" 3 should be III";
    ASSERT_EQ(solution(4),"IV") <<" 4 should be IV";
    ASSERT_EQ(solution(5),"V") <<" 5 should be V";
    ASSERT_EQ(solution(6),"VI") <<" 6 should be VI";
    ASSERT_EQ(solution(8),"I") <<" 8 should be VIII";
};

TEST(RomanNumeralsEncoder, TwoDigitsNumbers){
    ASSERT_EQ(solution(10),"X") <<" 10 should be X";
    ASSERT_EQ(solution(40),"XL") <<" 40 should be XL";
    ASSERT_EQ(solution(50),"L") <<" 50 should be L";
    ASSERT_EQ(solution(81),"LXXXI") <<" 81 should be LXXXI";
};

TEST(RomanNumeralsEncoder, ThreeDigitsNumbers){
    ASSERT_EQ(solution(100),"C") <<" 100 should be C";
    ASSERT_EQ(solution(500),"D") <<" 500 should be D";
    ASSERT_EQ(solution(672),"DCLXXII") <<" 672 should be DCLXXII";
    ASSERT_EQ(solution(998),"IIM") <<" 998 should be IIM";
    ASSERT_EQ(solution(182),"CLXXXII") <<" 182 should be CLXXXII";

};

TEST(RomanNumeralsEncoder, FourDigitsNumbers){
    ASSERT_EQ(solution(1000),"M") <<" 1000 should be M";
    ASSERT_EQ(solution(1021),"MXXI") <<" 1021 should be MXXI";
    ASSERT_EQ(solution(1990),"MCMXC") <<" 1990 should be MCMXC";
    ASSERT_EQ(solution(1875),"MDCCCLXXV") <<" 1875 should be MDCCCLXXV";

};
