
/**
 * @source https://leetcode.com/problems/sort-an-array/
 * @param {number[]} nums
 * @return {number[]}
 */
var sortArray = function (nums) {
    if (nums.length <= 1) return nums;

    for (let i = 1; i < nums.length; i++) {
        let j = i - 1;

        while (nums[j + 1] < nums[j] && j >= 0) {
            let tmp = nums[j + 1];
            nums[j + 1] = nums[j];
            nums[j] = tmp;

            j -= 1;
        }
    }
    return nums;
};