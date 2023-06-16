fn main() {
    // Immutable (By Default)
    let numbers = [1, 2, 3, 4, 5]; 
    let slice = &numbers[1..4]; 
    println!("{:?}", slice);

    // Mutable Slices
    let mut numbers1 = [1, 2, 3, 4, 5]; 
    let slice1 = &mut numbers1[1..4]; 
    println!("{:?}", slice1);
    // Modifying the elements of the slice
    slice1[0] = 10; 
    slice1[1] = 20; 
    slice1[2] = 30; 
    println!("{:?}", slice1);

}