use super::{Natural, Note};
use crate::pitch::Pitch;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PitchNote {
    pitch: Pitch,
    note: Note,
}

impl PitchNote {
    pub const fn new_unchecked(pitch: Pitch, note: Note) -> Self {
        Self { pitch, note }
    }

    pub const fn natural(letter: Natural) -> Self {
        Self::new_unchecked(Pitch::natural(letter), Note::natural(letter))
    }

    pub const fn from_flat(pitch: Pitch) -> Self {
        Self::new_unchecked(pitch, Note::from_flat(pitch))
    }

    pub const fn from_sharp(pitch: Pitch) -> Self {
        Self::new_unchecked(pitch, Note::from_sharp(pitch))
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
            note,
        }
    }
}
