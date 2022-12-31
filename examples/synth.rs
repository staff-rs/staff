use rodio::{OutputStream, Sink, Source};
use staff::{midi, synth::ChordSource, Chord};
use std::time::Duration;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let chord = Chord::major(midi!(C, 4));
    let frequencies = chord
        .into_iter()
        .map(|midi_note| midi_note.frequency() as _);

    let sample_rate = 48_000;
    let spacing_duration = Duration::from_millis(200);
    let source = ChordSource::guitar(sample_rate, spacing_duration, frequencies);
    
    sink.append(
        source
            .take_duration(Duration::from_secs_f32(3.))
            .amplify(0.20),
    );

    sink.sleep_until_end();
}
