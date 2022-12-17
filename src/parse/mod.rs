//! LilyPond format parsing

use crate::{
    midi::Octave,
    note::Accidental,
    render::{Chord, Duration, KeySignature, Measure, Note, Renderer, Staff},
    Key, Natural, Pitch,
};
use std::{iter::Peekable, str::Chars};

mod tokens;
pub use tokens::{Group, Token, Tokens};

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

impl<'a> From<&'a str> for Parser<'a> {
    fn from(input: &'a str) -> Self {
        let tokens = Tokens::from(input);
        Self { tokens }
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
