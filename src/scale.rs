use crate::note::{Accidental, Note, Pitch};
pub use crate::Interval;
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
    pitch: Pitch,
    note: Note,
    intervals: Iter<'a, Interval>,
}

impl Iterator for Scale<'_> {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(interval) = self.intervals.next() {
            self.pitch = self.pitch + *interval;
            let letter = self.note.letter.next();

            let natural_pitch = Pitch::from(letter);

            let a = if natural_pitch >= self.pitch {
                match natural_pitch - self.pitch {
                    Interval::UNISON => Accidental::Natrual,
                    Interval::MINOR_SECOND => Accidental::Sharp,
                    Interval::MAJOR_SECOND => Accidental::DoubleSharp,
                    a => panic!("{:?}", a),
                }
            } else {
                match self.pitch - natural_pitch {
                    Interval::MINOR_SECOND => Accidental::Flat,
                    Interval::MAJOR_SECOND => Accidental::DoubleFlat,
                    _ => todo!(),
                }
            };

            Some(core::mem::replace(&mut self.note, Note::new(letter, a)))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::note::{Letter, Note, Pitch};
    use super::{Scale, MAJOR_SCALE};

    #[test]
    fn it_works() {
        let scale = Scale {
            pitch: Pitch::C,
            note: Note::new(Letter::C, crate::note::Accidental::Natrual),
            intervals: MAJOR_SCALE.iter(),
        };

        for n in scale {
            dbg!(n);
        }
    }
}
