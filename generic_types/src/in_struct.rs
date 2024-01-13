// Every programming language has tools for effectively handling the duplication of concepts. In Rust, one such tool is generics

// We can also have multiple generics in the sane struct
struct Point<T> {
    x: T,
    y: U,
}

enum Options<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// having multiple generics in the sane struct allows us to have multiple data types
pub fn generic_types_struct() {
    let p1 = Point { x: 4, y: 5 };
    let p2 = Point { x: 3.3, y: 4.5 };
    let p3 = Point { x: "x", y: 4 };
}
