use crate::render::Diagram;

pub struct FretDiagram {
    diagram: Diagram,
}

impl FretDiagram {
    pub fn new(diagram: Diagram) -> Self {
        Self { diagram }
    }
}
