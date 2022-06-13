use std::{collections::HashMap, fmt::Write};

use crate::{Attribute, Host, IntoProperty, Node};

impl std::fmt::Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut props = Vec::new();
        write_host(f, self, &mut 0, &mut 0, Vec::new(), &mut props)?;
        write_props(f, props)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut props = Vec::new();
        write_node(f, self, &mut 0, &mut 0, Vec::new(), &mut props)?;
        write_props(f, props)
    }
}

fn write_props(
    f: &mut std::fmt::Formatter<'_>,
    props: Vec<(Vec<usize>, String, String)>,
) -> std::fmt::Result {
    f.write_str("<script>")?;
    for (shadow_path, prop_name, serialized_value) in props {
        write!(
            f,
            "(() => selectEl(document, {:?})[{:?}] = JSON.parse({:?}))();",
            shadow_path, prop_name, serialized_value
        )?;
    }
    f.write_str("</script>")
}

fn write_host(
    f: &mut std::fmt::Formatter<'_>,
    host: &Host,
    property_id: &mut usize,
    shadow_root_id: &mut usize,
    shadow_root_path: Vec<usize>,
    props: &mut Vec<(Vec<usize>, String, String)>,
) -> std::fmt::Result {
    f.write_char('<')?;
    f.write_str(host.tag)?;

    // attributes placed on <Host>
    write_attributes(f, &host.host_attrs, property_id, &shadow_root_path, props)?;

    // attributes serialized from state
    write_attributes(f, &host.state_attrs, property_id, &shadow_root_path, props)?;

    write_class(f, &host.host_attrs)?;

    // note that this has been rendered and initial state needs to be hydrated, not mounted
    // include the shadow root ID so we can find this element again to hydrate props
    f.write_str(" data-leptos-hydrate=\"")?;
    write!(f, "{}", *shadow_root_id)?;
    f.write_char('"')?;

    f.write_char('>')?;

    // Declarative Shadow DOM syntax
    f.write_str("<template shadowroot=\"open\">")?;

    // children
    for child in &host.children {
        write_node(
            f,
            child,
            property_id,
            shadow_root_id,
            shadow_root_path.clone(),
            props,
        )?;
    }

    f.write_str("</template>")?;

    // closing tag name
    f.write_str("</")?;
    f.write_str(host.tag)?;
    f.write_char('>')?;
    Ok(())
}

fn write_node(
    f: &mut std::fmt::Formatter<'_>,
    node: &Node,
    property_id: &mut usize,
    shadow_root_id: &mut usize,
    shadow_root_path: Vec<usize>,
    props: &mut Vec<(Vec<usize>, String, String)>,
) -> std::fmt::Result {
    match node {
        Node::Text(data) => f.write_str(data),
        Node::Element(element) => {
            f.write_char('<')?;
            f.write_str(&element.tag)?;

            // if has shadow root, need to update shadow root path and id before serializing attributes
            let shadow_root_path = if element.shadow_root.is_some() {
                *shadow_root_id += 1;
                let mut new_path = shadow_root_path.clone();
                new_path.push(*shadow_root_id);
                new_path
            } else {
                shadow_root_path
            };

            // attributes
            write_attributes(f, &element.attrs, property_id, &shadow_root_path, props)?;

            write_class(f, &element.attrs)?;

            // note that it needs to hydrate, not mount, if shadow DOM is present
            if element.shadow_root.is_some() {
                f.write_str(" data-leptos-hydrate=\"")?;
                write!(f, "{}", *shadow_root_id)?;
                f.write_char('"')?;
            }

            // children
            if element.is_self_closing() {
                f.write_str("/>")?;
            } else {
                f.write_char('>')?;

                // Shadow DOM, if present
                if let Some(root_fn) = &element.shadow_root {
                    let host = (root_fn)();
                    f.write_str("<template shadowroot=\"open\">")?;
                    for child in &host.children {
                        write_node(
                            f,
                            child,
                            property_id,
                            shadow_root_id,
                            shadow_root_path.clone(),
                            props,
                        )?;
                    }
                    f.write_str("</template>")?;
                }

                // innerHTML or children
                if let Some(html) = &element.inner_html {
                    f.write_str(html)?;
                } else {
                    let mut shadowless_path = shadow_root_path.clone();
                    shadowless_path.pop();

                    for child in &element.children {
                        write_node(
                            f,
                            child,
                            property_id,
                            shadow_root_id,
                            shadowless_path.clone(),
                            props,
                        )?;
                    }
                }

                // closing tag name
                f.write_str("</")?;
                f.write_str(&element.tag)?;
                f.write_char('>')?;
            }
            Ok(())
        }
    }
}

fn write_attributes(
    f: &mut std::fmt::Formatter<'_>,
    attrs: &[Attribute],
    property_id: &mut usize,
    shadow_root_path: &Vec<usize>,
    properties: &mut Vec<(Vec<usize>, String, String)>,
) -> std::fmt::Result {
    for attr in attrs {
        match attr {
            Attribute::Attribute(name, Some(value)) => {
                if name != "class" {
                    f.write_char(' ')?;
                    f.write_str(name)?;
                    f.write_str("=\"")?;
                    f.write_str(value)?;
                    f.write_char('"')?;
                }
            }
            Attribute::Property(name, value) => {
                // add to array to be serialized
                properties.push((
                    shadow_root_path.clone(),
                    name.to_string(),
                    value.serialize(),
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn write_class(f: &mut std::fmt::Formatter<'_>, attrs: &[Attribute]) -> std::fmt::Result {
    let classes = attrs
        .iter()
        .flat_map(|attr| match &attr {
            Attribute::Attribute(name, value) if name == "class" => {
                if let Some(value) = value {
                    Box::new(value.split(' ')) as Box<dyn Iterator<Item = &str>>
                } else {
                    Box::new(std::iter::empty())
                }
            }
            Attribute::Class(name, on) => {
                if *on {
                    Box::new(std::iter::once(name.as_str())) as Box<dyn Iterator<Item = &str>>
                } else {
                    Box::new(std::iter::empty())
                }
            }
            _ => Box::new(std::iter::empty()),
        })
        .collect::<Vec<_>>();

    if !classes.is_empty() {
        f.write_str(" class=\"")?;
        for class in classes {
            f.write_str(class)?;
            f.write_char(' ')?;
        }
        f.write_char('"')?;
    }

    Ok(())
}
/*
pub fn serialize_props(tree: &[Node]) -> Option<String> {
    let mut buf = String::new();
    let mut cid = 0;
    for node in tree {
        node.serialize_properties_with_property_id(&mut buf, &mut cid);
    }
    if buf.is_empty() {
        None
    } else {
        buf.push_str("\n}");
        Some(buf)
    }
}

impl Node {
    fn serialize_properties_with_property_id(&self, buf: &mut String, property_id: &mut usize) {
        match self {
            Node::Element(el) => {
                for attr in &el.attrs {
                    if let Attribute::Property(_, value) = attr {
                        if *property_id == 0 {
                            buf.push_str("{\n\t")
                        }
                        if *property_id > 0 {
                            buf.push_str(",\n\t")
                        }

                        // key
                        buf.push_str("\"p");
                        buf.push_str(&property_id.to_string());
                        buf.push_str("\": ");

                        // value
                        buf.push_str(&format!("JSON.parse({:?})", value.serialize()));

                        *property_id += 1;
                    }
                }

                for child in &el.children {
                    child.serialize_properties_with_property_id(buf, property_id);
                }
            }
            Node::Text(_) => {}
        }
    }
}
 */
