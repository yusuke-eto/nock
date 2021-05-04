use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() {
    let file = File::open("../nock10/src/poplular-names.txt").unwrap();
    let mut writer = BufWriter::new(File::create("src/format-popular-names2.txt").unwrap());
    let file = BufReader::new(file);
    for i in file.lines() {
        writer
            .write_all(format!("{}\n", i.unwrap().replace("\t", " ")).as_bytes())
            .unwrap();
    }
}
