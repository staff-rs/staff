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

    pub const fn from_flat(pitch: Pitch) -> Self {
        Self::new(pitch, Note::from_flat(pitch))
    }

    pub const fn from_sharp(pitch: Pitch) -> Self {
        Self::new(pitch, Note::from_sharp(pitch))
    }

    pub const fn pitch(self) -> Pitch {
        self.pitch
    }

    pub const fn note(self) -> Note {
        self.note
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
