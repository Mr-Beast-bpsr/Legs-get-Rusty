use std::fs::File;
use std::io::ErrorKind;
pub fn match_error_handling(){

    let f = File::open("hello.txt");
    let f= match f{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match  File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file:  {:?} ", e ) ,
            },
            _other_error => {panic!("ERRORR")}
        }
    };


    // We can always use unwrap just to throw an panic error instead of crashing

    // let fu = File::open("hello-kitty.txt").unwrap();




}