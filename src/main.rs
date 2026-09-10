use serde::Deserialize;
use serde_bencode;
use serde_json;
use std::env;

// Metainfo files (also known as .torrent files) are bencoded dictionaries with the following keys:
#[derive(Debug, Clone, Deserialize)]
struct Torrent {
    // The URL of the tracker
    announce: reqwest::Url,

    // This maps to dictionary, with keys describe below
    info: Info,
}

struct Info {
    // The name key maps to a UTF-8 encoded string which is the suggested name to save the file (or directory) as. It is purely advisory.
    name: String,

    // piece length maps to the number of bytes in each piece the file is split into. For the purposes of transfer, files are split into fixed-size pieces which are all the same length except for possibly the last one which may be truncated. piece length is almost always a power of two, most commonly 2 18 = 256 K (BitTorrent prior to version 3.2 uses 2 20 = 1 M as default).
    #[serde(rename = "piece length")]
    plength: usize,
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value);
        println!("{}", decoded_value.0.to_string());
    } else {
        println!("unknown command: {}", args[1])
    }
}
