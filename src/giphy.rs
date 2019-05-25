extern crate reqwest;
use http::{Request, Response};
use serde_json::{Result, Value};
use rand::Rng;
use std::process::Command;
use std::str;

pub fn getGiphy(text: String) {
    let uri = format!("http://api.giphy.com/v1/gifs/search?api_key=Y99QRjzrSNP0HucWPPtXMnNJh3ERdf1o&q={}", text);
    let body = reqwest::get(&uri).unwrap()
    .text().unwrap();
    getJson(&body)
}

fn getJson(data: &str) {
    let v: Value = serde_json::from_str(data).unwrap();
    let mut rng = rand::thread_rng();
    let rnumber = rng.gen_range(0, 25);
    let gif = v["data"][rnumber]["images"]["original"]["url"].to_string();
    let trimmed = gif.trim_matches('"');
    Command::new("gif-for-cli").arg(trimmed).status().unwrap();
}

