mod calculator;

use std::env;

use calculator::{CalibrationEquation, Operation};
use common::read_input;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input: String = read_input(args.get(1).unwrap_or(&String::from("day7/input.txt")));
    let equations: Vec<CalibrationEquation> = raw_input
        .lines()
        .map(|line| CalibrationEquation::from(line))
        .collect();
    let result: u64 = equations
        .par_iter()
        .map(|e| {
            // println!("Trying to solve {:?}", e);
            if e.try_solve(&[Operation::Sum, Operation::Mul]).is_some() {
                e.result()
            } else {
                0
            }
        })
        .sum();
    println!("Sum of solvable equation results: {}", result);

    let equations: Vec<CalibrationEquation> = raw_input
        .lines()
        .map(|line| CalibrationEquation::from(line))
        .collect();
    let result: u64 = equations
        .par_iter()
        .map(|e| {
            // println!("Trying to solve {:?}", e);
            if e.try_solve(&[Operation::Sum, Operation::Mul, Operation::Concat])
                .is_some()
            {
                e.result()
            } else {
                0
            }
        })
        .sum();
    println!(
        "Sum of solvable equation results (with || operator supported): {}",
        result
    );
}
