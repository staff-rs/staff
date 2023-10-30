use crate::{
    note::Accidental,
    time::{Duration, DurationKind},
    ui::Text,
};
use dioxus::prelude::*;
use dioxus_resize_observer::Rect;
use dioxus_signals::{use_signal, Signal};

#[component]
pub fn NoteLayout<'a>(
    cx: Scope<'a>,
    onlayout: EventHandler<'a, Layout>,
    font_size: f64,
    duration: Duration,
    #[props(!optional)] accidental: Option<Accidental>,
) -> Element<'a> {
    let layout: Signal<Option<Layout>> = use_signal(cx, || None);

    use_effect(cx, &*layout.read(), move |_| {
        if let Some(layout) = &*layout.read() {
            onlayout.call(layout.clone());
        }
        async {}
    });

    if let Some(accidental) = accidental {
        render!(
            Text {
                content: "{accidental}",
                font_family: "Noto Music",
                font_size: *font_size,
                onresize: move |rect: Rect| {
                    let ret = (*accidental, [rect.width(), rect.height()]);
                    let layout_ref = &mut *layout.write();
                    if let Some(layout) = layout_ref {
                        layout.accidental = Some(ret)
                    } else {
                        *layout_ref = Some(Layout {
                            accidental: Some(ret),
                            duration: *duration,
                        })
                    };
                }
            }
        )
    } else {
        None
    }
}

#[derive(Clone, PartialEq)]
pub struct Layout {
    pub accidental: Option<(Accidental, [f64; 2])>,
    pub duration: Duration,
}

impl Layout {
    pub fn width(&self) -> f64 {
        let mut w = 20.;

        w += match self.duration.kind {
            DurationKind::Eigth => 10.,
            DurationKind::Quarter => 20.,
            DurationKind::Half => 40.,
            DurationKind::Whole => 80.,
        };

        if let Some((_, size)) = self.accidental {
            w += size[0];
        }
        w
    }
}
