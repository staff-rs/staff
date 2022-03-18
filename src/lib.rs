#![feature(const_mut_refs)]
#![feature(const_replace)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_ptr_read)]

pub mod chord;

mod interval;
pub use interval::Interval;

pub mod key;

pub mod midi;

pub mod note;

mod pitch;

pub use pitch::Pitch;

pub mod scale;

pub mod set;

pub fn transpose(key: Pitch, note: Pitch, to: Pitch) -> Pitch {
    let f = key - note;
    to + f
}
