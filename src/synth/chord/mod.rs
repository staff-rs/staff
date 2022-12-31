use rodio::Source;
use std::{marker::PhantomData, time::Duration};

pub mod guitar;
pub use guitar::GuitarChord;

pub mod sine_wave;
pub use sine_wave::SineWaveChord;

pub trait Frequencies {
    type Frequencies<'a>: Iterator<Item = f32>
    where
        Self: 'a;

    fn frequencies<'a>(&'a mut self) -> Self::Frequencies<'a>;
}

/// A sound [`Source`] to play a chord or arpeggio.
pub struct ChordSource<T> {
    frequencies: T,
    sample_rate: u32,
    num_sample: u32,
    num_spacing_samples: u32,
}

impl<T> ChordSource<T>
where
    T: Frequencies,
{
    /// Create a new `ChordSource`.
    /// * `sample_rate`: the sample rate to play the chord (in hz)
    /// * `spacing_duration`: the duration between notes in order to arpeggiate them
    /// * `frequencies`: an [`Iterator`] of output frequencies
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

    pub fn builder() -> Builder<T> {
        Builder::default()
    }
}

impl ChordSource<GuitarChord> {
    /// Create a new guitar `ChordSource`.
    /// * `sample_rate`: the sample rate to play the chord (in hz)
    /// * `spacing_duration`: the duration between notes in order to arpeggiate them
    /// * `frequencies`: an [`Iterator`] of note frequencies
    pub fn guitar(
        sample_rate: u32,
        spacing_duration: Duration,
        freqs: impl IntoIterator<Item = f32>,
    ) -> Self {
        let mut guitar_chord = GuitarChord::default();
        guitar_chord.set_frequencies(sample_rate, freqs);
        Self::new(sample_rate, spacing_duration, guitar_chord)
    }
}

impl ChordSource<SineWaveChord> {
    /// Create a new sine wave `ChordSource`.
    /// * `sample_rate`: the sample rate to play the chord (in hz)
    /// * `spacing_duration`: the duration between notes in order to arpeggiate them
    /// * `frequencies`: an [`Iterator`] of note frequencies
    pub fn sine_waves(
        sample_rate: u32,
        spacing_duration: Duration,
        freqs: impl IntoIterator<Item = f32>,
    ) -> Self {
        let sine_waves = SineWaveChord::from_iter(freqs);
        Self::new(sample_rate, spacing_duration, sine_waves)
    }
}

impl<T> Iterator for ChordSource<T>
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

impl<T> Source for ChordSource<T>
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

#[derive(Clone, Debug)]
pub struct Builder<T> {
    sample_rate: u32,
    spacing_duration: Duration,
    _marker: PhantomData<T>,
}

impl<T> Default for Builder<T>
where
    T: Frequencies,
{
    fn default() -> Self {
        Self {
            sample_rate: 48_000,
            spacing_duration: Duration::ZERO,
            _marker: PhantomData,
        }
    }
}

impl<T> Builder<T>
where
    T: Frequencies,
{
    pub fn sample_rate(mut self, sample_rate: u32) -> Self {
        self.sample_rate = sample_rate;
        self
    }

    pub fn spacing_duration(mut self, spacing_duration: Duration) -> Self {
        self.spacing_duration = spacing_duration;
        self
    }

    /// Build a new `ChordSource`.
    /// * `frequencies`: an [`Iterator`] of output frequencies
    pub fn build(self, frequencies: T) -> ChordSource<T> {
        ChordSource::new(self.sample_rate, self.spacing_duration, frequencies)
    }
}

impl Builder<GuitarChord> {
    /// Build a new guitar `ChordSource`.
    /// * `frequencies`: an [`Iterator`] of note frequencies
    pub fn build_guitar(
        self,
        frequencies: impl IntoIterator<Item = f32>,
    ) -> ChordSource<GuitarChord> {
        let mut guitar_chord = GuitarChord::default();
        guitar_chord.set_frequencies(self.sample_rate, frequencies);
        self.build(guitar_chord)
    }
}

impl Builder<SineWaveChord> {
    /// Build a new sine wave `ChordSource`.
    /// * `frequencies`: an [`Iterator`] of note frequencies
    pub fn build_sine_waves(
        self,
        frequencies: impl IntoIterator<Item = f32>,
    ) -> ChordSource<SineWaveChord> {
        let sine_wave_chord = SineWaveChord::from_iter(frequencies);
        self.build(sine_wave_chord)
    }
}
