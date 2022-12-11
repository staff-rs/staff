use staff::note::Accidental;
use staff::render::{Chord, Duration, Note, Renderer};
use staff::{midi::Octave, Natural};

fn main() {
    let mut document = svg::Document::new();

    let renderer = Renderer::default();
    let chords = [
        Chord::new(
            &[
                Note::new(Natural::C, Octave::FOUR, Accidental::Sharp),
            ],
            Duration::Quarter,
            &renderer,
        ),
        /*
        Chord::new(&[], Duration::Quarter, &renderer),
        Chord::new(
            &[
                Note::new(Natural::C, Octave::FOUR),
                Note::new(Natural::D, Octave::FOUR),
                Note::new(Natural::E, Octave::FOUR),
                Note::new(Natural::G, Octave::FOUR),
            ],
            Duration::Half,
            &renderer,
        ),
        */
    ];
    renderer.svg(&mut document, &chords);

    svg::save("image.svg", &document).unwrap();
}
