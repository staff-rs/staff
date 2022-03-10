use crate::{note::Note, pitch::Pitch};

#[derive(Clone, Copy, Debug)]
pub struct PitchNote {
    pitch: Pitch,
    note: Note,
}

impl PitchNote {
    pub const fn new(pitch: Pitch, note: Note) -> Self {
        Self { pitch, note }
    }

    pub fn pitch(self) -> Pitch {
        self.pitch
    }

    pub fn note(self) -> Note {
        self.note
    }
}

impl From<Pitch> for PitchNote {
    fn from(pitch: Pitch) -> Self {
        Self {
            pitch,
            note: pitch.into(),
        }
    }
}

impl From<Note> for PitchNote {
    fn from(note: Note) -> Self {
        Self {
            pitch: note.into(),
            note: note,
        }
    }
}
