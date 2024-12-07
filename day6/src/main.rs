use rayon::prelude::*;
use std::{
    collections::HashSet,
    env, fs,
    path::Path,
    sync::{Arc, Mutex},
};

use map::{Cell, Direction, Map};

mod map;

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_input: String = read_input(args.get(1).unwrap_or(&String::from("day6/input.txt")));
    let mut current_state = Map::from(raw_input.as_str());
    while current_state.next() {
        // ...
    }

    println!(
        "Final number of visited cells: {}",
        current_state
            .iter()
            .filter(|cell| **cell == Cell::Visited)
            .count()
    );

    // PART 2
    let n_loops: Arc<Mutex<usize>> = Arc::default();
    let initial_state = Map::from(raw_input.as_str());

    (0..initial_state.len())
        .par_bridge()
        .for_each(|test_index| {
            let mut current_state = initial_state.clone();

            // Only test this cell if it is free
            if *current_state
                .get_index(test_index)
                .expect("Out of bounds index")
                == Cell::Free
            {
                current_state.set(test_index, Cell::Obstacle);
                // current_state.pretty_print();
                // println!("Testing moves");
                let mut guard_path_buffer: HashSet<(usize, Direction)> = HashSet::new();
                'walk_guard: while current_state.next() {
                    if let Some((current_guard_index, cell)) = current_state.get_guard() {
                        let guard = cell.as_guard();
                        if let Some(guard) = guard {
                            if guard_path_buffer.contains(&(current_guard_index, guard.direction().clone())) {
                                *n_loops.lock().unwrap() += 1;
                                break 'walk_guard;
                            } else {
                                guard_path_buffer.insert((current_guard_index, guard.direction().clone()));
                            }
                        }
                    }
                }
            }
        });

    println!(
        "Number of loop configurations detected: {}",
        n_loops.lock().unwrap()
    );
}

fn read_input<P: AsRef<Path>>(file: P) -> String {
    fs::read_to_string(file).expect("Could not read file")
}
