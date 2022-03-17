mod interval;
pub use interval::Interval;
use note::pitch_note::PitchNote;
use pitch::Pitch;

pub mod note;

pub mod pitch;

pub mod scale;

pub fn transpose(key: Pitch, note: Pitch, to: Pitch) -> Pitch {
    let f = key - note;
    to + f
}
