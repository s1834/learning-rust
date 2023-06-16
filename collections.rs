use std::collections::HashMap;


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

    // STRINGS
    println!("STRINGS");
    let mut hello = String::from("Hello"); // Create a String 
    hello.push_str(", world!"); // Append to the string
    println!("{}", hello); // Print the string
    // Iterate over the characters of the string
    for c in hello.chars() { 
        println!("{}", c);
    }
    println!("\n");

    // HASH MAPS
    println!("HASH MAPS");
    let mut scores = HashMap::new(); // Create an empty hash map 
    scores.insert(String::from("Alice"), 100); // Insert a key-value pair 
    scores.insert(String::from("Bob"), 90);
    println!("{:?}", scores); // Print the hash map
    
    // Access a value using the key
    let alice_score = scores.get("Alice");
    println!("Alice's score: {:?}", alice_score);
    
    // Iterate over the key-value pairs
    for (name, score) in &scores {
        println!("{}: {}", name, score);
    }
    println!("\n");

}