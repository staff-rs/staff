use super::Frequencies;
use rodio::source::SineWave;
use std::slice;

#[derive(Clone, Debug, Default)]
pub struct SineWaveChord {
    sine_waves: Box<[SineWave]>,
}

impl FromIterator<f32> for SineWaveChord {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        let mut sine_waves = Vec::new();
        for frequency in iter {
            sine_waves.push(SineWave::new(frequency));
        }

        Self {
            sine_waves: sine_waves.into_boxed_slice(),
        }
    }
}

pub struct SineWaveChordFrequencies<'a> {
    iter: slice::IterMut<'a, SineWave>,
}

impl Iterator for SineWaveChordFrequencies<'_> {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|sine_wave| sine_wave.next().unwrap())
    }
}

impl Frequencies for SineWaveChord {
    type Frequencies<'a> = SineWaveChordFrequencies<'a>;

    fn frequencies<'a>(&'a mut self) -> Self::Frequencies<'a> {
        SineWaveChordFrequencies {
            iter: self.sine_waves.iter_mut(),
        }
    }
}
