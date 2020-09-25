//! Types and functions to parse a token stream into a cascade.

use derivative::Derivative;

use crate::stream::{Delimiter, Group, Punct, Spanned, TokenStream, TokenTree};

/// Represents a CSS rule that starts with a preamble which usually contains the
/// selectors and ends with a `{ ... }` delimited group.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Rule<'a, S>
where
    S: TokenStream,
{
    /// This is the preamble: a slice of tokens that was encountered before the
    /// `{ ... }` delimited group and that didn't match any other grammar.
    pub selectors: &'a [TokenTree<S>],

    /// This is the `{ ... }` delimited group. The last rule in a parse might
    /// not have a group in the case of a syntax error.
    pub group: Option<&'a S::Group>,
}

/// Represents a CSS property that starts with a sequence of tokens which
/// usually is the name, a colon, another sequence of tokens which usually is
/// the value and finally ends with a semicolon. The semicolon is the parser's
/// anchor point: it starts by locating the semicolon and then parses backwards
/// until it has collected the name.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Property<'a, S>
where
    S: TokenStream,
{
    /// This is the preamble: a slice of tokens that was encountered before the
    /// colon and that didn't match any other grammar. The validator will emit
    /// an error if this slice is empty. Apart from that, this slice might
    /// contain any kind of token (including more colons).
    pub name: &'a [TokenTree<S>],

    /// This is the colon. This is missing if we find a semicolon, search
    /// backwards for the colon and can't find it. The validator will emit an
    /// error if this field is missing.
    pub colon: Option<&'a S::Punct>,

    /// This is a slice of tokens that were encountered between the semicolon
    /// and the colon. Although the property is parsed from right to left, this
    /// slice has the same order as the parser input. The validator will emit an
    /// error if this slice is empty. Part from that, this slice might contain
    /// any kind of token, except a colon.
    pub value: &'a [TokenTree<S>],

    /// This is the semicolon. This field is always present because it serves as
    /// the anchor point for our parser.
    pub semicolon: &'a S::Punct,
}

/// Represents a CSS item: either a property or a rule.
#[derive(Derivative)]
#[derivative(Debug)]
pub enum Item<'a, S>
where
    S: TokenStream,
{
    /// This represents a CSS rule that starts with a preamble which usually
    /// contains the selectors and ends with a `{ ... }` delimited group.
    Property(Property<'a, S>),

    /// This represents a CSS property that starts with a sequence of tokens
    /// which usually is the name, a colon, another sequence of tokens which
    /// usually is the value and finally ends with a semicolon. The semicolon is
    /// the parser's anchor point: it starts by locating the semicolon and then
    /// parses backwards until it has collected the name.
    Rule(Rule<'a, S>),
}

/// Parses zero or more rules from the given slice of tokens.
pub fn parse_rules<'a, S>(mut tokens: &'a [TokenTree<S>]) -> Vec<Rule<'a, S>>
where
    S: TokenStream,
{
    let mut rules = vec![];

    while !tokens.is_empty() {
        match tokens
            .iter()
            .enumerate()
            .find_map(|(i, token)| match token {
                TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => {
                    Some((i, group))
                }
                _ => None,
            }) {
            Some((position, group)) => {
                rules.push(Rule {
                    selectors: &tokens[..position],
                    group: Some(group),
                });

                tokens = &tokens[position + 1..];
            }
            None => {
                rules.push(Rule {
                    selectors: tokens,
                    group: None,
                });

                break;
            }
        }
    }

    rules
}

/// Parses a single property and zero or more preceeding rules from the given
/// slice of tokens.
pub fn parse_property<'a, S>(tokens: &'a [TokenTree<S>]) -> (Vec<Rule<'a, S>>, Property<'a, S>)
where
    S: TokenStream,
{
    let semicolon = match tokens.last() {
        Some(TokenTree::Punct(punct)) if punct.as_char() == ';' => punct.into(),
        _ => unreachable!(),
    };

    let colon_position = match tokens.iter().rposition(|token| match token {
        TokenTree::Punct(punct) if punct.as_char() == ':' => true,
        _ => false,
    }) {
        Some(position) => position,
        None => {
            return (
                vec![],
                Property {
                    name: &[],
                    colon: None,
                    value: &tokens[..tokens.len() - 1],
                    semicolon,
                },
            )
        }
    };

    let value = &tokens[colon_position + 1..tokens.len() - 1];

    let colon = match &tokens[colon_position] {
        TokenTree::Punct(punct) if punct.as_char() == ':' => punct,
        _ => unreachable!(),
    };

    let tokens = &tokens[..colon_position];

    let (rules, name) = match tokens.iter().rposition(|token| match token {
        TokenTree::Group(group) if group.delimiter() == Delimiter::Brace => true,
        _ => false,
    }) {
        Some(position) => (
            parse_rules(&tokens[..position + 1]),
            &tokens[position + 1..],
        ),
        None => (vec![], tokens),
    };

    (
        rules,
        Property {
            name,
            colon: Some(colon),
            value,
            semicolon,
        },
    )
}

