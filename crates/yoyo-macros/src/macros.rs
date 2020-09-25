use casco::Parser;
use proc_macro2::{Delimiter, TokenStream, TokenTree};

use super::{Automaton, Codegen, Driver};

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

    let parser = Parser::new(&Driver);
    let parse = parser.parse(group.stream().into_iter());

    assert_eq!(
        parse.errors.len(),
        0,
        "Encountered parse errors: {:#?}",
        parse.errors
    );

    let automaton = Automaton::new(&parse);

    let mut variants = automaton.variants().collect::<Vec<_>>();
    variants.sort_by_key(|(edge, _)| edge.presedence());
    variants.reverse();

    let codegen = Codegen::new(&variants);

    codegen.generate(ty)
}
