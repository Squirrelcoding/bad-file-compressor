use rayon::iter::{ParallelBridge, ParallelIterator};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::{fs::File, io::Read};

#[derive(Serialize, Deserialize, Debug)]
struct JsonData {
    words: Vec<String>,
}

pub fn compress(input: &str, output_file: &str) {
    let mut filee: File = File::open(input).unwrap();
    let mut string: String = String::new();
    let mut most_common_words: Vec<String> = Vec::new();
    filee.read_to_string(&mut string).unwrap();

    string = string.replace(".", "");
    string = string.replace("!", "");
    string = string.replace("\n", "");
    string = string.replace("\r", "");

    let words: Arc<Mutex<Vec<&str>>> = Arc::new(Mutex::new(Vec::new()));
    string
        .split(" ")
        .enumerate()
        .par_bridge()
        .for_each(|(_, b)| {
            words.lock().unwrap().push(b);
        });

    let filterr = &mut *words.lock().unwrap();
    filterr.retain(|a| a.len() > 2);
    for _ in 1..21 {
        let n: usize = filterr.len();
        filterr.sort_by(|a, b| a.cmp(b));
        let mut max_count: u8 = 1;
        let mut curr_count: u8 = 1;
        let mut res: &str = filterr[0];
        for i in 1..n {
            if filterr[i] == filterr[i - 1] {
                curr_count += 1;
            } else {
                if curr_count > max_count && filterr[i] != filterr[i - 1] {
                    max_count = curr_count;
                    res = filterr[i - 1];
                }
                curr_count = 1;
            }
        }
        if curr_count > max_count {
            res = filterr[n - 1];
        }
        let most_common: String = String::from(res);
        filterr.retain(|a| a.to_string() != most_common && a.len() > 2);
        most_common_words.push(most_common);
    }

    let compressed_id: Vec<&str> = vec![
        "A0", "A1", "A2", "A3", "A4", "A5", "A6", "A7", "A8", "A9", "B0", "B1", "B2", "B3", "B4",
        "B5", "B6", "B7", "B8", "B9",
    ];
    let hash = most_common_words
        .iter()
        .zip(compressed_id.iter())
        .collect::<HashMap<_, _>>();
    let mut file_data = String::new();
    let mut file2 = File::open(input).unwrap();
    file2.read_to_string(&mut file_data).unwrap();
    for (key, value) in hash {
        file_data = file_data.replace(key, value);
    }
    let mut output = File::create(format!("{}.txt", output_file)).unwrap();
    output.write_all(file_data.as_bytes()).unwrap();
    
    let json_buf = JsonData {
        words: most_common_words
    };

    let json = serde_json::to_string(&json_buf).unwrap();

    let mut json_file = File::create(format!("{}.json", output_file)).unwrap();

    json_file.write_all(json.as_bytes()).unwrap();
    print!("Successfully compressed file '{}'", input);
}
