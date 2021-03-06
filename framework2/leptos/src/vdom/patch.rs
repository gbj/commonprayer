use std::collections::HashSet;

use itertools::Itertools;

use crate::{
    add_listener, link::Link, remove_child, remove_listener, replace_with, Attribute,
    EventListener, Host, Node,
};
use wasm_bindgen::JsCast;

pub fn patch_host(
    this: &web_sys::HtmlElement,
    shadow_root: &web_sys::ShadowRoot,
    old: &Host,
    new: &mut Host,
    link: &Link,
) {
    patch_attrs(this, &old.host_attrs, &new.host_attrs);
    patch_children(
        shadow_root.unchecked_ref(),
        &old.children,
        &mut new.children,
        link,
    );
}

pub fn patch(el: &web_sys::HtmlElement, link: &Link, old: &Node, new: &mut Node) {
    // check total replacements first
    if should_replace(old, new) {
        return replace_with(el, &new.to_node(link));
    }

    // at this point, both nodes are Element nodes
    // cases where they differ have been handled, and replaced
    // Text nodes have already been compared, and are either replaced or identical
    if let (Node::Element(old), Node::Element(new)) = (old, new) {
        patch_attrs(el, &old.attrs, &new.attrs);
        patch_listeners(el, link, &old.listeners, &mut new.listeners);
        patch_children(el, &old.children, &mut new.children, link);
    }
}

fn patch_attrs(el: &web_sys::HtmlElement, old_attrs: &[Attribute], new_attrs: &[Attribute]) {
    for attr in old_attrs
        .iter()
        .chain(new_attrs.iter())
        .unique_by(|attribute| attribute.attr_id())
    {
        let old_attr = old_attrs
            .iter()
            .find(|s_attr| s_attr.attr_id() == attr.attr_id());
        let new_attr = new_attrs
            .iter()
            .find(|s_attr| s_attr.attr_id() == attr.attr_id());

        match (old_attr, new_attr) {
            (None, None) => {}
            (None, Some(attr)) => attr.set(el),
            (Some(attr), None) => attr.remove(el),
            (Some(old), Some(new)) => {
                if old != new {
                    new.set(el)
                }
            }
        }
    }
}

fn patch_listeners(
    el: &web_sys::HtmlElement,
    link: &Link,
    old: &[EventListener],
    new: &mut [EventListener],
) {
    let event_names = old
        .iter()
        .chain(new.iter())
        .map(|listener| listener.event_name.clone())
        .collect::<HashSet<_>>();
    for event_name in event_names {
        let old_listener = old
            .iter()
            .find(|s_listener| s_listener.event_name == event_name);
        let new_listener = new
            .iter_mut()
            .find(|s_listener| s_listener.event_name == event_name);

        match (old_listener, new_listener) {
            (None, None) => {}
            (None, Some(listener)) => add_listener(listener, el, link),
            (Some(listener), None) => remove_listener(listener, el),
            (Some(old), Some(new)) => {
                if old != new {
                    remove_listener(old, el);
                    add_listener(new, el, link);
                }
            }
        }
    }
}

fn patch_children(
    el: &web_sys::HtmlElement,
    old_children: &[Node],
    new_children: &mut [Node],
    link: &Link,
) {
    // remove additional children in old node that are beyond the length of the new node
    // (because we're not doing keyed diffs, these simply don't need to exist)
    // patch children where the length intersects
    // insert children that only exist in the new node
    for idx in 0..std::cmp::max(old_children.len(), new_children.len()) {
        match (old_children.get(idx), new_children.get_mut(idx)) {
            (None, None) => {}
            (None, Some(new)) => new.mount(el, link),
            (Some(_), None) => {
                if let Some(node) = el.child_nodes().get(idx as u32) {
                    remove_child(el, &node);
                }
            }
            (Some(old), Some(new)) => {
                if let Some(child_node) = el.child_nodes().get(idx as u32) {
                    patch(child_node.unchecked_ref(), link, old, new);
                }
            }
        }
    }
}

fn should_replace(old: &Node, new: &Node) -> bool {
    // different node types
    if std::mem::discriminant(old) != std::mem::discriminant(new) {
        true
    }
    // keys are different
    else if let (Some(old_key), Some(new_key)) = (old.key(), new.key()) {
        old_key != new_key
    }
    // two text nodes
    else if let (Node::Text(old), Node::Text(new)) = (old, new) {
        old != new
    }
    // different tag name
    else if let (Node::Element(old), Node::Element(new)) = (old, new) {
        old.tag != new.tag
    }
    // otherwise, not a replacement
    else {
        false
    }
}
