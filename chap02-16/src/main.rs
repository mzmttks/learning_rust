use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let n: usize = args[1].parse().unwrap();

    let mut f = File::open(&args[2]).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let nlines = content.split("\n").collect::<Vec<&str>>().len();

    let mut assigned_lines: Vec<usize> = Vec::new();
    for index in 0..n {
        assigned_lines.push((nlines + index) / n);
    }

    let lines:Vec<&str> = content.split("\n").collect();
    let mut current_lineno: usize = 0;
    for (index, assigned_line) in assigned_lines.iter().enumerate() {
        let mut f = File::create(format!("split-{}.txt", index)).unwrap();
        for index in current_lineno..(current_lineno + assigned_line) {
            let mut outstr = lines[index].trim().to_string();
            outstr.push('\n');
            f.write(outstr.as_bytes()).unwrap();
        }
        current_lineno += assigned_line;
    }
}
