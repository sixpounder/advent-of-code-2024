use std::env;

use report::ReportCollection;

mod report;

fn main() {
    let args: Vec<String> = env::args().collect();
    let collection = ReportCollection::from(args.get(1).unwrap_or(&String::from("input.txt")));
    println!("Number of safe reports: {}", collection.count_safe());
    println!(
        "Number of safe reports (with dampener): {}",
        collection.count_safe_dampened()
    );
}
