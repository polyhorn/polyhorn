use casco::domain::{Comma, GroupedBy, Parentheses, Parse, SeparatedBy};
use casco::stream::{Delimiter, Group, MultiSpan, Punct, Spanned, TokenStream, TokenTree};
use casco::{Item, StyleSheet};
use derivative::Derivative;
use polyhorn_ui::macros::style::{ParseError, Parser};
use std::marker::PhantomData;

use super::{PropertyValue, Spring, Transition};

pub struct Driver<S>(PhantomData<S>)
where
    S: TokenStream;

impl<S> Driver<S>
where
    S: TokenStream,
{
    pub fn new() -> Driver<S> {
        Driver(PhantomData)
    }
}

#[derive(Clone, Debug)]
pub enum Selector {
    Ampersand,
    State(String),
    ClassName(String),
    FromState(String),
    FromClassName(String),
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Rule<S>
where
    S: TokenStream,
{
    pub selectors_span: MultiSpan<S>,
    pub properties_span: S::Span,

    pub selectors: Vec<Selector>,
    pub items: Vec<Item<Driver<S>, S>>,
}

#[derive(Copy, Clone, Debug)]
pub struct Property<S>
where
    S: TokenStream,
{
    /// This is the span of the name tokens. For example, if the name contains
    /// consists of multiple identifiers separated by dashes, this will stretch
    /// multiple individual token spans.
    pub name_span: MultiSpan<S>,

    /// This is the span of the value tokens. For example, if the value consists
    /// of multiple tokens, this will stretch multiple individual token spans.
    pub value_span: MultiSpan<S>,

    /// This is the interpreted value of this property.
    pub value: PropertyValue,
}

impl<S> Driver<S>
where
    S: TokenStream,
{
    fn parse_selectors(&self, tokens: &[TokenTree<S>]) -> Result<Vec<Selector>, Error<S>> {
        Ok(vec![self.parse_selector(tokens)?])
    }

    fn parse_selector(&self, tokens: &[TokenTree<S>]) -> Result<Selector, Error<S>> {
        match tokens.first() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '&' => Ok(Selector::Ampersand),
            Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => {
                self.parse_state(&tokens[1..])
            }
            Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                self.parse_class_name(&tokens[1..])
            }
            Some(tt) => Err(Error::UnexpectedToken(tt.span())),
            _ => unimplemented!("Couldn't parse selector."),
        }
    }

    fn parse_class_name(&self, tokens: &[TokenTree<S>]) -> Result<Selector, Error<S>> {
        match tokens.first() {
            Some(TokenTree::Ident(ident)) => Ok(Selector::ClassName(ident.to_string())),
            Some(tt) => Err(Error::UnexpectedToken(tt.span())),
            _ => unimplemented!("Couldn't parse class name."),
        }
    }

    fn parse_state(&self, tokens: &[TokenTree<S>]) -> Result<Selector, Error<S>> {
        match tokens.first() {
            Some(TokenTree::Ident(ident)) => {
                let state = ident.to_string();

                match state.as_str() {
                    "from" => match tokens.get(1) {
                        Some(TokenTree::Group(group))
                            if group.delimiter() == Delimiter::Parenthesis =>
                        {
                            let stream = group.stream().into_iter().collect::<Vec<_>>();
                            let subselector = self.parse_selector(&stream)?;

                            match subselector {
                                Selector::State(state) => Ok(Selector::FromState(state)),
                                Selector::ClassName(name) => Ok(Selector::FromClassName(name)),
                                _ => unimplemented!("This error is not yet implemented."),
                            }
                        }
                        Some(tt) => Err(Error::UnexpectedToken(tt.span())),
                        _ => unreachable!("Couldn't parse from state."),
                    },
                    _ => Ok(Selector::State(ident.to_string())),
                }
            }
            Some(tt) => Err(Error::UnexpectedToken(tt.span())),
            _ => unimplemented!("Couldn't parse state."),
        }
    }

    fn parse_transition(&self, tokens: &[TokenTree<S>]) -> Result<Transition, Error<S>> {
        if let Ok(transition) = Parser::parse_enum(tokens) {
            return Ok(transition);
        }

        let start = tokens[0].span();

        if let (Some(name), tokens) = casco::domain::name(tokens) {
            match name.as_str() {
                "delay" => match GroupedBy::<Parentheses, f32>::parse(tokens) {
                    (Some(delay), []) => Ok(Transition::Delay(delay)),
                    (_, [token, ..]) => Err(Error::UnexpectedToken(token.span())),
                    (_, _) => Err(Error::UnexpectedToken(start)),
                },
                "ease-in-out" => match GroupedBy::<Parentheses, f32>::parse(tokens) {
                    (Some(duration), []) => Ok(Transition::EaseInOut(duration)),
                    (_, [token, ..]) => Err(Error::UnexpectedToken(token.span())),
                    (_, _) => Err(Error::UnexpectedToken(start)),
                },
                "spring" => {
                    match GroupedBy::<
                        Parentheses,
                        SeparatedBy<
                            Comma,
                            (f32, f32, f32, bool, bool),
                        >,
                    >::parse(tokens)
                    {
                        (
                            Some((
                                stiffness,
                                damping,
                                mass,
                                allows_overdamping,
                                overshoot_clamping,
                            )),
                            [],
                        ) => Ok(Transition::Spring(Spring {
                            stiffness,
                            damping,
                            mass,
                            allows_overdamping,
                            overshoot_clamping,
                        })),
                        (_, [token, ..]) => Err(Error::UnexpectedToken(token.span())),
                        (_, _) => Err(Error::UnexpectedToken(start)),
                    }
                }
                _ => Err(Error::UnexpectedToken(start)),
            }
        } else {
            Err(Error::UnexpectedToken(start))
        }
    }
}

