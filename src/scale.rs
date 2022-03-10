pub use crate::Interval;
use crate::{
    note::{pitch_note::PitchNote, Accidental, Note},
    pitch::Pitch,
};
use core::slice::Iter;

pub const MAJOR_SCALE: [Interval; 7] = [
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
];

pub struct Scale<'a> {
    pitch_note: PitchNote,
    intervals: Iter<'a, Interval>,
}

impl<'a> Scale<'a> {
    pub fn new(pitch_note: PitchNote, intervals: &'a [Interval]) -> Self {
        Self {
            pitch_note,
            intervals: intervals.iter(),
        }
    }

    pub fn major(pitch_note: PitchNote) -> Self {
        Self::new(pitch_note, &MAJOR_SCALE)
    }
}

impl Iterator for Scale<'_> {
    type Item = PitchNote;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(interval) = self.intervals.next() {
            let pitch = self.pitch_note.pitch() + *interval;

            let letter = self.pitch_note.note().letter.next();
            let natural_pitch = Pitch::from(letter);

            let accidental = if natural_pitch >= pitch {
                match natural_pitch - pitch {
                    Interval::UNISON => Accidental::Natrual,
                    Interval::MINOR_SECOND => Accidental::Flat,
                    Interval::MAJOR_SECOND => Accidental::DoubleFlat,
                    Interval::MAJOR_SEVENTH => Accidental::Sharp,
                    _ => todo!(),
                }
            } else {
                match pitch - natural_pitch {
                    Interval::MINOR_SECOND => Accidental::Sharp,
                    Interval::MAJOR_SECOND => Accidental::DoubleSharp,
                    _ => todo!(),
                }
            };

            Some(core::mem::replace(
                &mut self.pitch_note,
                PitchNote::new(pitch, Note::new(letter, accidental)),
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{note::pitch_note::PitchNote, pitch::Pitch, scale::Scale};

    #[test]
    fn it_works() {
        for note in Scale::major(PitchNote::from(Pitch::C)) {
            dbg!(note);
        }
    }
}
