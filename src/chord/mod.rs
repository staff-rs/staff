use crate::{midi::MidiNote, Interval};

mod kind;
pub use kind::ChordKind;

#[derive(Clone, Copy, Debug)]
pub enum ChordAccidental {
    Natural,
    Sharp,
    Flat,
}

#[derive(Clone, Copy, Debug)]
pub enum Seventh {
    Major,
    Minor,
}

#[derive(Clone, Copy, Debug)]
pub enum Third {
    Major,
    Minor,
    Sus2,
    Sus4,
}

#[derive(Clone, Copy, Debug)]
pub struct Builder {
    bass: Option<MidiNote>,
    no_root: bool,
    third: Third,
    fifth: Option<ChordAccidental>,
    sixth: bool,
    seventh: Option<ChordAccidental>,
    ninth: Option<ChordAccidental>,
    eleventh: Option<ChordAccidental>,
    thirteenth: Option<ChordAccidental>
}

impl Default for Builder {
    fn default() -> Self {
        Self {
            bass: None,
            no_root: true,
            third: Third::Major,
            fifth: None,
            sixth: false,
            seventh: None,
            ninth: None,
            eleventh: None,
            thirteenth: None
        }
    }
}

impl Builder {
    pub fn bass(mut self, bass: MidiNote) -> Self {
        self.bass = Some(bass);
        self
    }

    pub fn no_root(mut self) -> Self {
        self.no_root = true;
        self
    }

    pub fn third(mut self, third: Third) -> Self {
        self.third = third;
        self
    }

    pub fn major(self) -> Self {
        self.third(Third::Major)
    }

    pub fn minor(self) -> Self {
        self.third(Third::Minor)
    }

    pub fn sus2(self) -> Self {
        self.third(Third::Sus2)
    }

    pub fn sus4(self) -> Self {
        self.third(Third::Sus4)
    }

    pub fn build(self, root: MidiNote) -> Chord {
        Chord {
            root,
            builder: self,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Chord {
    root: MidiNote,
    builder: Builder,
}

impl Chord {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn empty(root: MidiNote) -> Self {
        Self::builder().build(root)
    }

    pub fn matches<I>(root: MidiNote, notes: I) -> Self
    where
        I: IntoIterator<Item = MidiNote>,
    {
        let mut me = Self::empty(root);

        for note in notes {
            if note > root {
                let interval = note - root;
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
