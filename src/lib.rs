//! Music theory library with midi, notes, chords, scales, and more
//!
//! # Examples
//!
//! Create a C Major (1st inversion) chord and iterate over its notes.
//! ```
//! use music_note::{midi, Chord, Pitch};
//!
//! // C/E
//! let chord = Chord::from_midi(
//!     midi!(C, 4),
//!     [midi!(E, 3), midi!(G, 3), midi!(C, 4)]
//! );
//!
//! assert_eq!(chord.to_string(), "C/E");
//!
//! let pitches = [Pitch::E, Pitch::G, Pitch::C];
//! assert!(chord.into_iter().eq(pitches));
//! ```
//!
//! Create a C Major scale and iterate over its notes.
//! ```
//! use music_note::{midi, Note, Scale};
//!
//! // C major
//! let scale = Scale::major(midi!(C, 4));
//!
//! assert!(scale.eq([
//!     midi!(C, 4),
//!     midi!(D, 4),
//!     midi!(E, 4),
//!     midi!(F, 4),
//!     midi!(G, 4),
//!     midi!(A, 4),
//!     midi!(B, 4),
//! ]));
//! ```

#![cfg_attr(not(test), no_std)]

pub mod chord;
pub use chord::Chord;

mod interval;
pub use interval::Interval;

mod key;
pub use key::KeySignature;

mod natural;
pub use natural::Natural;

pub mod midi;

pub mod note;
pub use note::Note;

mod pitch;
pub use pitch::Pitch;

pub mod scale;
pub use scale::Scale;

pub mod set;

/// ```
/// use music_note::{midi, Pitch};
/// use music_note::midi::Octave;
///
/// let midi = midi!(C, 4);
///
/// assert_eq!(midi.pitch(), Pitch::C);
/// assert_eq!(midi.octave(), Octave::FOUR);
/// ```
#[macro_export]
macro_rules! midi {
    ($pitch:ident, $octave:literal) => {
        music_note::midi::MidiNote::new(
            music_note::Pitch::$pitch,
            music_note::midi::Octave::new_unchecked($octave),
        )
    };
}
