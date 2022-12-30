use crate::midi::MidiNote;
use rodio::{source::SineWave, OutputStream, Sink, Source};
use std::time::Duration;

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

    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
