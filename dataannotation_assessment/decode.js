const fs = require('fs');

/**
	* Load file content
	* @param {string} file
	* @returns {Promise<string>}
	*/
function loadfile(file) {
	return new Promise((resolve, reject) => {
		fs.readFile(file, 'utf8', (err, data) => {
			if (err) {
				reject(err);
			} else {
				resolve(data);
			}
		});
	});
}

/**
	* Time complexity: O(n) | Space complexity: O(n) where n is the number of lines in the file
	* Decode the file
	* @param {string} fileName
	* @returns {Promise<string>}
	*/
async function decode(fileName) {
	let decoded = "";
	try {
		const content = await loadfile(fileName);

		const map = new Map(content
			.split("\n")
			.map(line => {
				const [num, word] = line.trim().split(" "); // remove trailing RETURN(\r) char and split by space
				return [parseInt(num), word];
			}));

		let prevIncrement = 1;
		for (let index = 0; index < map.size; index += prevIncrement) {
			decoded += map.get(index + 1) + " ";
			prevIncrement += 1;
		}

		decoded = decoded.trimEnd(); // remove trailing space

	} catch (err) {
		console.error(`Oops! Something went wrong!\nError: ${err.name}\nMessage: ${err.message}`, err);
	}

	return decoded;
}

module.exports = decode;

