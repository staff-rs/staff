use crate::{
    midi::Octave,
    render::{note::note_index, Renderer},
    Key,
};
use svg::Node;
use text_svg::Glpyh;

pub struct KeySignature<'r> {
    pub accidental_glyph: Glpyh<'r>,
    pub accidentals: Vec<(i64, f64)>,
}

impl<'r> KeySignature<'r> {
    pub fn new(key: Key, renderer: &'r Renderer) -> (Self, f64) {
        let mut width = 0.;

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

        let me = Self {
            accidental_glyph,
            accidentals,
        };

        (me, width + renderer.padding)
    }

    pub fn svg(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node) {
        for (index, accidental_x) in &self.accidentals {
            node.append(self.accidental_glyph.path(
                (x + *accidental_x) as _,
                (y + renderer.note_ry * (*index as f64)) as f32 - renderer.note_ry as f32 / 2.,
            ));
        }
    }
}
