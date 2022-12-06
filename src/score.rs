use svg::{
    node::element::{Ellipse, Line, Path, Rectangle},
    Node,
};

#[derive(Clone, Copy)]
pub enum Duration {
    Quarter,
    Half,
}

pub struct Note {
    index: i64,
    x: f64,
    duration: Duration,
}

struct BarLine {
    note: i64,
    is_left: bool,
    is_double: bool,
}

struct ChordLine {
    low: i64,
    high: i64,
}

pub struct Chord {
    width: f64,
    top: f64,
    notes: Vec<Note>,
    line: ChordLine,
    lines: Vec<BarLine>,
    is_upside_down: bool,
}

impl Chord {
    pub fn new(notes: &[i64], duration: Duration, renderer: &Renderer) -> Self {
        let high = *notes.iter().max().unwrap();
        let low = *notes.iter().min().unwrap();
        let top = if low < 0 {
            -low as f64 * renderer.note_ry + renderer.note_ry / 2.
        } else {
            0.
        };

        let staggered_spacing = 2.;
        let is_upside_down = low.min(high) < 5;

        let mut lines = Vec::new();

        let mut low_right = 0;
        let mut low_left = 0;
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

                let x = if is_left {
                    0.
                } else {
                    renderer.note_rx + staggered_spacing
                };
                if is_left {
                    high_left = high_left.max(index);
                    low_left = low_left.min(index);
                } else {
                    high_right = high_right.max(index);
                    low_right = low_right.min(index);
                }

                Note { index, x, duration }
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

        if low_right <= -2 {
            let mut i = -2;
            while i >= low_right {
                lines.push(BarLine {
                    note: i,
                    is_left: false,
                    is_double: false,
                });

                i -= 2;
            }
        }

        if low_left <= -2 {
            let mut i = -2;
            while i >= low_left {
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

                i -= 2;
            }
        }

        let mut width = if is_stagger {
            (renderer.note_rx + renderer.stroke_width) * 2. + staggered_spacing
        } else {
            (renderer.note_rx + renderer.stroke_width) * 4.
        };

        if !lines.is_empty() {
            width += renderer.note_rx;
        }

        let line = ChordLine { low, high };

        Self {
            is_upside_down,
            top,
            width,
            notes,
            line,
            lines,
        }
    }

    pub fn svg<T: Node>(&self, renderer: &Renderer, node: &mut T, x: f64, top: f64) {
        let note_line_extra = renderer.note_rx / 2.;

        let note_x = if !self.lines.is_empty() {
            x + note_line_extra
        } else {
            x
        };

        for note in &self.notes {
            let d = match note.duration {
                Duration::Quarter => include_str!("../svg/note_head.txt"),
                Duration::Half => include_str!("../svg/half_note_head.txt"),
            };
            node.append(
                Path::new()
                    .set("fill", "#000")
                    .set("fill-rule", "evenodd")
                    .set("d", d)
                    .set(
                        "transform",
                        format!(
                            "translate({}, {})",
                            note_x + note.x,
                            top + renderer.note_ry * (note.index as f64 - 1.)
                        ),
                    ),
            );
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

            let y = top + renderer.note_ry * line.note as f64;
            renderer.draw_line(node, x1, y, x2, y)
        }

        let line_x = note_x + renderer.note_rx + renderer.stroke_width;
        let chord_line_notes_size = 6.;
        if self.is_upside_down {
            renderer.draw_line(
                node,
                line_x,
                top - renderer.note_ry / 2. + self.line.low as f64 * renderer.note_ry,
                line_x,
                top + (self.line.high as f64 + chord_line_notes_size) * renderer.note_ry,
            )
        } else {
            renderer.draw_line(
                node,
                line_x,
                top + (self.line.low as f64 - chord_line_notes_size) * renderer.note_ry,
                line_x,
                top + renderer.note_ry / 2. + (self.line.high as f64) * renderer.note_ry,
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
        let width: f64 = chords.iter().map(|chord| chord.width).sum();
        node.append(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", width + self.padding * 3. + self.stroke_width * 2.)
                .set("height", 200),
        );

        let mut x = self.stroke_width + self.padding;

        let mut top = 0f64;
        for chord in chords {
            top = top.max(chord.top);
        }

        for chord in chords {
            chord.svg(self, node, x, top);
            x += chord.width;
        }

        for line in 0..5 {
            let y = top + (line * 2) as f64 * self.note_ry;
            self.draw_line(
                node,
                self.stroke_width / 2.,
                y,
                self.stroke_width + x + self.padding * 2.,
                y,
            );
        }

        for line in 0..2 {
            let x =
                line as f64 * (x + self.stroke_width + self.padding * 2.) + self.stroke_width / 2.;
            self.draw_line(
                node,
                x,
                top - self.stroke_width / 2.,
                x,
                top + self.note_ry * 8. + self.stroke_width / 2.,
            );
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
    use svg::{node::element::Rectangle, Node};

    use super::{Chord, Duration, Renderer};

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
            Chord::new(&[5, 10, 11, 12], Duration::Quarter, &renderer),
            Chord::new(&[6, 1, 2, 3], Duration::Half, &renderer),
            Chord::new(&[6, 5, 7], Duration::Quarter, &renderer),
            Chord::new(&[2, 3, 4, -5], Duration::Half, &renderer),
        ];
        renderer.svg(&mut document, &chords);

        svg::save("image.svg", &document).unwrap();
    }
}
