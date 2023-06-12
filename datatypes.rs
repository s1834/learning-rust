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
}