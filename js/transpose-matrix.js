// transpose a 2D matrix

/**
 * @param {Array<Array<number>>} matrix
 * @return {number[][]}
 */
export function transposeMatrix(matrix) {
  /** @constant {number} */
  const rows = matrix.length;
  const columns = matrix[0].length;

  /** type {Array<Array<number>>} */
  let transposedMatrix = [];

  for (let i = 0; i < columns; i++) {
    transposedMatrix[i] = [];
  }

  for (let row = 0; row < rows; row++) {
    for (let column = 0; column < columns; column++) {
      transposedMatrix[column][row] = matrix[row][column];
    }
  }

  return transposedMatrix;
}
