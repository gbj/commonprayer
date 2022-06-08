use crate::{remove_child, replace_with, Attribute, Link, Node};
use itertools::Itertools;
use wasm_bindgen::JsCast;

pub fn patch_dom(old: &web_sys::Node, new: &mut Node) {
    // check total replacements first
    if should_replace_dom(old, new) {
        return replace_with(old.unchecked_ref(), &new.to_node(&Link::empty()));
    }

    // at this point, both nodes are Element nodes
    // cases where they differ have been handled, and replaced
    // Text nodes have already been compared, and are either replaced or identical
    if let Node::Element(new) = new {
        patch_attrs_dom(old.unchecked_ref(), &new.attrs);
        patch_children_dom(old.unchecked_ref(), &mut new.children, &Link::empty());
    }
}

const ELEMENT_NODE: u16 = 1;
const TEXT_NODE: u16 = 3;

fn should_replace_dom(old: &web_sys::Node, new: &Node) -> bool {
    // different node types
    match (old.node_type(), new) {
        (ELEMENT_NODE, Node::Text(_)) => return true,
        (ELEMENT_NODE, Node::Element(_)) => {}
        (TEXT_NODE, Node::Element(_)) => return true,
        (TEXT_NODE, Node::Text(_)) => {}
        _ => return true,
    };

    // two text nodes
    if let Node::Text(new) = new {
        &old.text_content().unwrap_or_default() != new
    }
    // different tag name
    else if let Node::Element(new) = new {
        old.node_name().to_lowercase() != new.tag
    }
    // otherwise, not a replacement
    else {
        false
    }
}

fn patch_attrs_dom(old: &web_sys::HtmlElement, new_attrs: &[Attribute]) {
    let mut old_attrs = Vec::new();
    let old_attrs_dom = old.attributes();
    for idx in 0..old_attrs_dom.length() {
        if let Some(attr) = old_attrs_dom.item(idx) {
            old_attrs.push(Attribute::Attribute(attr.name(), Some(attr.value())));
        }
    }

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

        let el = old;
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

fn patch_children_dom(old: &web_sys::HtmlElement, new_children: &mut [Node], link: &Link) {
    let el = old;

    let mut old_children = Vec::new();
    let old_children_dom = old.child_nodes();
    for idx in 0..old_children_dom.length() {
        if let Some(old_child) = old_children_dom.item(idx) {
            old_children.push(old_child);
        }
    }

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
                patch_dom(old, new);
            }
        }
    }
}
