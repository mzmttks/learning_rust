use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut handle = File::open("../hightemp.txt").expect("Failed");
    let mut contents = String::new();
    handle.read_to_string(&mut contents).expect("Failed");

    let mut out1 = File::create("out1.txt").expect("Failed");
    let mut out2 = File::create("out2.txt").expect("Failed");

    for line in contents.trim().split("\n") {
        let values: Vec<&str> = line.split("\t").collect();
        let mut v1 = values[0].to_string();
        v1.push('\n');
        let mut v2 = values[1].to_string();
        v2.push('\n');
        out1.write(v1.as_bytes()).unwrap();
        out2.write(v2.as_bytes()).unwrap();
    }
}
