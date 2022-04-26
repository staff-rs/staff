use crate::scale::Degree;
use crate::{scale::Scale, Interval, Pitch};
use core::fmt;
use core::ops::Add;
use core::{array::IntoIter, fmt::Write};

/*
pub struct KeySignature {
    sharps: u8,
}

impl KeySignature {
    /// ```
    /// use music_note::key::KeySignature;
    /// use music_note::Pitch;
    ///
    /// let key = KeySignature::major(Pitch::D);
    /// assert_eq!(key.to_string(), "##")
    /// ```
    ///
    /// ```
    /// use music_note::key::KeySignature;
    /// use music_note::Pitch;
    ///
    /// let key = KeySignature::major(Pitch::F);
    /// assert_eq!(key.to_string(), "b")
    /// ```
    pub fn major(root: Pitch) -> Self {
        let mut pitch = Pitch::C;
        let mut alteration = 0;
        while pitch != root {
            pitch = pitch + Interval::PERFECT_FIFTH;
            alteration += 1;
        }

        Self { sharps: alteration }
    }

    pub fn sharps(self) -> u8 {
        self.sharps
    }

    pub fn flats(self) -> u8 {
        Pitch::B.into_byte() - self.sharps
    }
}

impl fmt::Display for KeySignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.sharps <= 6 {
            for _ in 0..self.sharps {
                f.write_char('#')?;
            }
        } else {
            for _ in self.sharps..=Pitch::B.into_byte() {
                f.write_char('b')?;
            }
        }

        Ok(())
    }
}

pub struct Key<T> {
    scale: Scale<T, IntoIter<Interval, 7>>,
    kinds: [ChordKind; 7],
}

impl<T> Key<T>
where
    T: Degree + Add<Interval>,
{
    pub fn new(scale: Scale<T, IntoIter<Interval, 7>>, kinds: [ChordKind; 7]) -> Self {
        Self { scale, kinds }
    }

    pub fn major(root: T) -> Self {
        Self::new(
            Scale::major(root),
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

    pub fn natural_minor(root: T) -> Self {
        Self::new(
            Scale::natural_minor(root),
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

    /*
    pub fn chords(self) -> impl Iterator<Item = Chord<T>> {
        self.scale
            .zip(self.kinds.into_iter())
            .map(move |(note, kind)| Chord::new(note, kind))
    }
    */
}

 */
