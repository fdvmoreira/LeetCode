/**
* 
* convert string to camel case
* 
*/

function toCamelCase(str){
  let newString="";
  let char='';
  
  // loop through the strig
  for(let i=0;i<str.length;i++){
    if(str[i]==='-'||str[i]==='_'){
      ++i;
      newString += str[i].toUpperCase();
      continue;
    }
    newString+= str[i];
  }
  return newString;
}
