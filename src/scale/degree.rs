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

impl<A> Degree for Note<A>
where
    A: Accidental,
{
    type State = Natural;

    fn state(self) -> Self::State {
        self.natural()
    }

    fn degree(self, state: &mut Self::State, interval: Interval) -> Self {
        let pitch = Pitch::from(self.clone()).add_interval(interval);
        let accidental = A::from_pitch(*state, pitch);
        let note = Self::new(*state, accidental);

        *state = *state + 1;
        note
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
