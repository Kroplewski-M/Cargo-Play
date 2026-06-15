# Cargo Play

A terminal-based music player written in Rust.
<img width="1918" height="1013" alt="2026-06-10-200341_hyprshot" src="https://github.com/user-attachments/assets/d67fb919-4585-4f01-a1a4-4fdf4df49415" />

![Rust](https://img.shields.io/badge/rust-2024-orange)

## Features

- Scans your system audio directory for music automatically
- Browse your library in a navigable table
- Queue tracks and skip through them
- Play, pause, loop, and adjust volume
- Supports MP3, FLAC, OGG, WAV, M4A, AAC, MP4

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
