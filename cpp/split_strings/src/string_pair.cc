//
// Created by Flavio Moreira on 04/07/22.
//
#include <string>
#include <vector>

#include "include/string_pair.h"

std::vector<std::string> solution(const std::string &s){
    std::vector<std::string> pairs;

    for(size_t i = 0; i < s.length();i=i+2){
        pairs.push_back(s.substr(i,1));
        if (i+1<s.length()) pairs.back().append(1,s.at(i+1));
    }

    // if the size is odd add the last character to the last position
    if (s.length()%2) pairs.back().append(1,'_');
    return pairs;
}

/**
 * source: https://www.codewars.com/kata/515de9ae9dcfc28eb6000001/train/cpp
 *
 *
*/