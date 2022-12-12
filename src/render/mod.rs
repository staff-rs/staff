use font_kit::{
    family_name::FamilyName, handle::Handle, properties::Properties, source::SystemSource,
};
use rusttype::Font;
use std::{fs::File, io::Read};
use svg::{
    node::element::{Line, Rectangle},
    Node,
};

pub mod chord;
pub use chord::{Chord, Duration, RenderNote};

mod note;
pub use note::Note;
use text_svg::Glpyh;

use crate::{midi::Octave, Key, Pitch};

use self::note::note_index;

pub struct KeySignature<'r> {
    key: Key,
    clef_glyph: Glpyh<'r>,
    accidental_glyph: Glpyh<'r>,
    width: f64,
}

impl<'r> KeySignature<'r> {
    pub fn f(key: Key, x: f64, y: f64, renderer: &'r Renderer, node: &mut impl Node) -> Self {
        let clef_glyph = Glpyh::new(&renderer.font, '𝄞', (renderer.note_ry * 10.) as _);
        let mut width = clef_glyph.bounding_box.width() as f64 + renderer.padding;

        // TODO
        let spacing = 1.;
        let c = if key.is_sharp() { '♯' } else { '♭' };
        let accidental_glyph = Glpyh::new(&renderer.font, c, (renderer.accidental_size) as _);
        for _ in key.into_iter() {
            width += accidental_glyph.bounding_box.width() as f64 + spacing;
        }

        Self {
            key,
            clef_glyph,
            accidental_glyph,
            width: width + renderer.padding,
        }
    }

    pub fn svg(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node) {
        node.append(self.clef_glyph.path(x as _, (y - renderer.note_ry) as _));

        // TODO
        let spacing = 1.;

        for natural in self.key.into_iter() {
            node.append(self.accidental_glyph.path(
                x as _,
                (y + renderer.note_ry * (note_index(natural, Octave::FIVE) as f64)) as f32
                    - self.accidental_glyph.bounding_box.height() / 2.,
            ));
        }
    }
}

pub struct Renderer {
    pub document_padding: f64,
    pub note_rx: f64,
    pub note_ry: f64,
    pub padding: f64,
    pub stroke_width: f64,
    pub accidental_size: f64,
    pub width: f64,
    pub font: Font<'static>,
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
            width: 200.,
            font,
        }
    }
}

impl Renderer {
    pub fn svg<T: Node>(
        &self,
        node: &mut T,
        chords: &[Chord],
        key_signature: Option<&KeySignature>,
    ) {
        node.append(
            Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", self.width)
                .set("height", 100),
        );

        let x = self.stroke_width + self.document_padding;
        let width: f64 = chords.iter().map(|chord| chord.width).sum();

        // TODO why multiply by 3?
        let extra =
            self.width - width - (self.document_padding + self.padding + self.stroke_width) * 3.;

        let mut top = 0f64;
        for chord in chords {
            top = top.max(chord.top);
        }
        top += self.document_padding;

        let mut chord_x = x + self.padding;

        if let Some(key_signature) = &key_signature {
            key_signature.svg(x, top, self, node);
            chord_x += self.padding;
        }

        for chord in chords {
            chord.svg(self, node, chord_x, top);

            let duration_spacing = match chord.duration {
                Duration::Quarter => 4.,
                Duration::Half => 2.,
            };
            chord_x += extra / duration_spacing + chord.width;
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
