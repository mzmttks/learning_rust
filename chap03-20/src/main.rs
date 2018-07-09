extern crate serde_json;
extern crate libflate;

#[macro_use]
extern crate serde_derive;

use std::io::prelude::*;
use std::fs::File;
use libflate::gzip::Decoder;

#[derive(Debug, Deserialize, Serialize)]
struct Value {
    text: String,
    title: String
}

// TODO: Make the function work
//       [the variable "content" lives too long]
// fn parse_gzfile(filename: &str) -> Result<&str, std::io::Error> {
//     let handle = File::open(filename)?;
//     let mut decoder = Decoder::new(&handle)?;
//     let mut content = Vec::new();
//     decoder.read_to_end(&mut content)?;
//     let decoded = std::str::from_utf8(&content).unwrap();
//     Ok(decoded)
// }

fn main() {
    // let decoded = parse_gzfile("../jawiki-country.json.gz").unwrap();
    let handle = File::open("../jawiki-country.json.gz").unwrap();
    let mut decoder = Decoder::new(&handle).unwrap();
    let mut content = Vec::new();
    decoder.read_to_end(&mut content).unwrap();
    let decoded = std::str::from_utf8(&content).unwrap();

    let keyword = "イギリス";
    let mut found_articles = Vec::new();

    for line in decoded.trim().split("\n") {
        let obj: Value = serde_json::from_str(line).unwrap();
        if obj.title.contains(&keyword) {
            found_articles.push(obj.text);
        }
    }
    let mut handle = File::create("england.txt").unwrap();
    for found_article in &found_articles {
        handle.write(&found_article.as_bytes()).unwrap();
        handle.write("\n".as_bytes()).unwrap();
    }
}
