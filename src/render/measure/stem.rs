use svg::Node;

use crate::render::Renderer;

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
        let line_x = x + renderer.note_rx + renderer.stroke_width;
        let chord_line_notes_size = 6.;
        if is_upside_down {
            let line_x = line_x + renderer.stroke_width;
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
