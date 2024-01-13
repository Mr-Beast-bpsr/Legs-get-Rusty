
struct Point <T> {
    x: T,
    y: T,
 }

// here implementation will work for every data type
 impl  <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
 }

// Here Point impl will only work if returned value is f64
impl Point <f64>{
    fn y (&self) -> f64 {
            &self.y
    }
}

pub fn generic_in_methods() {

    let p1 = Point { x: 4, y: 5 };


}
