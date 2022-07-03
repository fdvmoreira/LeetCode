#include <string>
#include <array>
#include "rot13.h"

// SOURCE: https://www.codewars.com/kata/530e15517bc88ac656000716/train/javascript
// **************** Pseudocode ****************
// ********************************************
// LOOK through message for english alphabet
// IF the character IS an alphabet
//     convert to lowercase if uppercase
//     convert it to ascii value
//     use the formula x = (x+n-97) MOD 26+97 lower case
//     ?? get the character at that position in the array
//     convert it to character again
//     save it to the new string array
//
//

/**
 *
 * @param message
 * @return
 */
std::string rot13(const std::string &message){
    std::string rot13;

    for (auto s : message) {
        if (isalpha(s)){
            char c = toascii(s);
            bool upper_case = std::isupper(s);

            if(upper_case)
                c = std::tolower(s);

            char ciphered_character = static_cast<char>((c + 13 - 97) % 26 + 97);

            ciphered_character = upper_case? static_cast<char >(std::toupper(ciphered_character)):ciphered_character;

            rot13.append(1,ciphered_character);
            continue;
        }
        rot13.append(1,s);
    }

    return rot13;
}
