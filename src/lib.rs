//! Music theory library with midi, notes, chords, scales, and more
//!
//! # Examples
//!
//! Create a C Major (1st inversion) chord and iterate over its notes.
//! ```
//! use staff::{midi, Chord, Pitch};
//!
//! // C/E
//! let notes = [midi!(E, 3), midi!(G, 3), midi!(C, 4)];
//! let chord = Chord::from_midi(midi!(C, 4), notes).unwrap();
//!
//! assert_eq!(chord.to_string(), "C/E");
//!
//! assert!(chord.into_iter().eq(notes));
//! ```
//!
//! Create a C Major scale and iterate over its notes.
//! ```
//! use staff::{midi, Note, Scale};
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

// TODO alloc
//#![cfg_attr(not(test), no_std)]

pub mod chord;
pub use chord::Chord;

pub mod duration;

pub mod fmt;
pub use fmt::Format;

pub mod fretboard;

mod interval;
pub use interval::Interval;

mod key;
pub use crate::key::Key;

#[cfg(feature = "svg")]
pub mod parse;

mod natural;
pub use natural::Natural;

pub mod midi;

pub mod note;
pub use note::Note;

mod pitch;
pub use pitch::Pitch;

#[cfg(feature = "render")]
pub mod render;

pub mod scale;
pub use scale::Scale;

pub mod set;

#[cfg(feature = "synth")]
pub mod synth;

/// ```
/// use staff::{midi, Pitch};
/// use staff::midi::Octave;
///
/// let midi = midi!(C, 4);
///
/// assert_eq!(midi.pitch(), Pitch::C);
/// assert_eq!(midi.octave(), Octave::FOUR);
/// ```
#[macro_export]
macro_rules! midi {
    ($pitch:ident, $octave:literal) => {
        staff::midi::MidiNote::new(
            staff::Pitch::$pitch,
            staff::midi::Octave::new_unchecked($octave),
        )
    };
}
