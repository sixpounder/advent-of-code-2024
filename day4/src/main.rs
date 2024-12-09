use std::env;

use common::{matrix::TraversableMatrix, read_input};
use itertools::Itertools;

static MATCH_SEQUENCES: [&str; 2] = ["XMAS", "SAMX"];
static MATCH_SEQUENCES_CROSS: [&str; 2] = ["MAS", "SAM"];

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input: String = read_input(args.get(1).unwrap_or(&String::from("day4/input.txt")));
    let matrix = TraversableMatrix::from(raw_input);
    println!("Word count: {}", inspect(&matrix));
    println!(
        "Word count on cross patterns: {}",
        inspect_cross_patterns(&matrix)
    );
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
        for window in diag.coord_sequence().iter().tuple_windows::<(_, _, _, _)>() {
            scan_window(
                (window.0 .1, window.1 .1, window.2 .1, window.3 .1),
                &mut match_counter,
            );
        }
    }

    match_counter
}

fn inspect_cross_patterns(matrix: &TraversableMatrix<char>) -> usize {
    let mut match_counter: usize = 0;

    for diag in matrix.left_diagonal_iter() {
        for window in diag.coord_sequence().iter().tuple_windows::<(_, _, _)>() {
            let window_slice = matrix.slice(window.0 .0, window.2 .0);
            for seq in MATCH_SEQUENCES_CROSS {
                if vec![window.0 .1, window.1 .1, window.2 .1]
                    .into_iter()
                    .join("")
                    .as_str()
                    .eq(seq)
                {
                    if let Some(cross_sequence) = window_slice.cross_slice() {
                        if MATCH_SEQUENCES_CROSS
                            .contains(&cross_sequence.sequence_content().as_str())
                        {
                            match_counter += 1;
                        }
                    }
                }
            }
        }
    }

    match_counter
}

fn scan_window(window: (&char, &char, &char, &char), match_counter: &mut usize) {
    for seq in MATCH_SEQUENCES {
        if join_tuple_xmas(window).as_str().eq(seq) {
            *match_counter += 1;
        }
    }
}

fn join_tuple_xmas<'a>(tuple: (&'a char, &'a char, &'a char, &'a char)) -> String {
    [tuple.0, tuple.1, tuple.2, tuple.3].iter().join("")
}
