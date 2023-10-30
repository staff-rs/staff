use dioxus::{
    core::{AttributeValue, DynamicNode},
    prelude::*,
};

#[component]
pub fn FretDiagram<'a>(
    cx: Scope<'a>,

    children: Element<'a>,

    /// Amount of strings in the diagram.
    #[props(default = 6)]
    string_count: u8,

    /// Amount of frets in the diagram.
    #[props(default = 5)]
    fret_count: u8,

    /// Top-most position of the diagram.
    #[props(default = 0.)]
    x: f64,

    /// Left-most position of the diagram.
    #[props(default = 0.)]
    y: f64,

    /// Width of the diagram.
    #[props(default = 150.)]
    width: f64,

    /// Height of the diagram.
    #[props(default = 200.)]
    height: f64,

    /// Stroke width of items in the diagram.
    #[props(default = 2.)]
    stroke_width: f64,
) -> Element<'a> {
    let fret_width = *width / (*string_count + 1) as f64;
    let fret_height = (*height - stroke_width * 6.) / (*fret_count + 1) as f64;

    let x = x + fret_width;
    let y = y + fret_height;

    let fret_lines = (2..=*fret_count).map(|i| {
        let line_y = fret_height * i as f64 + stroke_width * 5.5;
        render!(path {
            stroke: "#000",
            stroke_width: *stroke_width,
            d: "M{x} {line_y}L{width - fret_width} {line_y}"
        })
    });

    let string_lines = (1..=*string_count).map(|i| {
        let line_x = fret_width * i as f64 + stroke_width / 2.;
        render!(path {
            stroke: "#000",
            stroke_width: *stroke_width,
            d: "M{line_x} {y}L{line_x} {height - fret_height}"
        })
    });

    let elements = elements(children.as_ref().unwrap());
    let elements = elements.iter().map(|element| match element {
        FretDiagramElement::Fret(fret) => {
            render!(circle {
                cx: x + fret.string as f64 * fret_width,
                cy: y + (fret.index as f64 - 0.5) * fret_height + stroke_width * 4.5,
                r: fret_width.min(fret_height) / 2.,
                fill: "#000"
            })
        }
        FretDiagramElement::Frets(frets) => {
            render!(rect {
                x: x + frets.from as f64 * fret_width + stroke_width / 2.,
                y: y + frets.index as f64 * fret_height + stroke_width * 4.5,
                width: (frets.to - 1) as f64 * fret_width - stroke_width / 2.,
                height: (frets.index as f64 + 0.75) * fret_height,
                rx: fret_width.max(fret_height) / 2.,
                fill: "#000"
            })
        }
    });

    render! {
        path {
            stroke: "#000",
            stroke_width: *stroke_width * 4.,
            d: "M{x} {y + *stroke_width * 2.}L{width - fret_width} {y + *stroke_width * 2.}"
        }
        fret_lines,
        string_lines,
        elements
    }
}

pub struct Fret {
    index: u8,
    string: u8,
    is_muted: bool,
}
impl Fret {
    pub fn from_attrs(node: &VNode, attrs: &[TemplateAttribute]) -> Self {
        let mut index = None;
        let mut string = None;
        let mut is_muted = false;
        for attr in attrs {
            match attr {
                TemplateAttribute::Static {
                    name: _,
                    value: _,
                    namespace: _,
                } => todo!(),
                TemplateAttribute::Dynamic { id } => {
                    let attr = &node.dynamic_attrs[*id];
                    match attr.name {
                        "index" => {
                            if let AttributeValue::Int(i) = attr.value {
                                index = Some(i as _)
                            } else {
                                todo!()
                            }
                        }
                        "string" => {
                            if let AttributeValue::Int(i) = attr.value {
                                string = Some(i as _)
                            } else {
                                todo!()
                            }
                        }
                        "is_muted" => {
                            if let AttributeValue::Bool(b) = attr.value {
                                is_muted = b;
                            } else {
                                todo!()
                            }
                        }
                        _ => todo!(),
                    }
                }
            }
        }

        Fret {
            index: index.unwrap(),
            string: string.unwrap(),
            is_muted,
        }
    }
}

pub struct Frets {
    from: u8,
    to: u8,
    index: u8,
}
impl Frets {
    pub fn from_attrs(node: &VNode, attrs: &[TemplateAttribute]) -> Self {
        let mut from = None;
        let mut to = None;
        let mut index = None;
        for attr in attrs {
            match attr {
                TemplateAttribute::Static {
                    name: _,
                    value: _,
                    namespace: _,
                } => todo!(),
                TemplateAttribute::Dynamic { id } => {
                    let attr = &node.dynamic_attrs[*id];
                    match attr.name {
                        "from" => {
                            if let AttributeValue::Int(i) = attr.value {
                                from = Some(i as _)
                            } else {
                                todo!()
                            }
                        }
                        "to" => {
                            if let AttributeValue::Int(i) = attr.value {
                                to = Some(i as _)
                            } else {
                                todo!()
                            }
                        }
                        "index" => {
                            if let AttributeValue::Int(i) = attr.value {
                                index = Some(i as _)
                            } else {
                                todo!()
                            }
                        }
                        _ => todo!(),
                    }
                }
            }
        }

        Frets {
            from: from.unwrap(),
            to: to.unwrap(),
            index: index.unwrap(),
        }
    }
}

enum FretDiagramElement {
    Fret(Fret),
    Frets(Frets),
}

fn elements(node: &VNode) -> Vec<FretDiagramElement> {
    let mut elements = Vec::new();
    elements_inner(node, &mut elements);
    elements
}

fn elements_inner(node: &VNode, elements: &mut Vec<FretDiagramElement>) {
    for root in node.template.get().roots {
        match root {
            TemplateNode::Element {
                tag,
                namespace: _,
                attrs,
                children: _,
            } => {
                let elem = match *tag {
                    "fret" => FretDiagramElement::Fret(Fret::from_attrs(&node, attrs)),
                    "frets" => FretDiagramElement::Frets(Frets::from_attrs(&node, attrs)),
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
