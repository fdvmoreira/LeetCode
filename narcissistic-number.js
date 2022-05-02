function narcissistic(value) {
  // Code me to return true or false
  let digits = value.toString().split('');
  numDigits = digits.length;
  let res = digits.reduce((sum, digit)=>{
    return sum+Math.pow(digit,numDigits)
  },0);
  return res == value;
}
