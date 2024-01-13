// Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics
pub fn generic_types_function() {
    let number_list = vec![10, 1, 2, 03, 4, 5, 6];
    let largest = get_largest(number_list);
    println!("Largest number {}", largest);
    let char_list = vec!['a', 'b', 'f', 'd', 'e'];
    let largest = get_largest(char_list);
    println!("Largest character {}", largest);
}
//<T> here is our generic type keyword
//By adding T  we can easily generate results for both data types string and integer
fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
