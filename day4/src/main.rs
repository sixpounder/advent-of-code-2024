mod matrix;

use std::{env, fs, path::Path};
use matrix::*;

use itertools::Itertools;
use nalgebra::{Dyn, OMatrix};

static MATCH_SEQUENCES: [&str; 2] = ["XMAS", "SAMX"];
static MATCH_SEQUENCES_CROSS: [&str; 2] = ["MAS", "SAM"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input: String = read_input(args.get(1).unwrap_or(&String::from("input.txt")));
    let matrix = TraversableMatrix::from(raw_input);
    println!("Word count: {}", inspect(&matrix));
}

fn read_input<P: AsRef<Path>>(file: P) -> String {
    fs::read_to_string(file).expect("Could not read file")
}

fn inspect(matrix: &TraversableMatrix<char>) -> usize {
    let mut match_counter: usize = 0;

    // Scan rows
    for row in matrix.row_iter() {
        for window in row.iter().tuple_windows::<(_, _, _, _)>() {
            scan_window(window, &mut match_counter);
        }
    }

    // Scan columns
    for col in matrix.column_iter() {
        for window in col.iter().tuple_windows::<(_, _, _, _)>() {
            scan_window(window, &mut match_counter);
        }
    }

    // Scan diagonally
    for diag in matrix.diagonal_iter() {
        for window in diag.sequence().iter().tuple_windows::<(_, _, _, _)>() {
            scan_window(window, &mut match_counter);
        }
    }

    match_counter
}

/// Function to compute the "cross" of a given set of diagonal coordinates
fn compute_cross(coordinates: &[(usize, usize)]) -> Vec<(usize, usize)> {
    // Find the min and max y-coordinates
    let min_y = coordinates.iter().map(|&(_, y)| y).min().unwrap();
    let max_y = coordinates.iter().map(|&(_, y)| y).max().unwrap();

    // Compute the middle y-coordinate (center of the diagonal)
    let mid_y = (min_y + max_y) / 2;

    // Compute the cross coordinates
    coordinates
        .iter()
        .map(|&(x, y)| (x, 2 * mid_y - y)) // Reflect y across mid_y
        .collect()
}

fn scan_window(window: (&char, &char, &char, &char), match_counter: &mut usize) {
    for seq in MATCH_SEQUENCES {
        if join_tuple_xmas(window).as_str().eq(seq) {
            *match_counter += 1;
        }
    }
}

fn join_tuple_xmas<'a>(tuple: (&'a char, &'a char, &'a char, &'a char)) -> String {
    vec![tuple.0, tuple.1, tuple.2, tuple.3].iter().join("")
}
