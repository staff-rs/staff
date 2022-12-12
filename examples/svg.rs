use staff::note::Accidental;
use staff::render::{Chord, Duration, Note, Renderer};
use staff::{midi::Octave, Natural};
use staff::{Key, Pitch};

fn main() {
    let mut document = svg::Document::new();

    let mut renderer = Renderer::default();
    renderer.key_signature = Some(Key::major(Pitch::D));

    let chords = [
        Chord::new(
            &[Note::new(Natural::C, Octave::FOUR, Some(Accidental::Sharp))],
            Duration::Quarter,
            &renderer,
        ),
        Chord::new(&[], Duration::Quarter, &renderer),
        Chord::new(
            &[
                Note::new(Natural::C, Octave::FOUR, Some(Accidental::Flat)),
                Note::new(Natural::D, Octave::FOUR, None),
                Note::new(Natural::E, Octave::FOUR, None),
                Note::new(Natural::G, Octave::FOUR, Some(Accidental::Natural)),
            ],
            Duration::Half,
            &renderer,
        ),
    ];
    renderer.svg(&mut document, &chords);

    svg::save("image.svg", &document).unwrap();
}
