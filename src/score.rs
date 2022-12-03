use svg::{
    node::element::{Ellipse, Line, Path, Rectangle},
    Node,
};

pub struct RenderNote {
    x: i64,
    index: i64,
}

pub struct Bar {
    x: i64,
    y1: i64,
    y2: i64,
}

pub struct Beam {
    x: i64,
    y1: i64,
    y2: i64,
    y1_next: i64,
    y2_next: i64,
    y: i64,
}

pub struct RenderChord {
    pub notes: Vec<RenderNote>,
    pub duration: Duration,
    pub width: u64,
    pub bar: Option<Bar>,
    pub beam: Option<Beam>,
}

impl RenderChord {
    pub fn new(
        chords: &[Chord],
        index: &mut usize,
        duration: Duration,
        has_bar: &mut bool,
    ) -> Self {
        let notes = chords[*index].notes;
        let mut render_notes = Vec::new();
        let mut width = 0;
        let mut bar = None;
        let mut beam = None;

        if notes.is_empty() {
            width += 20;
        } else {
            let mut start_left = true;
            // Add width for note line
            bar = if *has_bar && duration != Duration::Whole {
                let low = *notes.iter().min().unwrap();
                let high = *notes.iter().max().unwrap();

                let extra = 40;
                let bar = if low > 10 - high {
                    Bar {
                        x: 1,
                        y1: note_y(low) + extra,
                        y2: note_y(high),
                    }
                } else {
                    start_left = false;
                    Bar {
                        x: NOTE_RX * 2 - 1,
                        y1: note_y(low),
                        y2: note_y(high) - extra,
                    }
                };
                Some(bar)
            } else {
                None
            };

            let mut is_staggered = false;
            for note in notes.iter().copied() {
                let x = if ((start_left && note & 1 == 0) || (!start_left && note & 1 == 0))
                    && (notes.contains(&(note + 1)) || notes.contains(&(note - 1)))
                {
                    let x = NOTE_RX * 2;
                    if !is_staggered {
                        is_staggered = true;
                        width += x as u64;
                    }
                    x
                } else {
                    0
                };

                if note >= 10 || note < 0 {
                    // TODO this is for a line
                    width += 8;
                }

                render_notes.push(RenderNote { x, index: note });
            }

            width += (NOTE_RX * 2) as u64;

            // TODO
            *has_bar = true;
            beam = if duration == Duration::Eigth {
                let low = *notes.iter().min().unwrap();
                let high = *notes.iter().max().unwrap();
                let diff = low.max(10 - high);

                if let Some(next) = chords.get(*index + 1) {
                    if next.duration == Duration::Eigth {
                        *has_bar = false;

                        let low_next = *next.notes.iter().min().unwrap();
                        let high_next = *next.notes.iter().max().unwrap();
                        let diff_next = low_next.max(10 - high_next);

                        let (x, y1, y2, y1_next, y2_next, y) = if diff > diff_next {
                            if low > 10 - high {
                                (
                                    0,
                                    note_y(low) + 40,
                                    note_y(high),
                                    note_y(low) + 40,
                                    note_y(high_next),
                                    note_y(low) + 40,
                                )
                            } else {
                                (
                                    -NOTE_RX + 1,
                                    note_y(low),
                                    note_y(high_next) - 40,
                                    note_y(low_next),
                                    note_y(high_next) - 40,
                                    note_y(high_next) - 40 + 4,
                                )
                            }
                        } else {
                            if low_next > 10 - high_next {
                                (
                                    0,
                                    note_y(low) + 40,
                                    note_y(high_next),
                                    note_y(low_next) + 40,
                                    note_y(high_next),
                                    note_y(high_next),
                                )
                            } else {
                                (
                                    -NOTE_RX,
                                    note_y(high),
                                    note_y(low_next) + 40,
                                    note_y(high_next),
                                    note_y(low_next) + 40,
                                    note_y(low_next) + 40 - 4,
                                )
                            }
                        };

                        Some(Beam {
                            x,
                            y1,
                            y2,
                            y1_next,
                            y2_next,
                            y,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };
        }

        Self {
            notes: render_notes,
            duration,
            width,
            bar,
            beam,
        }
    }

    pub fn svg<T: Node>(&self, node: &mut T, x: i64, spacing: i64) {
        if self.notes.is_empty() {
            match self.duration {
                Duration::Whole => node.append(
                    Rectangle::new()
                        .set("width", 20)
                        .set("height", 6)
                        .set("x", x)
                        .set("y", 62),
                ),
                Duration::Half => node.append(
                    Rectangle::new()
                        .set("width", 20)
                        .set("height", 6)
                        .set("x", x)
                        .set("y", 68),
                ),
                Duration::Quarter => {
                    node.append(
                        Path::new()
                            .set("transform", format!("translate({x}, 64)"))
                            .set("d", include_str!("../svg/quarter_rest.txt")),
                    );
                }
                Duration::Eigth => {
                    node.append(
                        Path::new()
                            .set("transform", format!("translate({x}, 64)"))
                            .set("d", include_str!("../svg/eigth_rest.txt")),
                    );
                }
            }

            return;
        }

        match self.duration {
            Duration::Whole => {
                for note in &self.notes {
                    write_note(
                        node,
                        x,
                        note.index,
                        Ellipse::new()
                            .set("fill", "none")
                            .set("stroke", "black")
                            .set("stroke-width", 2)
                            .set("cx", x + note.x + NOTE_RX * 2)
                            .set("cy", note_y(note.index))
                            .set("rx", NOTE_RX)
                            .set("ry", NOTE_RY),
                    );
                }
            }
            Duration::Half => {
                for note in &self.notes {
                    write_note(
                        node,
                        x,
                        note.index,
                        note_head(x + note.x, note.index)
                            .set("fill", "none")
                            .set("stroke", "black")
                            .set("stroke-width", 2),
                    );
                }
            }
            Duration::Quarter => {
                for note in &self.notes {
                    write_note(
                        node,
                        x,
                        note.index,
                        note_head(x + note.x, note.index)
                            .set("fill", "black")
                            .set("stroke-width", 2),
                    );
                }
            }
            Duration::Eigth => {
                for note in &self.notes {
                    write_note(
                        node,
                        x,
                        note.index,
                        note_head(x + note.x, note.index)
                            .set("fill", "black")
                            .set("stroke-width", 2),
                    );
                }
            }
        }

        if let Some(beam) = &self.beam {
            let next_x = x + spacing + beam.x + NOTE_RX * 4;
            node.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 8)
                    .set("x1", x + beam.x + NOTE_RX * 3 - 2)
                    .set("y1", beam.y)
                    .set("x2", next_x + NOTE_RX * 2 - 2)
                    .set("y2", beam.y),
            );

            node.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 2)
                    .set("x1", x + NOTE_RX * 2 - 1)
                    .set("y1", beam.y1)
                    .set("x2", x + NOTE_RX * 2 - 1)
                    .set("y2", beam.y2),
            );
            node.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 2)
                    .set("x1", next_x + NOTE_RX * 2 - 2)
                    .set("y1", beam.y1_next)
                    .set("x2", next_x + NOTE_RX * 2 - 2)
                    .set("y2", beam.y2_next),
            );
        } else if let Some(bar) = &self.bar {
            node.append(
                Line::new()
                    .set("fill", "none")
                    .set("stroke", "black")
                    .set("stroke-width", 2)
                    .set("x1", x + bar.x)
                    .set("y1", bar.y1)
                    .set("x2", x + bar.x)
                    .set("y2", bar.y2),
            );
        }
    }
}

