use crate::{midi::Octave, Natural};
use svg::Node;
use text_svg::Glpyh;

use super::{Note, Renderer};

#[derive(Clone, Copy)]
pub enum Duration {
    Quarter,
    Half,
}

pub struct RenderNote {
    pub index: i64,
    pub x: f64,
}

pub struct BarLine {
    pub note: i64,
    pub is_left: bool,
    pub is_double: bool,
}

pub struct ChordLine {
    pub low: i64,
    pub high: i64,
}

pub struct Chord {
    pub duration: Duration,
    pub width: f64,
    pub top: f64,
    pub notes: Vec<RenderNote>,
    pub stem: Option<ChordLine>,
    pub lines: Vec<BarLine>,
    pub is_upside_down: bool,
}

impl Chord {
    pub fn new(notes: &[Note], duration: Duration, renderer: &Renderer) -> Self {
        if notes.is_empty() {
            return Self {
                duration,
                is_upside_down: false,
                top: 0.,
                width: renderer.note_rx * 2.,
                notes: Vec::new(),
                stem: None,
                lines: Vec::new(),
            };
        }

        let high = *notes.iter().max().unwrap();
        let low = *notes.iter().min().unwrap();
        let top = if low < Note::new(Natural::F, Octave::FIVE) {
            -low.index as f64 * renderer.note_ry + renderer.note_ry / 2.
        } else {
            0.
        };

        let staggered_spacing = 2.;
        let is_upside_down = low.min(high) < Note::new(Natural::B, Octave::FIVE);

        let mut lines = Vec::new();

        let mut low_right = 0;
        let mut low_left = 0;
        let mut high_right = 0;
        let mut high_left = 0;

        let mut is_stagger = false;
        let notes = notes
            .iter()
            .copied()
            .map(|note| {
                let is_left = if notes.contains(&(Note::from(note.index - 1)))
                    || notes.contains(&Note::from(note.index + 1))
                {
                    is_stagger = true;
                    note.index & 1 != 0
                } else {
                    !is_upside_down
                };

                let x = if is_left {
                    0.
                } else {
                    renderer.note_rx + staggered_spacing
                };
                if is_left {
                    high_left = high_left.max(note.index);
                    low_left = low_left.min(note.index);
                } else {
                    high_right = high_right.max(note.index);
                    low_right = low_right.min(note.index);
                }

                RenderNote {
                    index: note.index,
                    x,
                }
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
            renderer.note_rx * 2.
        };

        if !lines.is_empty() {
            width += renderer.note_rx;
        }

        let line = ChordLine {
            low: low.index,
            high: high.index,
        };

        Self {
            duration,
            is_upside_down,
            top,
            width,
            notes,
            stem: Some(line),
            lines,
        }
    }

    pub fn svg<T: Node>(&self, renderer: &Renderer, node: &mut T, x: f64, top: f64) {
        if self.notes.is_empty() {
            match self.duration {
                Duration::Quarter => {
                    node.append(
                        Glpyh::new(&renderer.font, '𝄽', 75.)
                            .path((x + renderer.note_rx) as _, (renderer.note_ry * 3.) as _),
                    );
                }
                Duration::Half => todo!(),
            }

            return;
        }

        let note_line_extra = renderer.note_rx / 2.;

        let note_x = if !self.lines.is_empty() {
            x + note_line_extra
        } else {
            x
        };

        // Render note heads
        let c = match self.duration {
            Duration::Quarter => '𝅘',
            Duration::Half => '𝅗',
        };
        let glyph = Glpyh::new(&renderer.font, c, 75.);
        for note in &self.notes {
            node.append(glyph.path(
                (note_x + note.x) as _,
                (top + renderer.note_ry * (note.index as f64 - 1.)) as _,
            ));
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

        if let Some(stem) = &self.stem {
            let line_x = note_x + renderer.note_rx + renderer.stroke_width;
            let chord_line_notes_size = 6.;
            if self.is_upside_down {
                renderer.draw_line(
                    node,
                    line_x,
                    top - renderer.note_ry / 2. + stem.low as f64 * renderer.note_ry,
                    line_x,
                    top + (stem.high as f64 + chord_line_notes_size) * renderer.note_ry,
                )
            } else {
                renderer.draw_line(
                    node,
                    line_x,
                    top + (stem.low as f64 - chord_line_notes_size) * renderer.note_ry,
                    line_x,
                    top + renderer.note_ry / 2. + (stem.high as f64) * renderer.note_ry,
                )
            }
        }
    }
}
