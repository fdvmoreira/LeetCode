(function(){function r(e,n,t){function o(i,f){if(!n[i]){if(!e[i]){var c="function"==typeof require&&require;if(!f&&c)return c(i,!0);if(u)return u(i,!0);var a=new Error("Cannot find module '"+i+"'");throw a.code="MODULE_NOT_FOUND",a}var p=n[i]={exports:{}};e[i][0].call(p.exports,function(r){var n=e[i][1][r];return o(n||r)},p,p.exports,r,e,n,t)}return n[i].exports}for(var u="function"==typeof require&&require,i=0;i<t.length;i++)o(t[i]);return o}return r})()({1:[function(require,module,exports){
/**
 * Source
 * https://www.codewars.com/kata/523f5d21c841566fde000009/train/javascript
 * 
 * Instructions
 * 
 * Your goal in this kata is to implement a difference function, which subtracts one list from another and returns the result.
 * It should remove all values from list a, which are present in list b keeping their order.
 * arrayDiff([1,2],[1]) == [2]
 * 
 * If a value is present in b, all of its occurrences must be removed from the other:
 * arrayDiff([1,2,2,2,3],[2]) == [1,3]
 */

function arrayDiff(a = [], b = []) {
    let str = a.toString();
    let newStr = "";
    b.forEach(element => {
        if (str.includes()) newStr += str.replace(element, '');
        console.log(str);
    });
    return Array.from(newStr);
}

module.exports = arrayDiff;
},{}]},{},[1]);
