# Staff
[![crate](https://img.shields.io/crates/v/staff.svg)](https://crates.io/crates/staff)
[![documentation](https://docs.rs/staff/badge.svg)](https://docs.rs/staff)

[Website](https://staff-rs.github.io)

Music theory and score rendering library with midi, notes, chords, scales, and more.

## Usage
```rust
use staff::{midi, Chord, Pitch};

let chord = Chord::from_midi(
    midi!(C, 4),
    [midi!(E, 3), midi!(G, 3), midi!(C, 4)]
);

assert_eq!(chord.to_string(), "C/E");

let pitches = [Pitch::E, Pitch::G, Pitch::C];
assert!(chord.into_iter().eq(pitches));
```

## Features
* `render`: Enable `render` module
    * `svg`: Enable rendering to SVG
* `synth`: Enable `synth` module for 
* `serde`: Impl Deserialize and Serialize for many crate types
