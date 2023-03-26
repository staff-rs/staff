use crate::{
    render::staff::renderer::Renderer,
    time::{Duration, DurationKind},
};
use svg::Node;
use text_svg::Glpyh;

pub struct NoteHead {
    pub index: i64,
    pub x: f64,
}

impl NoteHead {
    pub fn new(index: i64, x: f64) -> Self {
        Self { index, x }
    }

    pub fn draw(
        &self,
        note_x: f64,
        top: f64,
        duration: Duration,
        renderer: &Renderer,
        node: &mut impl Node,
    ) {
        let c = match duration.kind {
            DurationKind::Quarter => '𝅘',
            DurationKind::Half => '𝅗',
            DurationKind::Whole => '𝅝',
        };
        let glyph = Glpyh::new(&renderer.font, c, 75.);
        let dot_glyph = if duration.is_dotted {
            Some(Glpyh::new(&renderer.font, '.', 75.))
        } else {
            None
        };

        self.draw_with_glyph(note_x, top, &glyph, dot_glyph.as_ref(), renderer, node)
    }

    pub fn draw_with_glyph(
        &self,
        note_x: f64,
        top: f64,
        glyph: &Glpyh,
        dot_glyph: Option<&Glpyh>,
        renderer: &Renderer,
        node: &mut impl Node,
    ) {
        if let Some(dot_glyph) = dot_glyph {
            node.append(dot_glyph.path(
                (note_x + self.x + renderer.note_rx * 1.5 + renderer.stroke_width) as _,
                (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
            ));
        }

        node.append(glyph.path(
            (note_x + self.x) as _,
            (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
        ));
    }

    pub fn draw_path(
        &self,
        note_x: f64,
        top: f64,
        duration: Duration,
        renderer: &Renderer,
        draw_path: impl FnMut(String),
    ) {
        let c = match duration.kind {
            DurationKind::Quarter => '𝅘',
            DurationKind::Half => '𝅗',
            DurationKind::Whole => '𝅝',
        };
        let glyph = Glpyh::new(&renderer.font, c, 75.);
        let dot_glyph = if duration.is_dotted {
            Some(Glpyh::new(&renderer.font, '.', 75.))
        } else {
            None
        };

        self.draw_path_with_glyph(note_x, top, &glyph, dot_glyph.as_ref(), renderer, draw_path)
    }

    pub fn draw_path_with_glyph(
        &self,
        note_x: f64,
        top: f64,
        glyph: &Glpyh,
        dot_glyph: Option<&Glpyh>,
        renderer: &Renderer,
        mut draw_path: impl FnMut(String),
    ) {
        if let Some(dot_glyph) = dot_glyph {
            let mut dot_path = String::new();
            dot_glyph.write_path(
                (note_x + self.x + renderer.note_rx * 1.5 + renderer.stroke_width) as _,
                (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
                &mut dot_path,
            );
            draw_path(dot_path);
        }

        let mut path = String::new();
        glyph.write_path(
            (note_x + self.x) as _,
            (top + renderer.note_ry * (self.index as f64 - 1.)) as _,
            &mut path,
        );
        draw_path(path);
    }
}
