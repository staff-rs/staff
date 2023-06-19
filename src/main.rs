use concoct::{
    composable::{state, Container, TextField},
    taffy::style::{AlignItems, JustifyContent},
    View,
};
use staff::{render::Diagram, ui::FretDiagram};

fn app() {
    Container::build_column(|| {
        let string_count = state(|| 4);

        let diagram = Diagram::new(string_count.cloned(), 5, 3);
        FretDiagram::new(diagram).view();

        TextField::build(string_count.cloned().to_string(), move |value| {
            string_count.set(value.parse().unwrap_or_default());
        })
        .view();
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
