use std::fs;

mod blocks;
mod mt940;

fn main() {
    let filename = "./tests/basic_test.txt";
    println!("In file {}", filename);

    let content: &str =
        &fs::read_to_string(filename).expect("Something went wrong reading the file");
    let mut swift = mt940::Mt940::new(content);

    swift.parse();
}
