//! Music theory library with midi, notes, chords, scales, and more
//!
//! ## Feature flags
//!
//! Staff uses a set of [feature flags] to reduce the amount of compiled code. It
//! is possible to just enable certain features over others. By default, staff
//! does not enable any features but allows one to enable a subset for their use
//! case. Below is a list of the available feature flags. You may also notice
//! above each function, struct and trait there is listed one or more feature flags
//! that are required for that item to be used. If you are new to staff it is
//! recommended that you use the `full` feature flag which will enable all public APIs.
//! Beware though that this will pull in many extra dependencies that you may not
//! need.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//!
//! - `full`: Enables all features listed below.
//! - `std`: Enables std, otherwise this crate will use `#![no_std]`
//! - `parse` Enables the `staff::parse` module.
//! - `fretboard` Enables the `staff::fretboard` module.
//! - `render` Enables the `staff::render` module.
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

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod chord;
pub use chord::Chord;

pub mod time;

pub mod fmt;
pub use fmt::Format;

#[cfg_attr(docsrs, doc(cfg(feature = "fretboard")))]
#[cfg(feature = "fretboard")]
pub mod fretboard;

mod interval;
pub use interval::Interval;

mod key;
pub use crate::key::Key;

mod natural;
pub use natural::Natural;

pub mod midi;

pub mod note;
pub use note::Note;

mod pitch;
pub use pitch::Pitch;

#[cfg_attr(docsrs, doc(cfg(feature = "render")))]
#[cfg(feature = "render")]
pub mod render;

pub mod scale;
pub use scale::Scale;

pub mod set;
pub use set::Set;

#[cfg_attr(docsrs, doc(cfg(feature = "synth")))]
#[cfg(feature = "synth")]
pub mod synth;

#[cfg_attr(docsrs, doc(cfg(feature = "ui")))]
#[cfg(feature = "ui")]
pub mod ui;

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
