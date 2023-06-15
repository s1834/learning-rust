fn square(num: i32) -> i32 {
    let result = num * num;
    result
}

fn add_one(num: i32) -> i32 {
    num + 1 // Increment the parameter by 1
}

fn main() {
    let number = 5;
    let square_result = square(number);
    println!("Square of {} is {}\n", number, square_result);

    let result = add_one(number);
    println!("Original Value: {}, Result: {}\n", number, result);
}