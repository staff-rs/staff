use svg::Node;

use crate::render::{staff::renderer::Renderer, Line};

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
        let chord_line_notes_size = 6.;
        if is_upside_down {
            let line_x = x + renderer.stroke_width / 2.;
            renderer.draw_line(
                node,
                line_x,
                y - renderer.note_ry / 2. + (self.low as f64 + 0.75) * renderer.note_ry,
                line_x,
                y + (self.high as f64 + chord_line_notes_size) * renderer.note_ry,
            )
        } else {
            let line_x = x + renderer.stroke_width + renderer.note_rx;
            renderer.draw_line(
                node,
                line_x,
                y + (self.low as f64 - chord_line_notes_size) * renderer.note_ry,
                line_x,
                y + renderer.note_ry / 2. + (self.high as f64 - 0.5) * renderer.note_ry,
            )
        }
    }

    pub fn draw(
        &self,
        x: f64,
        y: f64,
        is_upside_down: bool,
        renderer: &Renderer,
        mut draw_line: impl FnMut(Line),
    ) {
        let chord_line_notes_size = 6.;
        if is_upside_down {
            let line_x = x + renderer.stroke_width / 2.;
            draw_line(Line::new(
                line_x,
                y - renderer.note_ry / 2. + (self.low as f64 + 0.75) * renderer.note_ry,
                line_x,
                y + (self.high as f64 + chord_line_notes_size) * renderer.note_ry,
                renderer.stroke_width,
            ))
        } else {
            let line_x = x + renderer.stroke_width + renderer.note_rx;
            draw_line(Line::new(
                line_x,
                y + (self.low as f64 - chord_line_notes_size) * renderer.note_ry,
                line_x,
                y + renderer.note_ry / 2. + (self.high as f64 - 0.5) * renderer.note_ry,
                renderer.stroke_width,
            ));
        }
    }
}
