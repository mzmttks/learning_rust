use std::fs::File;

use std::io::prelude::*;


fn main() {
    let in1 = "out1.txt";
    let in2 = "out2.txt";
    let out = "merged.txt";

    let mut ihandle1 = File::open(in1).expect("File error");
    let mut ihandle2 = File::open(in2).expect("File error");
    let mut ohandle = File::create(out).expect("File error");

    let mut content1 = String::new();
    let mut content2 = String::new();
    let mut content3 = String::new();

    ihandle1.read_to_string(&mut content1).expect("Error");
    ihandle2.read_to_string(&mut content2).expect("Error");


    for (line1, line2) in content1.trim().split("\n").zip(content2.trim().split("\n")) {
        content3 += &format!("{},{}\n", line1, line2);
    }
    ohandle.write(content3.as_bytes()).unwrap();
}
