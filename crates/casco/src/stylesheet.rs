use derivative::Derivative;
use std::fmt::Debug;

use crate::cascade;
use crate::stream::{TokenStream, TokenTree};

/// Type of error for exceptions that occur during parsing of domain-specific
/// properties and rules.
#[derive(Derivative)]
#[derivative(Debug(bound = "D::Error: Debug"))]
pub enum Error<D, S>
where
    D: Driver<S> + ?Sized,
    S: TokenStream,
{
    /// This type of error is emitted when an exception occurs during parsing of
    /// the cascade.
    Cascade(cascade::Error<S>),

    /// This type of error is emitted by the driver's domain-specific parser.
    Domain(D::Error),
}

/// Controls how the domain-specific parts of a style sheet are parsed.
pub trait Driver<S>
where
    S: TokenStream,
{
    /// Controls how property names and values are parsed.
    type Property;

    /// Controls how selectors and rule bodies are parsed.
    type Rule;

    /// Type of error that is returned if the name or value of a property
    /// could not be parsed successfully or if the selectors or body of a rule
    /// could not be parsed successfully.
    type Error;

    /// Controls how property names and values are parsed.
    fn parse_property(
        &mut self,
        name: &[TokenTree<S>],
        value: &[TokenTree<S>],
    ) -> Result<Self::Property, Self::Error>;

    /// Controls how rule selectors and bodies are parsed.
    fn parse_rule(
        &mut self,
        selectors: &[TokenTree<S>],
        body: &S::Group,
    ) -> Result<Self::Rule, Vec<Error<Self, S>>>;
}

/// Domain-specific style sheet driven by an implementation of the `Driver`
/// trait.
#[derive(Derivative)]
#[derivative(Debug(bound = "Item<D, S>: Debug"))]
pub struct StyleSheet<D, S>
where
    D: Driver<S>,
    S: TokenStream,
{
    /// Domain-specific items that are in the style sheet.
    pub items: Vec<Item<D, S>>,
}

impl<D, S> StyleSheet<D, S>
where
    D: Driver<S>,
    S: TokenStream,
{
    /// Attempts to the given token stream into a domain-specific style sheet.
    pub fn parse(
        driver: &mut D,
        tokens: &[TokenTree<S>],
    ) -> Result<StyleSheet<D, S>, Vec<Error<D, S>>> {
        let items = cascade::parse(tokens);

        match cascade::validate(&items) {
            errors if !errors.is_empty() => {
                return Err(errors
                    .into_iter()
                    .map(|error| Error::Cascade(error))
                    .collect())
            }
            _ => {}
        };

        let mut errors = vec![];
        let mut results = vec![];

        for item in items {
            match item {
                cascade::Item::Property(property) => {
                    match driver.parse_property(property.name, property.value) {
                        Ok(result) => results.push(Item::Property(result)),
                        Err(error) => errors.push(Error::Domain(error)),
                    }
                }
                cascade::Item::Rule(rule) => {
                    match driver.parse_rule(rule.selectors, rule.group.unwrap()) {
                        Ok(result) => results.push(Item::Rule(result)),
                        Err(error) => errors.extend(error),
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(StyleSheet { items: results })
        } else {
            Err(errors)
        }
    }
}

/// Represents a single domain-specific item (either a property or a rule).
#[derive(Derivative)]
#[derivative(Debug(bound = "D::Property: Debug, D::Rule: Debug"))]
pub enum Item<D, S>
where
    D: Driver<S>,
    S: TokenStream,
{
    /// Represents a parsed CSS-like property that started with a sequence of
    /// tokens which usually is the name, a color, another sequence of tokens
    /// which is usually the value and finally ended with a semicolon, prior to
    /// being parsed into a domain-specific representation.
    Property(D::Property),

    /// Represents a parsed CSS-like rule that started with a preamble that
    /// usually contains the selectors and ended with a `{ ... }` delimited
    /// group, prior to being parsed into a domain-specific representation.
    Rule(D::Rule),
}
