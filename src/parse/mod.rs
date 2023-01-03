//! LilyPond format parsing

use crate::{
    midi::Octave,
    note::Accidental,
    render::{
        staff::{
            measure::{self, Measure},
            note::Note,
            renderer::Renderer,
        },
        Staff,
    },
    time::{Duration, DurationKind},
    Key, Natural,
};
use std::iter::Peekable;

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
    tokens: Peekable<Tokens<'a>>,
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
        let mut current_duration = Duration::new(DurationKind::Whole, false);
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
                            let (natural, _, accidental, _used) = parse_note_parts(literal);
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
                    loop {
                        match self.tokens.next() {
                            Some(Token::Literal(literal)) => {
                                let (note, _used) = parse_note(literal);
                                notes.push(note);
                            }
                            Some(Token::End(Group::Chord)) => {
                                if let Some(Token::Literal(literal)) = self.tokens.peek() {
                                    if let Some((duration, _)) = parse_duration(literal) {
                                        self.tokens.next();
                                        current_duration = duration;
                                    }
                                }
                                break;
                            }
                            _ => todo!(),
                        }
                    }

                    has_notes = true;
                    let chord = MeasureItem::Chord {
                        notes,
                        duration: current_duration,
                    };
                    if let Some(measure) = &mut current_measure {
                        measure.push(chord);
                    } else {
                        current_measure = Some(vec![chord]);
                    }
                }
                Some(Token::Literal(literal)) => {
                    let (note, used) = parse_note(literal);
                    if let Some((duration, _used)) = parse_duration(&literal[used..]) {
                        current_duration = duration;
                    }

                    has_notes = true;
                    let item = MeasureItem::Note {
                        note,
                        duration: current_duration,
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
        let tokens = Tokens::from(input).peekable();
        Self { tokens }
    }
}

fn parse_note(s: &str) -> (Note, usize) {
    let (natural, octave, accidental, pos) = parse_note_parts(s);
    (Note::new(natural, octave, accidental), pos)
}

fn parse_note_parts(s: &str) -> (Natural, Octave, Option<Accidental>, usize) {
    let natural_c = s.chars().next().unwrap();
    let natural = Natural::try_from(natural_c).unwrap();

    let mut pos = 1;
    let accidental = match s.chars().nth(pos) {
        Some('i') => {
            if s.chars().nth(pos + 1) != Some('s') {
                todo!()
            }
            pos += 2;
            Some(Accidental::Sharp)
        }
        Some('e') => {
            if s.chars().nth(pos + 1) != Some('s') {
                todo!()
            }
            pos += 2;
            Some(Accidental::Flat)
        }
        _ => None,
    };

    let mut i = 0;
    match s.chars().nth(pos) {
        Some('\'') => {
            pos += 1;
            i = 1;

            while s.chars().nth(pos) == Some('\'') {
                pos += 1;
                i += 1;
            }
        }
        Some(',') => {
            pos += 1;
            i = -1;

            while s.chars().nth(pos) == Some(',') {
                pos += 1;
                i -= 1;
            }
        }
        _ => {}
    }

    // TODO check octave
    (natural, Octave::new_unchecked(i + 3), accidental, pos)
}

fn parse_duration(s: &str) -> Option<(Duration, usize)> {
    let mut pos = 0;
    if let Some(c) = s.chars().nth(pos) {
        if let Some(n) = c.to_digit(10) {
            pos += 1;
            let kind = match n {
                4 => DurationKind::Quarter,
                2 => DurationKind::Half,
                1 => DurationKind::Whole,
                _ => todo!(),
            };

            let is_dotted = if s.chars().nth(pos + 1) == Some('.') {
                pos += 1;
                true
            } else {
                false
            };

            return Some((Duration::new(kind, is_dotted), pos));
        }
    }

    None
}
