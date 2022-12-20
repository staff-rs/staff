use crate::render::Renderer;
use svg::Node;
use text_svg::Glpyh;

pub struct Clef<'r> {
    pub clef_glyph: Glpyh<'r>,
}

impl<'r> Clef<'r> {
    pub fn new(renderer: &'r Renderer) -> (Self, f64) {
        let clef_glyph = Glpyh::new(&renderer.font, '𝄞', (renderer.note_ry * 10.) as _);
        let width = clef_glyph.bounding_box.width() as f64 + renderer.padding;
        let me = Self { clef_glyph };
        (me, width)
    }

    pub fn svg(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node) {
        node.append(self.clef_glyph.path(x as _, (y - renderer.note_ry) as _));
    }
}
