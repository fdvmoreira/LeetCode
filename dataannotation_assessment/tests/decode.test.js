const assert = require('node:assert');
const { describe, it } = require('node:test');
const decode = require('../decode.js');

describe("decode()", () => {
	it("It should read and decode the file", async () => {
		const fileName = "./tests/testing.data.txt";
		const expected = "one three six ten fifteen twenty-one twenty-eight";

		const actual = await decode(fileName);
		assert.strictEqual(actual, expected);
	});
});
