use proc_macro2::TokenStream;
use quote::quote_spanned;
use syn_rsx::{parse, Node, NodeType};

#[proc_macro]
pub fn view(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let hydrate_only = std::env::var("LEPTOS_HYDRATE") == Ok("true".to_string());

    match parse(tokens) {
        Ok(nodes) => {
            if nodes.len() == 1 {
                let processed_node = node_to_tokens(&nodes[0], hydrate_only).unwrap_or_else(|| {
                    quote::quote! {
                        leptos::View::Empty
                    }
                });
                quote::quote! {
                    #processed_node
                }
            } else {
                let processed_nodes = nodes
                    .iter()
                    .filter_map(|node| node_to_tokens(node, hydrate_only))
                    .collect::<Vec<_>>();

                if processed_nodes.is_empty() {
                    quote::quote! {
                        leptos::View::Empty
                    }
                } else {
                    quote::quote! {
                        leptos::View::Fragment(vec![
                            #(#processed_nodes,)*
                        ])
                    }
                }
            }
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}

fn node_to_tokens(node: &Node, hydrate_only: bool) -> Option<TokenStream> {
    match node.node_type {
        // Comments are omitted
        NodeType::Comment => None,
        // Doctypes are created, whether in a valid position or not
        NodeType::Doctype => {
            let value = node.value_as_string().unwrap();
            Some(quote::quote! {
                leptos::View::Doctype(#value)
            })
        }
        // Text and Block simply return their value: they will be converted to Views by parent Fragment or Element
        NodeType::Text | NodeType::Block => {
            let value = node.value.as_ref().unwrap();

            Some(quote::quote! {
                #value
            })
        }
        // Attributes distinguish between attributes, properties, and event listeners
        // TODO add class and style types
        NodeType::Attribute => {
            let name = node.name_as_string().unwrap();
            let value = node.value.as_ref();

            // Events
            if name.starts_with("on:") {
                let name = name.replace("on:", "");
                Some(quote::quote! {
                    .event(#name, #value)
                })
            }
            // Properties
            else if name.starts_with("prop:") {
                let name = name.replace("prop:", "");
                Some(quote::quote! {
                    .property(#name, #value)
                })
            }
            // Classes
            else if name.starts_with("class:") {
                let name = name.replace("class:", "").replace("_", "-");
                Some(quote::quote! {
                    .class(#name, #value)
                })
            }
            // Attributes
            // Attributes with values can simply be set
            else if let Some(value) = value {
                Some(quote::quote! {
                    .attribute(#name, #value)
                })
            }
            // Attributes with no value given (e.g., <div aria-hidden ...>) are set to a blank striing
            // Boolean attributes can have 1) an empty string or 2) a case-insensitive version of their own name as the value
            // see https://developer.mozilla.org/en-US/docs/Web/HTML/Attributes#boolean_attributes
            else {
                Some(quote::quote! {
                    .attribute(#name, "")
                })
            }
        }
        NodeType::Fragment => {
            // TODO can omit static children if hydrate_only?
            let children = node
                .children
                .iter()
                .filter_map(|node| match node.node_type {
                    NodeType::Element => node_to_tokens(node, hydrate_only).map(|tokens| {
                        quote::quote! {
                            #tokens
                        }
                    }),
                    NodeType::Text | NodeType::Block => {
                        let value = node.value.as_ref().unwrap();
                        Some(quote::quote! {
                            #value
                        })
                    }
                    _ => None,
                });

            Some(quote::quote! {
                leptos::View::Fragment(vec![
                    #(#children.into_view_child(),)*
                ])
            })
        }
        NodeType::Element => {
            let tag_name = node.name_as_string().unwrap();

            // Dynamic views: embed the view
            if tag_name == "dyn:view" {
                let view = node
                    .attributes
                    .iter()
                    .find(|attr| attr.name_as_string().unwrap() == "view")
                    .unwrap()
                    .value_as_block()
                    .unwrap();
                Some(quote::quote! {
                    #view
                })
            } else {
                let children = nodes_to_children(&node.children, hydrate_only);
                // Dynamic elements
                if tag_name.starts_with("dyn:") {
                    dynamic_element(node, tag_name.replace("dyn:", ""), &children)
                }
                // Static elements
                else {
                    static_element(node, hydrate_only, tag_name, &children)
                }
            }
        }
    }
}

fn nodes_to_children(nodes: &[Node], hydrate_only: bool) -> Vec<TokenStream> {
    nodes
        .iter()
        .filter_map(|node| match node.node_type {
            NodeType::Element => node_to_tokens(node, hydrate_only).map(|tokens| {
                quote::quote! {
                    #tokens
                }
            }),
            NodeType::Text | NodeType::Block => {
                let value = node.value.as_ref().unwrap();
                Some(quote::quote! {
                    #value
                })
            }
            _ => None,
        })
        .collect()
}

fn dynamic_element(node: &Node, tag_name: String, children: &[TokenStream]) -> Option<TokenStream> {
    let events = node
        .attributes
        .iter()
        .filter(|node| node.name_as_string().unwrap().starts_with("on:"))
        .map(|node| {
            let name = node.name_as_string().unwrap().replace("on:", "");
            let value = node.value.as_ref().unwrap();
            Some(quote::quote! {
                .event(#name, Box::new(#value))
            })
        })
        .collect::<Vec<_>>();

    let props = node
        .attributes
        .iter()
        .filter(|node| node.name_as_string().unwrap().starts_with("prop:"))
        .map(|node| {
            let name = node.name_as_string().unwrap().replace("on:", "");
            let value = node.value.as_ref().unwrap();
            Some(quote::quote! {
                .property(#name, #value)
            })
        })
        .collect::<Vec<_>>();

    let attributes = node
        .attributes
        .iter()
        .filter(|node| {
            let attr_name = node.name_as_string().unwrap();
            !attr_name.starts_with("on:") && !attr_name.starts_with("prop:")
        })
        .filter_map(|node| node_to_tokens(node, false))
        .collect::<Vec<_>>();

    if children.is_empty() {
        Some(quote::quote! {
            leptos::View::DynamicElement(
                leptos::DynamicElement::new(#tag_name)
                #(#attributes)*
                #(#props)*
                #(#events)*
            )
        })
    } else {
        Some(quote::quote! {
            leptos::View::DynamicElement(
                leptos::DynamicElement::new(#tag_name)
                #(#attributes)*
                #(#props)*
                #(#events)*
                #(.child(#children))*
            )
        })
    }
}

fn static_element(
    node: &Node,
    hydrate_only: bool,
    tag_name: String,
    children: &[TokenStream],
) -> Option<TokenStream> {
    let attributes = node
        .attributes
        .iter()
        .filter_map(|node| {
            let attr_name = node.name_as_string().unwrap();
            if attr_name.starts_with("on:") {
                let span = node.name_span().unwrap();
                Some(quote_spanned! {
                    span => compile_error!("event listeners cannot be attached to static elements");
                })
            } else if attr_name.starts_with("prop:") {
                let span = node.name_span().unwrap();
                Some(quote_spanned! {
                    span => compile_error!("property handlers cannot be attached to static elements");
                })
            } else {
                node_to_tokens(node, hydrate_only)
            }
        })
        .collect::<Vec<_>>();

    // partial hydration logic
    let can_omit_children = hydrate_only && only_has_static_children(node);
    let can_omit_self = hydrate_only;

    if can_omit_children {
        None
    }
    // when hydrating, can always omit oneself, even if not the children
    else if can_omit_self {
        Some(quote::quote! {
            leptos::View::Fragment(vec![
                #(#children,)*
            ])
        })
    }
    // in e.g., SSR, include it all
    else {
        Some(quote::quote! {
            leptos::View::StaticElement(
                // static element
                leptos::StaticElement::new(#tag_name)
                #(#attributes)*
                #(.child(#children))*
            )
        })
    }
}

// partial hydration logic is deceptively simple, if we're willing to mark every
// node that needs to be hydrated explicitly!
fn only_has_static_children(node: &Node) -> bool {
    // if node is dynamic, false
    if let Some(name) = node.name_as_string() {
        if name.starts_with("dyn:") {
            return false;
        }
    }

    // if any child nodes are dynamic, false
    for child in &node.children {
        if !only_has_static_children(child) {
            return false;
        }
    }

    true
}
