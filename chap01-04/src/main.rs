use std::collections::HashMap;

fn main() {
    let string = "Hi He Lied Because Boron Could Not Oxidize Fluorine. New Nations Might Also Sign Peace Security Clause. Arthur King Can.";
    let firsts = [1, 5, 6, 7, 8, 9, 15, 16, 19];
    let mut map: HashMap<&str, u8> = HashMap::new();

    for (i, word) in string.split_whitespace().enumerate(){
        if firsts.contains(&(i+1)) {
            print!("{}, ", &word[0..1]);
            map.insert(&word[0..1], i as u8);
        } else {
            print!("{}, ", &word[0..2]);
            map.insert(&word[0..2], i as u8);
        }
    }
    println!("");
    println!("{:?}", map);
}
