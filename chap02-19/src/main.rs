use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut handle = File::open(&args[1]).expect("ERROR");
    let mut contents = String::new();
    handle.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.trim().split("\n").collect();

    let mut wordcount:HashMap<&str, u8> = HashMap::new();
    for line in lines {
        for (i, city) in line.split("\t").enumerate() {
            if i == 0 {
                if wordcount.contains_key(&city) {
                    let mut num = wordcount.get_mut(&city).unwrap();
                    *num += 1;
                } else {
                    wordcount.insert(&city, 1);
                }
            }
        }
    }
    println!("{:?}", wordcount);
}
