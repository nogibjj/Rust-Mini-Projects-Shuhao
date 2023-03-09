use std::io::{self, Write};
use reqwest;
use serde::{Deserialize, Serialize};

const DATAMUSE_API_URL: &str = "https://api.datamuse.com/words";

#[derive(Debug, Deserialize, Serialize)]
struct DatamuseResponse {
    word: String,
    score: i32,
}

fn main() {
    for _ in 0..5 {
        print!("Give me a word!");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let word = input.trim();

        let query_params = [("rel_syn", word)];

        let response = reqwest::blocking::Client::new()
            .get(DATAMUSE_API_URL)
            .query(&query_params)
            .send()
            .expect("required API failed");

        let synonyms: Vec<DatamuseResponse> = response
            .json()
            .expect("Parse response failed");

        if synonyms.is_empty() {
            println!("Cant find synonyms of {}", word);
        } else {
            println!("Synonyms of {}：", word);
            for synonym in synonyms {
                println!(" - {}", synonym.word);
            }
        }
    }
}
