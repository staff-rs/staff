use super::Renderer;
use svg::Node;

mod clef;
pub use clef::Clef;

pub mod item;
pub use self::item::MeasureItem;

mod key_signature;
pub use key_signature::KeySignature;

mod note_head;
use self::item::MeasureItemKind;
pub use self::note_head::NoteHead;

mod stem;
pub use stem::Stem;

pub struct Measure<'r> {
    pub items: Vec<MeasureItem<'r>>,
    pub width: f64,
}

impl<'r> Measure<'r> {
    pub fn new(items: Vec<MeasureItem<'r>>, renderer: &'r Renderer) -> Self {
        let width: f64 = items.iter().map(|chord| chord.width).sum::<f64>()
            + renderer.padding * 2.
            + renderer.stroke_width * 2.;

        Self { items, width }
    }

    pub fn svg(
        &self,
        x: f64,
        y: f64,
        extra_width: f64,
        index: usize,
        renderer: &'r Renderer,
        node: &mut impl Node,
    ) {
        let mut top = y;
        for item in &self.items {
            match &item.kind {
                MeasureItemKind::Chord {
                    top: chord_top,
                    duration: _,
                    notes: _,
                    is_upside_down: _,
                    ledger_lines: _,
                    stem: _,
                    accidentals: _,
                } => {
                    top = top.max(*chord_top);
                }
                MeasureItemKind::Note {
                    top: note_top,
                    duration: _,
                    note: _,
                    is_upside_down: _,
                    has_ledger_line: _,
                    has_stem: _,
                    accidental: _,
                } => {
                    top = top.max(*note_top);
                }
                _ => {}
            }
        }
        top += renderer.document_padding;

        let mut chord_x = x + renderer.padding;

        for item in &self.items {
            item.svg(chord_x, top, renderer, node);
            chord_x += item.width;

            match &item.kind {
                MeasureItemKind::Chord {
                    top: _,
                    duration,
                    notes: _,
                    is_upside_down: _,
                    ledger_lines: _,
                    stem: _,
                    accidentals: _,
                }
                | MeasureItemKind::Note {
                    top: _,
                    duration,
                    note: _,
                    is_upside_down: _,
                    has_ledger_line: _,
                    has_stem: _,
                    accidental: _,
                } => chord_x += extra_width / (4. / duration.beats(4)),
                MeasureItemKind::KeySignature(_) => chord_x += renderer.padding,
                _ => {}
            }
        }

        let width = chord_x - x;
        for line in 0..5 {
            let y = top + (line * 2) as f64 * renderer.note_ry;
            renderer.draw_line(
                node,
                x,
                y,
                x + width + renderer.stroke_width + renderer.padding,
                y,
            );
        }

        if index == 0 {
            renderer.draw_line(
                node,
                x,
                top - renderer.stroke_width / 2.,
                x,
                top + renderer.note_ry * 8. + renderer.stroke_width / 2.,
            );
        }

        let line_x =
            x + (width + renderer.stroke_width + renderer.padding) + renderer.stroke_width / 2.;
        renderer.draw_line(
            node,
            line_x,
            top - renderer.stroke_width / 2.,
            line_x,
            top + renderer.note_ry * 8. + renderer.stroke_width / 2.,
        );
    }
}