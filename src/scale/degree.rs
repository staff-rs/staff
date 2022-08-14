use crate::{
    midi::MidiNote,
    note::{Accidental, Note},
    pitch::Pitch,
    Interval, Natural,
};

pub trait Degree {
    type State;

    fn state(self) -> Self::State;

    fn degree(self, state: &mut Self::State, interval: Interval) -> Self;
}

impl Degree for Note {
    type State = Natural;

    fn state(self) -> Self::State {
        self.natural
    }

    fn degree(self, state: &mut Self::State, interval: Interval) -> Self {
        let root: Pitch = self.into();
        let pitch = root + interval;

        let natural_pitch = Pitch::from(*state);
        let natural = *state;
        *state = *state + 1;

        let accidental = if pitch >= natural_pitch {
            match pitch - natural_pitch {
                Interval::UNISON => Accidental::Natural,
                Interval::MINOR_SECOND => Accidental::Sharp,
                Interval::MAJOR_SECOND => Accidental::DoubleSharp,
                Interval::MAJOR_SEVENTH => Accidental::Flat,
                _ => unimplemented!(),
            }
        } else {
            match natural_pitch - pitch {
                Interval::MINOR_SECOND => Accidental::Flat,
                Interval::MAJOR_SECOND => Accidental::DoubleFlat,
                Interval::MAJOR_SEVENTH => Accidental::Sharp,
                _ => unimplemented!(),
            }
        };

        Self::new(natural, accidental)
    }
}

impl Degree for Pitch {
    type State = ();

    fn state(self) -> Self::State {}

    fn degree(self, _state: &mut Self::State, interval: Interval) -> Self {
        self + interval
    }
}

impl Degree for MidiNote {
    type State = ();

    fn state(self) -> Self::State {}

    fn degree(self, _state: &mut Self::State, interval: Interval) -> Self {
        self + interval
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Scale;

    #[test]
    fn c_major() {
        let s = Scale::major(Note::from(Natural::C));
        dbg!(s.collect::<Vec<_>>());
    }

    #[test]
    fn g_flat_major() {
        let s = Scale::major(Note::flat(Natural::G));
        dbg!(s.collect::<Vec<_>>());
    }
}
