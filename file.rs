use std::io::{self, Read, Write};
use std::fs::File;

fn main() -> io::Result<()> {
    let file = File::open("input.txt");
    let mut contents = String::new();

    file?.read_to_string(&mut contents)?;
    println!("File contents: {}", contents);

    let mut output_file = File::create("output.txt")?;
    output_file.write_all(contents.as_bytes())?;

    Ok(())

}