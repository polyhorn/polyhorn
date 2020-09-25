//! Types and functions that implement the `style! { ... }` macro.

use casco::stream::IntoTokenStream;
use quote::quote;

mod codegen;
mod driver;
mod infer;
mod parser;
mod style;
mod to_tokens;
mod value;

#[cfg(test)]
mod tests;

pub use codegen::Codegen;
pub use driver::{Driver, Error, Property};
pub use infer::{StyleCompound, StyleKind};
pub use parser::{ParseError, Parser};
pub use style::{Diagnostic, Style, StyleBuilder};
pub use to_tokens::CompileError;
pub use value::{PositionType, PropertyValue};

/// Implementation of the `style! { ... }` macro.
pub fn style_impl(input: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let stream = input.into_token_stream();
    let tokens = stream.into_iter().collect::<Vec<_>>();
    let mut driver = Driver::new();
    let stylesheet = match casco::StyleSheet::parse(&mut driver, &tokens) {
        Ok(stylesheet) => stylesheet,
        Err(error) => {
            let error = error.into_iter().map(|error| to_tokens::CascoError(error));
            return quote! { #(#error)* };
        }
    };
    let properties = stylesheet
        .items
        .into_iter()
        .filter_map(|item| match item {
            casco::Item::Property(property) => Some(property),
            _ => None,
        })
        .collect::<Vec<_>>();

    let builder = StyleBuilder::new();
    let (style, diagnostics) = builder.build(&properties);

    Codegen::generate(style, diagnostics)
}
