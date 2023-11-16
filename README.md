# Osu Beatmap Parser for Rust

### **The project is currently in an early-development stage.**
#### Please **OPEN AN ISSUE** if you encounter any problems using the library.

---

A Rust library to read, parse and write contained data of an Osu! beatmap file to easily manipulate beatmap data in a Rust project.

This library was made according how a .osu beatmap file is structured explained on the official wiki of Osu!
(https://osu.ppy.sh/wiki/en/Client/File_formats/Osu_%28file_format%29).

## Usage

### Parsing a beatmap file (.osu)
```rust
use osu_beatmap_parser::BeatmapLevel;
use std::path::Path;

fn main() {
    let beatmap_path = Path::new("./assets/examples/test.osu");
    let mut beatmap: BeatmapLevel = BeatmapLevel::open(&beatmap_path).unwrap();

    // Editing the approach rate
    beatmap.difficulty.approach_rate = 9.;

    // Getting all the hit objects
    let objects = beatmap.hit_objects;
}
```
