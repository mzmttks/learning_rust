use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut handle = File::open("../hightemp.txt").unwrap();
    let mut content = String::new();
    handle.read_to_string(&mut content).unwrap();

    let mut uniques = Vec::new();
    for line in content.trim().split("\n") {
        let first: &str = line.split("\t").collect::<Vec<&str>>()[0];
        if uniques.contains(&first) == false {
            uniques.push(first);
        }
    }
    println!("{:?}", uniques)
}
