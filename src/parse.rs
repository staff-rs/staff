use crate::{
    midi::Octave,
    render::{Chord, Duration, Note, Renderer},
    Natural,
};

pub fn parse<'a>(renderer: &'a Renderer, input: &str) -> Vec<Chord<'a>> {
    let mut chars = input.chars().peekable();
    let mut duration = Duration::Quarter;

    let mut chords = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '\\' => todo!(),
            c => {
                let natural = Natural::try_from(c).unwrap();

                let mut i = 0;
                match chars.peek() {
                    Some('\'') => {
                        chars.next();
                        i = 1;

                        while chars.peek().copied() == Some('\'') {
                            chars.next();
                            i += 1;
                        }
                    }
                    Some(',') => {
                        chars.next();
                        i = -1;

                        while chars.peek().copied() == Some('\'') {
                            chars.next();
                            i -= 1;
                        }
                    }
                    _ => {}
                }

                if let Some(c) = chars.peek() {
                    if let Some(n) = c.to_digit(10) {
                        chars.next();
                        duration = match n {
                            4 => Duration::Quarter,
                            _ => todo!(),
                        };
                    }
                }

                match chars.next() {
                    Some(' ') | None => {}
                    Some(_) => todo!(),
                }

                // TODO check octave
                let note = Note::new(natural, Octave::new_unchecked(i + 3), None);
                chords.push(Chord::new(&[note], duration, &renderer));
            }
        }
    }

    chords
}

#[cfg(test)]
mod tests {
    use crate::{
        render::{measure::Measure, KeySignature, Renderer},
        Key, Pitch,
    };

    use super::parse;

    #[test]
    fn f() {
        let s = "c'4 e' g' c''";

        let renderer = Renderer::default();
        let chords = parse(&renderer, s);
        let key_signature = KeySignature::new(Key::major(Pitch::G), &renderer);
        let measure = Measure::new(chords, Some(key_signature));
        let svg = renderer.render(&measure);

        svg::save("ly.svg", &svg).unwrap();
    }
}
