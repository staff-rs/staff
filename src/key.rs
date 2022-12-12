//! Key signatures

use crate::{Interval, Natural, Pitch};
use core::fmt::{self, Write};

/// A key signature represented as the total number of sharps or flats.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key {
    /// The number of sharps in this key
    pub sharps: u8,
}

impl Key {
    /// ```
    /// use staff::{Key, Pitch};
    ///
    /// let key = Key::major(Pitch::D);
    /// assert_eq!(key.to_string(), "##")
    /// ```
    ///
    /// ```
    /// use staff::{Key, Pitch};
    ///
    /// let key = Key::major(Pitch::F);
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

    /// Returns the number of flats in this key.
    pub fn flats(self) -> u8 {
        Pitch::B.into_byte() - self.sharps
    }

    pub fn is_sharp(self) -> bool {
        self.sharps <= 6
    }
}

impl IntoIterator for Key {
    type Item = Natural;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        let (natural, remaining, semitones) = if self.sharps <= 6 {
            (Natural::F, self.sharps, 4)
        } else {
            (Natural::B, self.flats(), -4)
        };
        Iter {
            natural,
            remaining,
            step: semitones,
        }
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

pub struct Iter {
    natural: Natural,
    remaining: u8,
    step: i8,
}

impl Iterator for Iter {
    type Item = Natural;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining > 0 {
            let natural = self.natural;

            self.natural = natural + self.step;
            self.remaining -= 1;

            Some(natural)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_iters_sharps_for_d_major() {
        let key = Key::major(Pitch::D);
        let sharps: Vec<_> = key.into_iter().collect();
        assert_eq!(sharps, [Natural::F, Natural::C]);
    }
}
