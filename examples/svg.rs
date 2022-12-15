use staff::note::Accidental;
use staff::render::measure::Measure;
use staff::render::{Chord, Duration, KeySignature, Note, Renderer, Staff};
use staff::{midi::Octave, Natural};
use staff::{Key, Pitch};

fn main() {
    let renderer = Renderer::default();

    let chords = vec![
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

    let key_signature = KeySignature::new(Key::major(Pitch::G), &renderer);
    let measure = Measure::new(chords, Some(key_signature), &renderer);
    let mut staff = Staff::default();
    staff.push(&renderer, measure);

    let svg = renderer.render(&staff);
    svg::save("example.svg", &svg).unwrap();
}
