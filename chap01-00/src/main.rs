fn main() {
    let string = "stressed";
    let mut reversed = Vec::new();

    for i in string.chars().rev() {
        reversed.push(i.to_string());
    }

    println!("{}", reversed.concat());
}
