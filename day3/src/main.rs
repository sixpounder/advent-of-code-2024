mod program;

use std::{fs, path::Path};

use program::Program;

fn main() {
    let rc: i32 = Program::new(read_input("input.txt")).execute();
    println!("Program output: {}", rc);
}

fn read_input<P: AsRef<Path>>(file: P) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
