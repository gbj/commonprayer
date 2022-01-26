use std::pin::Pin;

use futures::{Stream, StreamExt};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;

use crate::{
    attribute::{DynamicAttributeValue, StaticAttributeValue},
    document,
    dynamic_element::{ClassStream, DynamicElement, EventListener, PropertyStream, StyleStream},
    log,
    operations::{
        append_child, create_comment_node, create_element, create_fragment, create_text_node,
        pick_up_comment_node, pick_up_text_node, remove_attribute, replace_with, set_attribute,
        set_data,
    },
    static_element::StaticElement,
};

/// A composable, platform-independent representation of the structure of the user interface
/// and its dynamic dependencies.
pub enum View {
    Doctype(&'static str),
    DynamicElement(DynamicElement),
    StaticElement(StaticElement),
    ViewStream(Pin<Box<dyn Stream<Item = View>>>),
    DynamicText(Pin<Box<dyn Stream<Item = String>>>),
    StaticText(String),
    Fragment(Vec<View>),
    Empty,
}

// Render methods
impl View {
    pub fn to_html(&self) -> String {
        self.to_html_with_hydration_key(0).1
    }

    fn to_html_with_hydration_key(&self, hydration_key: usize) -> (usize, String) {
        let mut hydration_key = hydration_key;
        let mut buffer = String::new();
        match self {
            View::Doctype(doctype) => {
                buffer.push_str("<!DOCTYPE ");
                buffer.push_str(doctype);
                buffer.push('>');
            }
            View::DynamicText(_) => {
                hydration_key += 1;
                // the two comment nodes separate this text node from its neighbors
                buffer.push_str("<!--hk");
                buffer.push_str(&hydration_key.to_string());
                // space needed to create a text node between the two comment nodes
                buffer.push_str("--> <!---->");
            }
            View::StaticText(value) => buffer.push_str(value.as_str()),
            View::Fragment(views) => {
                for view in views {
                    let (new_key, html) = view.to_html_with_hydration_key(hydration_key);
                    hydration_key = new_key;
                    buffer.push_str(&html);
                }
            }
            View::ViewStream(_) => {
                hydration_key += 1;
                // the comment node itself will be replaced during hydration/by new values, so no need for a text node
                buffer.push_str("<!--hk");
                buffer.push_str(&hydration_key.to_string());
                buffer.push_str("-->");
            }
            View::StaticElement(element) => {
                // opening tag name
                buffer.push('<');
                buffer.push_str(element.tag_name);

                // attributes
                for (attr_name, value) in &element.attributes {
                    let value = match value {
                        StaticAttributeValue::Static(value) => Some(value),
                        StaticAttributeValue::StaticOption(value) => value.as_ref(),
                    };
                    if let Some(value) = value {
                        buffer.push(' ');
                        buffer.push_str(attr_name);
                        buffer.push_str("=\"");
                        buffer.push_str(value);
                        buffer.push('"')
                    }
                }
                buffer.push('>');

                // empty elements can simply return here
                // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
                if let "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                | "meta" | "param" | "source" | "track" | "wbr" = element.tag_name
                {
                    return (hydration_key, buffer);
                }

                // children
                for child in &element.children {
                    let (new_key, html) = child.to_html_with_hydration_key(hydration_key);
                    hydration_key = new_key;
                    buffer.push_str(&html);
                }

                // closing tag name
                buffer.push_str("</");
                buffer.push_str(element.tag_name);
                buffer.push('>');
            }
            View::DynamicElement(element) => {
                hydration_key += 1;
                // opening tag name
                buffer.push('<');
                buffer.push_str(element.tag_name);

                // hydration key always added to dynamic elements
                buffer.push_str(" data-hk=\"");
                buffer.push_str(&hydration_key.to_string());
                buffer.push('"');

                // attributes are added only if static values
                // otherwise added when hydrating
                for (attr_name, value) in &element.attributes {
                    let value = match value {
                        DynamicAttributeValue::Static(value) => Some(value),
                        DynamicAttributeValue::StaticOption(value) => value.as_ref(),
                        _ => None,
                    };
                    if let Some(value) = value {
                        buffer.push(' ');
                        buffer.push_str(attr_name);
                        buffer.push_str("=\"");
                        buffer.push_str(value);
                        buffer.push('"')
                    }
                }
                buffer.push('>');

                // properties and events are ignored and added when hydrating

                // empty elements can simply return here
                // https://developer.mozilla.org/en-US/docs/Glossary/Empty_element
                if let "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link"
                | "meta" | "param" | "source" | "track" | "wbr" = element.tag_name
                {
                    return (hydration_key, buffer);
                }

                // children
                for child in &element.children {
                    let (new_key, html) = child.to_html_with_hydration_key(hydration_key);
                    hydration_key = new_key;
                    buffer.push_str(&html);
                }

                // closing tag name
                buffer.push_str("</");
                buffer.push_str(element.tag_name);
                buffer.push('>');
            }
            View::Empty => {}
        }
        (hydration_key, buffer)
    }

