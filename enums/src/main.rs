enum IpAddrKind {
    V4(String),
    V6(String),
}

// The advantage of enum is that we can have multiple types with their own data , its like having structs inside structs
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32, i32),
}

// can define impl for enums as well;
impl Message {
    fn quit(&self) {
        println!("Enum impl here");
    }
}
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(String::from("127.0.0.1:8080"));

    // Here match expression will match the inputs with given values

    let five = Some(6);
    let seven = plus_one(five);
    let none = plus_one(None);
}

fn route(ip_kind: IpAddrKind) {}

// Match expression is exuastive so we have to give value for each route
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(x) => Some(x + 1),
    };

    // if let  allows use to define let and conditions on one line
    if let Some(3) = x {
        print!("theree");
        x;
    };
    x
}
