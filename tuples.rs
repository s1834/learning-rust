fn get_person1() -> (String, u32, bool) {
    let name = String::from("Alice");
    let age = 30;
    let is_active = false;
    (name, age, is_active)
}

fn main() {
    // Simple Tuple Example
    let person = ("John", 25, true); 
    println!("{:?}", person);

    // Tuple access
    println!("Name: {}", person.0);
    println!("Age: {}", person.1);
    
    // Tuple Deconstructing
    let (name, age, is_active) = person; 
    println!("Name: {}", name); 
    println!("Age: {}", age); 
    println!("Is Active: {}", is_active);

    // Returning Tuples from Functions
    let person1 = get_person1();
    println!("Name: {}", person1.0);
    println!("Age: {}", person1.1);
    println!("Is Active: {}", person1.2);



}