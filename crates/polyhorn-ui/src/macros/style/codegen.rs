use casco::proc_macro2::TokenStream;
use quote::quote;

use super::{Diagnostic, Style};

/// Code generator that is used to convert a style and diagnostics to a token
/// stream that we can return to the compiler.
///
/// This is a enum without variants. It's only used to expose the code
/// generator's associated function.
pub enum Codegen {}

impl Codegen {
    /// Generates a new token stream from the given style and diagnostics.
    pub fn generate(
        style: Style,
        diagnostics: Vec<Diagnostic<TokenStream>>,
    ) -> proc_macro2::TokenStream {
        if !diagnostics.is_empty() {
            return quote! { #(#diagnostics)* };
        }

        match style {
            Style::Image(style) => quote! { #style },
            Style::Text(style) => quote! { #style },
            Style::View(style) => quote! { #style },
            _ => unimplemented!("This type of style cannot be codegen'd yet."),
        }
    }
}
