mod program;

use std::{env, fs, path::Path};

use program::Program;

fn main() {
    let args: Vec<String> = env::args().collect();
    let rc: i32 = Program::new(read_input(args.first().expect("Input file path not specified"))).execute();
    println!("Program output: {}", rc);
}

fn read_input<P: AsRef<Path>>(file: P) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
