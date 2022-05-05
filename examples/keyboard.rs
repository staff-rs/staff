use music_note::midi::MidiSet;
use music_note::{midi, Chord};

fn main() {
    let mut keyboard = MidiSet::default();
    keyboard.push(midi!(C, 4));
    keyboard.push(midi!(E, 4));
    keyboard.push(midi!(G, 4));

    let chord = Chord::from_iter(keyboard);
    println!("C E G is a {} chord", chord);
}
