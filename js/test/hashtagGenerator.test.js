/**
 * @source : https://www.codewars.com/kata/52449b062fb80683ec000024/train/javascript
 * Generate hastag from a text
 */
const chai = require('chai');
const Assert = chai.assert;
chai.config.truncateThreshold = 0;

const generateHashtag = require('../hashtagGenerator');

describe("Test Hashtag Generator", () => {
    it("tests should pass", () => {
        Assert.strictEqual(generateHashtag(""), false, "Expected an empty string to return false")
        Assert.strictEqual(generateHashtag(" ".repeat(200)), false, "Still an empty string")
        Assert.strictEqual(generateHashtag("Do We have A Hashtag"), "#DoWeHaveAHashtag", "Expected a Hashtag (#) at the beginning.")
        Assert.strictEqual(generateHashtag("Codewars"), "#Codewars", "Should handle a single word.")
        Assert.strictEqual(generateHashtag("Codewars Is Nice"), "#CodewarsIsNice", "Should remove spaces.")
        Assert.strictEqual(generateHashtag("Codewars is nice"), "#CodewarsIsNice", "Should capitalize first letters of words.")
        Assert.strictEqual(generateHashtag("code" + " ".repeat(140) + "wars"), "#CodeWars")
        Assert.strictEqual(generateHashtag("Looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong Cat"), false, "Should return false if the final word is longer than 140 chars.")
        Assert.strictEqual(generateHashtag("a".repeat(139)), "#A" + "a".repeat(138), "Should work")
        Assert.strictEqual(generateHashtag("a".repeat(140)), false, "Too long")
    });
});


