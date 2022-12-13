use svg::Node;
use text_svg::Glpyh;

use crate::{midi::Octave, Key};

use super::{note::note_index, Chord, Duration, Renderer};

pub struct KeySignature<'r> {
    clef_glyph: Glpyh<'r>,
    accidental_glyph: Glpyh<'r>,
    width: f64,
    accidentals: Vec<(i64, f64)>,
}

impl<'r> KeySignature<'r> {
    pub fn new(key: Key, renderer: &'r Renderer) -> Self {
        let clef_glyph = Glpyh::new(&renderer.font, '𝄞', (renderer.note_ry * 10.) as _);
        let mut width = clef_glyph.bounding_box.width() as f64 + renderer.padding;

        // TODO
        let spacing = 1.;
        let c = if key.is_sharp() { '♯' } else { '♭' };
        let accidental_glyph = Glpyh::new(&renderer.font, c, (renderer.accidental_size) as _);

        let accidentals = key
            .into_iter()
            .map(|natural| {
                let x = width;
                width += accidental_glyph.bounding_box.width() as f64 + spacing;
                (note_index(natural, Octave::FIVE), x)
            })
            .collect();

        Self {
            clef_glyph,
            accidental_glyph,
            width: width + renderer.padding,
            accidentals,
        }
    }

    pub fn svg(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node) {
        node.append(self.clef_glyph.path(x as _, (y - renderer.note_ry) as _));

        for (index, accidental_x) in &self.accidentals {
            node.append(self.accidental_glyph.path(
                (x + *accidental_x) as _,
                (y + renderer.note_ry * (*index as f64)) as f32
                    - self.accidental_glyph.bounding_box.height() / 2.,
            ));
        }
    }
}

pub struct Measure<'r> {
    chords: Vec<Chord<'r>>,
    key_signature: Option<KeySignature<'r>>,
    pub width: f64,
}

impl<'r> Measure<'r> {
    pub fn new(chords: Vec<Chord<'r>>, key_signature: Option<KeySignature<'r>>) -> Self {
        let width: f64 = key_signature
            .as_ref()
            .map(|key_signature| key_signature.width)
            .unwrap_or(0.)
            + chords.iter().map(|chord| chord.width).sum::<f64>();

        Self {
            chords,
            key_signature,
            width,
        }
    }

    pub fn svg(
        &self,
        x: f64,
        y: f64,
        extra_width: f64,
        renderer: &'r Renderer,
        node: &mut impl Node,
    ) {
        let mut top = y;
        for chord in &self.chords {
            top = top.max(chord.top);
        }
        top += renderer.document_padding;

        let mut chord_x = x + renderer.padding;

        if let Some(key_signature) = &self.key_signature {
            key_signature.svg(chord_x, top, renderer, node);
            chord_x += key_signature.width;
        }

        for chord in &self.chords {
            chord.svg(renderer, node, chord_x, top);

            let duration_spacing = match chord.duration {
                Duration::Quarter => 4.,
                Duration::Half => 2.,
            };
            chord_x += extra_width / duration_spacing + chord.width;
        }
        let width = chord_x - x;

        for line in 0..5 {
            let y = top + (line * 2) as f64 * renderer.note_ry;
            renderer.draw_line(
                node,
                x + renderer.stroke_width / 2.,
                y,
                x + width + renderer.stroke_width + renderer.padding,
                y,
            );
        }

        for line in 0..2 {
            let line_x = x
                + line as f64 * (width + renderer.stroke_width + renderer.padding)
                + renderer.stroke_width / 2.;
            renderer.draw_line(
                node,
                line_x,
                top - renderer.stroke_width / 2.,
                line_x,
                top + renderer.note_ry * 8. + renderer.stroke_width / 2.,
            );
        }
    }
}
