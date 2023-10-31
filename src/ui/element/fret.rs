use dioxus::{prelude::*, core::AttributeValue};

pub struct Fret {
    pub index: u8,
    pub string: u8,
    pub is_muted: bool,
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
