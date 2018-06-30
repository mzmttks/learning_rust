// 与えられた文字列の各文字を，以下の仕様で変換する関数cipherを実装せよ．

// 英小文字ならば(219 - 文字コード)の文字に置換
// その他の文字はそのまま出力
// この関数を用い，英語のメッセージを暗号化・復号化せよ．

fn cipher(input: &String) -> String {
    let mut output = Vec::new();
    for v in input.chars() {
        output.push((219 - v as u8) as char);
    }
    output.into_iter().collect()
}

fn main() {
    let input = "Always look on the bright side of life".to_string();
    println!("{}", cipher(&input));
    println!("{}", cipher(&cipher(&input)));
}
