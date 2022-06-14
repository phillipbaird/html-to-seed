use html5ever::{
    driver::ParseOpts,
    local_name, namespace_url, ns,
    rcdom::{Node, NodeData, RcDom},
    tendril::TendrilSink,
    Attribute, QualName,
};
use std::rc::Rc;

mod attribute_map;

const INDENT: &str = "    ";
const NEW_LINE: char = '\n';

pub fn html_to_seed(fragment: &str, use_typed_classes: bool) -> String {
    let dom = parse_fragment(fragment);

    let mut buffer = String::new();
    walk(&mut buffer, &dom.document, 0, use_typed_classes);
    buffer
}

fn parse_fragment(fragment: &str) -> RcDom {
    let parser = html5ever::driver::parse_fragment(
        RcDom::default(),
        ParseOpts::default(),
        QualName::new(None, ns!(html), local_name!("body")),
        Vec::new(),
    );
    parser.one(fragment)
}

fn walk(
    buffer: &mut String,
    node: &Rc<Node>,
    indents: usize,
    use_typed_classes: bool,
) {
    match &node.data {
        NodeData::Document => {
            // Bypass the document node.
            walk_children(buffer, node, indents, 0, use_typed_classes);
        },

        NodeData::Element {
            name,
            attrs,
            ..
        } => {
            assert!(name.ns == ns!(html) || name.ns == ns!(svg));

            if name.local == local_name!("html") {
                // Bypass the html node at the top of the node tree.
                walk_children(buffer, node, indents, 0, use_typed_classes);
            } else {
                walk_element(
                    buffer,
                    node,
                    indents,
                    name,
                    &attrs.borrow(),
                    use_typed_classes,
                );
            }
        },

        NodeData::Text {
            contents,
        } => {
            let contents = contents.borrow().to_string();
            // Remove excessive whitespace from the HTML text e.g. newlines etc etc.
            let contents =
                contents.split_whitespace().collect::<Vec<&str>>().join(" ");
            if !contents.is_empty() {
                let indent = INDENT.repeat(indents);
                buffer.push_str(&format!(
                    "{}\"{}\",\n",
                    indent,
                    contents.escape_default()
                ));
            }
        },

        NodeData::Comment {
            contents,
        } => {
            let indent = INDENT.repeat(indents);
            buffer.push_str(&format!(
                "{}// {}",
                indent,
                contents.escape_default()
            ));
        },

        // Drop all other nodes on the floor.
        _ => (),
    }
}

fn walk_children(
    buffer: &mut String,
    node: &Rc<Node>,
    indents: usize,
    increment: usize,
    use_typed_classes: bool,
) {
    for child in node.children.borrow().iter() {
        walk(buffer, child, indents + increment, use_typed_classes);
    }
}

