use std::{fs::File, io::Read};

use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::Font;
use svg::{
    node::element::{Line, Rectangle},
    Node,
};

pub mod chord;
pub use chord::{Chord, Duration, RenderNote};

mod note;
pub use note::Note;

pub struct Renderer {
    pub document_padding: f64,
    pub note_rx: f64,
    pub note_ry: f64,
    pub padding: f64,
    pub stroke_width: f64,
    pub spacing: f64,
    pub font: Font<'static>,
}

impl Default for Renderer {
    fn default() -> Self {
        let handle = SystemSource::new()
            .select_best_match(
                &[FamilyName::Title("Noto Music".to_owned())],
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
            spacing: 80.,
            font,
        }
    }
}

impl Renderer {
    pub fn svg<T: Node>(&self, node: &mut T, chords: &[Chord]) {
        let width: f64 = chords.iter().map(|chord| chord.width).sum();
        node.append(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set(
                    "width",
                    width + self.padding * 3. + self.stroke_width * 2. + self.document_padding * 2.,
                )
                .set("height", 200),
        );

        let x = self.stroke_width + self.document_padding;

        let mut top = 0f64;
        for chord in chords {
            top = top.max(chord.top);
        }
        top += self.document_padding;

        let mut chord_x = x + self.padding;
        for (index, chord) in chords.iter().enumerate() {
            chord.svg(self, node, chord_x, top);

            if index < chords.len() - 1 {
                let duration_spacing = match chord.duration {
                    Duration::Quarter => 4.,
                    Duration::Half => 2.,
                };
                chord_x += self.spacing / duration_spacing;
            }

            chord_x += chord.width;
        }
        let width = chord_x;

        for line in 0..5 {
            let y = top + (line * 2) as f64 * self.note_ry;
            self.draw_line(
                node,
                x + self.stroke_width / 2.,
                y,
                x + width + self.stroke_width + self.padding,
                y,
            );
        }

        for line in 0..2 {
            let line_x = x
                + line as f64 * (chord_x + self.stroke_width + self.padding)
                + self.stroke_width / 2.;
            self.draw_line(
                node,
                line_x,
                top - self.stroke_width / 2.,
                line_x,
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
