use crate::mt940::MT940;
use std::time::Instant;

mod block;
mod mt940;
mod tag;
mod utils;

fn main() {
    let content = include_str!("./test_messages/basic_test_4.txt");

    let start = Instant::now();
    let swift = MT940::new(content);
    let duration = start.elapsed();

    //eprintln!("swift = {:#?}", swift.user);
    eprintln!("swift = {:#?}", swift);
    eprintln!("Time elapsed: {:?}", duration);
}
