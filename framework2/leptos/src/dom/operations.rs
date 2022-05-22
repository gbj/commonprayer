#[macro_export]
macro_rules! is_server {
    () => {
        !cfg!(target_arch = "wasm32")
    };
}

use std::time::Duration;

use futures::Future;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};

pub fn log(s: &str) {
    if is_server!() {
        println!("{}", s);
    } else {
        web_sys::console::log_1(&JsValue::from_str(s));
    }
}

pub fn warn(s: &str) {
    if is_server!() {
        eprintln!("{}", s);
    } else {
        web_sys::console::warn_1(&JsValue::from_str(s));
    }
}

#[cfg(debug_assertions)]
pub fn debug_warn(s: &str) {
    if is_server!() {
        eprintln!("{}", s);
    } else {
        web_sys::console::warn_1(&JsValue::from_str(s));
    }
}

#[cfg(not(debug_assertions))]
pub fn debug_warn(s: &str) {}

pub fn window() -> web_sys::Window {
    web_sys::window().unwrap_throw()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap_throw()
}

pub fn body() -> Option<web_sys::HtmlElement> {
    document().body()
}

pub fn create_element(document: &web_sys::Document, tag_name: &str) -> web_sys::Element {
    document.create_element(tag_name).unwrap_throw()
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
    parent.append_child(child).unwrap_throw()
}

pub fn remove_child(parent: &web_sys::Element, child: &web_sys::Node) -> web_sys::Node {
    parent.remove_child(child).unwrap_throw()
}

pub fn insert_before(parent: &web_sys::Element, new: &web_sys::Node, existing: &web_sys::Node) {
    parent.insert_before(new, Some(existing)).unwrap_throw();
}

pub fn replace_with(old_node: &web_sys::Element, new_node: &web_sys::Node) {
    old_node.replace_with_with_node_1(new_node).unwrap_throw()
}

pub fn set_data(node: &web_sys::Text, value: &str) {
    node.set_data(value);
}

pub fn set_attribute(el: &web_sys::Element, attr_name: &str, value: &str) {
    el.set_attribute(attr_name, value).unwrap_throw()
}

pub fn remove_attribute(el: &web_sys::Element, attr_name: &str) {
    el.remove_attribute(attr_name).unwrap_throw()
}

pub fn set_property(el: &web_sys::Element, prop_name: &str, value: &Option<JsValue>) {
    let key = JsValue::from_str(prop_name);
    match value {
        Some(value) => js_sys::Reflect::set(el, &key, value).unwrap_throw(),
        None => js_sys::Reflect::delete_property(el, &key).unwrap_throw(),
    };
}

pub fn location() -> web_sys::Location {
    window().location()
}

pub fn descendants(el: &web_sys::Element) -> impl Iterator<Item = web_sys::Node> {
    let children = el.child_nodes();
    (0..children.length()).flat_map({
        move |idx| {
            let child = children.get(idx);
            if let Some(child) = child {
                // if an Element, send children
                if child.node_type() == 1 {
                    Box::new(descendants(&child.unchecked_into()))
                        as Box<dyn Iterator<Item = web_sys::Node>>
                }
                // otherwise, just the node
                else {
                    Box::new(std::iter::once(child)) as Box<dyn Iterator<Item = web_sys::Node>>
                }
            } else {
                Box::new(std::iter::empty()) as Box<dyn Iterator<Item = web_sys::Node>>
            }
        }
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

pub fn event_target_value(event: &web_sys::Event) -> String {
    event
        .target()
        .unwrap_throw()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .value()
}

pub fn event_target_checked(ev: &web_sys::Event) -> bool {
    ev.target()
        .unwrap()
        .unchecked_into::<web_sys::HtmlInputElement>()
        .checked()
}

pub fn event_target_selector(ev: &web_sys::Event, selector: &str) -> bool {
    matches!(
        ev.target().and_then(|target| {
            target
                .dyn_ref::<web_sys::Element>()
                .map(|el| el.closest(selector))
        }),
        Some(Ok(Some(_)))
    )
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

pub fn set_interval(cb: impl Fn() + 'static, duration: Duration) {
    let cb = Closure::wrap(Box::new(cb) as Box<dyn Fn()>).into_js_value();
    window()
        .set_interval_with_callback_and_timeout_and_arguments_0(
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

#[cfg(target_arch = "wasm32")]
pub fn spawn_local<F>(fut: F)
where
    F: Future<Output = ()> + 'static,
{
    wasm_bindgen_futures::spawn_local(fut)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn spawn_local<F>(_fut: F)
where
    F: Future<Output = ()> + 'static,
{
    // noop for now; useful for ignoring any async tasks on the server side
    // could be replaced with a Tokio dependency
}
