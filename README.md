# Staff
[![crate](https://img.shields.io/crates/v/staff.svg)](https://crates.io/crates/staff)
[![documentation](https://docs.rs/staff/badge.svg)](https://docs.rs/staff)

Music theory and score rendering library with midi, notes, chords, scales, and more.

![staff](https://raw.githubusercontent.com/staff-rs/staff/main/example.svg)

![fretboard](https://raw.githubusercontent.com/staff-rs/staff/main/fretboard.svg)


## Installation
This crate currently uses the [Noto Music](https://fonts.google.com/noto/specimen/Noto+Music) font by default.
```
cargo install staff
```

## Usage
`example.ly`
```
\clef treble
a'1
d'2. e'4
f'2 e'4 f'
b'2. a'4
g'2 fis'4 g'
```
`$ staff example.ly > example.svg`

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
