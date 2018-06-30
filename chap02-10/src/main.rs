use std::fs::File;

use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut f = File::open(&args[1]).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("Something is wrong");

    let mut number_of_lines: usize = 0;
    for (i, line) in contents.split('\n').enumerate(){
        number_of_lines = i;
    }
    println!("Number of lines: {:?}", number_of_lines);

}
