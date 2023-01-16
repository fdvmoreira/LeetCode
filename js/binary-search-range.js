/**
 * @source https://leetcode.com/problems/guess-number-higher-or-lower/ 
 * Forward declaration of guess API.
 * @param {number} num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * var guess = function(num) {}
 */

/**
 * @param {number} n
 * @return {number}
 */
var guessNumber = function (n) {
    let [L, R] = [0, n];

    while (L <= R) {
        let num = Math.floor((L + R) / 2);

        if (guess(num) < 0) {
            R = num - 1;
        } else if (guess(num) > 0) {
            L = num + 1;
        } else {
            return num
        }
    }
};