impl<S> casco::Driver<S> for Driver<S>
where
    S: TokenStream,
{
    type Error = Error<S>;
    type Property = Property<S>;
    type Rule = Rule<S>;

    /// Controls how property names and values are parsed.
    fn parse_property(
        &mut self,
        name: &[TokenTree<S>],
        value: &[TokenTree<S>],
    ) -> Result<Self::Property, Self::Error> {
        let name_span = MultiSpan::new(name);
        let value_span = MultiSpan::new(value);

        let (name, remaining) = match casco::domain::name(name) {
            (Some(name), remaining) => (name, remaining),
            _ => return Err(Error::UnexpectedToken(name[0].span())),
        };

        if let Some(first) = remaining.first() {
            return Err(Error::UnexpectedToken(first.span()));
        }

        let value = match name.as_str() {
            "opacity" => PropertyValue::Opacity(Parser::parse_number(value)?),
            "transition-opacity" => PropertyValue::TransitionOpacity(self.parse_transition(value)?),
            "transform" => PropertyValue::Transform(Parser::parse_transform(value)?),
            "transition-transform" => {
                PropertyValue::TransitionTransform(self.parse_transition(value)?)
            }
            _ => return Err(Error::UnrecognizedProperty(name_span)),
        };

        Ok(Property {
            name_span,
            value_span,
            value,
        })
    }

    fn parse_rule(
        &mut self,
        selectors: &[TokenTree<S>],
        body: &S::Group,
    ) -> Result<Self::Rule, Vec<casco::Error<Self, S>>> {
        let tokens = body.stream().into_iter().collect::<Vec<_>>();

        Ok(Rule {
            selectors_span: MultiSpan::new(selectors),
            properties_span: body.span(),
            selectors: match self.parse_selectors(selectors) {
                Ok(selectors) => selectors,
                Err(error) => return Err(vec![casco::Error::Domain(error)]),
            },
            items: StyleSheet::parse(self, &tokens)?.items,
        })
    }
}

#[derive(Derivative)]
#[derivative(Copy(bound = ""), Clone(bound = ""), Debug(bound = ""))]
pub enum Error<S>
where
    S: TokenStream,
{
    /// This error is emitted when the parser encounters an unexpected token.
    UnexpectedToken(S::Span),

    /// This error is emitted when the parser encounters a property with a name
    /// that it does not recognize.
    UnrecognizedProperty(MultiSpan<S>),

    Parse(ParseError<S>),
}

impl<S> From<ParseError<S>> for Error<S>
where
    S: TokenStream,
{
    fn from(error: ParseError<S>) -> Self {
        Error::Parse(error)
    }
}
