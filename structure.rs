// Structure Defination
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    is_active: bool,
}

fn main () {
    // Structure Instanciation
    let person = Person {
        name: String::from("John"), 
        age: 25,
        is_active: true,
    };
    println!("{:?}",person);

    // Strcture Field Access
    println!("Name: {}", person.name); 
    println!("Age: {}", person.age);
    
}