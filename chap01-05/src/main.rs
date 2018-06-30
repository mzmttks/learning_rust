/// 与えられたシーケンス（文字列やリストなど）からn-gramを作る関数を作成せよ．
/// この関数を用い，"I am an NLPer"という文から単語bi-gram，文字bi-gramを得よ．

use std::collections::HashMap;

fn ngram_char(input: &String) -> HashMap<(char, char), u8>{
    let mut ngram = HashMap::new();
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
fn ngram_word(input: &String) -> HashMap<(&str, &str), u8>{
    let mut ngram = HashMap::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    
    for (i, w) in words.iter().skip(1).enumerate() {
        let key = (words[i], *w);
        if ngram.contains_key(&key){
            let count = ngram.get_mut(&key).unwrap();
            *count += 1; 
        } else {
            ngram.insert(key, 1);
        }
    }
    ngram
}
fn main() {
    let string = "I am an NLPer".to_string();
    println!("{:?}", ngram_char(&string));
    println!("{:?}", ngram_word(&string));
}
