mod params;
mod rsx;
mod web_component;
use params::impl_params;
use rsx::node_to_tokens;
use web_component::impl_wc;

use quote::quote;
use syn_rsx::parse;

#[proc_macro]
pub fn view(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match parse(tokens) {
        Ok(nodes) => {
            if let Some(node) = nodes.get(0).and_then(node_to_tokens) {
                node
            } else {
                quote! {}
            }
        }
        Err(error) => error.to_compile_error(),
    }
    .into()
}

// Derive WebComponent trait for any struct
#[proc_macro_derive(WebComponent, attributes(prop))]
pub fn web_component_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_wc(&ast)
}

// Derive Params trait for routing
#[proc_macro_derive(Params, attributes(params))]
pub fn params_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_params(&ast)
}



