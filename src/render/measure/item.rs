use crate::{
    duration::{Duration, DurationKind},
    midi::Octave,
    note::Accidental,
    render::{note::note_index, Note, Renderer},
    Natural,
};
use svg::Node;
use text_svg::Glpyh;

pub struct NoteHead {
    pub index: i64,
    pub x: f64,
}

impl NoteHead {
    pub fn new(index: i64, x: f64) -> Self {
        Self { index, x }
    }

    pub fn draw(
        &self,
        note_x: f64,
        top: f64,
        duration: Duration,
        renderer: &Renderer,
        node: &mut impl Node,
    ) {
        let c = match duration.kind {
            DurationKind::Quarter => '𝅘',
            DurationKind::Half => '𝅗',
            DurationKind::Whole => '𝅝',
        };
        let glyph = Glpyh::new(&renderer.font, c, 75.);
        let dot_glyph = if duration.is_dotted {
            Some(Glpyh::new(&renderer.font, '.', 75.))
        } else {
            None
        };

        self.draw_with_glyph(note_x, top, &glyph, dot_glyph.as_ref(), renderer, node)
    }

    pub fn draw_with_glyph(
        &self,
        note_x: f64,
        top: f64,
        glyph: &Glpyh,
        dot_glyph: Option<&Glpyh>,
        renderer: &Renderer,
        node: &mut impl Node,
    ) {
        if let Some(dot_glyph) = dot_glyph {
            node.append(dot_glyph.path(
                (note_x + self.x + renderer.note_rx * 1.5 + renderer.stroke_width) as _,
                (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
            ));
        }

        node.append(glyph.path(
            (note_x + self.x) as _,
            (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
        ));
    }
}

pub struct LedgerLine {
    pub note: i64,
    pub is_left: bool,
    pub is_double: bool,
}

pub struct Stem {
    pub low: i64,
    pub high: i64,
}

impl Stem {
    pub fn new(low: i64, high: i64) -> Self {
        Self { low, high }
    }

    pub fn svg(
        &self,
        x: f64,
        y: f64,
        is_upside_down: bool,
        renderer: &Renderer,
        node: &mut impl Node,
    ) {
        let line_x = x + renderer.note_rx + renderer.stroke_width / 1.4;
        let chord_line_notes_size = 6.;
        if is_upside_down {
            let line_x = line_x + renderer.stroke_width / 1.4;
            renderer.draw_line(
                node,
                line_x,
                y - renderer.note_ry / 2. + (self.low as f64 + 0.75) * renderer.note_ry,
                line_x,
                y + (self.high as f64 + chord_line_notes_size) * renderer.note_ry,
            )
        } else {
            renderer.draw_line(
                node,
                line_x,
                y + (self.low as f64 - chord_line_notes_size) * renderer.note_ry,
                line_x,
                y + renderer.note_ry / 2. + (self.high as f64 - 0.75) * renderer.note_ry,
            )
        }
    }
}

pub struct ChordAccidental<'a> {
    pub glyph: Glpyh<'a>,
    pub index: i64,
    pub x: f64,
    pub is_flat: bool,
}

impl<'a> ChordAccidental<'a> {
    pub fn new(accidental: Accidental, index: i64, renderer: &'a Renderer) -> Self {
        let (c, is_flat) = match accidental {
            Accidental::Natural => ('♮', false),
            Accidental::Sharp => ('♯', false),
            Accidental::Flat => ('♭', true),
            _ => todo!(),
        };
        let glyph = Glpyh::new(&renderer.font, c, renderer.accidental_size as _);

        Self {
            glyph,
            index,
            x: 0.,
            is_flat,
        }
    }

    pub fn svg(&self, x: f64, y: f64, renderer: &'a Renderer, node: &mut impl Node) -> f32 {
        let y = if self.is_flat {
            (y + renderer.note_ry * (self.index as f64 + 1.)) as f32
                - self.glyph.bounding_box.height()
        } else {
            (y + renderer.note_ry * (self.index as f64)) as f32
                - self.glyph.bounding_box.height() / 2.
        };
        node.append(self.glyph.path((x + self.x) as _, y));

        self.glyph.bounding_box.width()
    }
}

pub enum MeasureItemKind<'r> {
    Rest,
    Note {
        note: NoteHead,
        is_upside_down: bool,
        has_ledger_line: bool,
        has_stem: bool,
        accidental: Option<ChordAccidental<'r>>,
    },
    Chord {
        notes: Vec<NoteHead>,
        is_upside_down: bool,
        ledger_lines: Vec<LedgerLine>,
        stem: Option<Stem>,
        accidentals: Vec<ChordAccidental<'r>>,
    },
}

