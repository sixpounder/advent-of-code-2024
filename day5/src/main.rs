mod safety_manual;

use std::{env, fs, path::Path};

use safety_manual::SafetyManualUpdates;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input: String = read_input(args.get(1).unwrap_or(&String::from("input.txt")));    
    let safety_manual = SafetyManualUpdates::from(raw_input);
    println!("Safe updates median sum: {}", safety_manual.correct_updates().iter().map(|i| i.median()).sum::<i32>());
    println!("Corrected unsafe updates median sum: {}", safety_manual.corrected_updates().iter().map(|i| i.median()).sum::<i32>());
}

fn read_input<P: AsRef<Path>>(file: P) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
