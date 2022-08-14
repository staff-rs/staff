use crate::Natural;
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

impl fmt::Display for Accidental {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Accidental::Natural => Ok(()),
            Accidental::Flat => f.write_char('b'),
            Accidental::DoubleFlat => f.write_str("bb"),
            Accidental::Sharp => f.write_char('#'),
            Accidental::DoubleSharp => f.write_str("##"),
        }
    }
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

impl From<Natural> for Note {
    fn from(natural: Natural) -> Self {
        Self::new(natural, Accidental::Natural)
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.natural, self.accidental)
    }
}

impl FromStr for Note {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let natural: Natural = if let Some(c) = chars.next() {
            c.try_into().unwrap()
        } else {
            return Err(Error::Empty);
        };

        let accidental = match chars.next() {
            Some('b') => match chars.next() {
                Some('b') => Accidental::DoubleFlat,
                Some(c) => return Err(c.into()),
                None => Accidental::Flat,
            },
            Some('#') => match chars.next() {
                Some('#') => Accidental::DoubleSharp,
                Some(c) => return Err(c.into()),
                None => Accidental::Sharp,
            },
            Some(c) => return Err(c.into()),
            None => Accidental::Natural,
        };

        Ok(Self::new(natural, accidental))
    }
}

#[derive(Debug)]
pub enum Error {
    Empty,
    Invalid(char),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("Empty note input"),
            Self::Invalid(c) => write!(f, "Invalid character `{}`", c),
        }
    }
}

impl From<char> for Error {
    fn from(c: char) -> Self {
        Self::Invalid(c)
    }
}
