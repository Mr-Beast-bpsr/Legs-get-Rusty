fn main() {
    //----------------Ownership rules--------------------
   // Each value in rust has a variable that's called its owner.
   // There can be only one owner at a time
   // When owner goes out of scop the value will be dropped.



/// Scope of a variable ...
    {// S has not been initialized in the scope
        let s = "Scope";
// S has been initialized in the scope

    } // After the scope of {   } ends s will be dropped and cannot be accessed outside of the scope.


    // Copy a variable

    let x  =10;
    // This will replace x and x cannot be accessed after its value is given to y
    let y = x;
    // To make variables accessible even after their value is given to another variable we use .clone() 
    let z = y.clone();
    println!("{}:{}", y,z);


    set_variable();
}



//// Function to explain how ownership of a string variable  can be taken by a function and the owner ship of integer remains the same

fn set_variable(){
    //Here set_variable has the ownership of v 
    let v =  String::from("YeeHooo");
    take_string(v);
    //After passing v as a parameter we loose its ownership and can't access it again
    // println!("{}",v) // Uncomment this to see error


    // Lets check Number 
    let i =100;
    // Here we passed in i s as a parameter
    take_integer(i);
    // But even after passing it as a parameter we don't loose its ownership and can access it again
    print!("{}",i);

    // Getting ownership from a function 
    // here strng variable receives ownership of some_string from function gives ownership;
    let strng = gives_ownership();

    print!("{}",strng);


    // Passing in references to a function to preserve the ownership
    let some_string_idk = String::from("Hippopotomonstrosesquippedaliophobia");
    let some_string_length = take_reference(&some_string_idk);
    // here you can see we can still  access the string as we only passed in reference to function.
    print!("{} has length of {} same as your ... ",some_string_idk,some_string_length);

    // Here references by default are immutable so to make them mutable inside new function we do following
    let mut some_string_idk2 = String::from("Hippopotomonstrosesquippedaliophobia");
    let some_string_length2 = take_reference_mut(&mut some_string_idk2);
    // here you can see we can still  access the string as we only passed in reference to function.
    print!("{} has length of {} same as your ... ",some_string_idk2,some_string_length2);


    // -----------References Rule------------
// We cannot pass in mutable copies of a variable as a parameter multiple times
// We can pass in immutable copies of a variable as a parameter multiple    times
// References must be valid they should not be mutable that will be dropped 
}



fn take_string(v: String){
    println!("{}",v);
    //After function ends v is dropped
}

fn take_integer(i: u8) {
    print!("{}",i);
}

//This function returns a string 
fn gives_ownership() -> String {
    let some_string = String::from("Something something");
    some_string
}

fn take_reference(s: &str) -> usize {
    let length = s.len();
    length
}
fn take_reference_mut(s: &mut str) -> usize {
    let length = s.len();
    length
}

