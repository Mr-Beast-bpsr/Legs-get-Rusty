// Panic in rust is used to throw errors when a condition is met of its called..

pub fn a(){
    b()
}

fn b() {
    c( )
}
 fn c(){
    d(69);
}
fn d(n :u32) {
    if n== 69 {
        panic!("You can't 69 here!");
    } 
}