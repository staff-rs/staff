use staff::{synth::chord, Chord, midi};

fn main() {
    chord(Chord::major(midi!(C, 4)));
}