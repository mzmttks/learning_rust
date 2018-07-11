extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    let filename = "../chap03-20/england.txt";
    let mut handle = File::open(filename).unwrap();
    let mut content = String::new();
    handle.read_to_string(&mut content).unwrap();

    let pattern = Regex::new(r"\[\[Category:(.*)\.*\]\]").unwrap();

    for line in content.trim().split("\n") {
        if line.contains("Category") {
            // println!("{}", line);
            let matched = pattern.captures(line).unwrap();
            let mut string = &matched[1];

            if string.contains("|") {
                for (i, s) in string.split("|").enumerate() {
                    if i == 0 {
                        println!("{}", s);
                    }
                }
            } else {
                println!("{}", string);
            }
        }
    }
}
