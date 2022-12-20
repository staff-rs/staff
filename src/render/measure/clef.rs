use crate::{
    midi::Octave,
    render::{note::note_index, Renderer},
    Key,
};
use svg::Node;
use text_svg::Glpyh;

pub struct Clef<'r> {
    pub clef_glyph: Glpyh<'r>,
    pub accidental_glyph: Glpyh<'r>,
    pub width: f64,
    pub accidentals: Vec<(i64, f64)>,
}

impl<'r> Clef<'r> {
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
