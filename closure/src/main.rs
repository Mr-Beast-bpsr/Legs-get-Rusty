
use std::thread;
use std::time::Duration;
fn main(){


let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};


//With type annotations added, the syntax of closures looks more similar to the syntax of functions.
// Here we define a function that adds 1 to its parameter and a closure that has the same behavior, for comparison.
// We’ve added some spaces to line up the relevant parts.
// This illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x:u32|             { x + 1 };
let add_one_v4 = |x: u32|               x + 1  ;

}