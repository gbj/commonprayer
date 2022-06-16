use std::{collections::HashMap, fmt::Write, pin::Pin, process::Output};

use futures::{
    channel::mpsc::{unbounded, UnboundedSender},
    stream::FuturesUnordered,
    Stream, StreamExt,
};

use crate::{AsyncElement, Attribute, Element, Host, IntoProperty, Node};

impl Node {
    pub fn html_stream(self) -> impl Stream<Item = String> {
        self.html_stream_with_async_nodes(String::new(), Vec::new())
    }

    fn html_stream_with_async_nodes(
        self,
        mut sync_buf: String,
        mut async_nodes: Vec<AsyncElement>,
    ) -> Pin<Box<dyn Stream<Item = String>>> {
        // stream sync HTML for whole tree
        let mut props = Vec::new();
        self.write_node_with_async_nodes(
            &mut sync_buf,
            &mut props,
            &mut async_nodes,
            &mut 0,
            &mut 0,
            Vec::new(),
        );
        write_props_with_buf(&mut sync_buf, props);

        // wait for all
        if async_nodes.is_empty() {
            Box::pin(futures::stream::once(async { sync_buf }))
        } else {
            let async_els = FuturesUnordered::new();
            for (idx, el) in async_nodes.into_iter().enumerate() {
                async_els.push(async move {
                    let mut node = el.ready.unwrap().await;
                    if let Node::Element(ref mut el) = &mut node {
                        el.attrs.push(Attribute::Attribute(
                            "data-leptos-async-ready".into(),
                            Some(idx.to_string()),
                        ))
                    } else {
                        eprintln!(
                            "AsyncElements must contain an Element node at their root, not Text."
                        );
                    }
                    format!("{}<script>updateStreamedNode({})</script>", node, idx)
                });
            }
            Box::pin(
                futures::stream::once(async move {
                    format!(
                        "{}<script>{}</script>",
                        sync_buf,
                        include_str!("ssr_streaming.js")
                    )
                })
                .chain(async_els),
            )
        }
    }

    fn write_node_with_async_nodes(
        self,
        buf: &mut String,
        props: &mut Vec<(Vec<usize>, String, String)>,
        async_nodes: &mut Vec<AsyncElement>,
        property_id: &mut usize,
        shadow_root_id: &mut usize,
        shadow_root_path: Vec<usize>,
    ) {
        match self {
            Node::Text(data) => buf.push_str(&data),
            Node::Element(element) => {
                write_element_with_async_nodes(
                    buf,
                    element,
                    property_id,
                    shadow_root_id,
                    shadow_root_path,
                    props,
                    async_nodes,
                );
            }
            Node::AsyncElement(mut async_el) => {
                if let Node::Element(ref mut el) = async_el.pending.as_mut() {
                    el.attrs.push(Attribute::Attribute(
                        "data-leptos-async-pending".into(),
                        Some(async_nodes.len().to_string()),
                    ));
                } else {
                    eprintln!(
                        "AsyncElements must contain an Element node at their root, not Text."
                    );
                }
                async_el.pending.clone().write_node_with_async_nodes(
                    buf,
                    props,
                    async_nodes,
                    property_id,
                    shadow_root_id,
                    shadow_root_path,
                );
                async_nodes.push(async_el);
            }
        };
    }
}

fn write_props_with_buf(buf: &mut String, props: Vec<(Vec<usize>, String, String)>) {
    buf.push_str("<script>");
    for (shadow_path, prop_name, serialized_value) in props {
        buf.push_str(&format!(
            "(() => selectEl(document, {:?})[{:?}] = JSON.parse({:?}))();",
            shadow_path, prop_name, serialized_value
        ));
    }
    buf.push_str("</script>");
}

fn write_element_with_async_nodes(
    buf: &mut String,
    element: Element,
    property_id: &mut usize,
    shadow_root_id: &mut usize,
    shadow_root_path: Vec<usize>,
    props: &mut Vec<(Vec<usize>, String, String)>,
    async_nodes: &mut Vec<AsyncElement>,
) {
    buf.push('<');
    buf.push_str(&element.tag);

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
    write_attributes(buf, &element.attrs, property_id, &shadow_root_path, props);

    write_class(buf, &element.attrs);

    // note that it needs to hydrate, not mount, if shadow DOM is present
    if element.shadow_root.is_some() {
        buf.push_str(" data-leptos-hydrate=\"");
        buf.push_str(&shadow_root_id.to_string());
        buf.push('"');
    }

    // children
    if element.is_self_closing() {
        buf.push_str("/>");
    } else {
        buf.push('>');

        // Shadow DOM, if present
        if let Some(root_fn) = &element.shadow_root {
            let host = (root_fn)();
            buf.push_str("<template shadowroot=\"open\">");
            for child in host.children {
                child.write_node_with_async_nodes(
                    buf,
                    props,
                    async_nodes,
                    property_id,
                    shadow_root_id,
                    shadow_root_path.clone(),
                );
            }
            buf.push_str("</template>");
        }

        // innerHTML or children
        if let Some(html) = &element.inner_html {
            buf.push_str(html);
        } else {
            let mut shadowless_path = shadow_root_path.clone();
            shadowless_path.pop();

            for child in element.children {
                child.write_node_with_async_nodes(
                    buf,
                    props,
                    async_nodes,
                    property_id,
                    shadow_root_id,
                    shadow_root_path.clone(),
                );
            }
        }

        // closing tag name
        buf.push_str("</");
        buf.push_str(&element.tag);
        buf.push('>');
    }
}

fn write_attributes(
    buf: &mut String,
    attrs: &[Attribute],
    property_id: &mut usize,
    shadow_root_path: &Vec<usize>,
    properties: &mut Vec<(Vec<usize>, String, String)>,
) {
    for attr in attrs {
        match attr {
            Attribute::Attribute(name, Some(value)) => {
                if name != "class" {
                    buf.push(' ');
                    buf.push_str(name);
                    buf.push_str("=\"");
                    buf.push_str(value);
                    buf.push('"');
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
}

fn write_class(buf: &mut String, attrs: &[Attribute]) {
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
        buf.push_str(" class=\"");
        for class in classes {
            buf.push_str(class);
            buf.push(' ');
        }
        buf.push('"');
    }
}
