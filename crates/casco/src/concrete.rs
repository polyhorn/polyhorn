//! Concrete implementations of the token stream and related traits.

use std::vec::IntoIter;

use crate::stream;

pub use stream::{Delimiter, Spacing};

/// Concrete implementation of a token tree.
pub type TokenTree = stream::TokenTree<TokenStream>;

/// Concrete implementation of a token stream.
#[derive(Clone, Debug)]
pub struct TokenStream {
    /// Tokens that this token stream consists of.
    pub tokens: Vec<TokenTree>,
}

impl TokenStream {
    /// Returns a new concrete token stream with the given tokens.
    pub fn new(tokens: Vec<TokenTree>) -> TokenStream {
        TokenStream { tokens }
    }
}

impl stream::TokenStream for TokenStream {
    type Group = Group;
    type Ident = Ident;
    type Literal = Literal;
    type Punct = Punct;
    type Span = Span;
}

impl IntoIterator for TokenStream {
    type Item = TokenTree;
    type IntoIter = IntoIter<TokenTree>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}

/// Concrete implementation of a group.
#[derive(Clone, Debug)]
pub struct Group {
    /// Contains the delimiter of this group.
    pub delimiter: Delimiter,

    /// Contains the tokens in this group.
    pub stream: TokenStream,

    /// Contains the span of this group, which starts at the opening delimiter
    /// and ends at the closing delimiter (both inclusive).
    pub span: Span,
}

impl Group {
    /// Returns a new concrete group with the given delimiter, token stream and
    /// span.
    pub fn new(delimiter: Delimiter, stream: TokenStream, span: Span) -> Group {
        Group {
            delimiter,
            stream,
            span,
        }
    }
}

impl stream::Group<TokenStream> for Group {
    fn delimiter(&self) -> Delimiter {
        self.delimiter
    }

    fn stream(&self) -> TokenStream {
        self.stream.clone()
    }
}

impl stream::Spanned<Span> for Group {
    fn span(&self) -> Span {
        self.span
    }
}

impl Into<TokenTree> for Group {
    fn into(self) -> TokenTree {
        TokenTree::Group(self)
    }
}

/// Concrete implementation of an identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Ident {
    /// Contains the string value of this identifier.
    pub string: String,

    /// Contains the span of this identifier.
    pub span: Span,
}

impl Ident {
    /// Returns a new concrete identifier with the given string value and span.
    pub fn new(string: &str, span: Span) -> Ident {
        Ident {
            string: string.to_owned(),
            span,
        }
    }
}

impl ToString for Ident {
    fn to_string(&self) -> String {
        self.string.clone()
    }
}

impl stream::Ident for Ident {}

impl stream::Spanned<Span> for Ident {
    fn span(&self) -> Span {
        self.span
    }
}

impl Into<TokenTree> for Ident {
    fn into(self) -> TokenTree {
        TokenTree::Ident(self)
    }
}

/// Concrete implementation of a literal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Literal {
    /// Contains the string value of this literal. If this literal is a char or
    /// string literal, this will also the delimiters (i.e.. `'` or `"`). If
    /// this literal is a numeric literal, this will contain the unparsed value
    /// as well as its suffix.
    pub string: String,

    /// Contains the span of this literal.
    pub span: Span,
}

impl Literal {
    /// Returns a new concrete literal with the given string value and span.
    pub fn new(string: &str, span: Span) -> Literal {
        Literal {
            string: string.to_owned(),
            span,
        }
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        self.string.clone()
    }
}

impl stream::Literal for Literal {}

impl stream::Spanned<Span> for Literal {
    fn span(&self) -> Span {
        self.span
    }
}

impl Into<TokenTree> for Literal {
    fn into(self) -> TokenTree {
        TokenTree::Literal(self)
    }
}

/// Concrete implementation of a punctuation character.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Punct {
    /// Contains the punctuation character itself.
    pub value: char,

    /// Contains the spacing of this punctuation character.
    pub spacing: Spacing,

    /// Contains the span of this punctuation character.
    pub span: Span,
}

impl Punct {
    /// Returns a new concrete punctuation character with the given value,
    /// spacing and span.
    pub fn new(value: char, spacing: Spacing, span: Span) -> Punct {
        Punct {
            value,
            spacing,
            span,
        }
    }
}

impl stream::Punct for Punct {
    fn as_char(&self) -> char {
        self.value
    }

    fn spacing(&self) -> Spacing {
        self.spacing
    }
}

impl stream::Spanned<Span> for Punct {
    fn span(&self) -> Span {
        self.span
    }
}

impl Into<TokenTree> for Punct {
    fn into(self) -> TokenTree {
        TokenTree::Punct(self)
    }
}

/// Concrete implementation of a span.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Span {
    /// Contains the index of the first character in this span.
    pub start: usize,

    /// Contains the index of the first character that is no longer in this
    /// span (i.e. exclusive).
    pub end: usize,
}

impl Span {
    /// Returns a new span with the given start and end indices.
    pub fn new(start: usize, end: usize) -> Span {
        Span { start, end }
    }
}
