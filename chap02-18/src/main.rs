use std::fs::File;
use std::io::prelude::*;

fn pick_temp(line: &str) -> f32 {
    let splits: Vec<&str> = line.split("\t").collect();
    splits[2].parse().unwrap()
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let mut handle = File::open(&args[1]).expect("ERROR");
    let mut contents = String::new();
    handle.read_to_string(&mut contents).unwrap();
    let lines: Vec<&str> = contents.trim().split("\n").collect();

    let mut numbers: Vec<f32> = Vec::new();
    for line in &lines {
        let num = pick_temp(line);
        if !numbers.contains(&num) {
            numbers.push(num);
        }
    }
    numbers.sort_by(|a, b| a.partial_cmp(b).unwrap());

    for number in numbers {
        for line in &lines {
            if pick_temp(line) == number {
                println!("{}", line);
            }
        }
    }
}
