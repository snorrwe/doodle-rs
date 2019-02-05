//! This crate provides `doodle`'s `Schema` derive macro
#![crate_type = "proc-macro"]

#[macro_use]
extern crate quote;
extern crate proc_macro;
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
            // TODO: type should be an enum 
            let x = field.ident.expect("Unnamed fields are not supported");
            let y = field.ty;
            let x = format!("{}", x);
            let tt = y.into_token_stream();
            let mut y = format!("{}", tt);
            y = match y.as_str() {
                "usize" | "isize" | "i32" | "u32" | "i8" | "u8" | "i16" | "u16" | "i64" | "u64"
                | "f32" | "f64" => "number".into(),
                "String" | "str" => "string".into(),
                _ => y,
            };
            if y.starts_with("Vec") {
                let inner_type =
                    y.split(|c| c == '<' || c == '>').collect::<Vec<_>>()[1].replace(' ', "");
                // TODO: above matching for the inner type
                y = format!("{}[]", inner_type);
            }
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

