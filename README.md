# Staff
[![crate](https://img.shields.io/crates/v/staff.svg)](https://crates.io/crates/staff)
[![documentation](https://docs.rs/staff/badge.svg)](https://docs.rs/staff)

Music theory CLI and library with midi, notes, chords, scales, and more.

## Installation
```
cargo install staff --features=cli
```

## Usage
#### Command-line interface
```
$ staff chord C#m7

C# F G# B
```
```
$ staff scale D dorian

D E F G A B C
```

#### Library
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
