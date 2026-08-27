//! Procedural macros for newtypes.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Index, Member, parse_macro_input};

/// Implements `serde::Serialize` by serializing the newtype's borrowed inner field directly.
///
/// Unlike Serde's `into` container attribute, this implementation does not clone or convert the newtype. The derive accepts tuple and named structs containing exactly one field.
#[proc_macro_derive(SerializeTransparent)]
pub fn derive_serialize_transparent(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_serialize_transparent(input).into()
}

fn expand_serialize_transparent(input: DeriveInput) -> TokenStream2 {
    let DeriveInput {
        ident,
        generics,
        data,
        ..
    } = input;
    let (field_type, member) = match data {
        Data::Struct(data) => match data.fields {
            Fields::Named(fields) => {
                let mut fields_iter = fields.named.into_iter();
                match (fields_iter.next(), fields_iter.next()) {
                    (Some(field), None) => match field.ident {
                        Some(field_ident) => (field.ty, Member::Named(field_ident)),
                        None => {
                            return syn::Error::new_spanned(&ident, "SerializeTransparent requires a named or tuple struct containing exactly one field").into_compile_error();
                        }
                    },
                    _ => {
                        return syn::Error::new_spanned(&ident, "SerializeTransparent requires a struct containing exactly one field").into_compile_error();
                    }
                }
            }
            Fields::Unnamed(fields) => {
                let mut fields_iter = fields.unnamed.into_iter();
                match (fields_iter.next(), fields_iter.next()) {
                    (Some(field), None) => (field.ty, Member::Unnamed(Index::from(0))),
                    _ => {
                        return syn::Error::new_spanned(&ident, "SerializeTransparent requires a struct containing exactly one field").into_compile_error();
                    }
                }
            }
            Fields::Unit => {
                return syn::Error::new_spanned(&ident, "SerializeTransparent requires a struct containing exactly one field").into_compile_error();
            }
        },
        Data::Enum(_) => {
            return syn::Error::new_spanned(&ident, "SerializeTransparent does not support enums").into_compile_error();
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(&ident, "SerializeTransparent does not support unions").into_compile_error();
        }
    };
    let where_predicates = generics
        .where_clause
        .as_ref()
        .map(|where_clause| &where_clause.predicates);
    let (impl_generics, type_generics, _) = generics.split_for_impl();

    quote! {
        #[automatically_derived]
        impl #impl_generics ::serde::Serialize for #ident #type_generics
        where
            #field_type: ::serde::Serialize,
            #where_predicates
        {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&self.#member, serializer)
            }
        }
    }
}

#[cfg(test)]
mod tests;
