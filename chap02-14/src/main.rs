use std::fs::File;
use std::io::prelude::*;



fn main() {
    let args: Vec<String> = std::env::args().collect();
    let nlines: usize =  args[1].parse().unwrap();

    let mut handle = File::open(&args[2]).expect("Error");
    let mut contents = String::new();
    handle.read_to_string(&mut contents).unwrap();

    for (iline, line) in contents.split("\n").enumerate() {
        if iline+1 > nlines {
            break
        }
        println!("{}", line);
    }
}
