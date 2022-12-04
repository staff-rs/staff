use svg::{
    node::element::{Ellipse, Line},
    Node,
};

pub struct Renderer {
    pub note_rx: f64,
    pub note_ry: f64,
}

impl Renderer {
    pub fn render_chord<T: Node>(&self, node: &mut T, notes: &[i64]) {
        for note in notes {
            node.append(
                Ellipse::new()
                    .set("cx", 50)
                    .set("cy", self.note_ry * *note as f64)
                    .set("rx", self.note_ry)
                    .set("ry", self.note_ry),
            )
        }

        for line in 0..5 {
            node.append(
                Line::new()
                .set("stroke", "#000")
                .set("stroke-width", 2)
                    .set("x1", 0)
                    .set("y1", line as f64 * self.note_ry)
                    .set("x2", 100)
                    .set("y2", line as f64 * self.note_ry),
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Renderer;

    #[test]
    fn f() {
        let mut document = svg::Document::new();

        let renderer = Renderer {
            note_rx: 10.,
            note_ry: 6.,
        };
        renderer.render_chord(&mut document, &[1, 2, 3]);
        svg::save("image.svg", &document).unwrap();
    }
}
