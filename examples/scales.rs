use music_note::{set::PitchSet, Note, Scale};

fn main() {
    for pitch in PitchSet::all() {
        let note = Note::from_flat(pitch);
        let scale = Scale::major(note);

        for note in scale {
            print!("{} ", note);
        }

        println!()
    }
}
