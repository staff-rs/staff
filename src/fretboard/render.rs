use crate::{
    fretboard::diagram::{Diagram, Range},
    render::{Line, Rectangle},
};
use std::mem;

#[cfg(feature = "svg")]
use svg::node::{element, Node};

#[cfg(feature = "wasm-bindgen")]
use wasm_bindgen::prelude::wasm_bindgen;

pub enum Marker {
    Rectangle(Rectangle),
    Cross { lines: [Line; 2] },
}

pub struct Renderer {
    pub diagram: Diagram,
    pub width: f64,
    pub height: f64,
    pub fret_width: f64,
    pub fret_height: f64,
}

impl Renderer {
    pub fn new(diagram: Diagram, width: f64, height: f64) -> Self {
        let fret_width = fret_width(width, diagram.strings());
        let fret_height = height / (diagram.frets() + 1) as f64;
        Self {
            diagram,
            width,
            height,
            fret_width,
            fret_height,
        }
    }


    pub fn render_grid(&self, padding: f64, mut draw_line: impl FnMut(Line)) {
        let x = self.fret_width / 2.;
        let y = 0.;
        let stroke_width = 2.;
        for idx in 0..self.diagram.strings() {
            let line_x = x + self.fret_width * idx as f64;
            draw_line(Line::new(
                line_x,
                y + self.fret_height,
                line_x,
                self.height - padding,
                stroke_width,
            ));
        }

        let line_y = y + self.fret_height;
        draw_line(Line::new(
            x - stroke_width / 2.,
            line_y,
            x + (self.fret_width * (self.diagram.strings() - 1) as f64) + stroke_width / 2.,
            line_y,
            stroke_width * 2.,
        ));

        for idx in 1..self.diagram.frets() {
            let line_y = line_y + self.fret_height * idx as f64;
            draw_line(Line::new(
                x - stroke_width / 2.,
                line_y,
                x + self.fret_width * (self.diagram.strings() - 1) as f64 + stroke_width / 2.,
                line_y,
                stroke_width,
            ));
        }
    }

    pub fn render_single_fretted(
        &self,
        x: f64,
        y: f64,
        stroke_width: f64,
        fret: &Range,
        mut draw_fretted: impl FnMut(Marker),
    ) {
        let x = x + self.fret_width / 2.;
        let draw_height = self.fret_height / 1.5;

        if fret.start >= fret.end {
            let x = x + self.fret_width * fret.start as f64 - self.fret_width / 4.;
            let lines = [
                Line::new(
                    x,
                    y + self.fret_height / 4.,
                    x + self.fret_width / 2.,
                    y + self.fret_height * 0.75,
                    stroke_width,
                ),
                Line::new(
                    x + self.fret_width / 2.,
                    y + self.fret_height / 4.,
                    x,
                    y + self.fret_height * 0.75,
                    stroke_width,
                ),
            ];
            draw_fretted(Marker::Cross { lines })
        } else {
            let mut rect = Rectangle::new(
                x + self.fret_width * fret.start as f64 - draw_height / 2.,
                fret.fret as f64 * self.fret_height + y + draw_height / 4.,
                self.fret_width * (fret.end - 1 - fret.start) as f64 + draw_height,
                draw_height,
                0.,
                true,
            );

            if fret.fret == 0 {
                rect.stroke_width = 2.;
                rect.is_filled = false;
            }

            draw_fretted(Marker::Rectangle(rect))
        }
    }

    pub fn render_fretted(
        &self,
        x: f64,
        y: f64,
        stroke_width: f64,
        mut draw_fretted: impl FnMut(Marker),
    ) {
        for fret in self.diagram.ranges() {
            self.render_single_fretted(x, y, stroke_width, fret, &mut draw_fretted)
        }
    }

    #[cfg(feature = "svg")]
    pub fn svg(
        &self,
        x: f64,
        y: f64,
        padding: f64,
        letter_spacing: f64,
        font_size: f64,
        font: &rusttype::Font,
    ) -> svg::Document {
        let glyphs_width = (font_size + letter_spacing) * 2.;
        let mut document = svg::Document::new()
            .set("width", self.width + glyphs_width + self.fret_width / 2.)
            .set("height", self.height);

        document.append(
            element::Rectangle::new()
                .set("fill", "#fff")
                .set("x", 0)
                .set("y", 0)
                .set("width", self.width + glyphs_width + self.fret_width / 2.)
                .set("height", self.height),
        );

        let mut x = x + padding;
        let y = y + padding;

        x += glyphs_width;

        if self.diagram.starting_fret > 0 {
            let mut glyph_x = x;

            let glyph_y = y + self.fret_height;
            let s = self.diagram.starting_fret.to_string();

            for c in s.chars().rev() {
                let glyph = text_svg::Glpyh::new(font, c, font_size as _);
                glyph_x -= glyph.bounding_box.width() as f64 + letter_spacing;
                document.append(glyph.path(glyph_x as _, glyph_y as _));
            }
        }

        self.render_grid(padding, |line| line.svg(x, &mut document));

        self.render_fretted(x, 0., 2., |fretted| match fretted {
            Marker::Cross { lines } => {
                for line in lines {
                    line.svg(0., &mut document)
                }
            }
            Marker::Rectangle(rect) => {
                rect.svg(&mut document);
            }
        });

        document
    }
}

fn fret_width(width: f64, strings: u8) -> f64 {
    width / strings as f64
}
