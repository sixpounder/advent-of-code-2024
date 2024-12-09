pub mod matrix;

use std::{fs, path::Path};

pub fn read_input<P: AsRef<Path>>(file: P) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
