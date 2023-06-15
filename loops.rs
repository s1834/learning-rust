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

}