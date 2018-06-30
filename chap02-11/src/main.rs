use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut contents = String::new();
    let mut handle = File::open("../hightemp.txt").expect("File not found.");
    handle.read_to_string(&mut contents).expect("Reading failed");

    println!("{}", contents.replace('\t', " "));
}
