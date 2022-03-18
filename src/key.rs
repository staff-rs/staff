use crate::{
    chord::{Chord, ChordKind},
    midi::{MidiNote, Octave},
    note::PitchNote,
    scale::Scale,
};

pub struct Key {
    octave: Octave,
    scale: Scale<'static>,
    kinds: [ChordKind; 7],
}

impl Key {
    pub fn new(octave: Octave, scale: Scale<'static>, kinds: [ChordKind; 7]) -> Self {
        Self {
            octave,
            scale,
            kinds,
        }
    }

    pub fn major(root: MidiNote) -> Self {
        Self::new(
            root.octave(),
            Scale::major(PitchNote::from_sharp(root.pitch())),
            [
                ChordKind::MajorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MajorSeventh,
                ChordKind::DominantSeventh,
                ChordKind::MinorSeventh,
                ChordKind::HalfDiminished,
            ],
        )
    }

    pub fn natural_minor(root: MidiNote) -> Self {
        Self::new(
            root.octave(),
            Scale::natural_minor(PitchNote::from_sharp(root.pitch())),
            [
                ChordKind::MajorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MinorSeventh,
                ChordKind::MajorSeventh,
                ChordKind::DominantSeventh,
                ChordKind::MinorSeventh,
                ChordKind::HalfDiminished,
            ],
        )
    }

    pub fn chords(self) -> impl Iterator<Item = Chord> {
        self.scale
            .zip(self.kinds.into_iter())
            .map(move |(note, kind)| Chord::new(MidiNote::new(note.pitch(), self.octave), kind))
    }
}

#[cfg(test)]
mod tests {
    use crate::{midi::Octave, Pitch};

    use super::*;

    #[test]
    fn f() {
        let key = Key::major(MidiNote::new(Pitch::C, Octave::FOUR));
        for chord in key.chords() {
            dbg!(chord);
        }
    }
}
