use crate::{
    midi::Octave,
    note::Accidental,
    render::{Chord, Duration, KeySignature, Measure, Note, Renderer, Staff},
    Key, Natural, Pitch,
};
use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub enum ClefKind {
    Treble,
    Bass,
}

#[derive(Debug)]
pub enum Command {
    Clef { kind: ClefKind },
}

#[derive(Debug)]
pub enum MeasureItem {
    Note {
        note: Note,
        duration: Duration,
        is_dotted: bool,
    },
    Chord {
        notes: Vec<Note>,
        duration: Duration,
        is_dotted: bool,
    },
}

#[derive(Debug)]
pub enum Item {
    Command(Command),
    Measure { items: Vec<MeasureItem> },
}

pub struct Parser<'a> {
    tokens: Tokens<'a>,
}

impl<'a> Parser<'a> {
    pub fn staff<'r>(&mut self, renderer: &'r Renderer) -> Staff<'r> {
        let mut staff = Staff::default();
        let mut key_signature = None;

        for item in self {
            match item {
                Item::Command(cmd) => match cmd {
                    Command::Clef { kind } => match kind {
                        ClefKind::Treble => {
                            key_signature = Some(KeySignature::new(Key::major(Pitch::C), renderer));
                        }
                        _ => todo!(),
                    },
                },
                Item::Measure { items } => {
                    let chords = items
                        .iter()
                        .map(|item| match item {
                            MeasureItem::Chord {
                                notes,
                                duration,
                                is_dotted,
                            } => Chord::new(notes, *duration, *is_dotted, renderer),
                            MeasureItem::Note {
                                note,
                                duration,
                                is_dotted,
                            } => Chord::new(&[*note], *duration, *is_dotted, renderer),
                        })
                        .collect();
                    let measure = Measure::new(chords, key_signature.take(), renderer);
                    staff.push(renderer, measure);
                }
            }
        }

        staff
    }
}

impl<'a> Iterator for Parser<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_duration = Duration::Whole;
        let mut current_measure: Option<Vec<MeasureItem>> = None;
        loop {
            match self.tokens.next() {
                Some(Token::Command(command)) => match command {
                    "clef" => {
                        if let Some(Token::Literal(literal)) = self.tokens.next() {
                            let kind = match literal {
                                "treble" => ClefKind::Treble,
                                "bass" => ClefKind::Bass,
                                s => todo!("{:?}", s),
                            };
                            break Some(Item::Command(Command::Clef { kind }));
                        } else {
                            todo!()
                        }
                    }
                    _ => todo!(),
                },
                Some(Token::Start(Group::Chord)) => {
                    let mut notes = Vec::new();
                    let mut is_dotted = false;
                    loop {
                        match self.tokens.next() {
                            Some(Token::Literal(literal)) => {
                                let mut chars = literal.chars().peekable();
                                let c = chars.next().unwrap();
                                let note = parse_note(
                                    c,
                                    &mut chars,
                                    &mut current_duration,
                                    &mut is_dotted,
                                );
                                notes.push(note);
                                break;
                            }
                            Some(Token::End(Group::Chord)) => break,
                            _ => todo!(),
                        }
                    }

                    let chord = MeasureItem::Chord {
                        notes,
                        duration: current_duration,
                        is_dotted,
                    };
                    if let Some(measure) = &mut current_measure {
                        measure.push(chord);
                    } else {
                        current_measure = Some(vec![chord]);
                    }
                }
                Some(Token::Literal(literal)) => {
                    let mut is_dotted = false;
                    let mut chars = literal.chars().peekable();
                    let c = chars.next().unwrap();
                    let note = parse_note(c, &mut chars, &mut current_duration, &mut is_dotted);

                    let item = MeasureItem::Note {
                        note,
                        duration: current_duration,
                        is_dotted,
                    };
                    if let Some(measure) = &mut current_measure {
                        measure.push(item);
                    } else {
                        current_measure = Some(vec![item]);
                    }
                }
                Some(Token::LineBreak) => {
                    if let Some(measure) = current_measure.take() {
                        break Some(Item::Measure { items: measure });
                    }
                }
                Some(_) => {}
                None => {
                    break if let Some(measure) = current_measure.take() {
                        Some(Item::Measure { items: measure })
                    } else {
                        None
                    }
                }
            }
        }
    }
}

pub enum Error<'a> {
    UnknownArgument(&'a str),
    UnknownCommand(&'a str),
    UnexpectedEOF,
}

#[derive(Debug)]
pub enum Group {
    Chord,
    Block,
}

#[derive(Debug)]
pub enum Token<'a> {
    Start(Group),
    End(Group),
    Command(&'a str),
    Literal(&'a str),
    LineBreak,
}

pub struct Tokens<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokens<'a> {
    fn parse_literal(&mut self) -> Option<&'a str> {
        let start = self.pos;
        loop {
            match self.input.chars().nth(self.pos) {
                Some(' ') => {
                    self.pos += 1;
                    break if self.pos - 1 > start {
                        Some(&self.input[start..self.pos - 1])
                    } else {
                        None
                    };
                }
                Some('>') | Some('\n') => {
                    break if self.pos > start {
                        Some(&self.input[start..self.pos])
                    } else {
                        None
                    }
                }
                None => {
                    break if self.pos > start {
                        Some(&self.input[start..self.pos])
                    } else {
                        None
                    }
                }
                Some(_) => self.pos += 1,
            }
        }
    }
}

impl<'a> From<&'a str> for Tokens<'a> {
    fn from(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.input.chars().nth(self.pos) {
                Some('\\') => {
                    self.pos += 1;
                    if let Some(literal) = self.parse_literal() {
                        break Some(Token::Command(literal));
                    } else {
                        todo!()
                    }
                }
                Some('{') => {
                    self.pos += 1;
                    break Some(Token::Start(Group::Block));
                }
                Some('}') => {
                    self.pos += 1;
                    break Some(Token::End(Group::Block));
                }
                Some('<') => {
                    self.pos += 1;
                    break Some(Token::Start(Group::Chord));
                }
                Some('>') => {
                    self.pos += 1;
                    break Some(Token::End(Group::Chord));
                }
                Some('\n') => {
                    self.pos += 1;
                    break Some(Token::LineBreak);
                }
                Some(_c) => {
                    if let Some(literal) = self.parse_literal() {
                        break Some(Token::Literal(literal));
                    }
                }
                None => break None,
            }
        }
    }
}

fn parse_note(
    c: char,
    chars: &mut Peekable<Chars>,
    duration: &mut Duration,
    is_dotted: &mut bool,
) -> Note {
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

    parse_duration(chars, duration);

    if chars.peek() == Some(&'.') {
        chars.next();
        *is_dotted = true;
    }

    // TODO check octave
    Note::new(natural, Octave::new_unchecked(i + 3), accidental)
}

fn parse_duration(chars: &mut Peekable<Chars>, duration: &mut Duration) {
    if let Some(c) = chars.peek() {
        if let Some(n) = c.to_digit(10) {
            chars.next();
            *duration = match n {
                4 => Duration::Quarter,
                2 => Duration::Half,
                1 => Duration::Whole,
                _ => todo!(),
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Parser;
    use crate::{parse::Tokens, render::Renderer};

    #[test]
    fn f() {
        let input = include_str!("../test.ly");
        let tokens = Tokens::from(input);
        let mut score = Parser { tokens };

        let renderer = Renderer::default();
        let staff = score.staff(&renderer);

        let svg = renderer.render(&staff);
        svg::save("example.svg", &svg).unwrap();
    }
}
