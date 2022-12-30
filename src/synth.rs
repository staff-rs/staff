use crate::midi::MidiNote;
use rand::{thread_rng, Rng};
use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::time::Duration;

pub struct Guitar {
    sample_rate: u32,
    signal: Box<[f32]>,
    pos: usize,
}

impl Guitar {
    pub fn new(freq: f32, sample_rate: u32) -> Self {
        let period = (sample_rate as f32 / freq).round() as usize;
        let mut rng = thread_rng();
        let signal = std::iter::repeat_with(|| rng.gen_range(-1.0..1.0))
            .take(period)
            .collect();

        Self {
            sample_rate,
            signal,
            pos: 0,
        }
    }
}

impl Iterator for Guitar {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let output = (self.signal[self.pos] + self.signal[(self.pos + 1) % self.signal.len()]) / 2.;
        self.signal[self.pos] = output;

        self.pos += 1;
        if self.pos >= self.signal.len() {
            self.pos = 0;
        }

        Some(output)
    }
}

impl Source for Guitar {
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

pub struct Chord {
    sine_waves: Vec<SineWave>,
    num_sample: usize,
    num_spacing_samples: usize,
}

impl Chord {
    pub fn new(sine_waves: Vec<SineWave>, spacing_duration: Duration) -> Self {
        let spacing_nanos =
            spacing_duration.as_secs() * 1_000_000_000 + spacing_duration.subsec_nanos() as u64;
        let num_spacing_samples = spacing_nanos / 1_000_000 * 48;

        Self {
            sine_waves,
            num_sample: 0,
            num_spacing_samples: num_spacing_samples as _,
        }
    }
}

impl Iterator for Chord {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let count = self
            .num_sample
            .checked_div(self.num_spacing_samples)
            .unwrap_or_default();
        self.num_sample += 1;
        Some(
            self.sine_waves
                .iter_mut()
                .take(count + 1)
                .map(|sine_wave| sine_wave.next().unwrap())
                .sum(),
        )
    }
}

impl Source for Chord {
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
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        // TODO
        None
    }
}

pub fn chord(midi_notes: impl IntoIterator<Item = MidiNote>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = Chord::new(
        midi_notes
            .into_iter()
            .map(|midi_note| SineWave::new(midi_note.frequency() as _))
            .collect(),
        Duration::from_millis(200),
    )
    .take_duration(Duration::from_secs_f32(2.))
    .amplify(0.20);

    sink.append(Guitar::new(440., 48_000));

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
