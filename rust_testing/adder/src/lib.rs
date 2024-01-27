
#[derive(Debug)]
struct Rectangle {
    height: u64,
    width: u64,
}
//--------------------------Implementing Methods------------------------------------
// Implementations can have functions and methods implecate with our structs

impl Rectangle {
    fn area_method(&self) -> u64 {
        self.height * self.width
    }
    fn can_hold_method(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

// AssociatedFunction
// These functions are similar to methods but they just don't take self as an parameter and are assigned differently

impl Rectangle {
    fn square(size: u64) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

pub fn add_two(a:i32) -> i32{
    internal_adder(a,68)
} 
fn internal_adder(a:i32,b:i32) -> i32{
    a+b
}
#[cfg(test)]
mod test{
// Using super to import internal functions from the file.
    use super::*;

    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle {
            width:8,
            height:7,
        };
        let smaller = Rectangle {
            width:4,
            height:6,
        };


        assert!(larger.can_hold_method(&smaller));
        // assert!(smaller.can_hold_method(&larger));
        // Assert checks for a conditions to be true 
        // Assert_eq checks whether the two conditions are equal
        // Assert_ne does the opposite of assert_eq
    }

}

