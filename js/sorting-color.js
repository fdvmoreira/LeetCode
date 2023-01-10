/**
 * Sorting colors using the backet sort algorithm
 * @source https://leetcode.com/problems/sort-colors/submissions/
 * 
 * @param {number[]} nums
 * @return {void} Do not return anything, modify nums in-place instead.
 */
var sortColors = function (nums) {
    const [RED, WHITE, BLUE] = [0, 1, 2];
    let counts = new Array(3).fill(0);

    nums.forEach(num => {
        switch (num) {
            case 0:
                counts[RED] += 1;
                break;
            case 1:
                counts[WHITE] += 1;
                break;
            case 2:
                counts[BLUE] += 1;
                break;
            default:
        }
    });

    let nIndex = 0;
    counts.forEach((count, index) => {
        for (let i = 0; i < count; i++) {
            nums[nIndex] = index;
            nIndex += 1;
        }
    });

    return nums;
};