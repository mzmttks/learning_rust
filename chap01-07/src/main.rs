/// 引数x, y, zを受け取り「x時のyはz」という文字列を返す関数を実装せよ．
/// さらに，x=12, y="気温", z=22.4として，実行結果を確認せよ．


fn printer(x: u8, y: &str, z: f32) {
    println!("{}の時の{}は{}", x, y, z);
}

fn main() {
    let x = 12;
    let y = "気温";
    let z = 22.4;
    printer(x, y, z);
}
