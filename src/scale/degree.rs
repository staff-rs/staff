use crate::{
    midi::MidiNote,
    note::{Accidental, Note, PitchNote},
    pitch::Pitch,
    Interval,
};

pub trait Degree: Copy {
    fn next_degree(self, interval: Interval) -> Self
    where
        Self: Sized;
}

impl Degree for PitchNote {
    fn next_degree(self, interval: Interval) -> Self
    where
        Self: Sized,
    {
        let pitch = self.pitch().add_interval(interval);
        let letter = self.note().letter.next();
        let natural_pitch = Pitch::natural(letter);

        let accidental = if natural_pitch.into_byte() >= pitch.into_byte() {
            match natural_pitch.sub(pitch) {
                Interval::UNISON => Accidental::Natrual,
                Interval::MINOR_SECOND => Accidental::Flat,
                Interval::MAJOR_SECOND => Accidental::DoubleFlat,
                Interval::MAJOR_SEVENTH => Accidental::Sharp,
                _ => todo!(),
            }
        } else {
            match pitch.sub(natural_pitch) {
                Interval::MINOR_SECOND => Accidental::Sharp,
                Interval::MAJOR_SECOND => Accidental::DoubleSharp,
                _ => todo!(),
            }
        };

        PitchNote::new(pitch, Note::new(letter, accidental))
    }
}

impl Degree for MidiNote {
    fn next_degree(self, interval: Interval) -> Self
    where
        Self: Sized,
    {
        self + interval
    }
}