    pub fn hydrate(self, parent: &web_sys::Element) {
        let mut starting_key = 0;
        self.hydrate_with_hydration_key(parent, &mut starting_key);
    }

    fn hydrate_with_hydration_key(self, parent: &web_sys::Element, hydration_key: &mut usize) {
        match self {
            View::DynamicText(mut stream) => {
                *hydration_key += 1;

                let node = pick_up_text_node(parent.unchecked_ref(), *hydration_key);

                if let Some(node) = node {
                    spawn_local(async move {
                        while let Some(value) = stream.next().await {
                            set_data(&node, &value);
                        }
                    })
                }
            }
            View::DynamicElement(element) => {
                *hydration_key += 1;

                if let Ok(Some(el)) =
                    parent.query_selector(&format!("[data-hk=\"{}\"]", hydration_key))
                {
                    add_event_listeners(&el, element.event_listeners);
                    add_reactive_attributes(&el, element.attributes);
                    add_reactive_classes(&el, element.classes);
                    add_reactive_styles(&el, element.styles);
                    add_reactive_properties(&el, element.properties);
                } else {
                    log(&format!(
                        "WARNING: could not find element with hydration key {}",
                        hydration_key
                    ));
                }

                // hydrate children
                for child in element.children {
                    child.hydrate_with_hydration_key(parent, hydration_key);
                }
            }
            View::StaticElement(element) => {
                for child in element.children {
                    child.hydrate_with_hydration_key(parent, hydration_key);
                }
            }
            View::Fragment(children) => {
                for child in children {
                    child.hydrate_with_hydration_key(parent, hydration_key);
                }
            }
            View::ViewStream(mut stream) => {
                *hydration_key += 1;
                let node = pick_up_comment_node(parent.unchecked_ref(), *hydration_key);
                if let Some(mut current_node) = node {
                    spawn_local(async move {
                        while let Some(view) = stream.next().await {
                            let new_node = view.client_side_render();
                            replace_with(current_node.unchecked_ref(), &new_node.clone());
                            current_node = new_node;
                        }
                    })
                } else {
                    log(&format!(
                        "WARNING: could not find comment with hydration key {}",
                        hydration_key
                    ));
                }
            }
            _ => {}
        };
    }

