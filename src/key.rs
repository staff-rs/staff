use crate::{Interval, Pitch};
use core::fmt::{self, Write};

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
