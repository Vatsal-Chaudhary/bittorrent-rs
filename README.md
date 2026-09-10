# bittorrent-rs

A BitTorrent client written in Rust.

Originally started as the [CodeCrafters "Build Your Own BitTorrent"](https://app.codecrafters.io/courses/bittorrent/overview) challenge and extended into a personal project.

## Features

- Bencode decoder (strings, integers, lists, dictionaries)
- Torrent file parsing
- Info hash calculation
- Tracker communication (HTTP)
- Peer handshake
- Piece downloading
- (add more as you implement them)

## Usage

```bash
# Decode bencoded data
cargo run -- decode "d3:foo3:bar5:helloi52ee"

# Show torrent info
cargo run -- info sample.torrent

# Discover peers
cargo run -- peers sample.torrent

# Download a piece
cargo run -- download_piece -o piece.bin sample.torrent 0

# Download the full file
cargo run -- download -o output.bin sample.torrent
