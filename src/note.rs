use crate::{Interval, Natural, Pitch};
use core::{
    fmt::{self, Write},
    str::FromStr,
};

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
        match self.accidental {
            Accidental::Natural => Ok(()),
            Accidental::Flat => f.write_char('b'),
            Accidental::DoubleFlat => f.write_str("bb"),
            Accidental::Sharp => f.write_char('#'),
            Accidental::DoubleSharp => f.write_str("##"),
        }
    }
}

impl FromStr for Note {
    type Err = Option<char>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let natural: Natural = if let Some(c) = chars.next() {
            c.try_into().unwrap()
        } else {
            return Err(None);
        };

        let note = match chars.next() {
            Some('b') => match chars.next() {
                Some('b') => Self::double_flat(natural),
                Some(c) => return Err(Some(c)),
                None => Note::flat(natural),
            },
            Some('#') => match chars.next() {
                Some('#') => Note::double_sharp(natural),
                Some(c) => return Err(Some(c)),
                None => Note::sharp(natural),
            },
            Some(c) => return Err(Some(c)),
            None => natural.into(),
        };

        Ok(note)
    }
}
