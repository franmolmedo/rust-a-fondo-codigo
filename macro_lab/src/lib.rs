//! Real procedural macros used by the verifiable chapter 50 lab.

use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    Data, DeriveInput, Fields, Ident, ItemFn, LitStr, Path, Token, parse_macro_input, parse_quote,
};

#[derive(Default)]
struct EntityOptions {
    id: Option<LitStr>,
    crate_path: Option<Path>,
}

fn parse_entity_options(input: &DeriveInput) -> syn::Result<EntityOptions> {
    let mut options = EntityOptions::default();

    for attribute in &input.attrs {
        if !attribute.path().is_ident("entity") {
            continue;
        }
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("id") {
                if options.id.is_some() {
                    return Err(meta.error("`id` can only be specified once"));
                }
                options.id = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("crate_path") {
                if options.crate_path.is_some() {
                    return Err(meta.error("`crate_path` can only be specified once"));
                }
                options.crate_path = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error("unknown option; expected `id` or `crate_path`"))
            }
        })?;
    }

    Ok(options)
}

fn expand_entity(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_entity_options(&input)?;
    let id = options.id.ok_or_else(|| {
        syn::Error::new_spanned(&input.ident, "missing `#[entity(id = \"field\")]`")
    })?;
    let crate_path = options
        .crate_path
        .unwrap_or_else(|| parse_quote!(::course_macro_api));

    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => {
                return Err(syn::Error::new_spanned(
                    &input.ident,
                    "`Entity` requires a struct with named fields",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "`Entity` can only be derived for structs",
            ));
        }
    };

    let id_name = id.value();
    let field_exists = fields.iter().any(|field| {
        field
            .ident
            .as_ref()
            .is_some_and(|ident| ident == id_name.as_str())
    });
    if !field_exists {
        return Err(syn::Error::new_spanned(
            &id,
            format!("field `{id_name}` does not exist in this struct"),
        ));
    }

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #crate_path::Entity for #name #type_generics #where_clause {
            fn entity_name() -> &'static str {
                ::core::stringify!(#name)
            }

            fn id_field() -> &'static str {
                #id
            }
        }
    })
}

/// Derives the `Entity` trait without adding bounds unused by the expansion.
#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_entity(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Converts a list of identifiers into a static slice containing their names.
#[proc_macro]
pub fn field_names(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let names = parse_macro_input!(input with parser);
    let names = names
        .iter()
        .map(|name| LitStr::new(&name.to_string(), name.span()))
        .collect::<Vec<_>>();
    quote! { &[#(#names),*] }.into()
}

/// Minimal attribute macro that rejects arguments and preserves the function.
#[proc_macro_attribute]
pub fn preserve_item(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = proc_macro2::TokenStream::from(attribute);
    if !attribute.is_empty() {
        return syn::Error::new_spanned(attribute, "`preserve_item` does not accept arguments")
            .to_compile_error()
            .into();
    }

    let function = parse_macro_input!(item as ItemFn);
    quote!(#function).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn c50_invalid_options_report_the_specific_problem() {
        let cases = [
            (
                quote!(
                    #[entity(id = "id", id = "other")]
                    struct A {
                        id: u64,
                    }
                ),
                "`id` can only be specified once",
            ),
            (
                quote!(
                    #[entity(id = "id", crate_path = api, crate_path = other)]
                    struct A {
                        id: u64,
                    }
                ),
                "`crate_path` can only be specified once",
            ),
            (
                quote!(
                    #[entity(id = "id", crate_paht = api)]
                    struct A {
                        id: u64,
                    }
                ),
                "unknown option; expected `id` or `crate_path`",
            ),
            (
                quote!(
                    #[entity(id = "missing")]
                    struct A {
                        id: u64,
                    }
                ),
                "field `missing` does not exist in this struct",
            ),
        ];
        for (input, expected) in cases {
            let error = expand_entity(syn::parse2(input).unwrap()).unwrap_err();
            assert_eq!(error.to_string(), expected);
        }
    }
}
