use crate::{Interval, Pitch};
use core::fmt::{self, Write};

/// A key signature represented as the total number of sharps or flats.
#[derive(Clone, Copy, Debug)]
pub struct Key {
    sharps: u8,
}

impl Key {
    /// ```
    /// use music_note::{KeySignature, Pitch};
    ///
    /// let key = KeySignature::major(Pitch::D);
    /// assert_eq!(key.to_string(), "##")
    /// ```
    ///
    /// ```
    /// use music_note::{KeySignature, Pitch};
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

    /// Returns the number of sharps in this key.
    pub fn sharps(self) -> u8 {
        self.sharps
    }

    /// Returns the number of flats in this key.
    pub fn flats(self) -> u8 {
        Pitch::B.into_byte() - self.sharps
    }
}

impl fmt::Display for Key {
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
