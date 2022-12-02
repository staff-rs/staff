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

impl Chord {
    pub fn f(&self, doc: &mut Document) {
        for note in self.notes.clone() {
            doc.append(
                Ellipse::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("cx", 50)
                    .set("cy", (13 - note) * 10)
                    .set("rx", 10)
                    .set("ry", 5),
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
            chords: vec![chord],
        };

        let mut document = svg::Document::new();
        measure.f(&mut document);

        svg::save("image.svg", &document).unwrap();
    }
}
