use crate::Interval;
use core::ops::Sub;

mod builder;
pub use builder::{Builder, ChordAccidental, Third};

mod kind;
pub use kind::ChordKind;

#[derive(Clone, Copy, Debug)]
pub struct Chord<T> {
    root: T,
    builder: Builder<T>,
}

impl<T> Chord<T> {
    pub fn builder() -> Builder<T> {
        Builder::default()
    }

    pub fn empty(root: T) -> Self {
        Self::builder().build(root)
    }

    pub fn matches<I>(root: T, notes: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: PartialOrd + Sub<Output = Interval> + Clone,
    {
        let mut me = Self::empty(root);

        for note in notes {
            if note > me.root {
                let interval = note - me.root.clone();
                match interval {
                    Interval::UNISON => {
                        me.builder.no_root = false;
                    }
                    Interval::MAJOR_SECOND => {
                        me.builder.third = Third::Sus2;
                    }
                    Interval::MINOR_THIRD => {
                        me.builder.third = Third::Minor;
                    }
                    Interval::MAJOR_THIRD => me.builder.third = Third::Major,
                    Interval::PERFECT_FOURTH => {
                        me.builder.third = Third::Sus4;
                    }
                    Interval::PERFECT_FIFTH => {
                        me.builder.fifth = Some(ChordAccidental::Natural);
                    }
                    Interval::MINOR_SEVENTH => me.builder.seventh = Some(ChordAccidental::Flat),
                    Interval::MAJOR_SEVENTH => me.builder.seventh = Some(ChordAccidental::Natural),
                    _ => todo!(),
                }
            } else {
                me.builder.bass = Some(note);
            }
        }

        me
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        midi::{MidiNote, Octave},
        Pitch,
    };

    #[test]
    fn f() {
        dbg!(super::Chord::matches(
            MidiNote::new(Pitch::C, Octave::FOUR),
            [
                MidiNote::new(Pitch::C, Octave::FOUR),
                MidiNote::new(Pitch::E, Octave::FOUR),
                MidiNote::new(Pitch::G, Octave::FOUR)
            ]
        ));
    }
}
