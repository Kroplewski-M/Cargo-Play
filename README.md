# cargo_play

A terminal-based music player written in Rust.

![Rust](https://img.shields.io/badge/rust-2024-orange)

## Features

- Scans your system audio directory for music automatically
- Browse your library in a navigable table
- Queue tracks and skip through them
- Play, pause, and adjust volume
- Supports MP3, FLAC, OGG, WAV, M4A, OPUS, AAC

## Installation

Requires Rust and Cargo.

```bash
git clone https://github.com/mateuszk/cargo_play
cd cargo_play
cargo build --release
./target/release/cargo_play
```

## Usage

On launch, cargo_play scans your system audio directory (`~/Music` on Linux/macOS) and populates the library.

## Built With

- [ratatui](https://github.com/ratatui-org/ratatui) — TUI framework
- [rodio](https://github.com/RustAudio/rodio) — Audio playback
- [lofty](https://github.com/Serial-ATA/lofty-rs) — Audio metadata
- [crossterm](https://github.com/crossterm-rs/crossterm) — Terminal backend
- [walkdir](https://github.com/BurntSushi/walkdir) — Directory traversal
