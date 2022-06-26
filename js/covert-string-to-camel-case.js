/**
* 
* convert string to camel case
* 
*/

function toCamelCase(str){
  let newString="";
  
  // loop through the strig
  for(let i=0;i<str.length;i++){
    
    // if the char - or _ is present
    const isDashOrUnderscorePresent = (str[i]==='-'||str[i]==='_');
    if(isDashOrUnderscorePresent){
      
      // increment the counter to the position of next character and make it uppercase
      ++i;
      // convert the character to uppercase and assign it to new string string
      newString += str[i].toUpperCase();
      
      //jump to the next iteration
      continue;
    }
    newString+= str[i];
  }
  return newString;
}
