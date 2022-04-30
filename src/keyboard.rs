use crate::{midi::MidiNote, Chord};

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

    pub fn midis(&self) -> impl Iterator<Item = MidiNote> + '_ {
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
        self.midis().next().map(|root| self.chord_with_root(root))
    }

    pub fn chord_with_root(&self, root: MidiNote) -> Chord {
        Chord::from_midi(root, self.midis())
    }
}
