fn main() {
    let raw_string = "Now I need a drink, alcoholic of course, after the heavy lectures involving quantum mechanics.";
    let mut lens = Vec::new();

    let string = &raw_string.replace(",", "").replace(".", "");

    for w in string.split_whitespace(){
        lens.push(w.len());
    }
    println!("{:?}", lens);
}
