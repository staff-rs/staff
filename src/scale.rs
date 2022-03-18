use crate::{
    midi::MidiNote,
    note::{Accidental, Note, PitchNote},
    pitch::Pitch,
    Interval,
};
use core::mem::{self};
use std::array::IntoIter;

pub const MAJOR_SCALE: [Interval; 7] = [
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
];

pub const NATURAL_MINOR_SCALE: [Interval; 7] = [
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MINOR_SECOND,
    Interval::MAJOR_SECOND,
    Interval::MAJOR_SECOND,
];

pub trait ScaleItem: Copy {
    fn next_scale_item(self, interval: Interval) -> Self
    where
        Self: Sized;
}

impl ScaleItem for PitchNote {
    fn next_scale_item(self, interval: Interval) -> Self
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

impl ScaleItem for MidiNote {
    fn next_scale_item(self, interval: Interval) -> Self
    where
        Self: Sized,
    {
        self + interval
    }
}

pub struct Scale<T, I> {
    item: T,
    intervals: I,
}

impl<T, I> Scale<T, I>
where
    T: ScaleItem,
    I: Iterator<Item = Interval>,
{
    pub fn new(root: T, intervals: I) -> Self {
        Self {
            item: root,
            intervals,
        }
    }
}

impl<T: ScaleItem> Scale<T, IntoIter<Interval, 7>> {
    pub fn major(root: T) -> Self {
        Self::new(root, MAJOR_SCALE.into_iter())
    }

    pub fn natural_minor(root: T) -> Self {
        Self::new(root, NATURAL_MINOR_SCALE.into_iter())
    }
}

impl<T, I> Iterator for Scale<T, I>
where
    T: ScaleItem,
    I: Iterator<Item = Interval>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(interval) = self.intervals.next() {
            let next = self.item.next_scale_item(interval);
            Some(mem::replace(&mut self.item, next))
        } else {
            None
        }
    }
}
