#![feature(const_mut_refs)]
#![feature(const_replace)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_ptr_read)]

pub mod chord;

mod interval;
pub use interval::Interval;

pub mod midi;

pub mod note;

pub mod pitch;
pub use pitch::Pitch;

pub mod scale;

pub fn transpose(key: Pitch, note: Pitch, to: Pitch) -> Pitch {
    let f = key - note;
    to + f
}

#[derive(Default, Debug, PartialEq, Eq)]
pub struct PitchSet {
    pitches: u16,
}

impl PitchSet {
    /// ```
    /// use music::{Pitch, PitchSet};
    ///
    /// let ps: PitchSet = [Pitch::C, Pitch::D].into_iter().collect();
    /// ```
    pub fn push(&mut self, pitch: Pitch) {
        self.pitches |= 1 << pitch.into_byte() as u16;
    }
}

impl FromIterator<Pitch> for PitchSet {
    fn from_iter<T: IntoIterator<Item = Pitch>>(iter: T) -> Self {
        let mut pitch_set = Self::default();
        for pitch in iter {
            pitch_set.push(pitch);
        }
        pitch_set
    }
}

impl Iterator for PitchSet {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pitches != 0 {
            let trailing = self.pitches.trailing_zeros();
            self.pitches &= self.pitches - 1;
            Some(Pitch::from_byte(trailing as u8))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f() {
        let ps: PitchSet = [Pitch::C, Pitch::D].into_iter().collect();
        for p in ps {
            dbg!(p);
        }
    }
}
