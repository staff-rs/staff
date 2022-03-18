use super::MidiNote;
use crate::note::Note;
use core::fmt;

pub struct MidiNoteDisplay {
    midi_note: MidiNote,
    note: Note,
}

impl MidiNoteDisplay {
    pub fn from_sharp(midi_note: MidiNote) -> Self {
        Self {
            midi_note,
            note: Note::from_sharp(midi_note.pitch()),
        }
    }
}

impl fmt::Display for MidiNoteDisplay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.note, self.midi_note.octave())
    }
}
