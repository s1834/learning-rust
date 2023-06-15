fn main() {
    let number = 10; 
    if number < 0 {
        println!("Number is negative"); 
    } else if number > 0 {
        println!("Number is positive"); 
    } else {
        println!("Number is zero");
    }

    let fruit = "apple"; 
    match fruit {
        "apple" => println!("It's an apple"), 
        "banana" => println!("It's a banana"), 
        _ => println!("It's something else")
    }

}