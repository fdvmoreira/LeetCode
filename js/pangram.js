/**
 * Detect Pangram
 * source : codewars
 * check if the string is a pangram - contains every letter of english alphabet
 * 
 * @param {String} str 
 * @returns 
 */

function isPangram(str) {

    let alphabet = [];
    for (let code = "a".charCodeAt(0), index = 0; code <= "z".charCodeAt(0); code++, index++) {
        alphabet[index] = String.fromCharCode(code);
    }

    return alphabet.every(element => str.includes(element) || str.includes(element.toUpperCase()));
}

module.exports = isPangram;