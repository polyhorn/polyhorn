//! Functions that implement the `test!(...)` macro.

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;

/// Implementation of the `test!(...)` macro.
pub fn test_impl(input: TokenStream) -> TokenStream {
    fn first_ident(input: TokenStream) -> Option<Ident> {
        for token in input {
            match token {
                TokenTree::Ident(ident)
                    if !["async", "pub", "fn"].contains(&ident.to_string().as_str()) =>
                {
                    return Some(ident)
                }
                _ => continue,
            }
        }

        None
    }

    let name = first_ident(input.clone()).unwrap();

    quote! {
        #input

        polyhorn_test::register!(concat!(module_path!(), "::", stringify!(#name)), #name);
    }
}
