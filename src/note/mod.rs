use crate::{Interval, Natural, Pitch};
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    Natural,
    Flat,
    DoubleFlat,
    Sharp,
    DoubleSharp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    pub natural: Natural,
    pub accidental: Accidental,
}

impl Note {
    pub fn new(natural: Natural, accidental: Accidental) -> Self {
        Self {
            natural,
            accidental,
        }
    }

    pub fn flat(natural: Natural) -> Self {
        Self::new(natural, Accidental::Flat)
    }

    pub fn double_flat(natural: Natural) -> Self {
        Self::new(natural, Accidental::DoubleFlat)
    }

    pub fn sharp(natural: Natural) -> Self {
        Self::new(natural, Accidental::Sharp)
    }

    pub fn double_sharp(natural: Natural) -> Self {
        Self::new(natural, Accidental::DoubleSharp)
    }
}

impl From<Note> for Pitch {
    fn from(note: Note) -> Self {
        let pitch: Pitch = note.natural.into();
        match note.accidental {
            Accidental::Natural => pitch,
            Accidental::Flat => pitch - Interval::MINOR_SECOND,
            Accidental::DoubleFlat => pitch - Interval::MAJOR_SECOND,
            Accidental::Sharp => pitch + Interval::MINOR_SECOND,
            Accidental::DoubleSharp => pitch + Interval::MAJOR_SECOND,
        }
    }
}

impl From<Natural> for Note {
    fn from(natural: Natural) -> Self {
        Self::new(natural, Accidental::Natural)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.natural.fmt(f)?;
        todo!()
    }
}
