use svg::{
    node::element::{Ellipse, Line},
    Node,
};

use crate::note;

pub struct Note {
    index: i64,
    x: f64,
}

pub struct Chord {
    width: f64,
    notes: Vec<Note>,
}

impl Chord {
    pub fn new(notes: &[i64], renderer: &Renderer) -> Self {
        let low = *notes.iter().min().unwrap();
        let high = *notes.iter().max().unwrap();

        let is_upside_down = low + 10 > high;

        let mut is_staggered = false;
        let notes = notes
            .iter()
            .copied()
            .map(|index| {
                let x = if index & 1 == 0 && notes.contains(&(index - 1)) {
                    is_staggered = true;
                    if is_upside_down {
                        renderer.note_rx
                    } else {
                        0.
                    }
                } else {
                    if is_upside_down {
                        0.
                    } else {
                        renderer.note_rx
                    }
                };
                Note { index, x }
            })
            .collect();

        let width = if is_staggered {
            renderer.note_rx * 2.
        } else {
            renderer.note_rx * 4.
        };

        Self { width, notes }
    }

    pub fn svg<T: Node>(&self, renderer: &Renderer, node: &mut T) {
        for note in &self.notes {
            node.append(
                Ellipse::new()
                    .set("cx", note.x + renderer.note_rx / 2.)
                    .set("cy", renderer.note_ry * note.index as f64)
                    .set("rx", renderer.note_ry)
                    .set("ry", renderer.note_ry),
            )
        }
    }
}

pub struct Renderer {
    pub note_rx: f64,
    pub note_ry: f64,
}

impl Renderer {
    pub fn svg<T: Node>(&self, node: &mut T, chord: Chord) {
       chord.svg(self, node);

        for line in 0..5 {
            node.append(
                Line::new()
                    .set("stroke", "#000")
                    .set("stroke-width", 2)
                    .set("x1", 0)
                    .set("y1", (line * 2) as f64 * self.note_ry)
                    .set("x2", chord.width)
                    .set("y2", (line * 2) as f64 * self.note_ry),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Chord, Renderer};

    #[test]
    fn f() {
        let mut document = svg::Document::new();

        let renderer = Renderer {
            note_rx: 10.,
            note_ry: 6.,
        };
        renderer.svg(&mut document, Chord::new(&[5, 6, 7], &renderer));
        svg::save("image.svg", &document).unwrap();
    }
}
