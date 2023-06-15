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

    // ASSIGNMENT OPERATORS
    println!("ASSIGNMENT OPERATORS");
    let mut c = 5;
    c += 3; //equivalent to c = c + 3 
    println!("c: {}", c);
    c -= 2; //equivalent to c = c - 2 
    println!("c: {}", c);
    c *= 4; //equivalent to c = c * 4 
    println!("c: {}", c);
    c /= 2; //equivalent to c = c / 2 
    println!("c: {}", c);
    c %= 3; //equivalent to c = c % 3 
    println!("c: {}", c);
    println!("\n");

    // BITWISE OPERATORS
    println!("BITWISE OPERATORS");
    let d = 0b1010; // binary representation of 10
    let e = 0b1100; // binary representation of 12
    let and_result = d & e;
    let or_result = d | e;
    let xor_result = d ^ e;
    let not_d = !d;
    println!("AND Result: {:b}", and_result); //Output: 0b1000
    println!("OR Result: {:b}", or_result); // Output: 0b1110
    println!("XOR Result: {:b}", xor_result); // Output: 0b0110
    println!("NOT D: {:b}", not_d); // Output: -1011 (two's complement representation)
    println!("\n");

    // RANGE OPERATOR
    println!("RANGE OPERATOR");
    // Exclusive range
    for num in 1..5 {
        println!("{}", num); 
    }
    // Output: 1, 2, 3, 4
    // Inclusive range
    for num in 1..=5 { 
        println!("{}", num);
    }
    // Output: 1, 2, 3, 4, 5
    println!("\n");

    // DEREF OPERATOR ('*')
    println!("DEREF OPERATOR");
    let number = 42;
    let number_ref = &number;
    println!("{}", *number_ref);
    println!("\n");
    
    }