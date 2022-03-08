use core::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

impl Pitch {
    pub const C: Self = Self(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Letter {
    C,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    Natrual,
    Flat,
    DoubleFlat,
    Sharp,
    DoubleSharp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    letter: Letter,
    accidental: Accidental,
}

impl Note {
    pub const fn new(letter: Letter, accidental: Accidental) -> Self {
        Self { letter, accidental }
    }

    pub const fn natural(letter: Letter) -> Self {
        Self::new(letter, Accidental::Natrual)
    }
}

impl From<Pitch> for Note {
    fn from(note: Pitch) -> Self {
        match note {
            Pitch::C => Self::natural(Letter::C),
            _ => todo!(),
        }
    }
}
