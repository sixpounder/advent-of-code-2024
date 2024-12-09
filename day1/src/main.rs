use std::{collections::HashSet, env, fs, path::Path};

fn main() {
    // PART ONE
    let args: Vec<String> = env::args().collect();
    let (list1, list2) = read_input(args.get(1).unwrap_or(&String::from("input.txt")));
    println!("Total distance = {}", distance_of(&list1, &list2));

    // PART TWO
    let similarity_score = similarity_score(&list1, &list2);
    println!("Similarity score is {}", similarity_score);
}

fn similarity_score(list1: &[u32], list2: &[u32]) -> u32 {
    let left = filter_uniq(list1.to_owned());
    left.iter().fold(0u32, |acc, el| {
        let n_times = list2.iter().filter(|right| *right == el).count();
        acc + (el * n_times as u32)
    })
}

fn read_input<P: AsRef<Path>>(file: P) -> (Vec<u32>, Vec<u32>) {
    let raw_content = fs::read_to_string(file).expect("Could not read file");
    let mut list1: Vec<u32> = vec![];
    let mut list2: Vec<u32> = vec![];

    raw_content.lines().for_each(|line| {
        let chunks: Vec<&str> = line.split("   ").collect();
        list1.push(chunks.first().unwrap().parse::<u32>().unwrap());
        list2.push(chunks.get(1).unwrap().parse::<u32>().unwrap());
    });

    list1.sort();
    list2.sort();

    (list1, list2)
}

fn distance_of(list1: &[u32], list2: &[u32]) -> u32 {
    list1.iter().enumerate().fold(0u32, |acc, (idx, left)| {
        let right = list2.get(idx).unwrap();
        let distance = left.abs_diff(*right);
        acc + distance
    })
}

fn filter_uniq(vec: Vec<u32>) -> Vec<u32> {
    vec.into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}
