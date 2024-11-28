use std::env;
use hello::get_greeting_message;

fn main() {
    let args: Vec<String> = env::args().collect();
    let greeting = get_greeting_message(&args);
    println!("{}", greeting);
}
