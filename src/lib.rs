extern crate proc_macro;

use proc_macro::TokenStream;
use std::collections::HashMap;

use quote::quote;
use syn::{ItemStruct, LitStr, parse_macro_input};
use syn::{Ident, Token};
use syn::parse::{Parse, Parser, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use url::Url;

#[derive(Debug)]
struct Args(Vec<LitStr>);

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Args(
            Punctuated::<syn::LitStr, Token![,]>::parse_terminated(input)?
                .into_iter()
                .collect(),
        ))
    }
}

#[proc_macro_attribute]
pub fn parse_url(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let item_span = item_struct.span();
    let args = parse_macro_input!(args as Args);

    let mut flags: HashMap<String, Vec<String>> = HashMap::new();
    for arg in args.0 {
        let parsed = Url::parse(&arg.value()).unwrap();
        for (name, value) in parsed.query_pairs() {
            if let Some(values) = flags.get_mut(&name.to_string()) {
                if !value.is_empty() {
                    values.push(value.to_string())
                }
            } else if value.is_empty() {
                flags.insert(name.to_string(), Vec::new());
            } else {
                flags.insert(name.to_string(), vec![value.to_string()]);
            }
        }
    }

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for field in flags.iter().flat_map(|(name, value)| {
            // choose correct span
            let ident = Ident::new(name, item_span);
            if value.is_empty() {
                syn::Field::parse_named.parse2(quote! {
                    pub #ident: Option<String>
                })
            } else {
                syn::Field::parse_named.parse2(quote! {
                    pub #ident: String
                })
            }
        }) {
            fields.named.push(field);
        }
    }

    quote! {
        #item_struct
    }
    .into()
}
