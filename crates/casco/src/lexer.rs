//! Lexer implementation that turns a string into a spanned concrete token
//! stream.

#![cfg(feature = "lexer")]

use logos::Logos;

use crate::concrete::{
    Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree,
};

#[derive(PartialEq, Logos)]
enum Token<'a> {
    #[regex("[a-zA-Z]+")]
    Ident(&'a str),

    #[regex(r"[\+\-\*/%\^!&\|=@_\.,;:#$\?]")]
    Punct(&'a str),

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("(")]
    ParenthesisOpen,

    #[token(")")]
    ParenthesisClose,

    #[regex("(0|([1-9][0-9]*))(\\.([0-9]+)|)([a-z]*)")]
    NumericLiteral(&'a str),

    #[regex("\"(:?[^\"]|\\\\\")*\"")]
    StringLiteral(&'a str),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

/// Lexes the given string into a token stream.
pub fn lex(string: &str) -> Option<TokenStream> {
    let mut lexer = Token::lexer(string).spanned();

    fn lex_trees<'a, I>(
        lexer: &mut I,
        end: Option<Token>,
    ) -> Option<(Vec<TokenTree>, Option<logos::Span>)>
    where
        I: Iterator<Item = (Token<'a>, logos::Span)>,
    {
        let mut results = vec![];

        while let Some(token) = lexer.next() {
            match token {
                (token, span) if end.as_ref().map(|end| end == &token).unwrap_or_default() => {
                    return Some((results, Some(span)))
                }
                (Token::Ident(ident), span) => {
                    results.push(Ident::new(ident, Span::new(span.start, span.end)).into())
                }
                (Token::Punct(punct), span) => results.push(
                    Punct::new(
                        punct.chars().next().unwrap(),
                        Spacing::Alone,
                        Span::new(span.start, span.end),
                    )
                    .into(),
                ),
                (Token::NumericLiteral(string), span) => {
                    results.push(Literal::new(string, Span::new(span.start, span.end)).into())
                }
                (Token::StringLiteral(string), span) => {
                    results.push(Literal::new(string, Span::new(span.start, span.end)).into())
                }
                (Token::BracketOpen, start) => {
                    let (trees, end) = lex_trees(lexer, Some(Token::BracketClose))?;

                    results.push(
                        Group::new(
                            Delimiter::Bracket,
                            TokenStream::new(trees),
                            Span::new(start.start, end?.end),
                        )
                        .into(),
                    )
                }
                (Token::BraceOpen, start) => {
                    let (trees, end) = lex_trees(lexer, Some(Token::BraceClose))?;

                    results.push(
                        Group::new(
                            Delimiter::Brace,
                            TokenStream::new(trees),
                            Span::new(start.start, end?.end),
                        )
                        .into(),
                    )
                }
                (Token::ParenthesisOpen, start) => {
                    let (trees, end) = lex_trees(lexer, Some(Token::ParenthesisClose))?;

                    results.push(
                        Group::new(
                            Delimiter::Parenthesis,
                            TokenStream::new(trees),
                            Span::new(start.start, end?.end),
                        )
                        .into(),
                    )
                }
                (_, _) => return None,
            }
        }

        Some((results, None))
    }

    Some(TokenStream::new(lex_trees(&mut lexer, None)?.0))
}

#[cfg(test)]
mod tests {
    use super::lex;

    #[test]
    fn test_scanner() {
        println!("Lex: {:#?}", lex("background-color: red; a { b: 1.5px; }"));
    }
}
