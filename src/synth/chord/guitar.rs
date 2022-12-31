use std::slice;
use rand::{thread_rng, Rng};
use crate::synth::{guitar::Index, GuitarSource};
use super::Frequencies;

#[derive(Clone, Debug, Default)]
pub struct GuitarChord {
    frequencies: Vec<f32>,
    strings: Vec<Index>,
}

impl GuitarChord {


    pub fn set_frequencies(&mut self, sample_rate: u32, freqs: impl IntoIterator<Item = f32>) {
        self.strings.clear();

        for freq in freqs {
            let period = (sample_rate as f32 / freq).round() as usize;
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

pub struct GuitarChordFrequencies<'a> {
    frequencies: &'a mut [f32],
    indices: slice::IterMut<'a, Index>,
    start: usize,
}

impl Iterator for GuitarChordFrequencies<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(index) = self.indices.next() {
            let frequencies = &mut self.frequencies[self.start..self.start + index.len];
            self.start += index.len;

            let mut guitar_string = GuitarSource::new(frequencies, index.clone());
            let output = guitar_string.next().unwrap();
            index.pos = guitar_string.index.pos;
            Some(output)
        } else {
            None
        }
    }
}

impl Frequencies for GuitarChord {
    type Frequencies<'a> = GuitarChordFrequencies<'a>;

    fn frequencies<'a>(&'a mut self) -> Self::Frequencies<'a> {
        GuitarChordFrequencies {
            frequencies: &mut self.frequencies,
            indices: self.strings.iter_mut(),
            start: 0,
        }
    }
}
