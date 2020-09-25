pub mod scanner {
    //! This module re-exports the types we borrow from `proc_macro2`.
    pub use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Span, TokenTree};
}

mod driver;
mod parser;
mod types;

pub use driver::Driver;
pub use parser::Parser;
pub use types::{Item, Parse, Rule};

#[cfg(test)]
mod tests {
    use super::scanner::{Ident, TokenTree};
    use super::{Driver, Parser};
    use quote::quote;

    #[test]
    fn test_end_to_end() {
        let input = quote! {
            opacity: 1.0;

            :hover {
                opacity: 0.5;
            }
        };

        pub struct Selector;
        pub struct Property;
        pub struct Error;

        pub struct Example;

        impl Driver for Example {
            type Selector = Selector;
            type Property = Property;
            type Error = Error;

            fn parse_selector<S>(&self, scanner: &mut S) -> Result<Self::Selector, Self::Error>
            where
                S: Iterator<Item = TokenTree>,
            {
                let _ = scanner.collect::<Vec<_>>();
                Ok(Selector)
            }

            fn parse_property<S>(
                &self,
                name: Ident,
                value: &mut S,
            ) -> Result<Self::Property, Self::Error>
            where
                S: Iterator<Item = TokenTree>,
            {
                assert_eq!(name.to_string(), "opacity");
                let _ = value.collect::<Vec<_>>();
                Ok(Property)
            }
        }

        let driver = Example;
        let parser = Parser::new(&driver);
        let result = parser.parse(input.into_iter());
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].is_property(), true);
        assert_eq!(result.items[1].is_rule(), true);
        assert_eq!(result.errors.len(), 0);
    }
}
