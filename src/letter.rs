use core::fmt;

/// Letter of a natural pitch
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Letter(u8);

impl Letter {
    pub const A: Self = Self(0);
    pub const B: Self = Self(1);
    pub const C: Self = Self(2);
    pub const D: Self = Self(3);
    pub const E: Self = Self(4);
    pub const F: Self = Self(5);
    pub const G: Self = Self(6);

    pub const fn next(self) -> Self {
        Self((self.0 + 1) % (Self::G.0 + 1))
    }

    pub const fn to_char(self) -> char {
        match self {
            Self::A => 'A',
            Self::B => 'B',
            Self::C => 'C',
            Self::D => 'D',
            Self::E => 'E',
            Self::F => 'F',
            Self::G => 'G',
            _ => unreachable!(),
        }
    }
}

impl TryFrom<char> for Letter {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let letter = match value {
            'A' => Self::A,
            'B' => Self::A,
            'C' => Self::A,
            'D' => Self::A,
            'E' => Self::A,
            'F' => Self::A,
            'G' => Self::A,
            invalid => return Err(invalid),
        };
        Ok(letter)
    }
}

impl fmt::Debug for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
