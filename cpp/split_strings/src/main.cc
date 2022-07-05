//
// Created by Flavio Moreira on 04/07/22.
//
#include <iostream>
#include "include/string_pair.h"

int main([[maybe_unused]] int argc, [[maybe_unused]] char **argv){
    for(auto s:solution("abcde45"))
        std::cout <<s<<"\n";
    return 0;
}