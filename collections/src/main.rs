use std::collections::HashMap;

fn main() {
 // Vectors are dynamic arrays ALSO known as 'Vec' . It is  a collection type that allows you to store and munipulate a variable number of elements in a contiguous memory block.
    // Vectors are stored on heap
    // How to initialize new vectors with no data
    let mut v = Vec::new();
     v.push(2);
    v.push(1);
    v.push(0);
    // initialize new vector with data
    let mut v2 = vec![3,2,1,0];

    // Access data from vector
    
    let third = &v2[2];
     //  ^This method of accessing the data from the vector can cause runtime errors as even if we add wrong index it won't give errors;
// To avoid run time issues we use get method
    match v2.get(2){
        Some(third) => print!("{}", third),
        None => print!("No Element"),
    }

    // using vectors inside  for-loop

    for i in &mut v2{
        println!("{}", i);
    }

    // Using vectors  to store enum values 

    // Creating an enum
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

// Creating vector with enum values

    let row = vec![
        // Here we can use enum to give each cell a unique value 
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(69.69),
        SpreadsheetCell::Text(String::from("Yes it is what it is!"))

    ];


    // Strings are stored as a collection of UTF-8 encoded bytes
    let mut s = String::from("Hello b");
    s.push_str("itch");
    s.push('!');
    println!("{}",s);

    // We can also use + to append strings.
    let s1 = String::from("Hello   ");
    let s2 = String::from("Who's there?");
    let s3 = s1+&s2;

    // We cannot call s1 after we used it to append strings 
    // println!("{}", s1);

    // We can also use format macro methods as this does not take the ownership of variable,
    
    let s4 = format!("{}{}", s3, s2);
    print!("{}", s4);

    let hello = String::from("भो सद इके?");
    for b in hello.bytes(){
        println!("{}", b);
    }
    for b in hello.chars(){
        println!("{}", b);
    }

use unicode_segmentation::UnicodeSegmentation;

    for b in hello.graphemes(true){
        println!("{}", b);
    }
    let mut blue = String::from("Blue");
    let yellow = String::from("Yellow");
    // Hashmaps allows us to store keys values in memory
    // An empty hashmap will always give error messages
  let mut scores = HashMap::new();

scores.insert(&blue,10);
scores.insert(&yellow,40);

let socre = scores.get(&blue);
// This will replace the original value of blue
scores.insert(&blue, 30);

    // or_insert  
// This method checks whether value is already in the dictionary if not then inserts
    scores.entry(&String::from("Yellow")).or_insert(90);



    //  Update a value in a hashmap based on a old value
 let text =  "hello world fucked up world hello !";
 let mut  map = HashMap::new();
 for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    // count represents the number or the value at the given word or key

    *count += 1;
    println!("{}",count)
 }
println!("{:?}", map);
}
