use anyhow::Context;
use clap::{Parser, Subcommand};
use serde::Deserialize;
use serde_bencode;
use serde_json;
use std::path::PathBuf;

use hashes::Hashes;

/// Metainfo files (also known as .torrent files) are bencoded dictionaries with the following keys:
#[derive(Debug, Clone, Deserialize)]
struct Torrent {
    // The URL of the tracker
    announce: String,

    info: Info,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Decode { value: String },
    Info { torent: PathBuf },
}

#[derive(Debug, Clone, Deserialize)]
struct Info {
    /// suggested name to save the file (or directory) as. It is purely advisory.
    ///
    /// In the single file case, the name key is the name of a file, in the muliple file case,
    /// it's the name of a directory.
    name: String,

    /// Number of bytes in each piece the file is split into.
    ///
    /// For the purposes of transfer, files are split into fixed-size pieces which are all the same length except
    /// for possibly the last one which may be truncated. piece length is almost always a power of two,
    /// most commonly 2^18 = 256 K (BitTorrent prior to version 3.2 uses 2 20 = 1 M as default).
    #[serde(rename = "piece length")]
    plength: usize,

    /// each entry of `pieces` is the SHA1 hash of the piece at the corresponding index.
    pieces: Hashes,

    #[serde(flatten)]
    keys: Keys,
}

/// There is also a key `length` or a key `files`, but not both or neither. ,
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Keys {
    /// If length is present then the download represents a single file
    SingleFile {
        // length maps to the length of the file in bytes
        length: usize,
    },

    /// Otherwise it represents a set of files which go in a directory structure.
    /// For the purposes of the other keys in `Info`, the multi-file case is treated as only
    /// having a single file by concatenating the files in the order they appear in the files list.
    /// The files list is the value files maps to, and is a list of dictionaries containing the following keys:
    MultiFile { file: Vec<File> },
}

#[derive(Debug, Clone, Deserialize)]
struct File {
    /// The length of the file, in bytes.
    length: usize,

    /// Subdirectory names of this file, the last of which is the actual file name
    /// (a zero length list is an error case).
    path: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Decode { value } => {
            // let v: serde_json::Map<String, serde_json::Value> =
            //    serde_bencode::from_str(&value).unwrap();
            // println!("{v:?}");
            unimplemented!("serde_bincode -> serde_json::Value is borked")
        }
        Command::Info { torent } => {
            let mut dot_torrent = std::fs::read(torent).context("open torrent file")?;
            let t: Torrent =
                serde_bencode::from_bytes(&dot_torrent).context("parse torrent file")?;
            eprintln!("{t:?}");
            println!("Tracker URL: {}", t.announce);
            if let Keys::SingleFile { length } = t.info.keys {
                println!("Length: {length}")
            } else {
                todo!()
            }
        }
    }

    Ok(())
}

mod hashes {
    use serde::de::{self, Deserialize, Deserializer, Visitor};
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Hashes(Vec<[u8; 20]>);
    struct HashesVisitor;

    impl<'de> Visitor<'de> for HashesVisitor {
        type Value = Hashes;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a byte string whose length is multiple of 20")
        }

        fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.len() % 20 != 0 {
                return Err(E::custom(format!("length is {}", v.len())));
            }
            Ok(Hashes(
                v.chunks_exact(20)
                    .map(|slice_20| slice_20.try_into().expect("guaranteed to be length 20"))
                    .collect(),
            ))
        }
    }

    impl<'de> Deserialize<'de> for Hashes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_bytes(HashesVisitor)
        }
    }
}
