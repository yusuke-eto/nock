use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let files = File::open("src/poplular-names.txt").unwrap();
    let files = BufReader::new(files);
    println!("{}", files.lines().count());
}
