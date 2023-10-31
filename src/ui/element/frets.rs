use dioxus::{prelude::*, core::AttributeValue};

pub struct Frets {
    pub from: u8,
    pub to: u8,
    pub index: u8,
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