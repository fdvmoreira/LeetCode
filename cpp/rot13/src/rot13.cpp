//
// Created by moreira on 29/06/22.
//

#include <iostream>
#include "rot13.h"
#include <string>

#include <array>
#include <numeric>
#include <algorithm>

/**
 *
 * @param message
 * @return
 */
std::string rot13(const std::string &message);

int main([[maybe_unused]]int argc, [[maybe_unused]]char **argv){

    std::cout<< rot13("hello world!")<<std::endl;
    return 0;
}
// SOURCE: https://www.codewars.com/kata/530e15517bc88ac656000716/train/javascript
// **************** Pseudocode ****************
// ********************************************
// LOOK through message for english alphabet
// IF the character IS an alphabet
//     convert to lowercase if uppercase
//     convert it to ascii value
//     use the formula x = (x+n) MOD 26
//     ?? get the character at that position in the array
//     convert it to character again
//     save it to the new string array
//
//


std::string rot13(const std::string &message){
    std::string alphabet;
    for (char c = 'a';c <= 'z';c++){
        alphabet.append(std::to_string(c));
    }

    return alphabet;
}