/// Parses the given token stream and returns a vector of items. These items
/// might be missing fields. Use `validate(items)` to check for any errors. In
/// addition, this function does not recursively parse rules.
pub fn parse<'a, S>(mut tokens: &'a [TokenTree<S>]) -> Vec<Item<'a, S>>
where
    S: TokenStream,
{
    let mut items = vec![];

    while !tokens.is_empty() {
        // Read until we find a `;`.
        let end = tokens.iter().position(|token| match token {
            TokenTree::Punct(punct) if punct.as_char() == ';' => true,
            _ => false,
        });

        match end {
            // If we find a `;`, we assume that [0 .. end) consists of zero or
            // more rules and one property.
            Some(end) => {
                // This parses zero or more rules and one property.
                let (rules, property) = parse_property(&tokens[..end + 1]);
                items.extend(rules.into_iter().map(|rule| Item::Rule(rule)));
                items.push(Item::Property(property));
                tokens = &tokens[end + 1..];
            }
            // If we don't find a `;`, we assume that [0 .. end) consists of
            // zero or more rules of which the last rule might not have a block.
            None => {
                // This parses zero or more rules.
                let rules = parse_rules(tokens);
                items.extend(rules.into_iter().map(|rule| Item::Rule(rule)));
                break;
            }
        };
    }

    items
}

/// Type of error that is returned during validation.
#[derive(Derivative)]
#[derivative(Debug)]
pub enum Error<S>
where
    S: TokenStream,
{
    /// This error is returned when a property's name is missing. The span
    /// points to the token that was encountered instead.
    MissingPropertyName(S::Span),

    /// This error is returned when a : is missing. The span points to the token
    /// that was encountered instead.
    MissingColon(S::Span),

    /// This error is returned when a property's value is missing. The span
    /// points to the token that was encountered instead.
    MissingPropertyValue(S::Span),

    /// This error is returned when a selector is missing. The span points to
    /// the token that was encountered instead (which will always be the `{ ...
    /// }` group).
    MissingSelector(S::Span),

    /// This error is returned when a rule block is missing. The span points to
    /// the last selector that was encountered before the rule group was
    /// expected.
    MissingRuleGroup(S::Span),
}

/// Validates the given parse and returns any errors that it encounters.
pub fn validate<'a, S>(items: &[Item<S>]) -> Vec<Error<S>>
where
    S: TokenStream,
{
    let mut errors = vec![];

    for item in items {
        match item {
            Item::Property(property) => {
                if property.name.is_empty() {
                    errors.push(Error::MissingPropertyName(
                        property
                            .colon
                            .map(|colon| colon.span())
                            .or_else(|| property.value.first().map(|value| value.span()))
                            .unwrap_or(property.semicolon.span()),
                    ));
                } else if property.colon.is_none() {
                    errors.push(Error::MissingPropertyName(
                        property
                            .value
                            .first()
                            .map(|value| value.span())
                            .unwrap_or(property.semicolon.span()),
                    ));
                } else if property.value.is_empty() {
                    errors.push(Error::MissingPropertyValue(property.semicolon.span()));
                }
            }
            Item::Rule(rule) => {
                if let Some(selector) = rule.selectors.last() {
                    if rule.group.is_none() {
                        errors.push(Error::MissingRuleGroup(selector.span()));
                    }
                } else {
                    if let Some(group) = rule.group {
                        errors.push(Error::MissingSelector(group.span()));
                    }
                }
            }
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::lexer::lex;

    #[test]
    fn test_parser() {
        let stream = lex(":initial, :hover {
            background-color: { Color::red() };
        }")
        .unwrap();

        let parse = parse(stream.tokens.as_slice());
        let errors = validate(&parse[..]);
        println!("Result: {:#?} ({:#?})", parse, errors);
    }
}
