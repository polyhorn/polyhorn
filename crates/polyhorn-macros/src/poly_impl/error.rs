use proc_macro2::{Span, TokenTree};

use super::tokenizer::Path;

pub enum Error {
    ExpectedPropertyName(TokenTree),
    ExpectedPropertyValue(TokenTree),
    ExpectedDot(TokenTree),
    ExpectedIdentifier(TokenTree),
    ExpectedColon(TokenTree),
    ExpectedClosingAngle(TokenTree),
    ExpectedOpeningAngle(TokenTree),
    ExpectedPreamble(TokenTree),
    UnrecognizedClosingTag(Path, Path),
}

impl Error {
    pub fn message(&self) -> String {
        match self {
            Error::ExpectedPropertyName(token) => {
                format!("Expected property name, got `{}` instead.", token)
            }
            Error::ExpectedPropertyValue(token) => {
                format!("Expected property value, got `{}` instead.", token)
            }
            Error::ExpectedDot(token) => format!("Expected `...`, got `{}` instead.", token),
            Error::ExpectedIdentifier(token) => {
                format!("Expected identifier, got `{}` instead.", token)
            }
            Error::ExpectedColon(token) => format!("Expected `:`, got `{}` instead.", token),
            Error::ExpectedClosingAngle(token) => format!("Expected `>`, got `{}` instead.", token),
            Error::ExpectedOpeningAngle(token) => format!("Expected `<`, got `{}` instead.", token),
            Error::ExpectedPreamble(token) => format!(
                "Expected component name or `~` for built-ins, got `{}` instead.",
                token
            ),
            Error::UnrecognizedClosingTag(expected, actual) => {
                format!("Expected `{}`, got `{}` instead.", expected, actual)
            }
        }
    }

    pub fn start(&self) -> Span {
        match self {
            Error::ExpectedPropertyName(token) => token.span(),
            Error::ExpectedPropertyValue(token) => token.span(),
            Error::ExpectedDot(token) => token.span(),
            Error::ExpectedIdentifier(token) => token.span(),
            Error::ExpectedColon(token) => token.span(),
            Error::ExpectedClosingAngle(token) => token.span(),
            Error::ExpectedOpeningAngle(token) => token.span(),
            Error::ExpectedPreamble(token) => token.span(),
            Error::UnrecognizedClosingTag(_, actual) => actual.parts.first().unwrap().ident.span(),
        }
    }

    pub fn end(&self) -> Span {
        match self {
            Error::ExpectedPropertyName(token) => token.span(),
            Error::ExpectedPropertyValue(token) => token.span(),
            Error::ExpectedDot(token) => token.span(),
            Error::ExpectedIdentifier(token) => token.span(),
            Error::ExpectedColon(token) => token.span(),
            Error::ExpectedClosingAngle(token) => token.span(),
            Error::ExpectedOpeningAngle(token) => token.span(),
            Error::ExpectedPreamble(token) => token.span(),
            Error::UnrecognizedClosingTag(_, actual) => actual.parts.last().unwrap().ident.span(),
        }
    }
}
