use crate::mt940::*;
use std::fs;
use std::time::Instant;

mod block;
mod mt940;
mod tag;
mod utils;

fn main() {
    let content = &fs::read_to_string("./test_messages/basic_test_3.txt").expect("Unable to read file");

    let start = Instant::now();
    let swift = MT940::new(content);
    let duration = start.elapsed();

    // eprintln!("swift = {:#?}", swift.data.text.tag_64);
    eprintln!("swift = {:#?}", swift);
    eprintln!("Time elapsed: {:?}", duration);
}
