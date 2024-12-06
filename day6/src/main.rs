use std::{env, fs, path::Path};

use map::{Cell, Map};

mod map;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input: String = read_input(args.get(1).unwrap_or(&String::from("day6/input.txt")));
    let mut current_state = Map::from(raw_input.as_str());
    while let Some(next_state) = current_state.into_next() {
        current_state = next_state;
        //current_state.pretty_print();
        //println!(" ********************************* ");
    }

    println!("Final number of visited cells: {}", current_state.iter().filter(|cell| **cell == Cell::Visited).count())
    
}

fn read_input<P: AsRef<Path>>(file: P) -> String {
    fs::read_to_string(file).expect("Could not read file")
}