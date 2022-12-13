# Staff
[![crate](https://img.shields.io/crates/v/staff.svg)](https://crates.io/crates/staff)
[![documentation](https://docs.rs/staff/badge.svg)](https://docs.rs/staff)

Music theory and score rendering library with midi, notes, chords, scales, and more.

![staff](https://raw.githubusercontent.com/staff-rs/staff/main/example.svg)

## Installation
```
cargo install staff --features=cli
```

## Usage
`example.ly`
```
c'4 eis'2 g'4
f''2 ees'4 c''
```
`$ staff engrave example.ly > example.svg`

## Library
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
