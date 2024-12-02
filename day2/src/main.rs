use report::ReportCollection;

mod report;

fn main() {
    let collection = ReportCollection::from("input.txt");
    println!("Number of safe reports: {}", collection.count_safe());
    println!(
        "Number of safe reports (with dampener): {}",
        collection.count_safe_dampened()
    );
}
