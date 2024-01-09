struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

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
fn main() {
    println!("Structs!");

    // Creating a user data using struct
    let mut user1 = User {
        email: String::from("bpsr.rana@gmail.com"),
        username: String::from("bpsr.rana"),
        active: true,
        sign_in_count: 10,
    };

    // Accessing the user data from struct
    let name = user1.username;

    // Updating user data one by one
    user1.username = String::from("bpsr.rana_admin");

    println!("{}", name);
    // Using functions to create a new user
    let user2 = build_user(String::from("Blast_Op@gmail.com"), String::from("blastOp"));
    println!("{}", user2.username);

    // Tuple struct ;

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Using tuple struct to make things shorter
    let rect = (30, 80);
    let area_calc = area(rect);
    print!("{}", area_calc);

    // Using implementation methods to shorten the code
    let rect = Rectangle {
        width: 66,
        height: 99,
    };
    let other_rect = Rectangle {
        width: 46,
        height: 34,
    };
    let can_it = rect.can_hold_method(&other_rect);
    println!("{}", can_it);
    println!("rect {:#?}", rect);
    print!("Rectangle Area: {}", rect.area_method());

    // Initializing Associated Methods

    let square = Rectangle::square(32);
    println!("square {:#?}", square)
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: false,
        sign_in_count: 0,
    }
}

// Using tuple to give meaning to fn parameters

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
