// Simple Function
fn square(num: i32) -> i32 {
    let result = num * num;
    result
}

// Call by Value (By Default)
fn add_one(num: i32) -> i32 {
    num + 1 // Increment the parameter by 1
}

// Call by Refrence
fn add_one_by_ref(num: &mut i32) {
    *num += 1 // Increment the value pointed by the refrence
}

fn main() {
    let number = 5;
    let square_result = square(number);
    println!("Square of {} is {}\n", number, square_result);

    let result = add_one(number);
    println!("Original Value: {}, Result: {}\n", number, result);

    let mut ref_number = 5;
    add_one_by_ref(&mut ref_number);
    println!("Modified Value: {}\n", ref_number);

}