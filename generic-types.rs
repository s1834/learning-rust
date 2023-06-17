use std::fmt::Display;

struct Pair <T> {
    first: T,
    second: T,
}

fn print_pair<T: Display>(pair: Pair<T>) {
    println!("First: {}, Second: {}", pair.first, pair.second);
}

fn print_and_return<T: Display>(value: T) -> T {
    println!("Value: {}", value);
    value
}

fn main () {
    let pair_of_integers = Pair{ first: 1, second: 2 };
    let pair_of_strings = Pair { first: "Hello", second: "World"};

    print_pair(pair_of_integers);
    print_pair(pair_of_strings);

    let value = print_and_return(42);
    println!("Returned value: {}", value);
    
}