decodeMorse = function(morseCode){
  //your code here
  let cleanCode = morseCode.trim();
  let words = cleanCode.split("   ");
  let message='';
  words.forEach((w)=>{
    let arr = w.split(' ').map((c)=>MORSE_CODE[c]);
    message += arr.join("")+' ';
  });
  return message.trim();
}
