//! Sheet music engraving
use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::Font;
use std::{fs::File, io::Read};
use svg::{
    node::element::{Line, Rectangle},
    Document, Node,
};

pub mod fretboard;

pub mod measure;

mod note;
pub use self::note::Note;

pub mod staff;
pub use self::staff::Staff;

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
        let handle = SystemSource::new()
            .select_best_match(
                &[
                    FamilyName::Title("Noto Music".to_owned()),
                    FamilyName::Serif,
                ],
                &Properties::new(),
            )
            .unwrap();

        let font = match handle {
            Handle::Path { path, font_index } => {
                let mut file = File::open(path).unwrap();
                let mut buf = Vec::new();
                file.read_to_end(&mut buf).unwrap();
                Font::try_from_vec_and_index(buf, font_index).unwrap()
            }
            Handle::Memory { bytes, font_index } => {
                Font::try_from_vec_and_index(bytes.to_vec(), font_index).unwrap()
            }
        };

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
