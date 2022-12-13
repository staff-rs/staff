use crate::{
    midi::Octave,
    note::Accidental,
    render::{Chord, Duration, Measure, Note, Renderer},
    Natural,
};

pub fn parse_measures<'a>(renderer: &'a Renderer, input: &str) -> Vec<Measure<'a>> {
    input
        .lines()
        .map(|line| Measure::new(parse_chords(renderer, line), None))
        .collect()
}

pub fn parse_chords<'a>(renderer: &'a Renderer, input: &str) -> Vec<Chord<'a>> {
    let mut chars = input.chars().peekable();
    let mut duration = Duration::Quarter;

    let mut chords = Vec::new();

    while let Some(c) = chars.next() {
        match c {
            '\\' => todo!(),
            c => {
                let natural = Natural::try_from(c).unwrap();

                let accidental = match chars.peek() {
                    Some('i') => {
                        chars.next();
                        if chars.next() != Some('s') {
                            todo!()
                        }
                        Some(Accidental::Sharp)
                    }
                    Some('e') => {
                        chars.next();
                        if chars.next() != Some('s') {
                            todo!()
                        }
                        Some(Accidental::Flat)
                    }
                    _ => None,
                };

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

                        while chars.peek().copied() == Some(',') {
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
                            2 => Duration::Half,
                            _ => todo!(),
                        };
                    }
                }

                match chars.next() {
                    Some(' ') | None => {}
                    Some(c) => panic!("{:?} {:?}", natural, c),
                }

                // TODO check octave
                let note = Note::new(natural, Octave::new_unchecked(i + 3), accidental);
                chords.push(Chord::new(&[note], duration, &renderer));
            }
        }
    }

    chords
}

#[cfg(test)]
mod tests {
    use super::parse_measures;
    use crate::render::Renderer;

    #[test]
    fn f() {
        let s = "c'4 eis'2 g'4\nf''2 ees'4 c''";

        let renderer = Renderer::default();
        let measures = parse_measures(&renderer, s);

        let svg = renderer.render(&measures);

        svg::save("ly.svg", &svg).unwrap();
    }
}
