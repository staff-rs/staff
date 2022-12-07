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
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            document_padding: 10.,
            note_rx: 10.,
            note_ry: 6.,
            padding: 10.,
            stroke_width: 2.,
            spacing: 20.,
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

#[cfg(test)]
mod tests {
    use super::{Chord, Duration, Note, Renderer};
    use crate::{midi::Octave, Natural};

    #[test]
    fn it_renders() {
        let mut document = svg::Document::new();

        let renderer = Renderer::default();
        let chords = [Chord::new(
            &[
                Note::new(Natural::E, Octave::FOUR),
                Note::new(Natural::C, Octave::FIVE),
                Note::new(Natural::F, Octave::FIVE),
            ],
            Duration::Quarter,
            &renderer,
        )];
        renderer.svg(&mut document, &chords);

        svg::save("image.svg", &document).unwrap();
    }
}