pub struct MeasureItem<'r> {
    pub kind: MeasureItemKind<'r>,
    pub duration: Duration,
    pub width: f64,
    pub top: f64,
}

impl<'r> MeasureItem<'r> {
    pub fn rest(duration: Duration, _is_dotted: bool, renderer: &Renderer) -> Self {
        Self {
            kind: MeasureItemKind::Rest,
            duration,
            top: 0.,
            width: renderer.note_rx * 2.,
        }
    }

    pub fn note(duration: Duration, note: Note, renderer: &'r Renderer) -> Self {
        let top = if note.index < note_index(Natural::F, Octave::FIVE) {
            -note.index as f64 * renderer.note_ry + renderer.note_ry / 2.
        } else {
            0.
        };
        let is_upside_down = note.index < note_index(Natural::B, Octave::FIVE);

        let (accidental, accidental_width) = if let Some(accidental) = note.accidental {
            let chord_accidental = ChordAccidental::new(accidental, note.index, renderer);
            let accidental_width = chord_accidental.glyph.bounding_box.width() as _;
            (Some(chord_accidental), accidental_width)
        } else {
            (None, 0.)
        };

        let render_note = NoteHead::new(note.index, 0.);
        let mut width = renderer.note_rx * 2.;

        let beats = duration.beats(4);

        width += renderer.min_spacing / beats;
        width += accidental_width;

        let has_ledger_line = note.index < -2 || note.index > 10;
        if has_ledger_line {
            width += renderer.note_rx;
        }

        if duration.is_dotted {
            width += renderer.note_rx * 2.;
        }

        let has_stem = duration.kind != DurationKind::Whole;
        let kind = MeasureItemKind::Note {
            note: render_note,
            has_ledger_line,
            has_stem,
            accidental,
            is_upside_down,
        };
        Self {
            kind,
            top,
            width,
            duration,
        }
    }

