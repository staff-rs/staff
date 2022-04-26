use crate::{midi::MidiNote, Chord, Pitch};

pub struct Keyboard {
    keys: [bool; 128],
}

impl Keyboard {
    pub fn key_mut(&mut self, midi: MidiNote) -> &mut bool {
        &mut self.keys[midi.into_byte() as usize]
    }

    pub fn press(&mut self, midi: MidiNote) {
        *self.key_mut(midi) = true;
    }

    pub fn release(&mut self, midi: MidiNote) {
        *self.key_mut(midi) = false;
    }

    pub fn midi_notes(&self) -> impl Iterator<Item = MidiNote> + '_ {
        self.keys
            .iter()
            .enumerate()
            .filter_map(|(pos, is_pressed)| {
                if *is_pressed {
                    Some(MidiNote::from_byte(pos as _))
                } else {
                    None
                }
            })
    }

    pub fn chord(&self) -> Option<Chord> {
        let mut pitches = self.midi_notes().map(|note| note.pitch());
        if let Some(root) = pitches.next() {
            let mut chord = Chord::from_root(root);
            chord.extend(pitches);
            Some(chord)
        } else {
            None
        }
    }

    pub fn chord_with_root(&self, root: Pitch) -> Chord {
        let mut chord = Chord::from_root(root);

        let mut pitches = self.midi_notes().map(|note| note.pitch());
        if let Some(pitch) = pitches.next() {
            chord.pitches.push(pitch);

            if pitch != root {
                chord.bass = Some(pitch);
            }

            chord.extend(pitches);
        }

        chord
    }
}
