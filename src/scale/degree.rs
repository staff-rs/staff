use crate::{
    midi::MidiNote,
    note::{Accidental, Note},
    pitch::Pitch,
    Interval,
};

/// The degree of a `Scale`.
pub trait Degree: Copy {
    fn next_degree(self, interval: Interval) -> Self;
}

impl Degree for Pitch {
    fn next_degree(self, interval: Interval) -> Self {
        self + interval
    }
}

impl Degree for MidiNote {
    fn next_degree(self, interval: Interval) -> Self {
        self + interval
    }
}

// TODO clone
impl<A> Degree for Note<A>
where
    A: Copy + Accidental,
{
    fn next_degree(self, interval: Interval) -> Self {
        let pitch = Pitch::from(self).add_interval(interval);
        let natural = self.natural.next();
        let accidental = A::from(natural, pitch);
        Self::new(natural, accidental)
    }
}
