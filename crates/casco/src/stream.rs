//! Types and traits that provide an abstraction over `proc_macro2`.

use derivative::Derivative;
use std::fmt::Debug;

/// The main type provided by this crate, representing an abstract stream of
/// tokens, or, more specifically, a sequence of token trees. The type provides
/// an interface for iterating over those token trees and, conversely,
/// collecting a number of token trees into one stream.
pub trait TokenStream: IntoIterator<Item = TokenTree<Self>> {
    /// A delimited token stream.
    type Group: Group<Self> + Spanned<Self::Span> + Clone + Debug;

    /// An identifier.
    type Ident: Ident + Spanned<Self::Span> + Clone + Debug;

    /// A literal string (`"hello"`), byte string (`b"hello"`), character
    /// (`'a'`), byte character (`b'a'`), an integer or floating point number
    /// with or without a suffix (`1`, `1u8`, `2.3`, `2.3f32`). Boolean literals
    /// like `true` and `false` do not belong here, they are `Ident`s.
    type Literal: Literal + Spanned<Self::Span> + Clone + Debug;

    /// An `Punct` is a single punctuation character like `+`, `-` or `#`.
    type Punct: Punct + Spanned<Self::Span> + Clone + Debug;

    /// A region of source code, along with macro expansion information.
    type Span: Copy + Debug;
}

/// Trait that converts a type into a token stream.
pub trait IntoTokenStream {
    /// This is the type of token stream that this trait converts into.
    type TokenStream: TokenStream;

    /// Converts this type into a token stream.
    fn into_token_stream(self) -> Self::TokenStream;
}

impl<T> IntoTokenStream for T
where
    T: TokenStream,
{
    type TokenStream = T;

    fn into_token_stream(self) -> Self::TokenStream {
        self
    }
}

/// A single token or a delimited sequence of token trees (e.g. `[1, (), ..]`).
#[derive(Copy, Derivative)]
#[derivative(Clone, Debug)]
pub enum TokenTree<S>
where
    S: TokenStream + ?Sized,
{
    /// A delimited token stream.
    Group(S::Group),

    /// An identifier.
    Ident(S::Ident),

    /// An `Punct` is a single punctuation character like `+`, `-` or `#`.
    Punct(S::Punct),

    /// A literal string (`"hello"`), byte string (`b"hello"`), character
    /// (`'a'`), byte character (`b'a'`), an integer or floating point number
    /// with or without a suffix (`1`, `1u8`, `2.3`, `2.3f32`). Boolean literals
    /// like `true` and `false` do not belong here, they are `Ident`s.
    Literal(S::Literal),
}

/// A delimited token stream.
///
/// A `Group` internally contains a `TokenStream` which is surrounded by
/// `Delimiter`s.
pub trait Group<S>
where
    S: TokenStream + ?Sized,
{
    /// Returns the delimiter of this `Group`.
    fn delimiter(&self) -> Delimiter;

    /// Returns the `TokenStream` of tokens that are delimited in this `Group`.
    /// Note that the returned token stream does not include the delimiter
    /// returned above.
    fn stream(&self) -> S;
}

/// An identifier (`ident`).
pub trait Ident: ToString {}

/// A literal string (`"hello"`), byte string (`b"hello"`), character (`'a'`),
/// byte character (`b'a'`), an integer or floating point number with or without
/// a suffix (`1`, `1u8`, `2.3`, `2.3f32`). Boolean literals like `true` and
/// `false` do not belong here, they are `Ident`s.
pub trait Literal: ToString {}

/// A `Punct` is a single punctuation character like `+`, `-` or `#`.
///
/// Multi-character operations like `+=` are represented as two instances of
/// `Punct` with different forms of `Spacing` returned.
pub trait Punct {
    /// Returns the value of this punctuation character as `char`.
    fn as_char(&self) -> char;

    /// Returns the spacing of this punctuation character, indicating whether
    /// it's immediately followed by another `Punct` in the token stream, so
    /// they can potentially be combined into a multi-character operator
    /// (`Joint`), or it's followed by some other token or whitespace (`Alone`)
    /// so the operator has certainly ended.
    fn spacing(&self) -> Spacing;
}

/// Describes how a sequence of token trees is delimited.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Delimiter {
    /// `( ... )`
    Parenthesis,

    /// `{ ... }`
    Brace,

    /// `[ ... ]`
    Bracket,

    /// `Ø ... Ø` An implicit delimiter, that may, for example, appear around
    /// tokens coming from a "macro variable" `$var`. It is important to
    /// preserve operator priorities in cases like `$var * 3` where `$var` is
    /// `1 + 2`. Implicit delimiters may not survive roundtrip of a token stream
    /// through a string.
    None,
}

/// Whether a `Punct` is followed immediately by another `Punct` or followed by
/// another token or whitespace.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Spacing {
    /// e.g. `+` is `Alone` in `+ =`, `+ident` or `+()`.
    Alone,

    /// e.g. `+` is `Joint` in `+=` or `'#`. Additionally, single quote `'` can
    /// join with identifiers to form lifetimes 'ident.
    Joint,
}

/// Trait that is implemented by everything that has a span (e.g. groups,
/// idents, literals, puncts, token trees).
pub trait Spanned<S>
where
    S: ?Sized,
{
    /// Returns the span for this punctuation character.
    fn span(&self) -> S;
}

impl<S> Spanned<S::Span> for TokenTree<S>
where
    S: TokenStream + ?Sized,
{
    fn span(&self) -> S::Span {
        match self {
            TokenTree::Group(group) => group.span(),
            TokenTree::Ident(ident) => ident.span(),
            TokenTree::Punct(punct) => punct.span(),
            TokenTree::Literal(literal) => literal.span(),
        }
    }
}

/// Wraps multiple spans into a single struct.
#[derive(Derivative)]
#[derivative(Copy, Clone, Debug, PartialEq(bound = "S::Span: PartialEq"))]
pub struct MultiSpan<S>
where
    S: TokenStream,
{
    first: Option<S::Span>,
    last: Option<S::Span>,
}

impl<S> MultiSpan<S>
where
    S: TokenStream,
{
    /// This creates a new `MultiSpan` that spans all of the given tokens.
    pub fn new(trees: &[TokenTree<S>]) -> MultiSpan<S> {
        MultiSpan {
            first: trees.first().map(Spanned::span),
            last: trees.last().map(Spanned::span),
        }
    }

    /// This creates a new `MultiSpan` from a single span.
    pub fn single(span: S::Span) -> MultiSpan<S> {
        MultiSpan {
            first: Some(span),
            last: Some(span),
        }
    }

    /// Returns the first span in this multi-span.
    pub fn first(&self) -> Option<S::Span> {
        self.first
    }

    /// Returns the last span in this multi-span.
    pub fn last(&self) -> Option<S::Span> {
        self.last
    }
}
