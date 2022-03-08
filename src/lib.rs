use std::{ops::Add, fmt::Debug};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval {
    semitones: u8,
}

impl Interval {
    pub const MINOR_SECOND: Self = Self::new(1);

    pub const MAJOR_SECOND: Self = Self::new(2);

    pub const MINOR_THIRD: Self = Self::new(3);

    pub const fn new(semitones: u8) -> Self {
        Self { semitones }
    }

    pub const fn semitones(self) -> u8 {
        self.semitones
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.semitones() + rhs.semitones())
    }
}


#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

impl Pitch {
    pub const C: Self = Self(0);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Letter{
    C
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    Natrual,
    Flat,
    DoubleFlat,
    Sharp,
    DoubleSharp
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Note {
    letter: Letter,
    accidental: Accidental
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
            _ => todo!()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
