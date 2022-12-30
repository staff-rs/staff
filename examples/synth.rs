use staff::{midi, synth::chord, Chord};

fn main() {
    chord(Chord::major(midi!(C, 4)));
}
