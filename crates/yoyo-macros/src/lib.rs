mod automaton;
mod codegen;
mod driver;
mod macros;
mod types;

use automaton::{Automaton, Edge};
use codegen::Codegen;
use driver::Driver;
use macros::yoyo_impl;
use types::{Spring, Style, Transition};

use proc_macro::TokenStream;

#[proc_macro]
pub fn yoyo(input: TokenStream) -> TokenStream {
    yoyo_impl(input.into()).into()
}

#[cfg(test)]
mod tests {
    use super::{Automaton, Driver};
    use casco::Parser;
    use quote::quote;

    #[test]
    fn test() {
        let input = quote! {
            opacity: 0.0;
            opacity_transition: ease_in_out;

            .rest {
                opacity: 1.0;
            }

            :exit {
            }
        };

        let driver = Driver;
        let parser = Parser::new(&driver);
        let parse = parser.parse(input.into_iter());

        let automaton = Automaton::new(&parse);

        let mut variants = automaton.variants().collect::<Vec<_>>();
        variants.sort_by_key(|(edge, _)| edge.presedence());
        variants.reverse();

        println!("Variants: {:#?}", variants);
    }
}
