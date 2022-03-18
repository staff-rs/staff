use crate::{
    note::{Accidental, Note, PitchNote},
    pitch::Pitch,
    Interval,
};
use core::mem::{self, MaybeUninit};

pub const MAJOR_SCALE: &[Interval] = &[
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
];

pub const NATURAL_MINOR_SCALE: &[Interval] = &[
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
];

pub struct Scale<'a> {
    pitch_note: PitchNote,
    intervals: &'a [Interval],
    index: usize,
}

impl<'a> Scale<'a> {
    pub const fn new(root: PitchNote, intervals: &'a [Interval]) -> Self {
        Self {
            pitch_note: root,
            intervals,
            index: 0,
        }
    }

    pub const fn major(root: PitchNote) -> Self {
        Self::new(root, MAJOR_SCALE)
    }

    pub const fn natural_minor(root: PitchNote) -> Self {
        Self::new(root, NATURAL_MINOR_SCALE)
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

            Some(mem::replace(
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

/// ```
/// use music::note::{Letter, PitchNote};
/// use music::scale::{scale, MAJOR_SCALE};
///
/// const C_MAJOR: [PitchNote; 7] = scale(PitchNote::natural(Letter::C), MAJOR_SCALE);
/// assert_eq!(
///     C_MAJOR,
///     [
///         PitchNote::natural(Letter::C),
///         PitchNote::natural(Letter::D),
///         PitchNote::natural(Letter::E),
///         PitchNote::natural(Letter::F),
///         PitchNote::natural(Letter::G),
///         PitchNote::natural(Letter::A),
///         PitchNote::natural(Letter::B)
///     ]
/// )
/// ```
pub const fn scale<const LEN: usize>(
    root: PitchNote,
    intervals: [Interval; LEN],
) -> [PitchNote; LEN] {
    let mut array: [MaybeUninit<PitchNote>; LEN] = MaybeUninit::uninit_array();

    let mut scale = Scale::new(root, &intervals);

    let mut i = 0;
    while let Some(note) = scale.next_note() {
        array[i] = MaybeUninit::new(note);
        i += 1;
    }

    unsafe { (&array as *const _ as *const [PitchNote; LEN]).read() }
}
