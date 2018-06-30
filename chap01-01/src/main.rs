fn main() {
    let string = "パタトクカシー";

    for (i, s) in string.chars().enumerate() {
        if (i+1) % 2 == 1{
            print!("{}", s);
        }
    }
    println!("");
}
