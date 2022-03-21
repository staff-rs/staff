use crate::Interval;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Accidental {
    Natural,
    Flat,
    DoubleFlat,
    Sharp,
    DoubleSharp,
}

impl Accidental {
    pub fn interval(self) -> Interval {
        match self {
            Self::Natural => Interval::UNISON,
            Self::Flat | Self::Sharp => Interval::MINOR_SECOND,
            Self::DoubleFlat | Self::DoubleSharp => Interval::MAJOR_SECOND,
        }
    }
}

impl TryFrom<char> for Accidental {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let accidental = match value {
            'b' => Self::Flat,
            '#' => Self::Sharp,
            invalid => return Err(invalid),
        };
        Ok(accidental)
    }
}
