
use rodio::Source;
use std::{time::Duration};

mod guitar;
pub use guitar::{GuitarChord, GuitarString};

pub trait Frequencies {
    type Frequencies<'a>: Iterator<Item = f32>
    where
        Self: 'a;

    fn frequencies<'a>(&'a mut self) -> Self::Frequencies<'a>;
}

pub struct Chord<T> {
    frequencies: T,
    sample_rate: u32,
    num_sample: u32,
    num_spacing_samples: u32,
}

impl<T> Chord<T> {
    pub fn new(sample_rate: u32, spacing_duration: Duration, frequencies: T) -> Self {
        let spacing_nanos = spacing_duration.as_secs() as u32 * 1_000_000_000
            + spacing_duration.subsec_nanos() as u32;
        let num_spacing_samples = spacing_nanos / 1_000_000 * (sample_rate / 1_000);

        Self {
            frequencies,
            sample_rate,
            num_sample: 0,
            num_spacing_samples: num_spacing_samples as _,
        }
    }
}

impl<T> Iterator for Chord<T>
where
    T: Frequencies,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self
            .num_sample
            .checked_div(self.num_spacing_samples)
            .unwrap_or_default();
        self.num_sample += 1;

        Some(self.frequencies.frequencies().take(count as _).sum())
    }
}

impl<T> Source for Chord<T>
where
    T: Frequencies,
{
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
