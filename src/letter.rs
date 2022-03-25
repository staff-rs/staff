use core::fmt;

/// Natural of a natural pitch
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Natural {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl Natural {
    pub const fn next(self) -> Self {
        let byte = (self as u8 + 1) % (Self::G as u8 + 1);
        // Safety: `byte` is guranteed to be in range of `Natural`
        unsafe { core::mem::transmute(byte) }
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
        }
    }
}

impl TryFrom<char> for Natural {
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

impl fmt::Debug for Natural {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Natural {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
