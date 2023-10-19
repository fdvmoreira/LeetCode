/**
 *
    Write a function that takes in a non-empty list of non-empty strings and
    returns a list of characters that are common to all strings in the list,
    ignoring multiplicity.

 *
 *
 */

/**
 * @param {string[]} strings
 */
export function commonCharacters(strings: string[]): string[] {
  let commonChars: string[] = [];

  const longestString = new Set(strings[getLongestStrIndex(strings)]);

  for (const char of longestString) {
    if (isInEvery(char, strings)) commonChars.push(char);
  }

  return commonChars;
}

function isInEvery(char: string, strings: string[]) {
  let b = false;
  for (const str of strings) {
    b = str.includes(char) ? true : false;
    if (!b) break;
  }
  return b;
}

function getLongestStrIndex(strings: string[]): number {
  let index = 0;

  strings.forEach((str, idx) => {
    if (str.length > strings[index].length) {
      index = idx;
    }
  });

  return index;
}
// O(n*m) time | O(m) space - where n is the length of the array of strings and m is the longest string.
//
