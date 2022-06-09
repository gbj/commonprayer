use quote::{quote, quote_spanned};
use syn::{spanned::Spanned};

pub fn impl_params(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        fields
            .named
            .iter()
            .map(|field| {
				let field_name_string = &field.ident.as_ref().unwrap().to_string();
				let ident = &field.ident;
				let ty = &field.ty;
				let span = field.span().unwrap();

				quote_spanned! {
					span.into() => #ident: map.get(#field_name_string)
						.ok_or_else(|| leptos2::router::RouterError::MissingParam(#field_name_string.to_string()))
						.and_then(|value| value.parse::<#ty>().map_err(|e| leptos2::router::RouterError::Params(Box::new(e))))?
				}
			})
            .collect()
    } else {
        vec![]
    };

    let gen = quote! {
        impl Params for #name {
            fn from_map(map: &std::collections::HashMap<String, String>) -> Result<Self, leptos2::router::RouterError> {
				Ok(Self {
					#(#fields,)*
				})
			}
        }
    };
    gen.into()
}