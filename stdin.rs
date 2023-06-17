use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    println!("Enter your name: ");
    stdin.lock().read_line(&mut buffer).expect("Failed to read line");

    println!("Hello, {}!", buffer);

}