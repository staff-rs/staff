use svg::Node;
use text_svg::Glpyh;

use crate::render::staff::renderer::{Draw, Renderer};

pub struct Clef<'r> {
    pub glyph: Glpyh<'r>,
}

impl<'r> Clef<'r> {
    pub fn new(renderer: &'r Renderer) -> (Self, f64) {
        let clef_glyph = Glpyh::new(&renderer.font, '𝄞', (renderer.note_ry * 10.) as _);
        let width = clef_glyph.bounding_box.width() as f64 + renderer.padding;
        let me = Self { glyph: clef_glyph };
        (me, width)
    }

    pub fn path(&self, x: f64, y: f64, renderer: &Renderer) -> String {
        let mut path = String::new();
        self.glyph
            .write_path(x as _, (y - renderer.note_ry) as _, &mut path);
        path
    }
}

impl Draw for Clef<'_> {
    fn draw(&self, x: f64, y: f64, renderer: &Renderer, node: &mut impl Node) {
        node.append(self.glyph.path(x as _, (y - renderer.note_ry) as _));
    }
}