pub struct Chord<'a> {
    pub notes: &'a [i64],
    pub duration: Duration,
}

pub struct Measure {
    pub chords: Vec<RenderChord>,
    pub width: u64,
    pub clef: Option<Clef>,
}

impl Measure {
    pub fn new(chords: &[Chord], clef: Option<Clef>) -> Self {
        let mut render_chords: Vec<RenderChord> = Vec::new();
        let mut width = if clef.is_some() { 50 } else { 0 };

        let mut index = 0;
        let mut bar = true;
        while let Some(chord) = chords.get(index) {
            let render_chord = RenderChord::new(&chords, &mut index, chord.duration, &mut bar);
            width += render_chord.width;
            render_chords.push(render_chord);
            index += 1;
        }

        Self {
            chords: render_chords,
            width,
            clef,
        }
    }

    pub fn svg<T: Node>(&self, node: &mut T, x: i64) {
        let spacing = 10;
        let mut chord_x = x + spacing;

        if let Some(clef) = self.clef {
            clef.write_svg(node);
            chord_x += 50;
        }

        for chord in &self.chords {
            chord.svg(node, chord_x, spacing);
            chord_x += chord.width as i64 + spacing;
        }

        for line in 0..5 {
            let y = line * NOTE_RY * 2 + 50;

            node.append(
                Line::new()
                    .set("x1", x)
                    .set("y1", y)
                    .set("x2", x + chord_x as i64)
                    .set("y2", y)
                    .set("stroke", "#000")
                    .set("stroke-width", 2),
            )
        }

        for line in 0..2 {
            let line_x = x + line * chord_x as i64;

            node.append(
                Line::new()
                    .set("x1", line_x)
                    .set("y1", 50)
                    .set("x2", line_x)
                    .set("y2", 98)
                    .set("stroke", "#000"),
            )
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    Eigth,
    Quarter,
    Half,
    Whole,
}

#[derive(Clone, Copy, Debug)]
pub enum Clef {
    Treble,
}

impl Clef {
    pub fn write_svg<T: Node>(self, doc: &mut T) {
        doc.append(
            Path::new()
                .set("transform", "translate(0, -11)")
                .set("d", include_str!("../svg/treble_clef_d")),
        );
    }
}

const NOTE_RX: i64 = 8;
const NOTE_RY: i64 = 6;

fn note_y(note: i64) -> i64 {
    (17 - note) * NOTE_RY - 4
}

fn note_head(x: i64, note: i64) -> Ellipse {
    let cx = x;
    let cy = note_y(note);

    Ellipse::new()
        .set("cx", cx + NOTE_RX)
        .set("cy", cy)
        .set("rx", NOTE_RX)
        .set("ry", NOTE_RY)
}

fn write_note<T: Node, U: Node>(doc: &mut T, x: i64, note: i64, node: U) {
    const WIDTH: i64 = 6;

    if note >= 10 {
        let mut n = 0;
        while n <= note {
            doc.append(
                Line::new()
                    .set("stroke", "black")
                    .set("stroke-width", 2)
                    .set("x1", x - WIDTH)
                    .set("y1", note_y(n))
                    .set("x2", x + NOTE_RX * 2 + WIDTH)
                    .set("y2", note_y(n)),
            );
            n += 2;
        }
    } else if note < 0 {
        let mut n = 0;
        while n >= note {
            doc.append(
                Line::new()
                    .set("stroke", "black")
                    .set("stroke-width", 2)
                    .set("x1", x - WIDTH)
                    .set("y1", note_y(n))
                    .set("x2", x + NOTE_RX * 2 + WIDTH)
                    .set("y2", note_y(n)),
            );
            n -= 2;
        }
    }

    doc.append(node);
}

#[cfg(test)]
mod tests {
    use super::{Chord, Clef, Duration, Measure};

    #[test]
    fn f() {
        let measure = Measure::new(
            &[
                Chord {
                    notes: &[-4],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: &[-3, -2, -1],
                    duration: Duration::Eigth,
                },
                Chord {
                    notes: &[],
                    duration: Duration::Half,
                },
                Chord {
                    notes: &[10],
                    duration: Duration::Quarter,
                },
            ],
            Some(Clef::Treble),
        );

        let mut document = svg::Document::new();
        measure.svg(&mut document, 10);

        svg::save("image2.svg", &document).unwrap();
    }
}
