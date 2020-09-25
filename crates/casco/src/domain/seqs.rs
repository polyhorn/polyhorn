use std::marker::PhantomData;

use crate::stream::{Delimiter, Group as _, Punct, TokenStream, TokenTree};

/// Trait that is implemented by types that can assist in parsing a type from a
/// slice of tokens.
pub trait Parse<S>
where
    S: TokenStream,
{
    /// The type that is parsed.
    type Output;

    /// This function should return an instance of the type if it can be parsed,
    /// or `None` otherwise, and a slice of tokens that remain after parsing if
    /// successful (or the original slice otherwise).
    fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]);
}

pub trait Group {
    fn as_delimiter() -> Delimiter;
}

/// Delimiter character `( ... )` that can be used in conjunction with
/// `GroupedBy`.
pub enum Parentheses {}

impl Group for Parentheses {
    fn as_delimiter() -> Delimiter {
        Delimiter::Parenthesis
    }
}

pub trait Separator {
    fn as_char() -> Option<char>;
}

/// Punctuation character `,` that can be used in conjunction with
/// `SeparatedBy`.
pub enum Comma {}

impl Separator for Comma {
    fn as_char() -> Option<char> {
        Some(',')
    }
}

/// Punctuation character `/` that can be used in conjunction with
/// `SeparatedBy`.
pub enum Slash {}

impl Separator for Slash {
    fn as_char() -> Option<char> {
        Some('/')
    }
}

/// Parsable type that reads a token stream from the contents of a group token
/// that is delimited with the specified punctuation token.
///
/// ```rust
/// use casco::domain::{GroupedBy, Parentheses};
///
/// let result = GroupedBy::<Parentheses, bool>::parse(&[
///     /* "(true)" */
/// ]);
/// ```
pub struct GroupedBy<G, T> {
    phantom: PhantomData<(G, T)>,
}

impl<S, G, T> Parse<S> for GroupedBy<G, T>
where
    S: TokenStream,
    G: Group,
    T: Parse<S>,
{
    type Output = T::Output;

    fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]) {
        let start = tokens;

        match tokens {
            [TokenTree::Group(group), remaining @ ..] if group.delimiter() == G::as_delimiter() => {
                let tokens = group.stream().into_iter().collect::<Vec<_>>();
                match T::parse(&tokens) {
                    (Some(value), []) => (Some(value), remaining),
                    _ => (None, start),
                }
            }
            _ => (None, tokens),
        }
    }
}

/// Parsable type that repeatedly and alternatingly invokes a sub-parser and
/// consumes a punctuation token.
pub struct SeparatedBy<P, T> {
    phantom: PhantomData<(P, T)>,
}

macro_rules! tuple_impl {
    (@parse $first:ident $($second:ident)*) => {
        fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]) {
            let start = tokens;

            #[allow(non_snake_case)]
            let ($first, tokens) = match $first::parse(tokens) {
                (Some(value), tokens) => (value, tokens),
                _ => return (None, start),
            };

            $(
                let tokens = if let Some(char) = P::as_char() {
                    match tokens.first() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == char => &tokens[1..],
                        _ => return (None, start),
                    }
                } else {
                    tokens
                };

                #[allow(non_snake_case)]
                let ($second, tokens) = match $second::parse(tokens) {
                    (Some($second), tokens) => ($second, tokens),
                    _ => return (None, start),
                };
            )*

            (Some(($first, $($second,)*)), tokens)
        }
    };
    ($($generics:ident),*) => {
        impl<S, P, $($generics),*> Parse<S> for SeparatedBy<P, ($($generics),*)>
        where
            S: TokenStream,
            P: Separator,
            $(
                $generics: Parse<S>,
            )*
        {
            type Output = ($($generics::Output,)*);

            tuple_impl!(@parse $($generics)*);
        }
    };
}

tuple_impl!(A, B);
tuple_impl!(A, B, C);
tuple_impl!(A, B, C, D);
tuple_impl!(A, B, C, D, E);
tuple_impl!(A, B, C, D, E, F);

macro_rules! slice_impl {
    (@parse $first:ident $($second:ident)*) => {
        fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]) {
            let start = tokens;

            #[allow(non_snake_case)]
            let ($first, tokens) = match T::parse(tokens) {
                (Some(value), tokens) => (value, tokens),
                _ => return (None, start),
            };

            $(
                let tokens = if let Some(char) = P::as_char() {
                    match tokens.first() {
                        Some(TokenTree::Punct(punct)) if punct.as_char() == char => &tokens[1..],
                        _ => return (None, start),
                    }
                } else {
                    tokens
                };

                #[allow(non_snake_case)]
                let ($second, tokens) = match T::parse(tokens) {
                    (Some($second), tokens) => ($second, tokens),
                    _ => return (None, start),
                };
            )*

            (Some([$first, $($second,)*]), tokens)
        }
    };
    ($count:tt, $($names:ident),*) => {
        impl<S, P, T> Parse<S> for SeparatedBy<P, [T; $count]>
        where
            S: TokenStream,
            P: Separator,
            T: Parse<S>,
        {
            type Output = [T::Output; $count];

            slice_impl!(@parse $($names)*);
        }
    };
}

slice_impl!(6, a, b, c, d, e, f);
slice_impl!(16, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p);

impl<S> Parse<S> for bool
where
    S: TokenStream,
{
    type Output = bool;

    fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]) {
        match tokens.first() {
            Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
                "true" | "yes" | "on" => (Some(true), &tokens[1..]),
                "false" | "no" | "off" => (Some(false), &tokens[1..]),
                _ => (None, tokens),
            },
            _ => (None, tokens),
        }
    }
}

impl<S> Parse<S> for f32
where
    S: TokenStream,
{
    type Output = f32;

    fn parse<'a>(tokens: &'a [TokenTree<S>]) -> (Option<Self::Output>, &'a [TokenTree<S>]) {
        super::number(tokens)
    }
}
