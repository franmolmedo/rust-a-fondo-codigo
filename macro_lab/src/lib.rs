//! Procedural macros reales usadas por el laboratorio verificable del capítulo 50.

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
                    return Err(meta.error("`id` solo puede declararse una vez"));
                }
                options.id = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("crate_path") {
                if options.crate_path.is_some() {
                    return Err(meta.error("`crate_path` solo puede declararse una vez"));
                }
                options.crate_path = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error("opción desconocida; se esperaba `id` o `crate_path`"))
            }
        })?;
    }

    Ok(options)
}

fn expand_entity(input: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let options = parse_entity_options(&input)?;
    let id = options.id.ok_or_else(|| {
        syn::Error::new_spanned(&input.ident, "falta `#[entity(id = \"campo\")]`")
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
                    "`Entity` requiere una struct con campos nombrados",
                ));
            }
        },
        _ => {
            return Err(syn::Error::new_spanned(
                &input.ident,
                "`Entity` solo puede derivarse para structs",
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
            format!("el campo `{id_name}` no existe en esta struct"),
        ));
    }

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = input.generics.split_for_impl();
    Ok(quote! {
        impl #impl_generics #crate_path::Entity for #name #type_generics #where_clause {
            fn entity_name() -> &'static str {
                stringify!(#name)
            }

            fn id_field() -> &'static str {
                #id
            }
        }
    })
}

/// Deriva el trait runtime `Entity` sin añadir bounds que la expansión no usa.
#[proc_macro_derive(Entity, attributes(entity))]
pub fn derive_entity(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_entity(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.to_compile_error().into(),
    }
}

/// Convierte una lista de identificadores en un slice estático de sus nombres.
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

/// Attribute macro mínima que valida que no haya argumentos y preserva la función.
#[proc_macro_attribute]
pub fn preserve_item(attribute: TokenStream, item: TokenStream) -> TokenStream {
    let attribute = proc_macro2::TokenStream::from(attribute);
    if !attribute.is_empty() {
        return syn::Error::new_spanned(attribute, "`preserve_item` no acepta argumentos")
            .to_compile_error()
            .into();
    }

    let function = parse_macro_input!(item as ItemFn);
    quote!(#function).into()
}
