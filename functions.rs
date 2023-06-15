fn square(num: i32) -> i32 {
    let result = num * num;
    result
}

fn main() {
    let number = 5;
    let square_result = square(number);
    println!("Square of {} is {}\n", number, square_result);
}