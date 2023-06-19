use concoct::{
    composable::{Container, Text, TextField},
    dimension::DevicePixels,
    taffy::style::{AlignItems, JustifyContent},
    View,
};
use staff::{render::Diagram, ui::FretDiagram};

fn app() {
    Container::build_column(|| {
        let diagram = Diagram::new(6, 5, 3);
        FretDiagram::new(diagram).view();
    })
    .align_items(AlignItems::Center)
    .justify_content(JustifyContent::Center)
    .flex_grow(1.)
    .view();
}

#[tokio::main]
async fn main() {
    concoct::render::run(app)
}
