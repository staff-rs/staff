use concoct::composable::Container;
use concoct::dimension::{Dimension, Size, DevicePixels};
use concoct::skia_safe::{colors, Color4f, Paint, Point};
use concoct::taffy::style::{AlignItems, JustifyContent};
use concoct::{
    composable::{Element, Text},
    modify::ModifyExt,
    render::run,
    Modifier, Modify, View,
};
use staff::render::Diagram;

pub struct FretDiagram {
    diagram: Diagram,
}

impl FretDiagram {
    pub fn new(diagram: Diagram) -> Self {
        Self { diagram }
    }
}

impl View for FretDiagram {
    #[track_caller]
    fn view(self) {
        let string_count = self.diagram.strings();
        let fret_count = self.diagram.frets();
        let starting_fret = self.diagram.starting_fret;
        let stroke_width = 2.;

        Container::build_row(move || {
            Text::build(starting_fret.to_string())
                .font_size(48.dp())
                .view();

            Container::build(|| {}, Default::default())
                .size(Size::from(Dimension::Points(400.)))
                .modifier(
                    Modifier
                        .background_color(colors::RED)
                        .draw(move |layout, canvas| {
                            let width = layout.size.width / (string_count + 1) as f32;
                            let gap = width /2.;
                            let start =  layout.location.x + gap;

                            let height = layout.size.height / fret_count as f32;

                            let mut paint = Paint::new(colors::BLACK, None);
                            paint.set_stroke(true);
                            paint.set_stroke_width(stroke_width);

                            for i in 0..=string_count {
                                let p1 = Point::new(
                                   start+ (width * i as f32),
                                    layout.location.y,
                                );
                                let p2 = Point::new(
                                    start + (width * i as f32),
                                    layout.location.y + layout.size.height,
                                );
                                canvas.draw_line(p1, p2, &paint);
                            }

                            for i in 0..=fret_count {
                                let p1 = Point::new(
                                    start,
                                    layout.location.y + (height * i as f32),
                                );
                                let p2 = Point::new(
                                    layout.location.x + layout.size.width - gap,
                                    layout.location.y + (height * i as f32),
                                );
                                canvas.draw_line(p1, p2, &paint);
                            }
                        }),
                )
                .view();
        })
        .justify_content(JustifyContent::Center)
        .view();
    }
}

fn app() {
    Container::build_column(|| {
        Text::build("Chord Editor").font_size(48.dp()).view();

        let diagram = Diagram::new(6, 5, 3);
        FretDiagram::new(diagram).view();
    })
    .align_items(AlignItems::Center)
    .justify_content(JustifyContent::SpaceBetween)
    .flex_grow(1.)
    .view();
}

#[tokio::main]
async fn main() {
    run(app)
}
