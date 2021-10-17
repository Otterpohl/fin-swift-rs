use std::fs;
use std::time::Instant;

mod blocks;
mod mt940;
mod tags;

fn main() {
    let filename = "./tests/basic_test_3.txt";
    let content: &str = &fs::read_to_string(filename).expect("Unable to read file");
    let start = Instant::now();
    let mut swift = mt940::Mt940::new(content);
    swift.parse();
    let duration = start.elapsed();

    println!("output = {:#?}", swift);
    println!("Time elapsed: {:?}", duration);
}
