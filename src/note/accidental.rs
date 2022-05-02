use crate::{Interval, Natural, Pitch};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AccidentalKind {
    Natural,
    Single,
    Double,
}

pub trait Accidental {
    fn accidental(kind: AccidentalKind, natural: Natural) -> Pitch;

    fn from(natural: Natural, pitch: Pitch) -> AccidentalKind;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sharp {}

impl Accidental for Sharp {
    fn accidental(kind: AccidentalKind, natural: Natural) -> Pitch {
        let pitch = Pitch::natural(natural);
        match kind {
            AccidentalKind::Natural => pitch,
            AccidentalKind::Single => pitch + Interval::MINOR_SECOND,
            AccidentalKind::Double => pitch + Interval::MAJOR_SECOND,
        }
    }

    fn from(natural: Natural, pitch: Pitch) -> AccidentalKind {
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
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Flat {}

impl Accidental for Flat {
    fn accidental(kind: AccidentalKind, natural: Natural) -> Pitch {
        let pitch = Pitch::natural(natural);
        match kind {
            AccidentalKind::Natural => pitch,
            AccidentalKind::Single => pitch - Interval::MINOR_SECOND,
            AccidentalKind::Double => pitch - Interval::MAJOR_SECOND,
        }
    }

    fn from(natural: Natural, pitch: Pitch) -> AccidentalKind {
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
                x => panic!("{:?}", x),
            }
        }
    }
}
