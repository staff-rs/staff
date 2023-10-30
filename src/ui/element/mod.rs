use dioxus::{core::DynamicNode, prelude::*};

mod note;
pub use note::Note;

#[derive(Clone, PartialEq, Eq)]
pub enum StaffElement {
    Br,
    Hr,
    Clef(Clef),
    Note(Note),
}

#[derive(Clone, PartialEq, Eq)]
pub struct Clef {}

pub fn elements(node: &VNode) -> Vec<StaffElement> {
    let mut elements = Vec::new();
    elements_inner(node, &mut elements);
    elements
}

fn elements_inner(node: &VNode, elements: &mut Vec<StaffElement>) {
    for root in node.template.get().roots {
        match root {
            TemplateNode::Element {
                tag,
                namespace: _,
                attrs,
                children: _,
            } => {
                let elem = match *tag {
                    "note" => StaffElement::Note(Note::from_attrs(&node, attrs)),
                    "br" => StaffElement::Br,
                    "hr" => StaffElement::Hr,
                    "clef" => StaffElement::Clef(Clef {}),
                    _ => todo!(),
                };
                elements.push(elem);
            }
            TemplateNode::Dynamic { id } => {
                let node = &node.dynamic_nodes[*id];
                match node {
                    DynamicNode::Fragment(nodes) => {
                        for node in *nodes {
                            elements_inner(node, elements)
                        }
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        };
    }
}
