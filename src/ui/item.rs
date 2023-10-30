use super::element::{self};

pub struct Item {
    pub element: element::Element,
    pub x: f64,
    pub y: f64,
}

pub fn items() {}

/*
pub fn items<'a>(node: &'a VNode<'a>, width: f64) -> impl Iterator<Item = (Item, bool)> + 'a {
    let mut x = 0.;
    let mut y = 0.;
    let mut is_newline = true;

    node.template
        .get()
        .roots
        .iter()
        .map(move |root| match root {
            TemplateNode::Element {
                tag,
                namespace: _,
                attrs,
                children: _,
            } => {
                let (elem, elem_width) = match *tag {
                    "note" => (element::Element::Note(Note::from_attrs(&node, attrs)), 80.),
                    "br" => (element::Element::Br(Br {}), 30.),
                    "clef" => (element::Element::Clef(Clef {}), 60.),
                    _ => todo!(),
                };

                let item = Item {
                    element: elem,
                    x: x,
                    y: y,
                };

                x += elem_width;

                let old_is_newline = is_newline;
                is_newline = x > width;
                if is_newline {
                    x = 0.;
                    y += 120.;
                }

                (item, old_is_newline)
            }
            _ => todo!(),
        })
}
 */
