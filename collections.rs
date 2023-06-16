fn main() {
    // VECTORS
    println!("VECTORS");
    let mut numbers: Vec<i32> = Vec::new(); // Create an empty vector 
    numbers.push(1); // Add an element to the vector 
    numbers.push(2);
    numbers.push(3);

    println!("{:?}", numbers); // Print the vector
    
    // Access elements of the vector
    let first_element = numbers[0]; 
    println!("First element: {}", first_element);
    
    // Iterate over the vector
    for number in &numbers { 
        println!("{}", number);
    } 
    println!("\n");

    // ARRAYS
    println!("ARRAYS");
    let numbers1: [i32; 3] = [1, 2, 3]; // Create an array 
    println!("{:?}", numbers1); // Print the array
    // Access elements of the array
    let first_element = numbers1[0];
    println!("First element: {}", first_element);
    // Iterate over the array
    for number in &numbers1 {
        println!("{}", number); 
    }
    println!("\n");
    
}