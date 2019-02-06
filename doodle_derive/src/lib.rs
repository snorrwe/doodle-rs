//! This crate provides `doodle`'s `Schema` derive macro
#![crate_type = "proc-macro"]

#[macro_use]
extern crate quote;
extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use self::proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(Schema)]
pub fn derive_schema(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    implement_derive_schema(input)
}

fn implement_derive_schema(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let generics = input.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fields = match input.data {
        Data::Struct(data) => data.fields,
        _ => panic!("Only structs are supported!"),
    };
    let fields = match fields {
        Fields::Named(ref fields) => fields.named.iter().cloned().map(|field| {
            let x = field.ident.expect("Unnamed fields are not supported");
            let x = format!("{}", x);
            let y = field.ty;
            let tt = y.into_token_stream();
            let y = format!("{}", tt);
            let y = serialize_type(y.as_str());
            quote! {
                Field {
                    name: #x,
                    json_ty: #y,
                }
            }
        }),
        _ => panic!("Only named fields are supported!"),
    };
    let diplay_name = format!("{}", name);
    let implementation = quote! {
        impl #impl_generics SchemaMeta for #name #ty_generics #where_clause {
            fn get_fields() -> &'static [Field] {
                &[#(#fields),*]
            }

            fn get_name() -> &'static str {
                #diplay_name
            }
        }

        impl #impl_generics Schema for #name #ty_generics #where_clause {}

    };
    TokenStream::from(implementation)
}

fn serialize_type(y: &str) -> proc_macro2::TokenStream {
    if y.starts_with("Vec") {
        serialize_vec(y)
    } else if y.starts_with("Option") {
        let split = split_generics(y);
        assert_eq!(split.len(), 3);
        let ty = split[1].replace(' ', "");
        let ty = serialize_type(ty.as_str());
        quote!( Type::Nullable(& #ty) )
    } else {
        let ty = into_json_type(y);
        match ty {
            Some(ty) => quote! ( Type::Simple(#ty) ),
            None => quote! ( Type::Ref(#y) ),
        }
    }
}

fn serialize_vec(y: &str) -> proc_macro2::TokenStream {
    let split = split_generics(y);
    assert_eq!(
        split.len(),
        3,
        "Vector is of incorrect type, expected: Vec < ... >"
    );
    let inner_type = split[1].replace(' ', "");
    let inner_type = serialize_type(inner_type.as_str());
    quote! ( Type::Array(& #inner_type) )
}

fn split_generics(y: &str) -> Vec<&str> {
    let mut depth = 0;
    y.split(|c| {
        let mut result = false;
        if c == '<' {
            if depth == 0 {
                result = true;
            }
            depth += 1;
        } else if c == '>' {
            depth -= 1;
            if depth == 0 {
                result = true;
            }
        }
        result
    })
    .collect::<Vec<_>>()
}

fn into_json_type(ty: &str) -> Option<String> {
    match ty {
        "usize" | "isize" | "i32" | "u32" | "i8" | "u8" | "i16" | "u16" | "i64" | "u64" | "f32"
        | "f64" => Some("number".into()),
        "String" | "str" => Some("string".into()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_serialization() {
        let input = "Vec<Vec<i32>>";
        let result = serialize_vec(input);

        let result = format!("{}", result);

        assert_eq!(
            result,
            "Type :: Array ( & Type :: Array ( & Type :: Simple ( \"number\" ) ) )"
        );
    }
}

