fn main() {
    let str1 = "パトカー";
    let str2 = "タクシー";

    for (x1, x2) in str1.chars().zip(str2.chars()) {
        print!("{}{}", x1, x2);
    }
    println!("");
}
