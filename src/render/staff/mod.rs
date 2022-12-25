pub mod measure;
use self::measure::Measure;

pub mod renderer;
use self::renderer::{Draw, Renderer};

pub mod note;

pub struct Row<'r> {
    pub measures: Vec<Measure<'r>>,
    pub width: f64,
}

#[derive(Default)]
pub struct Staff<'r> {
    pub rows: Vec<Row<'r>>,
}

impl<'r> Staff<'r> {
    pub fn push(&mut self, renderer: &Renderer, measure: Measure<'r>) {
        if let Some(row) = self.rows.last_mut() {
            let width = row.width + measure.width;
            if width < renderer.width {
                row.measures.push(measure);
                row.width = width;
                return;
            }
        }

        let row = Row {
            width: measure.width,
            measures: vec![measure],
        };
        self.rows.push(row);
    }
}

impl Draw for Staff<'_> {
    fn draw(&self, x: f64, mut y: f64, renderer: &Renderer, node: &mut impl svg::Node) {
        let mut x = x + renderer.stroke_width + renderer.document_padding;

        for row in &self.rows {
            let measures_width = row
                .measures
                .iter()
                .map(|measure| measure.width)
                .sum::<f64>();
            let remaining = renderer.width - measures_width - renderer.document_padding * 2.;
            let measure_exta = remaining / row.measures.len() as f64;

            for (index, measure) in row.measures.iter().enumerate() {
                x = measure.svg(x, y, measure_exta, index, renderer, node);
            }

            y += 100.;
        }
    }
}
