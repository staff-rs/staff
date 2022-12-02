use svg::{
    node::element::{Ellipse, Line},
    Document, Node,
};

use crate::{
    midi::{MidiNote, MidiSet, Octave},
    Pitch,
};

pub enum Duration {
    Quarter,
    Half,
    Whole,
}

pub struct Measure {
    chords: Vec<Chord>,
}

impl Measure {
    pub fn f(&self, doc: &mut Document) {
        for line in 0..5 {
            let y = line * 20 + 50;
            doc.append(
                Line::new()
                    .set("x1", 0)
                    .set("y1", y)
                    .set("x2", 500)
                    .set("y2", y)
                    .set("stroke", "#000"),
            )
        }

        for chord in &self.chords {
            chord.f(doc);
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
    pub fn f(&self, doc: &mut Document) {
        for note in &self.notes {
            doc.append(
                Ellipse::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("cx", 50)
                    .set("cy", y(*note))
                    .set("rx", 10)
                    .set("ry", 5),
            );
        }

        let low = *self.notes.iter().min().unwrap();
        let high = *self.notes.iter().max().unwrap();

        if low > 10 - high {
            doc.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("x1", 40)
                    .set("y1", y(low) + 40)
                    .set("x2", 40)
                    .set("y2", y(high)),
            );
        } else {
            doc.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("x1", 60)
                    .set("y1", y(low))
                    .set("x2", 60)
                    .set("y2", y(high) - 40),
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use svg::node::element::path::Data;
    use svg::node::element::{Ellipse, Path};

    use crate::midi::{MidiNote, Octave};
    use crate::Pitch;

    use super::{Chord, Duration, Measure};

    #[test]
    fn f() {
        let chord = Chord {
            notes: vec![5, 7, 9],
            duration: Duration::Half,
        };

        let measure = Measure {
            chords: vec![
                Chord {
                    notes: vec![5, 7, 9],
                    duration: Duration::Half,
                },
                Chord {
                    notes: vec![-2, 0, 2],
                    duration: Duration::Half,
                },
            ],
        };

        let mut document = svg::Document::new();
        measure.f(&mut document);

        svg::save("image.svg", &document).unwrap();
    }
}
