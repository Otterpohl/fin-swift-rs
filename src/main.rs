use std::fs;
use std::time::Instant;

mod blocks;
mod message;
mod mt940;
mod tags;

fn main() {
    let filename = "./tests/test_messages/basic_test_3.txt";
    let content = &fs::read_to_string(filename).expect("Unable to read file");
    
    let start = Instant::now();
    let swift = message::Message::new(content);
    let duration = start.elapsed();

    eprintln!("swift = {:#?}", swift);
    eprintln!("Time elapsed: {:?}", duration);
}
