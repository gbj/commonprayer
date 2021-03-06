use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::Expr;
use syn_rsx::{Node, NodeType};

pub fn node_to_tokens(node: &Node) -> Option<TokenStream> {
    match node.node_type {
        NodeType::Element => {
            let tag = node.name_as_string();
            let span = node.name_span().unwrap();

            if tag.is_some() {
                element_node(node)
            } else {
                Some(quote_spanned! {
                    span => compile_error!("blocks in tag name position are not supported")
                })
            }
        }
        NodeType::Text | NodeType::Block => {
            let value = node.value.as_ref().unwrap();

            Some(quote! {
                #value
            })
        }
        NodeType::Fragment => {
            let children = node.children.iter().filter_map(node_to_tokens);
            Some(quote! {
                vec![
                    #(#children,)*
                ]
            })
        }
        NodeType::Comment => None,
        NodeType::Doctype => None,
        NodeType::Attribute => None,
    }
}

fn element_node(node: &Node) -> Option<TokenStream> {
    let listeners = listeners_from_node(node, "on:");

    let classes = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let attr_name = attr.name_as_string().unwrap();
            if attr_name.starts_with("class:") {
                let class_name = attr_name.replace("class:", "");
                let value = attr.value.as_ref();
                let span = attr.name_span().unwrap();
                Some(
                    value
                        .map({
                            let class_name = class_name.clone();
                            move |value| {
                                quote_spanned! {
                                    span => leptos2::Attribute::Class(#class_name.to_string(), #value)
                                }
                            }
                        })
                        .unwrap_or_else(|| {
                            quote_spanned! {
                                span => leptos2::Attribute::Class(#class_name.to_string(), true)
                            }
                        }),
                )
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let attributes = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let attr_name = attr.name_as_string().unwrap();
            if true_attr(attr) || attr_name.starts_with("prop:") {
                let real_attr_name = if attr_name.starts_with("prop:") {
                    attr_name.replace("prop:", "")
                } else if attr_name.starts_with('_') {
                    attr_name.replacen('_', "", 1)
                } else {
                    attr_name.to_string()
                };
                let value = attr.value.as_ref();
                let span = attr.name_span().unwrap();
                value.map(|value| {
                    if attr_name.starts_with("prop:") {
                        quote_spanned! {
                            span => leptos2::property(#real_attr_name, #value)
                        }
                    } else {
                        quote_spanned! {
                            span => leptos2::attribute(#real_attr_name, #value)
                        }
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let span = node.name_span().unwrap();

    let attributes = quote_spanned! {
        span => vec![
             #(#attributes,)*
             #(#classes,)*
        ]
    };

    let tag = node.name_as_string().unwrap();

    let children = node
        .children
        .iter()
        .filter_map(node_to_tokens)
        .collect::<Vec<_>>();

    if tag == "Host" {
        let window_listeners = listeners_from_node(node, "window:");
        let document_listeners = listeners_from_node(node, "document:");
        let foreign_listeners = foreign_listeners_from_node(node);

        Some(quote_spanned! {
            span => leptos2::Host {
                tag: Self::tag(),
                host_attrs: #attributes,
                state_attrs: self.state_to_attributes(),
                listeners: #listeners,
                window_listeners: #window_listeners,
                document_listeners: #document_listeners,
                foreign_listeners: #foreign_listeners,
                children: vec![]
            }
            #(.child(#children))*
        })
    } else {
        // can refer to components by their Rust type and automatically insert the Custom Element tag
        let is_component = tag.chars().next().unwrap().is_ascii_uppercase() && tag != "Host";

        // for SSR, will embed a function that allows you to recreate the shadow DOM
        // so that we can render it declaratively
        let shadow_root = if is_component {
            let component_name: TokenStream = tag.parse().unwrap();
            let attrs = node
                .attributes
                .iter()
                .filter(|attr| true_attr(attr))
                .filter_map(|attr| match (attr.name_as_string(), &attr.value) {
                    (Some(name), Some(value)) => {
                        let span = attr.name_span().unwrap();
                        Some(quote_spanned! { span => #value.to_attribute(#name.to_string()) })
                    }
                    _ => None,
                });
            let props = node.attributes.iter().filter_map(|attr| {
                let attr_name = attr.name_as_string().unwrap();
                if attr_name.starts_with("prop:") {
                    let prop_name = attr_name.replace("prop:", "");
                    let prop_name = prop_name.parse::<TokenStream>().unwrap();

                    let span = attr.name_span().unwrap();

                    attr.value.as_ref().map(|value| {
                        quote_spanned! {
                            span => host.#prop_name = #value;
                        }
                    })
                } else {
                    None
                }
            });

            let span = node.name_span().unwrap();

            quote_spanned! {
                span => if is_server!() {
                    Some({
                        let component_attrs = #component_name::attributes();

                        let mut host = #component_name::default();
                        for attr in [#(#attrs,)*] {
                            if let leptos2::Attribute::Attribute(name, value) = attr {
                                if component_attrs.contains(&name.as_str()) {
                                    host.set_attribute(name, value);
                                }
                            }
                        }

                        #(#props;)*

                        (std::rc::Rc::new(move || {
                            host.view()
                        }) as std::rc::Rc<dyn Fn() -> leptos2::Host>)
                    })
                } else {
                    None
                }
            }
        } else {
            quote_spanned! { span => None }
        };

        // element tag
        let tag = if is_component {
            let component_name: TokenStream = tag.parse().unwrap();
            let span = node.name_span().unwrap();
            quote_spanned! { span => #component_name::tag() }
        } else {
            quote_spanned! { span => #tag.to_string() }
        };

        // key attr
        let key = if let Some(key) = node
            .attributes
            .iter()
            .find(|attr| attr.name_as_string() == Some("key".to_string()))
        {
            let span = key.name_span().unwrap();
            let value = key.value.as_ref().unwrap();
            quote_spanned! {
                span => Some(#value.to_string())
            }
        } else {
            quote_spanned! { span => None }
        };

        Some(quote_spanned! {
            span => leptos2::Node::Element(leptos2::Element {
                tag: #tag.to_string(),
                key: #key,
                attrs: #attributes,
                listeners: #listeners,
                children: vec![],
                shadow_root: #shadow_root,
                inner_html: None
            }
            #(.child(#children))*)
        })
    }
}

fn true_attr(node: &Node) -> bool {
    let attr_name = node.name_as_string().unwrap();
    !attr_name.starts_with("class:")
        && !attr_name.starts_with("prop:")
        && !attr_name.starts_with("on:")
        && !attr_name.starts_with("window:")
        && !attr_name.starts_with("document:")
        && !attr_name.starts_with("foreign:")
        && attr_name != "key"
}

fn listeners_from_node(node: &Node, starting_phrase: &str) -> proc_macro2::TokenStream {
    let listeners = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let attr_name = attr.name_as_string().unwrap();
            if attr_name.starts_with(starting_phrase) {
                let event_name = attr_name.replace(starting_phrase, "");
                let handler = attr.value.as_ref();
                let span = attr.name_span().unwrap();
                handler.map(|handler| {
                    quote_spanned! {
                        span => (#event_name, std::panic::Location::caller(), std::rc::Rc::new( #handler) as std::rc::Rc<dyn Fn(_) -> _>)
                    }
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let span = node.name_span().unwrap();

    if listeners.is_empty() {
        quote_spanned! { span => Vec::new() }
    } else {
        quote_spanned! {
            span => leptos2::vdom::build_listeners(leptos2::vdom::Listeners::from([
                #(#listeners,)*
            ]))
        }
    }
}

fn foreign_listeners_from_node(node: &Node) -> proc_macro2::TokenStream {
    let listeners = node
        .attributes
        .iter()
        .filter_map(|attr| {
            let span = attr.name_span().unwrap();
            let attr_name = attr.name_as_string().unwrap();
            if attr_name.starts_with("foreign:") {
                let event_name = attr_name.replace("foreign:", "");
                let (selector, handler) = match attr.value.as_ref().unwrap() {
                    Expr::Tuple(tuple) => {
                        let selector = tuple.elems.first();
                        let map_fn = tuple.elems.last();
                        (selector.unwrap(), map_fn.unwrap())
                    }
                    _ => {
                        panic!("expected a tuple with selector and handler")
                    }
                };
                Some(quote_spanned! {
                    span => (
                        #event_name.to_string(),
                        // allowed because this lets us use a format!() string as identifier here
                        #[allow(clippy::redundant_clone)]
                        #selector.to_string(),
                        std::panic::Location::caller(),
                        std::rc::Rc::new(#handler) as std::rc::Rc<dyn Fn(_) -> _>
                    )
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let span = node.name_span().unwrap();

    if listeners.is_empty() {
        quote_spanned! { span => Vec::new() }
    } else {
        quote_spanned! {
            span => leptos2::vdom::build_foreign_listeners(vec![
                #(#listeners,)*
            ])
        }
    }
}
