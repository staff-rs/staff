//! LilyPond format parsing

use crate::{
    duration::{Duration, DurationKind},
    midi::Octave,
    note::Accidental,
    render::{
        measure::{self},
        Measure, Note, Renderer, Staff,
    },
    Key, Natural,
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
pub enum MeasureItem {
    Clef {
        kind: ClefKind,
    },
    Key(Key),
    Note {
        note: Note,
        duration: Duration,
    },
    Chord {
        notes: Vec<Note>,
        duration: Duration,
    },
}

#[derive(Debug)]
pub enum Item {
    Measure { items: Vec<MeasureItem> },
}

pub struct Parser<'a> {
    tokens: Tokens<'a>,
}

impl<'a> Parser<'a> {
    pub fn staff<'r>(&mut self, renderer: &'r Renderer) -> Staff<'r> {
        let mut staff = Staff::default();

        for item in self {
            match item {
                Item::Measure { items } => {
                    let chords = items
                        .iter()
                        .map(|item| match item {
                            MeasureItem::Clef { kind: _ } => measure::MeasureItem::clef(renderer),
                            MeasureItem::Key(key) => {
                                measure::MeasureItem::key_signature(*key, renderer)
                            }
                            MeasureItem::Chord { notes, duration } => {
                                measure::MeasureItem::chord(*duration, notes, renderer)
                            }
                            MeasureItem::Note { note, duration } => {
                                measure::MeasureItem::note(*duration, *note, renderer)
                            }
                        })
                        .collect();

                    let measure = Measure::new(chords, renderer);
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
        let mut current_duration = DurationKind::Whole;
        let mut current_measure: Option<Vec<MeasureItem>> = None;
        let mut has_notes = false;
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
                            let clef = MeasureItem::Clef { kind };
                            if let Some(measure) = &mut current_measure {
                                measure.push(clef);
                            } else {
                                current_measure = Some(vec![clef]);
                            }
                        } else {
                            todo!()
                        }
                    }
                    "key" => {
                        if let Some(Token::Literal(literal)) = self.tokens.next() {
                            let mut chars = literal.chars().peekable();
                            let c = chars.next().unwrap();
                            let mut is_dotted = false;
                            let (natural, _, accidental) = parse_note_parts(
                                c,
                                &mut chars,
                                &mut current_duration,
                                &mut is_dotted,
                            );
                            let root = crate::Note::new(
                                natural,
                                accidental.unwrap_or(Accidental::Natural),
                            );

                            let key = if let Some(Token::Command(cmd)) = self.tokens.next() {
                                match cmd {
                                    "major" => Key::major(root.into()),
                                    _ => todo!(),
                                }
                            } else {
                                todo!()
                            };

                            let item = MeasureItem::Key(key);
                            if let Some(measure) = &mut current_measure {
                                measure.push(item);
                            } else {
                                current_measure = Some(vec![item]);
                            }
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

                    has_notes = true;
                    let chord = MeasureItem::Chord {
                        notes,
                        duration: Duration::new(current_duration, is_dotted),
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

                    has_notes = true;
                    let item = MeasureItem::Note {
                        note,
                        duration: Duration::new(current_duration, is_dotted),
                    };
                    if let Some(measure) = &mut current_measure {
                        measure.push(item);
                    } else {
                        current_measure = Some(vec![item]);
                    }
                }
                Some(Token::LineBreak) => {
                    if has_notes {
                        if let Some(measure) = current_measure.take() {
                            break Some(Item::Measure { items: measure });
                        }
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
    duration: &mut DurationKind,
    is_dotted: &mut bool,
) -> Note {
    let (natural, octave, accidental) = parse_note_parts(c, chars, duration, is_dotted);
    Note::new(natural, octave, accidental)
}

fn parse_note_parts(
    c: char,
    chars: &mut Peekable<Chars>,
    duration: &mut DurationKind,
    is_dotted: &mut bool,
) -> (Natural, Octave, Option<Accidental>) {
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
    (natural, Octave::new_unchecked(i + 3), accidental)
}

fn parse_duration(chars: &mut Peekable<Chars>, duration: &mut DurationKind) {
    if let Some(c) = chars.peek() {
        if let Some(n) = c.to_digit(10) {
            chars.next();
            *duration = match n {
                4 => DurationKind::Quarter,
                2 => DurationKind::Half,
                1 => DurationKind::Whole,
                _ => todo!(),
            };
        }
    }
}
