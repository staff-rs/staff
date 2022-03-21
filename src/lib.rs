#![cfg_attr(not(test), no_std)]
#![feature(const_mut_refs)]
#![feature(const_replace)]
#![feature(maybe_uninit_uninit_array)]
#![feature(const_ptr_read)]

pub mod chord;

mod interval;
pub use interval::Interval;

pub mod key;

mod letter;
pub use letter::Letter;

pub mod midi;

pub mod note;
pub use note::{Accidental, Note};

mod pitch;
pub use pitch::Pitch;

pub mod scale;

mod set;
pub use set::Set;