fn walk_element(
    buffer: &mut String,
    element: &Rc<Node>,
    indents: usize,
    name: &QualName,
    attrs: &[Attribute],
    use_typed_classes: bool,
) {
    let indent_0 = INDENT.repeat(indents);
    let indent_1 = INDENT.repeat(indents + 1);
    let indent_2 = INDENT.repeat(indents + 2);

    buffer.push_str(&format!("{}{}![", indent_0, name.local));

    let is_empty_element = attrs.len() + element.children.borrow().len() == 0;

    let maybe_class_attr = if use_typed_classes {
        attrs.iter().filter(|a| is_class_attribute(a)).last()
    } else {
        None
    };

    // Remove class attribute if using typed classes and class attribute exists.
    let mut attrs: Vec<&Attribute> = match maybe_class_attr {
        Some(class_attr) => attrs.iter().filter(|a| *a != class_attr).collect(),
        None => attrs.iter().collect(),
    };

    // Remove unnecessary SVG namespace attribute.
    attrs.retain(|&a| {
        a.name.local != local_name!("xmlns")
            || &*a.value != "http://www.w3.org/2000/svg"
    });

    if !is_empty_element {
        buffer.push(NEW_LINE)
    }

    if let Some(class_attr) = maybe_class_attr {
        let class_strs: Vec<&str> =
            class_attr.value.split_whitespace().collect();
        let multiple_classes = class_strs.len() > 1;

        if !class_strs.is_empty() {
            // Open class macro.
            buffer.push_str(&format!("{}C![", indent_1));
            if multiple_classes {
                buffer.push(NEW_LINE)
            }

            for (i, class_str) in class_strs.iter().enumerate() {
                // Indent multiple classes.
                if multiple_classes {
                    buffer.push_str(&indent_2)
                }

                buffer.push_str(&format!("C.{},", to_typed_class(class_str)));

                // Add a new-line if there are further attributes.
                let not_last_class = i < (class_strs.len() - 1);
                if not_last_class {
                    buffer.push(NEW_LINE)
                }
            }

            // Close class macro.
            if multiple_classes {
                buffer.push_str(&format!("\n{}],\n", indent_1));
            } else {
                buffer.push_str(" ],\n");
            }
        }
    }

    if !attrs.is_empty() {
        // Open the attributes macro.
        buffer.push_str(&format!("{}attrs!{{ ", indent_1));

        // Start mutiple attributes on a new line.
        let multiple_attrs = attrs.len() > 1;
        if multiple_attrs {
            buffer.push(NEW_LINE)
        }

        for (i, attr) in attrs.iter().enumerate() {
            let attr_name =
                attribute_map::seed_attr_name(&attr.name.local.to_string());

            // Indent multiple attributes.
            if multiple_attrs {
                buffer.push_str(&indent_2)
            }

            buffer
                .push_str(&format!("At::{} => \"{}\",", attr_name, attr.value));

            // Add a new-line if there are further attributes.
            let not_last_attr = i < (attrs.len() - 1);
            if not_last_attr {
                buffer.push(NEW_LINE)
            }
        }

        // Close the attributes macro.
        if multiple_attrs {
            buffer.push_str(&format!("\n{}}},\n", indent_1));
        } else {
            buffer.push_str(" },\n");
        }
    }

    walk_children(buffer, element, indents, 1, use_typed_classes);

    // Close the element.
    buffer.push_str(&format!(
        "{}],\n",
        if is_empty_element {
            String::default()
        } else {
            indent_0
        }
    ));
}

fn is_class_attribute(attr: &html5ever::Attribute) -> bool {
    attr.name.local.to_string().to_lowercase() == "class"
}

fn to_typed_class(class: &str) -> String {
    class.replace("/", "of").replace("-", "_").replace(":", "__")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn remove_whitespace(s: &str) -> String {
        s.chars().filter(|c| !c.is_whitespace()).collect()
    }

    #[test]
    fn div_div() {
        assert_eq!(
            remove_whitespace(&html_to_seed("<div><div></div></div>", false)),
            "div![div![],],"
        );
        assert_eq!(
            remove_whitespace(&html_to_seed(
                r#"<div><div id="myid" class="flex"></div></div>"#,
                false
            )),
            r#"div![div![attrs!{At::Id=>"myid",At::Class=>"flex",},],],"#
        );
        assert_eq!(
            remove_whitespace(&html_to_seed(
                r#"<div><div class="flex flex-col"></div></div>"#,
                true
            )),
            r#"div![div![C![C.flex,C.flex_col,],],],"#
        );
        assert_eq!(
            remove_whitespace(&html_to_seed(
                r#"<div><div id="myid" class="flex flex-col"></div></div>"#,
                true
            )),
            r#"div![div![C![C.flex,C.flex_col,],attrs!{At::Id=>"myid",},],],"#
        );
    }

    #[test]
    fn typed_class() {
        assert_eq!(to_typed_class("bg-purple-600"), "bg_purple_600");
        assert_eq!(
            to_typed_class("hover:border-gray-800"),
            "hover__border_gray_800"
        );
        assert_eq!(to_typed_class("sm:-m-4"), "sm___m_4");
        assert_eq!(to_typed_class("w-1/2"), "w_1of2");
    }
}
