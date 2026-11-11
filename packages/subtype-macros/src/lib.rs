//! Procedural macros for newtypes.

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::ext::IdentExt;
use syn::{Data, DeriveInput, Index, Member, parse_macro_input};

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
    let fields = match data {
        Data::Struct(data) => data.fields,
        Data::Enum(_) => {
            return syn::Error::new_spanned(&ident, "SerializeTransparent does not support enums").into_compile_error();
        }
        Data::Union(_) => {
            return syn::Error::new_spanned(&ident, "SerializeTransparent does not support unions").into_compile_error();
        }
    };
    let mut fields = fields.into_iter();
    let (Some(field), None) = (fields.next(), fields.next()) else {
        return syn::Error::new_spanned(&ident, "SerializeTransparent requires a struct containing exactly one field").into_compile_error();
    };
    let member = field
        .ident
        .map(Member::Named)
        .unwrap_or(Member::Unnamed(Index::from(0)));
    let field_type = field.ty;
    let mut serializer = format_ident!("__Serializer");
    let mut serializer_value = format_ident!("__serializer");
    while generics
        .type_params()
        .map(|param| &param.ident)
        .chain(generics.const_params().map(|param| &param.ident))
        .any(|ident| ident.unraw() == serializer || ident.unraw() == serializer_value)
    {
        serializer = format_ident!("_{serializer}");
        serializer_value = format_ident!("_{serializer_value}");
    }
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
            fn serialize<#serializer>(&self, #serializer_value: #serializer) -> ::core::result::Result<#serializer::Ok, #serializer::Error>
            where
                #serializer: ::serde::Serializer,
            {
                ::serde::Serialize::serialize(&self.#member, #serializer_value)
            }
        }
    }
}

#[cfg(test)]
mod tests;
