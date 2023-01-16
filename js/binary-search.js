/**
 * @source https://leetcode.com/problems/binary-search/
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function (nums, target) {
    let [L, R] = [0, nums.length - 1];

    while (L <= R) {
        let mid = Math.floor((L + R) / 2);

        if (nums[mid] < target) {
            L = mid + 1;
        } else if (nums[mid] > target) {
            R = mid - 1;
        } else {
            return mid;
        }
    }

    return -1;
};