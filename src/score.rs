use svg::{
    node::element::{Ellipse, Line},
    Node,
};

pub struct Note {
    index: i64,
    x: f64,
}

struct BarLine {
    note: i64,
    is_left: bool,
    is_double: bool,
}

pub struct Chord {
    width: f64,
    notes: Vec<Note>,
    lines: Vec<BarLine>,
}

impl Chord {
    pub fn new(notes: &[i64], renderer: &Renderer) -> Self {
        let low = *notes.iter().min().unwrap();
        let high = *notes.iter().max().unwrap();
        let is_upside_down = low.min(high) < 5;

        let mut lines = Vec::new();

        let mut high_right = 0;
        let mut high_left = 0;
        let mut is_stagger = false;
        let notes = notes
            .iter()
            .copied()
            .map(|index| {
                let is_left = if notes.contains(&(index - 1)) || notes.contains(&(index + 1)) {
                    is_stagger = true;

                    index & 1 != 0
                } else {
                    !is_upside_down
                };

                let x = if is_left { 0. } else { renderer.note_rx };
                if is_left {
                    high_left = high_left.max(index);
                } else {
                    high_right = high_right.max(index);
                }

                Note { index, x }
            })
            .collect();

        if high_right > 10 {
            let mut i = 10;
            while i <= high_right {
                lines.push(BarLine {
                    note: i,
                    is_left: false,
                    is_double: false,
                });

                i += 2;
            }
        }

        if high_left > 10 {
            let mut i = 10;
            while i <= high_left {
                if let Some(line) = lines.iter_mut().find(|line| (**line).note == i) {
                    line.is_double = true;
                    line.is_left = true;
                } else {
                    lines.push(BarLine {
                        note: i,
                        is_left: true,
                        is_double: false,
                    });
                }

                i += 2;
            }
        }

        let mut width = if is_stagger {
            (renderer.note_rx + renderer.stroke_width) * 2.
        } else {
            (renderer.note_rx + renderer.stroke_width) * 4.
        };

        if !lines.is_empty() {
            width += renderer.note_rx;
        }

        Self {
            width,
            notes,
            lines,
        }
    }

    pub fn svg<T: Node>(&self, renderer: &Renderer, node: &mut T, x: f64) {
        let note_line_extra = renderer.note_rx / 2.;

        let note_x = if !self.lines.is_empty() {
            x + note_line_extra
        } else {
            x
        };

        for note in &self.notes {
            node.append(
                Ellipse::new()
                    .set("fill", "transparent")
                    .set("stroke", "#000")
                    .set("stroke-width", renderer.stroke_width)
                    .set(
                        "cx",
                        note_x + renderer.stroke_width + note.x + renderer.note_rx / 2.,
                    )
                    .set("cy", renderer.note_ry * note.index as f64)
                    .set("rx", renderer.note_ry)
                    .set("ry", renderer.note_ry),
            )
        }

        for line in &self.lines {
            let x1 = if line.is_left {
                x
            } else {
                renderer.note_rx + x
            };

            let x2 = if line.is_double {
                x1 + (note_line_extra + renderer.note_rx + renderer.stroke_width) * 2.
            } else {
                x1 + (note_line_extra * 2.) + renderer.note_rx + renderer.stroke_width
            };

            renderer.draw_line(
                node,
                x1,
                renderer.note_ry * line.note as f64,
                x2,
                renderer.note_ry * line.note as f64,
            )
        }
    }
}

pub struct Renderer {
    pub note_rx: f64,
    pub note_ry: f64,
    pub padding: f64,
    pub stroke_width: f64,
}

impl Renderer {
    pub fn svg<T: Node>(&self, node: &mut T, chords: &[Chord]) {
        let mut x = self.stroke_width + self.padding;

        for chord in chords {
            chord.svg(self, node, x);
            x += chord.width;
        }

        for line in 0..5 {
            self.draw_line(
                node,
                self.stroke_width / 2.,
                (line * 2) as f64 * self.note_ry,
                self.stroke_width + x + self.padding * 2.,
                (line * 2) as f64 * self.note_ry,
            );
        }

        for line in 0..2 {
            let x =
                line as f64 * (x + self.stroke_width + self.padding * 2.) + self.stroke_width / 2.;
            self.draw_line(node, x, 0., x, self.note_ry * 8. + self.stroke_width / 2.);
        }
    }

    fn draw_line<T: Node>(&self, node: &mut T, x1: f64, y1: f64, x2: f64, y2: f64) {
        node.append(
            Line::new()
                .set("stroke", "#000")
                .set("stroke-width", self.stroke_width)
                .set("x1", x1)
                .set("y1", y1)
                .set("x2", x2)
                .set("y2", y2),
        )
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
            padding: 10.,
            stroke_width: 2.,
        };

        let chords = [
            Chord::new(&[5, 10, 11, 12], &renderer),
            Chord::new(&[6, 1, 2, 3], &renderer),
            Chord::new(&[10, 11, 12], &renderer),
            Chord::new(&[1, 2, 3], &renderer),
        ];
        renderer.svg(&mut document, &chords);

        svg::save("image.svg", &document).unwrap();
    }
}
