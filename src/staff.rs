use svg::{
    node::element::{Ellipse, Line},
    Document, Node,
};

use crate::{
    midi::{MidiNote, MidiSet, Octave},
    Pitch,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    Eigth,
    Quarter,
    Half,
    Whole,
}

pub struct Measure {
    chords: Vec<Chord>,
}

impl Measure {
    pub fn f(&self, doc: &mut Document, x: i64) {
        for line in 0..5 {
            let y = line * 20 + 50;

            doc.append(
                Line::new()
                    .set("x1", x)
                    .set("y1", y)
                    .set("x2", x + 210)
                    .set("y2", y)
                    .set("stroke", "#000"),
            )
        }

        for line in 0..2 {
            let line_x = line * 210 + x;

            doc.append(
                Line::new()
                    .set("x1", line_x)
                    .set("y1", 50)
                    .set("x2", line_x)
                    .set("y2", 130)
                    .set("stroke", "#000"),
            )
        }

        let mut chord_x = x + 10;
        let mut beam = None;

        let mut iter = self.chords.iter().peekable();
        while let Some(chord) = iter.next() {
            chord.f(doc, chord_x);

            if chord.duration != Duration::Eigth {
                if let Some(((start_x, start_y), Some((end_x, end_y)))) = beam {
                    doc.append(
                        Line::new()
                            .set("x1", start_x + 20)
                            .set("y1", start_y + 2)
                            .set("x2", end_x + 20)
                            .set("y2", end_y + 2)
                            .set("stroke", "#000")
                            .set("stroke-width", 4),
                    )
                }
            }

            chord_x += match chord.duration {
                Duration::Whole => 200,
                Duration::Half => {
                    chord.draw_note_line(doc, chord_x);
                    200 / 2
                }
                Duration::Quarter => {
                    chord.draw_note_line(doc, chord_x);
                    200 / 4
                }
                Duration::Eigth => {
                    let y = chord.draw_note_line(doc, chord_x);

                    if let Some((_start, end)) = &mut beam {
                        *end = Some((chord_x, y));
                    } else {
                        beam = Some(((chord_x, y), None))
                    }

                    if let Some(next) = iter.peek() {
                        if next.duration == Duration::Eigth {
                            50
                        } else {
                            80
                        }
                    } else {
                        80
                    }
                }
            };
        }
    }
}

pub struct Chord {
    // TODO use Set
    notes: Vec<i64>,
    duration: Duration,
}

fn y(note: i64) -> i64 {
    (13 - note) * 10
}

impl Chord {
    pub fn f(&self, doc: &mut Document, x: i64) {
        match self.duration {
            Duration::Whole => {
                for note in &self.notes {
                    doc.append(
                        Ellipse::new()
                            .set("fill", "none")
                            .set("stroke", "black")
                            .set("cx", x + 10)
                            .set("cy", y(*note))
                            .set("rx", 10)
                            .set("ry", 5),
                    );
                }
            }
            Duration::Half => {
                for note in &self.notes {
                    doc.append(
                        Ellipse::new()
                            .set("fill", "none")
                            .set("stroke", "black")
                            .set("cx", x + 10)
                            .set("cy", y(*note))
                            .set("rx", 10)
                            .set("ry", 5),
                    );
                }
            }
            Duration::Quarter => {
                for note in &self.notes {
                    doc.append(
                        Ellipse::new()
                            .set("fill", "black")
                            .set("cx", x + 10)
                            .set("cy", y(*note))
                            .set("rx", 10)
                            .set("ry", 5),
                    );
                }
            }
            Duration::Eigth => {
                for note in &self.notes {
                    doc.append(
                        Ellipse::new()
                            .set("fill", "blue")
                            .set("cx", x + 10)
                            .set("cy", y(*note))
                            .set("rx", 10)
                            .set("ry", 5),
                    );
                }
            }
        }
    }

    fn draw_note_line(&self, doc: &mut Document, x: i64) -> i64 {
        let low = *self.notes.iter().min().unwrap();
        let high = *self.notes.iter().max().unwrap();

        if low > 10 - high {
            doc.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("x1", x)
                    .set("y1", y(low) + 40)
                    .set("x2", x)
                    .set("y2", y(high)),
            );
            y(low) + 40
        } else {
            doc.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("x1", x + 20)
                    .set("y1", y(low))
                    .set("x2", x + 20)
                    .set("y2", y(high) - 40),
            );
            y(high) - 40
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Chord, Duration, Measure};

    #[test]
    fn f() {
        let measure = Measure {
            chords: vec![
                Chord {
                    notes: vec![5, 7, 9],
                    duration: Duration::Half,
                },
                Chord {
                    notes: vec![-2],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![-2, 0, 2],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: vec![-2, 0, 2],
                    duration: Duration::Quarter,
                },
            ],
        };

        let mut document = svg::Document::new();
        measure.f(&mut document, 10);

        svg::save("image.svg", &document).unwrap();
    }
}
