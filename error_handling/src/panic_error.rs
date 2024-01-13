// Panic in rust is used to throw errors when a condition is met of its called..

pub fn a(x : u32 ){
    b(x)
}

fn b(x : u32) {
    c(x )
}
 fn c(x : u32){
    d(x);
}
fn d(x :u32) {
    if x != 69 {
        panic!("You can't 69 here!");
    } 
}