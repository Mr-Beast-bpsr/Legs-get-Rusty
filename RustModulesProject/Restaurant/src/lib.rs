pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// If you mark a struct as pub its keywords will still be private;
// If you mark a enum as pub its keywords will become public;
// You can manually mark a struct keyword public for use.
// Creating modules
mod front_of_house {
    // To make a mod accessible by outside we use pub  
    pub mod hosting {
// This will make add_to_waitlist public function as well and it can be used outside 
       pub fn add_to_waitlist (){

        }
    }

}

pub fn eat_at_restaurant(){

    // Absolute path : Starts from crate/Environment
    crate::front_of_house::hosting::add_to_waitlist;

    // Relative path : Starts from file path
    front_of_house::hosting::add_to_waitlist;
}



//  Using relative path in a super queuer

fn serve_order(){}

mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        //Super is used to access a function from parent module when in a sub module 
        super::serve_order();


    }
}


//// Importing module into the scope 
/// This type of definition tells rust to get the package from same name file.
mod front_of_house;

// using "use" to import the module we described above
// Adding pub to use keyword would not only allow us to import the module but also export it as a module
use self::front_of_house::hosting;
// Using nested paths to import multiple modules from the same package
use rand::{Rng,CryptoRng, ErrorKind::Transient};
// Using self inside nested paths to shorten the path
use std::io::{self,Write};
// By importing front_of_house we can  access the module itself 
pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(0,5000);
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
