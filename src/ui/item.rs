use super::{
    element::Note,
    layout::{Layout, LayoutElement},
};
use dioxus::prelude::*;
use dioxus_signals::{use_selector, use_signal, ReadOnlySignal, Signal};

#[derive(Clone, PartialEq)]
pub enum ItemKind {
    Br,
    Hr,
    Note { layout: Layout, note: Note },
}

#[derive(Clone, PartialEq)]
pub struct Item {
    pub x: f64,
    pub y: f64,
    pub kind: ItemKind,
}

pub fn use_items<T>(
    cx: Scope<T>,
    layouts: Signal<Vec<Signal<LayoutElement>>>,
    width: f64,
) -> ReadOnlySignal<Vec<(Item, bool)>> {
    let width_signal = use_signal(cx, || width);
    use_effect(cx, &width, |w| {
        width_signal.set(w);
        async {}
    });

    use_selector(cx, move || {
        let mut y = 0.;
        let mut left = 0.;
        let mut is_newline = true;

        let layouts_ref = layouts.read();
        let width = *width_signal.read();

        layouts_ref
            .iter()
            .map(|signal| {
                let layout_element = &*signal.read();

                let old_is_newline = is_newline;
                is_newline = false;
                if left >= width && width > 0. {
                    left = 0.;
                    y += 140.;
                    is_newline = true;
                }

                let item = match layout_element {
                    LayoutElement::Br => {
                        left = 0.;
                        y += 140.;
                        is_newline = true;

                        Item {
                            x: left,
                            y,
                            kind: ItemKind::Br,
                        }
                    }
                    LayoutElement::Hr => {
                        let x = left;
                        left += 30.;
                        Item {
                            x,
                            y,
                            kind: ItemKind::Hr,
                        }
                    }
                    LayoutElement::Note { layout, element } => {
                        let x = left;
                        left += layout.width();

                        Item {
                            x,
                            y,
                            kind: ItemKind::Note {
                                note: element.clone(),
                                layout: layout.clone(),
                            },
                        }
                    }
                    _ => todo!(),
                };
                (item, old_is_newline)
            })
            .collect::<Vec<_>>()
    })
}
