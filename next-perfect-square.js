function findNextSquare(sq) {
  // Return the next square if sq is a perfect square, -1 otherwise
  let sqrt = Math.sqrt(sq);
  //if(sqrt != Number.parseInt(sqrt,10)) return -1;
  if(Math.ceil(sqrt) != Math.floor(sqrt)) return -1;
  return Math.pow(sqrt+1,2);
}
