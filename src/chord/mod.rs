use crate::{midi::MidiNote, Accidental, Interval};
use core::ops::{Add, Sub};

mod kind;
pub use kind::ChordKind;

#[derive(Debug)]
pub enum ChordAccidental {
    Natural,
    Sharp,
    Flat,
}

#[derive(Debug)]
pub enum Seventh {
    Major,
    Minor,
}

#[derive(Debug)]
pub enum Third {
    Major,
    Minor,
    Sus2,
    Sus4,
}

#[derive(Debug)]
pub struct Chord {
    root: MidiNote,
    bass: Option<MidiNote>,
    no_root: bool,
    third: Third,
    fifth: Option<ChordAccidental>,
    sixth: bool,
    seventh: Option<ChordAccidental>,
    ninth: Option<ChordAccidental>,
    eleventh: Option<ChordAccidental>,
}

impl Chord {
    pub fn m<I>(root: MidiNote, notes: I) -> Self
    where
        I: IntoIterator<Item = MidiNote>,
    {
        let mut me = Self {
            root,
            bass: None,
            no_root: true,
            third: Third::Major,
            fifth: None,
            sixth: false,
            seventh: None,
            ninth: None,
            eleventh: None,
        };

        for note in notes {
            if note > root {
                let interval = note - root;
                match interval {
                    Interval::UNISON => {
                        me.no_root = false;
                    }
                    Interval::MAJOR_SECOND => {
                        me.third = Third::Sus2;
                    }
                    Interval::MINOR_THIRD => {
                        me.third = Third::Minor;
                    }
                    Interval::MAJOR_THIRD => me.third = Third::Major,
                    Interval::PERFECT_FOURTH => {
                        me.third = Third::Sus4;
                    }
                    Interval::PERFECT_FIFTH => {
                        me.fifth = Some(ChordAccidental::Natural);
                    }
                    Interval::MINOR_SEVENTH => me.seventh = Some(ChordAccidental::Flat),
                    Interval::MAJOR_SEVENTH => me.seventh = Some(ChordAccidental::Natural),
                    _ => todo!(),
                }
            } else {
                me.bass = Some(note);
            }
        }

        me
    }
}

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    use crate::{
        midi::{MidiNote, Octave},
        Pitch,
    };

    #[test]
    fn f() {
        dbg!(super::Chord::m(
            MidiNote::new(Pitch::C, Octave::FOUR),
            [
                MidiNote::new(Pitch::C, Octave::FOUR),
                MidiNote::new(Pitch::E, Octave::FOUR),
                MidiNote::new(Pitch::G, Octave::FOUR)
            ]
        ));
    }
}
