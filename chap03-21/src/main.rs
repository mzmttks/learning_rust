use std::fs::File;
use std::io::prelude::*;

fn main() {
    let filename = "../chap03-20/england.txt";
    let mut handle = File::open(&filename).unwrap();
    let mut content = String::new();
    handle.read_to_string(&mut content).unwrap();

    for line in content.trim().split("\n") {
        if line.contains("Category") {
            println!("{}", line);
        }
    }
}
