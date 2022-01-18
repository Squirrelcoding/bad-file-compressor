
use std::io::{prelude::*};
use std::{
    fs::File,
};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct JsonData {
    words: Vec<String>,
}

impl JsonData {
    fn extract_words(&self) -> &Vec<String> {
        &self.words
    }
}

pub fn decompress(input: &str, output: &str, json: &str) {
    let mut file: File = File::open(input).unwrap();
    let mut json_data: File = File::open(json).unwrap();
    let mut string = String::new();
    json_data.read_to_string(&mut string).unwrap();
    let data: JsonData = serde_json::from_str(&string).unwrap();
    let words = data.extract_words();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    let ids = vec!["A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "B0", "B1", "B2", "B3", "B4", "B5", "B6", "B7", "B8", "B9"];
    for i in 0..words.len() {
        let id = &ids[i][..];
        let word = &words[i][..];
        buf = buf.replace(id, word);
    }
    let mut file: File = File::create(output).unwrap();
    file.write_all(buf.as_bytes()).unwrap();
    print!("Successfully decompressed file '{}' into '{}'", input, output);
}