    /// Creates a fresh node and maintains it, following state changes
    pub fn client_side_render(self) -> web_sys::Node {
        let document = document();
        match self {
            View::ViewStream(mut stream) => {
                let node = create_comment_node(&document);
                spawn_local({
                    let mut old_node = node.clone();
                    async move {
                        while let Some(view) = stream.next().await {
                            let new_node = view.client_side_render();
                            replace_with(old_node.unchecked_ref(), &new_node);
                            old_node = new_node;
                        }
                    }
                });
                node.unchecked_into()
            }
            View::DynamicElement(element) => {
                let el = create_element(&document, element.tag_name);

                add_event_listeners(&el, element.event_listeners);
                add_reactive_attributes(&el, element.attributes);
                add_reactive_classes(&el, element.classes);
                add_reactive_styles(&el, element.styles);
                add_reactive_properties(&el, element.properties);

                for child in element.children {
                    append_child(&el, &child.client_side_render());
                }

                el.unchecked_into()
            }
            View::StaticElement(element) => {
                let el = create_element(&document, element.tag_name);

                for (attr_name, value) in element.attributes {
                    let value = match value {
                        StaticAttributeValue::Static(value) => Some(value),
                        StaticAttributeValue::StaticOption(value) => value,
                    };
                    if let Some(value) = value {
                        set_attribute(&el, attr_name, &value);
                    }
                }

                for child in element.children {
                    append_child(&el, &child.client_side_render());
                }

                el.unchecked_into()
            }
            View::DynamicText(mut stream) => {
                let node = create_text_node(&document, "");
                spawn_local({
                    let node = node.clone();
                    async move {
                        while let Some(data) = stream.next().await {
                            set_data(&node, data.as_str());
                        }
                    }
                });
                node.unchecked_into()
            }
            View::StaticText(data) => create_text_node(&document, data.as_str()).unchecked_into(),
            View::Fragment(children) => {
                let fragment = create_fragment(&document);
                for child in children {
                    append_child(fragment.unchecked_ref(), &child.client_side_render());
                }
                fragment.unchecked_into()
            }
            _ => create_comment_node(&document),
        }
    }
}

fn add_event_listeners(el: &web_sys::Element, listeners: Vec<(&'static str, EventListener)>) {
    for (event_name, listener) in listeners {
        let closure = Closure::wrap(listener as Box<dyn Fn(_)>).into_js_value();
        el.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())
            .unwrap_throw();
    }
}

fn add_reactive_attributes(
    el: &web_sys::Element,
    attributes: Vec<(&'static str, DynamicAttributeValue)>,
) {
    for (attr_name, value) in attributes {
        match value {
            DynamicAttributeValue::Dynamic(mut stream) => {
                let el = el.clone();
                spawn_local(async move {
                    while let Some(value) = stream.next().await {
                        set_attribute(&el, attr_name, &value);
                    }
                })
            }
            DynamicAttributeValue::DynamicOption(mut stream) => {
                let el = el.clone();
                spawn_local(async move {
                    while let Some(value) = stream.next().await {
                        if let Some(value) = value {
                            set_attribute(&el, attr_name, &value);
                        } else {
                            remove_attribute(&el, attr_name);
                        }
                    }
                })
            }
            // static values already set in HTML, no need to hydrate
            DynamicAttributeValue::Static(_) => {}
            DynamicAttributeValue::StaticOption(_) => {}
        }
    }
}

fn add_reactive_classes(el: &web_sys::Element, classes: Vec<(&'static str, ClassStream)>) {
    for (class_name, mut stream) in classes {
        spawn_local({
            let el = el.clone();
            async move {
                while let Some(active) = stream.next().await {
                    if active {
                        el.class_list().add_1(class_name).unwrap_throw();
                    } else {
                        el.class_list().remove_1(class_name).unwrap_throw();
                    }
                }
            }
        });
    }
}

fn add_reactive_styles(el: &web_sys::Element, styles: Vec<(&'static str, StyleStream)>) {
    for (style_name, mut stream) in styles {
        spawn_local({
            let el = el.clone();
            async move {
                while let Some(value) = stream.next().await {
                    let style = el.unchecked_ref::<web_sys::HtmlElement>().style();
                    style.set_property(&style_name, &value).unwrap_throw();
                }
            }
        });
    }
}

fn add_reactive_properties(el: &web_sys::Element, properties: Vec<(&'static str, PropertyStream)>) {
    for (prop_name, mut stream) in properties {
        spawn_local({
            let el = el.clone();
            let prop_name = JsValue::from_str(prop_name);
            async move {
                while let Some(value) = stream.next().await {
                    match value {
                        Some(value) => unsafe {
                            js_sys::Reflect::set(&el, &prop_name, &JsValue::from_str(&value))
                                .unwrap_throw()
                        },
                        None => unsafe {
                            js_sys::Reflect::delete_property(&el, &prop_name).unwrap_throw()
                        },
                    };
                }
            }
        });
    }
}
