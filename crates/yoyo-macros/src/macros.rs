use casco::StyleSheet;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;

use super::{Automaton, CascoError, Codegen, Driver};

pub fn yoyo_impl(input: TokenStream) -> TokenStream {
    let mut input = input.into_iter();

    let ty = match input.next() {
        Some(TokenTree::Ident(ident)) => ident,
        _ => unimplemented!("TODO: emit error."),
    };

    let _comma = match input.next() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ',' => punct,
        _ => unimplemented!("TODO: emit error."),
    };

    let group = match input.next() {
        Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Brace => group,
        _ => unimplemented!("TODO: emit error."),
    };

    match input.next() {
        None => {}
        _ => unimplemented!("TODO: emit error."),
    };

    let stream = casco::proc_macro2::TokenStream::from(group.stream());

    let stylesheet =
        match StyleSheet::parse(&mut Driver::new(), &stream.into_iter().collect::<Vec<_>>()) {
            Ok(stylesheet) => stylesheet,
            Err(error) => {
                let error = error.into_iter().map(|error| CascoError(error));
                return quote! { #(#error)* };
            }
        };

    let automaton = Automaton::new(stylesheet);

    let mut variants = automaton.variants().collect::<Vec<_>>();
    variants.sort_by_key(|(edge, _)| edge.presedence());
    variants.reverse();

    let codegen = Codegen::new(&variants);

    codegen.generate(ty)
}
