fn main() { 
    // ARITHMETIC OPERATORS
    println!("ARITHMETIC OPERATORS");
    let a = 5; 
    let b = 3;
    let sum = a + b;
    let difference = a - b; 
    let product = a * b;
    let quotient = a / b; 
    let remainder = a % b; 
    let negation = -a;
    println!("Sum: {}", sum); 
    println!("Difference: {}", difference);
    println!("Product: {}", product); 
    println!("Quotient: {}", quotient); 
    println!("Remainder: {}", remainder); 
    println!("Negation: {}", negation);
    println!("\n");

    // COMPARISON OPERATORS
    println!("COMPARISON OPERATORS");
    let equals = a == b;
    let not_equals = a != b;
    let greater_than = a > b;
    let greater_than_or_equal = a >= b; let less_than = a < b;
    let less_than_or_equal = a <= b;
    println!("Equals: {}", equals);
    println!("Not Equals: {}", not_equals);
    println!("Greater Than: {}", greater_than);
    println!("Greater Than or Equal: {}", greater_than_or_equal); 
    println!("Less Than: {}", less_than);
    println!("Less Than or Equal: {}", less_than_or_equal);
    println!("\n");

    // LOGICAL OPERATORS
    println!("LOGICAL OPERATORS");
    let a = true;
    let b = false;
    let and_result = a && b;
    let or_result = a || b;
    let not_a = !a;
    println!("AND Result: {}", and_result);
    println!("OR Result: {}", or_result);
    println!("NOT A: {}", not_a);
    println!("\n");

    }