mod panic_error;
mod match_error;
mod question_mark_error;

fn main() {
    println!("Hello, world!");
    panic_error::a(69);
    match_error::match_error_handling();
    question_mark_error::question_error_handling();
}
