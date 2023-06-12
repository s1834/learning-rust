const APP_NAME: &str = "MyApp";

fn main() {
    // INTEGER
    println!("INTEGERS");
    let result = 10; //i32 by default
    let age: u32 = 20;
    let sum: i32 = 5 - 15;
    let mark: isize = 10;
    let count: usize = 30;
    println!("Result value: {}", result);
    println!("Sum is {} and age is {}", sum, age);
    println!("Mark is {} and count is {}", mark, count);
    println!("\n");

    // FLOATING-POINT
    println!("FLOATING-POINT");
    let result1 = 10.00; // f64 by default
    let interest: f32 = 8.35;
    let cost: f64 = 15000.600; // double precision
    println!("Result value is {}", result1);
    println!("interest is {}", interest);
    println!("cost is {}", cost);
    println!("\n");

    // BOOLEANS
    println!("BOOLEANS");
    let is_fun: bool = true;
    println!("Is Rust Programming Fun? {}", is_fun);
    println!("\n");

    // CHARACTERS
    println!("CHARACTERS");
    let special_character = '@'; // default
    let alphabet: char = 'A';
    let emoji: char = '😁';
    println!("special character is {}", special_character);
    println!("alphabet is {}", alphabet);
    println!("emoji is {}", emoji);
    println!("\n");

    // VARIABLES
    println!("VARIABLES");
    let message = "Hello, World!"; // Inferred type: &str
    let age: u32 = 30; // Explicit type annotation: unsigned 32-bit integer
    println!("{}", message);
    println!("Age: {}", age);
    let mut count = 0; // Mutable variable
    println!("Count: {}", count);
    count = 5; // Reassigning value to the mutable variable
    println!("Updated count: {}", count);
    let count1 = 10;
    println!("Count1: {}", count1);
    let count1 = count1 * 2; // Shadowing the previous 'count'
    println!("Updated count1: {}", count1);
    println!("\n");

    // CONSTANTS
    println!("Constants");
    const PI: f32 = 3.14;
    const MAX_ATTEMPTS: u8 = 5;
    println!("Value of PI: {}", PI);
    println!("Maximum attemts: {}", MAX_ATTEMPTS);
    println!("Welcome to {}", APP_NAME);
    println!("\n");
    
}