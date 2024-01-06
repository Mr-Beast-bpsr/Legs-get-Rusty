fn main() {
    // Using let for mutable variables
    let mut x = 5;
    println!("The Value of x is: {}", x);
    x = 6;
    println!("The Value of x is: {}", x);
    // Show casing const variables " In const variables we use ALL_CAPS" : const variables require data type assignment
    const SUBS_COUNT: u32 = 100_000;

    println!("Subs count: {}", SUBS_COUNT);

    // What is shadowing ?
    //  Shadowing in rust is when we re assign a let variable to a different data type. It preserves immutablity

    let shadow = 5;
    println!("Shadowing: {}", shadow);
    // Variable here is not mutable so we can't just assign it to a different data type so we just re initialize it  as its a let variable
    let shadow = ("shadow").to_string();
    println!("Shadowing: {}", shadow);
    primitive_data_types();
    compound_data_types();
    let sum = return_type_fn(99, 62);
    println!("{}", sum);
    // Expains control flow statements 
    let control_majnu = control_flow_fn(19,199);
    println!("{}", control_majnu);
}
// Convetion is always snake case
fn primitive_data_types() {
    // Integers
    // Integers can be of 8-bi 16-bit 32 bit 64 bit 128 bit or arch(architecture based)

    let a = 98_222; // Decimal
    let b = 0xfff; // Hex
    let c = 0o77; // Octal
    let d = 0b1111_0000; // Binary
    let e = b'A'; // Byte;
    let f: u8 = 255; // Max in u8

    // Floating-point numbers
    let g: f64 = 2.0;
    // Booleans

    let h: bool = true;
    let i: bool = false;
    // Character

    let j: char = 'z';
    let k: char = 'Z';

    println!(
        "a: {}, b: {}, c: {}, d: {}, e: {}, f: {}, g: {}, h: {}, i: {}, j: {}, k: {}",
        a, b, c, d, e, f, g, h, i, j, k
    );
}

fn compound_data_types() {
    // A tuple type consists of multiple elements inside a  ()
    let tup = ("This is a string inside tuple", 100_000);
    // To extract values from a tuple
    let (string_data, number_data) = tup;
    // To extract values from tuple by place.
    let string_by_count = tup.0;

    println!(
        "Tuple: {} : {} : {}",
        string_data, number_data, string_by_count
    );

    // Array data type
    let error_codes = [200, 404, 500];

    // Assign values by index
    let not_found = error_codes[1];

    // Assign values to array indexes

    let byte = [0; 8];
    let unbyte = [1; 8];
}

fn return_type_fn(x: i32, y: i32) -> i32 {
    // Print line statements are statements as they don't have a return value
    println!("x: {}", x);
    println!("y {}", y);
    // sum is an expression
    let sum = x + y;

    return sum;
}

fn control_flow_fn(x: i32, y: i32) -> bool {
   
   
    // Returners loop
    // Loop consists of of four parts  1 counter 2 loop 3 break 4  break return value that is assigned to result
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
        println!("{}", counter);
    };
    println!("Loop value: {}", result);

// While loop 
  while counter >1 {
    println!("{}", counter);
    counter -= 1;
  }
    println!("While END: {}", counter);

// For loop 
    let a = [10,20,40,80,100,200,300, 400, 500 ];
    for elements in a.iter() {
        counter += elements;
        println!("Elemental value is {}", counter);
    }

    // Control flow / Conditional statements
    if counter> x + y {
        println!("Counter is {}", counter);
    } else if counter  < y + x {
        println!("Counter is {}", counter);
    } else {
        return false;
    }

    // IF ELSE INSIDE A VARIABLE
    let conditional = if x > y {  true } else {  false };
    return  conditional;
}