    pub fn chord(duration: Duration, notes: &[Note], renderer: &'r Renderer) -> Self {
        let high = notes.iter().map(|note| note.index).max().unwrap();
        let low = notes.iter().map(|note| note.index).min().unwrap();
        let top = if low < note_index(Natural::F, Octave::FIVE) {
            -low as f64 * renderer.note_ry + renderer.note_ry / 2.
        } else {
            0.
        };

        let staggered_spacing = renderer.stroke_width / 2.;
        let is_upside_down = low.min(high) < note_index(Natural::B, Octave::FIVE);

        let mut ledger_lines = Vec::new();
        let mut accidentals = Vec::new();

        let mut low_right = 0;
        let mut low_left = 0;
        let mut high_right = 0;
        let mut high_left = 0;

        let mut is_stagger = false;
        let mut accidental_width = 0f64;
        let notes = notes
            .iter()
            .copied()
            .map(|note| {
                if let Some(accidental) = note.accidental {
                    let chord_accidental = ChordAccidental::new(accidental, note.index, renderer);
                    accidental_width =
                        accidental_width.max(chord_accidental.glyph.bounding_box.width() as _);
                    accidentals.push(chord_accidental);
                }

                let is_left = if notes
                    .iter()
                    .find(|n| n.index == note.index - 1 || n.index == note.index + 1)
                    .is_some()
                {
                    is_stagger = true;
                    note.index & 1 != 0
                } else {
                    !is_upside_down
                };

                let x = if is_left {
                    0.
                } else {
                    if !is_stagger {
                        renderer.note_rx + renderer.stroke_width
                    } else {
                        renderer.note_rx + staggered_spacing
                    }
                };
                if is_left {
                    high_left = high_left.max(note.index);
                    low_left = low_left.min(note.index);
                } else {
                    high_right = high_right.max(note.index);
                    low_right = low_right.min(note.index);
                }

                NoteHead {
                    index: note.index,
                    x,
                }
            })
            .collect();

        if high_right > 10 {
            let mut i = 10;
            while i <= high_right {
                ledger_lines.push(LedgerLine {
                    note: i,
                    is_left: false,
                    is_double: false,
                });

                i += 2;
            }
        }

        if high_left > 8 {
            let mut i = 8;
            while i <= high_left {
                if let Some(line) = ledger_lines.iter_mut().find(|line| (**line).note == i) {
                    line.is_double = true;
                    line.is_left = true;
                } else {
                    ledger_lines.push(LedgerLine {
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
                ledger_lines.push(LedgerLine {
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
                if let Some(line) = ledger_lines.iter_mut().find(|line| (**line).note == i) {
                    line.is_double = true;
                    line.is_left = true;
                } else {
                    ledger_lines.push(LedgerLine {
                        note: i,
                        is_left: true,
                        is_double: false,
                    });
                }

                i -= 2;
            }
        }

        let mut width = if is_stagger {
            (renderer.note_rx * 2. + renderer.stroke_width) * 2. + staggered_spacing
        } else {
            renderer.note_rx * 2.
        };

        let beats = duration.beats(4);
        width += renderer.min_spacing / beats;

        width += accidental_width;

        if !ledger_lines.is_empty() {
            width += renderer.note_rx;
        }

        if duration.is_dotted {
            width += renderer.note_rx * 2.;
        }

        let stem = if duration.kind != DurationKind::Whole {
            Some(Stem::new(low, high))
        } else {
            None
        };

        let kind = MeasureItemKind::Chord {
            notes,
            ledger_lines,
            stem,
            is_upside_down,
            accidentals,
        };
        Self {
            kind,
            top,
            width,
            duration,
        }
    }

    pub fn svg(&self, mut x: f64, top: f64, renderer: &Renderer, node: &mut impl Node) {
        match &self.kind {
            MeasureItemKind::Rest => match self.duration.kind {
                DurationKind::Quarter => {
                    node.append(Glpyh::new(&renderer.font, '𝄽', 75.).path(
                        (x + renderer.note_rx) as _,
                        (self.top + renderer.note_ry * 3.) as _,
                    ));
                }
                DurationKind::Half => todo!(),
                DurationKind::Whole => todo!(),
            },
            MeasureItemKind::Chord {
                notes,
                ledger_lines,
                stem,
                is_upside_down,
                accidentals,
            } => {
                let mut accidentals_width = 0.;
                if !accidentals.is_empty() {
                    for chord_accidental in accidentals {
                        let width = chord_accidental.svg(x, top, &renderer, node);
                        accidentals_width = width as f64;
                    }
                    x += accidentals_width + renderer.note_rx / 2.;
                }

                let note_line_extra = renderer.note_rx / 2.;
                let note_x = if !ledger_lines.is_empty() {
                    x + note_line_extra
                } else {
                    x
                };

                // Render note heads
                let c = match self.duration.kind {
                    DurationKind::Quarter => '𝅘',
                    DurationKind::Half => '𝅗',
                    DurationKind::Whole => '𝅝',
                };
                let glyph = Glpyh::new(&renderer.font, c, 75.);
                let dot_glyph = if self.duration.is_dotted {
                    Some(Glpyh::new(&renderer.font, '.', 75.))
                } else {
                    None
                };

                for note in notes {
                    note.draw_with_glyph(note_x, top, &glyph, dot_glyph.as_ref(), renderer, node)
                }

                for line in ledger_lines {
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

                if let Some(stem) = &stem {
                    stem.svg(note_x, top, *is_upside_down, renderer, node);
                }
            }
            MeasureItemKind::Note {
                note,
                is_upside_down,
                has_ledger_line,
                has_stem,
                accidental: _,
            } => {
                let note_line_extra = renderer.note_rx / 2.;
                let note_x = if *has_ledger_line {
                    x + note_line_extra
                } else {
                    x
                };

                note.draw(note_x, top, self.duration, renderer, node);

                if *has_stem {
                    let stem = Stem::new(note.index, note.index);
                    stem.svg(note_x, top, *is_upside_down, renderer, node);
                }
            }
        }
    }
}
