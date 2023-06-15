fn main() {
    let mut count = 0;
    
    // 'loop' loop
    println!("'loop' loop");
    loop {
        println!("Current count: {}", count); 
        count += 1;
        if count == 5 {
            break; 
        }
    }
    println!("\n");

    count = 0;

    // 'while' loop
    println!("'while' loop");
    while count < 5 {
        println!("Current count: {}", count);
        count += 1;
    }
    println!("\n");
    
}