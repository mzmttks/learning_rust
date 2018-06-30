use std::collections::HashMap;

type Ngram = HashMap<(char, char), u8>;

fn ngram_char(input: &String) -> Ngram{
    let mut ngram = Ngram::new();
    let chars: Vec<char> = input.chars().collect();

    for (i, w) in chars.iter().skip(1).enumerate() {
        let key = (chars[i], *w);
        if ngram.contains_key(&key){
            let count = ngram.get_mut(&key).unwrap();
            *count += 1;
        } else {
            ngram.insert(key, 1);
        }
    }
    ngram
}

fn union(h1: &Ngram, h2: &Ngram) -> Ngram {
    let mut ngram = h1.clone();
    for (key, value) in h2 {
        if ngram.contains_key(&key) {
            let count = ngram.get_mut(&key).unwrap();
            *count += *value
        } else {
            ngram.insert(*key, *value);
        }
    }

    ngram
}

fn main() {
    let str1 = "paraparaparadise";
    let str2 = "paragraph";

    let ngram1 = ngram_char(&str1.to_string());
    let ngram2 = ngram_char(&str2.to_string());

    println!("{:?}", union(&ngram1, &ngram2));
    // println!("{:?}", intersection(&ngram1, &ngram2));
    // println!("{:?}", diff(&ngram1, &ngram2));
    println!("{}: {:?}", &str1, &ngram1.contains_key(&('s', 'e')));
    println!("{}: {:?}", &str2, &ngram2.contains_key(&('s', 'e')));

    println!("Hello, world!");
}
