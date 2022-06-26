const sumDigPow = require('./../sum-of-digits');
const Assert = require('chai').assert;


describe("Example Tests", function () {
    it("Basic test", () => {

        Assert.deepEqual(sumDigPow(1, 10), [1, 2, 3, 4, 5, 6, 7, 8, 9], "sumDigPow(1, 10) to return [1, 2, 3, 4, 5, 6, 7, 8, 9]")
        Assert.deepEqual(sumDigPow(1, 100), [1, 2, 3, 4, 5, 6, 7, 8, 9, 89], "sumDigPow(1, 100) to return [1, 2, 3, 4, 5, 6, 7, 8, 9, 89]")
        Assert.deepEqual(sumDigPow(10, 100), [89], "sumDigPow(10, 100)to return [89]")
        Assert.deepEqual(sumDigPow(90, 100), [], "sumDigPow(90, 100) to return []")
        Assert.deepEqual(sumDigPow(90, 150), [135], "sumDigPow(90, 150) to return [135]")
        Assert.deepEqual(sumDigPow(50, 150), [89, 135], "sumDigPow(50, 150) to return [89, 135]")
        Assert.deepEqual(sumDigPow(10, 150), [89, 135], "sumDigPow(10, 150) to return [89, 135]")
    });
});