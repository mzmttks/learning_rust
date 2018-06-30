use std::fs::File;

use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nlines: usize = args[1].parse().unwrap();
    
    let mut handle = File::open(&args[2]).unwrap();
    let mut contents = String::new();
    handle.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.trim().split("\n").collect();
    for index in lines.len()-nlines .. lines.len() {
        println!("{}", &lines[index]);
    }
}
