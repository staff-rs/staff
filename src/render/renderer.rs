use rusttype::Font;
use svg::{node::element::Rectangle, Document, Node};

use super::{font, fretboard::Line};

pub trait Draw {
    fn draw(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node);
}

pub struct Renderer {
    pub document_padding: f64,
    pub note_rx: f64,
    pub note_ry: f64,
    pub padding: f64,
    pub stroke_width: f64,
    pub accidental_size: f64,
    pub width: f64,
    pub height: f64,
    pub font: Font<'static>,
    pub min_spacing: f64,
}

impl Default for Renderer {
    fn default() -> Self {
        let font = font();

        Self {
            document_padding: 20.,
            note_rx: 10.,
            note_ry: 6.,
            padding: 10.,
            stroke_width: 1.5,
            accidental_size: 80.,
            width: 500.,
            height: 200.,
            font,
            min_spacing: 18.,
        }
    }
}

impl Renderer {
    pub fn render(&self, draw: &impl Draw) -> Document {
        let mut document = svg::Document::new()
            .set("width", self.width)
            .set("height", self.height);

        document.append(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", self.width)
                .set("height", self.height),
        );

        draw.draw(0., 0., self, &mut document);

        document
    }

    pub fn draw_line<T: Node>(&self, node: &mut T, x1: f64, y1: f64, x2: f64, y2: f64) {
        Line::new(x1, y1, x2, y2, self.stroke_width).svg(0., node)
    }
}
