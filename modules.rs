// Define a module named 'my_module'
mod my_module {
    // Define a function inside the module 
    pub fn greet() {
        println!("Hello from my_module!"); 
    }
}

// Use the function from the module
fn main() { 
    my_module::greet();
}