use quote::{quote, quote_spanned};
use syn::{spanned::Spanned};

pub fn impl_wc(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let tag = format!("l-{}", slugify(&name.to_string()));

    let attrs = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
            .named
            .iter()
            .filter(|field| matches!(field.vis, syn::Visibility::Public(_)))
            .collect()
    } else {
        vec![]
    };

    let observed_attributes = attrs
        .iter()
        .filter(|field| {
            !field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| slugify(&field.ident.as_ref().unwrap().to_string()));

    let observed_properties = attrs
        .iter()
        .filter(|field| {
            field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| slugify(&field.ident.as_ref().unwrap().to_string()));

    let set_attribute = attrs
        .iter()
        .filter(|field| {
            !field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| {
            let attr_name = slugify(&field.ident.as_ref().unwrap().to_string());
            let ident = &field.ident;
            let ty = &field.ty;
            let span = field.span().unwrap();

            quote_spanned! {
                span.into() => #attr_name => {
                    if let Some(#ident) = new_value.and_then(|value| value.parse::<#ty>().ok()) {
                        self.#ident = #ident;
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let span = name.span();

    let set_attribute = if set_attribute.is_empty() {
        quote_spanned! { span => {} }
    } else {
        quote_spanned! {
            span => match attr_name.as_str() {
                #(#set_attribute,)*
                _ => {}
            }
        }
    };

    let set_property = attrs
        .iter()
        .filter(|field| {
            field.attrs.iter().any(|attr| {
                attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
            })
        })
        .map(|field| {
            let attr_name = slugify(&field.ident.as_ref().unwrap().to_string());
            let ident = &field.ident;
            let ty = &field.ty;

            let span = field.span().unwrap();

            quote_spanned! {
                span.into() => #attr_name => {
                    match wasm_bindgen::JsValue::into_serde::<#ty>(&new_value) {
                        Ok(val) => {
                            self.#ident = val;
                        },
                        Err(e) => leptos2::warn(&format!("[set_property] deserialization error {}", e.to_string()))
                    }
                }
            }
        })
        .collect::<Vec<_>>();

    let set_property = if set_property.is_empty() {
        quote_spanned! { span => {} }
    } else {
        quote_spanned! {
            span => match prop_name.as_str() {
                #(#set_property,)*
                _ => {}
            }
        }
    };

    let state_to_attributes = attrs.iter().map(|field| {
        let attr_name = slugify(&field.ident.as_ref().unwrap().to_string());
        let ident = &field.ident.as_ref().unwrap();

        let prop = field.attrs.iter().any(|attr| {
            attr.path.get_ident().map(|ident| ident.to_string()) == Some("prop".to_string())
        });

        let span = field.span();

        if prop {
            quote_spanned! {
                span => leptos2::property(
                    #attr_name,
                    Box::new(self.#ident.clone())
                )
            }
        } else {
            quote_spanned! {
                span => leptos2::attribute(
                    #attr_name,
                    self.#ident.to_string()
                )
            }
        }
    });

    let gen = quote! {
        impl WebComponent for #name {
            fn tag() -> &'static str {
                #tag
            }

            fn attributes() -> &'static [&'static str] {
               &[#(#observed_attributes,)*]
            }

            fn properties() -> &'static [&'static str] {
               &[#(#observed_properties,)*]
            }

            fn set_attribute(&mut self, attr_name: String, new_value: Option<String>) {
                #set_attribute
            }

            fn set_property(&mut self, prop_name: String, new_value: wasm_bindgen::JsValue) {
                #set_property
            }

            fn state_to_attributes(&self) -> Vec<Attribute> {
                vec![#(#state_to_attributes,)*]
            }
        }
    };
    gen.into()
}

const UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn slugify(unslugged: &str) -> String {
    let mut splits = unslugged.split_inclusive(UPPERCASE).enumerate().peekable();

    let mut buffer = String::new();

    while let Some((i, part)) = splits.next() {
        let is_first = i == 0;
        let len = part.len();

        if is_first {
            buffer.push_str(&part.to_ascii_lowercase())
        } else {
            let is_last = splits.peek().is_none();
            for (j, ch) in part.chars().enumerate() {
                if j == len - 1 && (!is_last || ch.is_ascii_uppercase()) {
                    buffer.push('-');
                    buffer.push(ch.to_ascii_lowercase());
                } else {
                    buffer.push(ch.to_ascii_lowercase());
                }
            }
        }
    }
    buffer.replace('_', "-")
}