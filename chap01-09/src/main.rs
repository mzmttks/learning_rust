extern crate rand;

use rand::{thread_rng, Rng};

fn shuffle(input: &str) -> String {
    let mut output = Vec::new();
    let mut rng = thread_rng();
    
    let mut index = Vec::new();
    for i in 1..input.len()-1 {
        index.push(i);
    }
    rng.shuffle(&mut index);

    output.push(&input[0..1]);
    
    for i in index {
        output.push(&input[i..i+1]);
    }

    output.push(&input[input.len()-1..input.len()]);    
    output.join("")
}

fn main() {
    let string1 = "I couldn't believe that I could actually understand what I was reading : the phenomenal power of the human mind .";
    let mut string2 = Vec::new();

    for word in string1.split_whitespace() {
        if word.len() > 4 {
            string2.push(shuffle(word));
        } else {
            string2.push(word.to_string());
        }
    }
    println!("{}", string1);
    println!("{}", string2.join(" "));
}
