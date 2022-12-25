use super::NoteHead;
use crate::{
    midi::Octave,
    render::staff::{
        note::note_index,
        renderer::{Draw, Renderer},
    },
    Key,
};
use svg::Node;
use text_svg::Glpyh;

pub struct KeySignature<'r> {
    pub glyph: Glpyh<'r>,
    pub accidentals: Vec<NoteHead>,
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
                NoteHead::new(note_index(natural, Octave::FIVE), x)
            })
            .collect();

        let me = Self {
            glyph: accidental_glyph,
            accidentals,
        };

        (me, width + renderer.padding)
    }
}

impl Draw for KeySignature<'_> {
    fn draw(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node) {
        for note_head in &self.accidentals {
            node.append(self.glyph.path(
                (x + note_head.x) as _,
                (y + renderer.note_ry * (note_head.index as f64)) as f32
                    - renderer.note_ry as f32 / 2.,
            ));
        }
    }
}
