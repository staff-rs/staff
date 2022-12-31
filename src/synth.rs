use rand::{thread_rng, Rng};
use rodio::Source;
use std::{borrow::BorrowMut, time::Duration};

pub struct GuitarString<T> {
    frequencies: T,
    index: Index,
}

impl<T> GuitarString<T>
where
    T: AsMut<[f32]>,
{
    pub fn new(frequencies: T, index: Index) -> Self {
        Self { frequencies, index }
    }
}

impl<T> Iterator for GuitarString<T>
where
    T: AsMut<[f32]>,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let frequencies = self.frequencies.as_mut();
        let output = (frequencies[self.index.pos]
            + frequencies[(self.index.pos + 1) % frequencies.len()])
            / 2.;
        frequencies[self.index.pos] = output;

        self.index.pos += 1;
        if self.index.pos >= frequencies.len() {
            self.index.pos = 0;
        }

        Some(output)
    }
}

pub struct Chord<T> {
    iter: T,
    num_sample: usize,
    num_spacing_samples: usize,
}

impl<T: Iterator<Item = f32>> Iterator for Chord<T> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self
            .num_sample
            .checked_div(self.num_spacing_samples)
            .unwrap_or_default();
        self.num_sample += 1;

        Some(self.iter.borrow_mut().take(count).sum())
    }
}
#[derive(Clone)]
pub struct Index {
    pos: usize,
    len: usize,
}

pub struct GuitarChord {
    frequencies: Vec<f32>,
    strings: Vec<Index>,
    sample_rate: u32,
    num_sample: usize,
    num_spacing_samples: usize,
}

impl GuitarChord {
    pub fn new(sample_rate: u32, spacing_duration: Duration) -> Self {
        let spacing_nanos = spacing_duration.as_secs() as u32 * 1_000_000_000
            + spacing_duration.subsec_nanos() as u32;
        let num_spacing_samples = spacing_nanos / 1_000_000 * (sample_rate / 1_000);

        Self {
            frequencies: Vec::new(),
            strings: Vec::new(),
            sample_rate,
            num_sample: 0,
            num_spacing_samples: num_spacing_samples as _,
        }
    }

    pub fn set_frequencies(&mut self, freqs: impl IntoIterator<Item = f32>) {
        self.strings.clear();

        for freq in freqs {
            let period = (self.sample_rate as f32 / freq).round() as usize;
            self.strings.push(Index {
                pos: 0,
                len: period,
            });

            let mut rng = thread_rng();
            for _ in 0..period {
                let noise = rng.gen_range(-1.0..1.0);
                self.frequencies.push(noise);
            }
        }
    }
}

impl Iterator for GuitarChord {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self
            .num_sample
            .checked_div(self.num_spacing_samples)
            .unwrap_or_default();
        self.num_sample += 1;

        let mut start = 0;
        let sum = self
            .strings
            .iter_mut()
            .take(count)
            .map(|index| {
                let frequencies = &mut self.frequencies[start..start + index.len];
                start += index.len;

                let mut guitar_string = GuitarString::new(frequencies, index.clone());
                let output = guitar_string.next().unwrap();
                index.pos = guitar_string.index.pos;
                output
            })
            .sum();
        Some(sum)
    }
}

impl Source for GuitarChord {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        // TODO
        None
    }
}
