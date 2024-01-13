

use std::fs::File;
use std::io::Read;
use std::io;
pub fn question_error_handling() -> Result<String, io :: Error  >{
// If we fail instead of panic the function will end

let mut s = String::new();
    let mut f = File::open("hello.txt")?;
   

   // If this funcition fails we return error
   f.read_to_string(&mut s)?; 
   Ok(s)



}