use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::Font;
use std::{fs::File, io::Read};
use svg::{
    node::element::{Line, Rectangle},
    Document, Node,
};

pub mod chord;
pub use chord::{Chord, Duration, RenderNote};

pub mod measure;
pub use measure::{KeySignature, Measure};

mod note;
pub use note::Note;

pub struct Renderer {
    pub document_padding: f64,
    pub note_rx: f64,
    pub note_ry: f64,
    pub padding: f64,
    pub stroke_width: f64,
    pub accidental_size: f64,
    pub width: f64,
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
            document_padding: 10.,
            note_rx: 10.,
            note_ry: 6.,
            padding: 10.,
            stroke_width: 2.,
            accidental_size: 80.,
            width: 500.,
            font,
            min_spacing: 30.,
        }
    }
}

impl Renderer {
    pub fn render(&self, staff: &Staff) -> Document {
        let mut document = svg::Document::new()
            .set("width", self.width)
            .set("height", 500);

        document.append(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", self.width)
                .set("height", 500),
        );

        let mut y = 0.;
        for row in &staff.rows {
            let mut x = self.stroke_width + self.document_padding;

            let measures_width = row
                .measures
                .iter()
                .map(|measure| measure.width)
                .sum::<f64>();
            let remaining = self.width - measures_width - self.document_padding * 2.;
            let measure_exta = remaining / row.measures.len() as f64;

            for measure in &row.measures {
                measure.svg(x, y, measure_exta, self, &mut document);
                x += measure.width + measure_exta;
            }

            y += 100.;
        }

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

pub struct Row<'r> {
    measures: Vec<Measure<'r>>,
    width: f64,
}

#[derive(Default)]
pub struct Staff<'r> {
    rows: Vec<Row<'r>>,
}

impl<'r> Staff<'r> {
    pub fn push(&mut self, renderer: &Renderer, measure: Measure<'r>) {
        if let Some(row) = self.rows.last_mut() {
            let width = row.width + measure.width;
            if width < renderer.width {
                row.measures.push(measure);
                row.width = width;
                return;
            }
        }

        let row = Row {
            width: measure.width,
            measures: vec![measure],
        };
        self.rows.push(row);
    }
}
