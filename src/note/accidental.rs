use crate::{Interval, Natural, Pitch};
use core::fmt::{self, Write};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccidentalKind {
    Natural,
    Single,
    Double,
}

pub trait Accidental {
    fn into_pitch(kind: AccidentalKind, natural: Natural) -> Pitch;

    fn from_pitch(natural: Natural, pitch: Pitch) -> AccidentalKind;

    fn write_fmt(kind: AccidentalKind, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flat {}

impl Accidental for Flat {
    fn into_pitch(kind: AccidentalKind, natural: Natural) -> Pitch {
        let pitch = Pitch::natural(natural);
        match kind {
            AccidentalKind::Natural => pitch,
            AccidentalKind::Single => pitch - Interval::MINOR_SECOND,
            AccidentalKind::Double => pitch - Interval::MAJOR_SECOND,
        }
    }

    fn from_pitch(natural: Natural, pitch: Pitch) -> AccidentalKind {
        let natural_pitch = Pitch::natural(natural);
        if pitch >= natural_pitch {
            match pitch.sub(natural_pitch) {
                Interval::UNISON => AccidentalKind::Natural,
                Interval::MINOR_SECOND => AccidentalKind::Single,
                Interval::MAJOR_SEVENTH => AccidentalKind::Single,
                x => panic!("{:?}", x),
            }
        } else {
            match natural_pitch.sub(pitch) {
                Interval::MINOR_SECOND => AccidentalKind::Single,
                Interval::MAJOR_SECOND => AccidentalKind::Double,
                Interval::MAJOR_SEVENTH => AccidentalKind::Single,
                x => panic!("{:?}", x),
            }
        }
    }

    fn write_fmt(kind: AccidentalKind, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match kind {
            AccidentalKind::Natural => Ok(()),
            AccidentalKind::Single => f.write_char('b'),
            AccidentalKind::Double => f.write_str("bb"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sharp {}

impl Accidental for Sharp {
    fn into_pitch(kind: AccidentalKind, natural: Natural) -> Pitch {
        let pitch = Pitch::natural(natural);
        match kind {
            AccidentalKind::Natural => pitch,
            AccidentalKind::Single => pitch + Interval::MINOR_SECOND,
            AccidentalKind::Double => pitch + Interval::MAJOR_SECOND,
        }
    }

    fn from_pitch(natural: Natural, pitch: Pitch) -> AccidentalKind {
        let natural_pitch = Pitch::natural(natural);
        if pitch >= natural_pitch {
            match pitch.sub(natural_pitch) {
                Interval::UNISON => AccidentalKind::Natural,
                Interval::MINOR_SECOND => AccidentalKind::Single,
                Interval::MAJOR_SECOND => AccidentalKind::Double,
                _ => unimplemented!(),
            }
        } else {
            match natural_pitch.sub(pitch) {
                Interval::MAJOR_SEVENTH => AccidentalKind::Single,
                _ => unimplemented!(),
            }
        }
    }

    fn write_fmt(kind: AccidentalKind, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match kind {
            AccidentalKind::Natural => Ok(()),
            AccidentalKind::Single => f.write_char('#'),
            AccidentalKind::Double => f.write_str("##"),
        }
    }
}
