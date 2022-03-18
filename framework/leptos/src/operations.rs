use std::time::Duration;

use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

use crate::is_server;

pub fn log(s: &str) {
    unsafe {
        web_sys::console::log_1(&JsValue::from_str(s));
    }
}

pub fn warn(s: &str) {
    unsafe {
        web_sys::console::warn_1(&JsValue::from_str(s));
    }
}

pub fn window() -> web_sys::Window {
    web_sys::window().expect("could not find window object")
}

pub fn document() -> web_sys::Document {
    window()
        .document()
        .expect("could not find window.document object")
}

pub fn body() -> Option<web_sys::HtmlElement> {
    document().body()
}

pub fn create_element(document: &web_sys::Document, tag_name: &str) -> web_sys::Element {
    document
        .create_element(tag_name)
        .expect("could not create element")
}

pub fn create_text_node(document: &web_sys::Document, data: &str) -> web_sys::Text {
    document.create_text_node(data)
}

pub fn create_fragment(document: &web_sys::Document) -> web_sys::DocumentFragment {
    document.create_document_fragment()
}

pub fn create_comment_node(document: &web_sys::Document) -> web_sys::Node {
    document.create_comment("").unchecked_into()
}

pub fn append_child(parent: &web_sys::Element, child: &web_sys::Node) -> web_sys::Node {
    parent.append_child(child).expect("could not append child")
}

pub fn replace_with(old_node: &web_sys::Element, new_node: &web_sys::Node) {
    old_node
        .replace_with_with_node_1(new_node)
        .expect("could not replace node");
}

pub fn set_data(node: &web_sys::Text, value: &str) {
    node.set_data(value);
}

pub fn set_attribute(el: &web_sys::Element, attr_name: &str, value: &str) {
    el.set_attribute(attr_name, value)
        .expect("could not set attribute on element");
}

pub fn remove_attribute(el: &web_sys::Element, attr_name: &str) {
    el.remove_attribute(attr_name)
        .expect("could not remove attribute on element");
}

pub fn location() -> web_sys::Location {
    window().location()
}

pub fn descendants(el: &web_sys::Element) -> impl Iterator<Item = web_sys::Element> {
    let children = el.children();
    (0..children.length()).filter_map({
        let children = children.clone();
        move |idx| children.item(idx)
    })
}

/// Current window.location.hash without the beginning #
pub fn location_hash() -> Option<String> {
    if is_server!() {
        None
    } else {
        location().hash().ok().map(|hash| hash.replace('#', ""))
    }
}

pub fn location_pathname() -> Option<String> {
    location().pathname().ok()
}

pub fn event_target_value(event: web_sys::Event) -> String {
    event
        .target()
        .unwrap_throw()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .value()
}

pub fn request_animation_frame(cb: impl Fn() + 'static) {
    let cb = Closure::wrap(Box::new(cb) as Box<dyn Fn()>).into_js_value();
    window()
        .request_animation_frame(cb.as_ref().unchecked_ref())
        .unwrap_throw();
}

pub fn set_timeout(cb: impl Fn() + 'static, duration: Duration) {
    let cb = Closure::wrap(Box::new(cb) as Box<dyn Fn()>).into_js_value();
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            cb.as_ref().unchecked_ref(),
            duration.as_millis().try_into().unwrap_throw(),
        )
        .unwrap_throw();
}

// Hydration operations to find text and comment nodes
pub fn pick_up_comment_node(
    parent: &web_sys::HtmlElement,
    node_idx: usize,
) -> Option<web_sys::Node> {
    let mut node_identifier = String::from("hk");
    node_identifier.push_str(&node_idx.to_string());

    let walker = document()
        .create_tree_walker_with_what_to_show(parent, 128) // = NodeFilter.SHOW_COMMENT
        .unwrap_throw();
    while let Some(node) = walker.next_node().unwrap_throw() {
        if let Some(value) = node.node_value() {
            if value == node_identifier {
                return Some(node);
            }
        }
    }
    None
}

pub fn pick_up_text_node(parent: &web_sys::HtmlElement, node_idx: usize) -> Option<web_sys::Text> {
    let mut node_identifier = String::from("hk");
    node_identifier.push_str(&node_idx.to_string());

    let walker = document()
        .create_tree_walker(parent) //_with_what_to_show(&node, 128) // = NodeFilter.SHOW_COMMENT
        .unwrap_throw();
    while let Some(node) = walker.next_node().unwrap_throw() {
        let next_value = node.node_value();
        if next_value.is_some() && next_value.unwrap_throw() == node_identifier {
            let next_node = walker.next_node().unwrap_throw();
            if let Some(node) = next_node {
                // if it's Node.TEXT_NODE
                if node.node_type() == 3 {
                    return Some(node.unchecked_into());
                }
            }
        }
    }
    None
}
