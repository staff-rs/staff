use std::mem::MaybeUninit;

pub use crate::Interval;
use crate::{
    note::{pitch_note::PitchNote, Accidental, Letter, Note},
    pitch::Pitch,
};

pub const C_MAJOR: [PitchNote; 7] = {
    let mut array: [MaybeUninit<PitchNote>; 7] = MaybeUninit::uninit_array();

    let mut scale = Scale::major(PitchNote::new(Pitch::C, Note::natural(Letter::C)));

    let mut i = 0;
    while let Some(note) = scale.next_note() {
        array[i] = MaybeUninit::new(note);
        i += 1;
    }

    unsafe { (&array as *const _ as *const [PitchNote; 7]).read() }
};

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
    intervals: &'a [Interval],
    index: usize,
}

impl<'a> Scale<'a> {
    pub const fn new(pitch_note: PitchNote, intervals: &'a [Interval]) -> Self {
        Self {
            pitch_note,
            intervals,
            index: 0,
        }
    }

    pub const fn major(pitch_note: PitchNote) -> Self {
        Self::new(pitch_note, &MAJOR_SCALE)
    }

    pub const fn next_note(&mut self) -> Option<PitchNote> {
        if self.index < self.intervals.len() {
            let interval = self.intervals[self.index];
            self.index += 1;

            let pitch = self.pitch_note.pitch().add_interval(interval);

            let letter = self.pitch_note.note().letter.next();
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

            Some(core::mem::replace(
                &mut self.pitch_note,
                PitchNote::new(pitch, Note::new(letter, accidental)),
            ))
        } else {
            None
        }
    }
}

impl Iterator for Scale<'_> {
    type Item = PitchNote;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_note()
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
