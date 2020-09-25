//! Implementation of the token stream and related traits for `proc_macro2`.

use crate::stream;

/// Wrapper around the `proc_macro2` `TokenStream`.
pub struct TokenStream(proc_macro2::TokenStream);

/// Wrapper around the `proc_macro2` `IntoIter`.
pub struct IntoIter(proc_macro2::token_stream::IntoIter);

impl From<proc_macro2::TokenStream> for TokenStream {
    fn from(value: proc_macro2::TokenStream) -> Self {
        TokenStream(value)
    }
}

impl Into<proc_macro2::TokenStream> for TokenStream {
    fn into(self) -> proc_macro2::TokenStream {
        self.0
    }
}

impl stream::IntoTokenStream for proc_macro2::TokenStream {
    type TokenStream = TokenStream;

    fn into_token_stream(self) -> Self::TokenStream {
        TokenStream(self)
    }
}

impl stream::TokenStream for TokenStream {
    type Group = proc_macro2::Group;
    type Ident = proc_macro2::Ident;
    type Literal = proc_macro2::Literal;
    type Punct = proc_macro2::Punct;
    type Span = proc_macro2::Span;
}

impl IntoIterator for TokenStream {
    type IntoIter = IntoIter;
    type Item = stream::TokenTree<TokenStream>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.0.into_iter())
    }
}

impl Iterator for IntoIter {
    type Item = stream::TokenTree<TokenStream>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.0.next()? {
            proc_macro2::TokenTree::Group(group) => stream::TokenTree::Group(group),
            proc_macro2::TokenTree::Ident(ident) => stream::TokenTree::Ident(ident),
            proc_macro2::TokenTree::Punct(punct) => stream::TokenTree::Punct(punct),
            proc_macro2::TokenTree::Literal(literal) => stream::TokenTree::Literal(literal),
        })
    }
}

impl stream::Spanned<proc_macro2::Span> for proc_macro2::Group {
    fn span(&self) -> proc_macro2::Span {
        proc_macro2::Group::span(self)
    }
}

impl stream::Group<TokenStream> for proc_macro2::Group {
    fn delimiter(&self) -> stream::Delimiter {
        match proc_macro2::Group::delimiter(self) {
            proc_macro2::Delimiter::Brace => stream::Delimiter::Brace,
            proc_macro2::Delimiter::Bracket => stream::Delimiter::Bracket,
            proc_macro2::Delimiter::Parenthesis => stream::Delimiter::Parenthesis,
            proc_macro2::Delimiter::None => stream::Delimiter::None,
        }
    }

    fn stream(&self) -> TokenStream {
        TokenStream(proc_macro2::Group::stream(self))
    }
}

impl stream::Ident for proc_macro2::Ident {}

impl stream::Spanned<proc_macro2::Span> for proc_macro2::Ident {
    fn span(&self) -> proc_macro2::Span {
        proc_macro2::Ident::span(self)
    }
}

impl stream::Literal for proc_macro2::Literal {}

impl stream::Spanned<proc_macro2::Span> for proc_macro2::Literal {
    fn span(&self) -> proc_macro2::Span {
        proc_macro2::Literal::span(self)
    }
}

impl stream::Punct for proc_macro2::Punct {
    fn as_char(&self) -> char {
        proc_macro2::Punct::as_char(self)
    }

    fn spacing(&self) -> stream::Spacing {
        match proc_macro2::Punct::spacing(self) {
            proc_macro2::Spacing::Alone => stream::Spacing::Alone,
            proc_macro2::Spacing::Joint => stream::Spacing::Joint,
        }
    }
}

impl stream::Spanned<proc_macro2::Span> for proc_macro2::Punct {
    fn span(&self) -> proc_macro2::Span {
        proc_macro2::Punct::span(self)
    }
}
