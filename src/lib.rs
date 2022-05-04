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
