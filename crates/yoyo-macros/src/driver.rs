use casco::scanner::{Delimiter, Ident, Literal, Punct, TokenTree};
use std::num::ParseFloatError;

use super::{Spring, Style, Transition};

pub struct Driver;

#[derive(Clone, Debug)]
pub enum Selector {
    Ampersand,
    State(String),
    ClassName(String),
    FromState(String),
    FromClassName(String),
}

#[derive(Copy, Clone, Debug)]
pub enum Property {
    Opacity(f32),
    OpacityTransition(Transition),
    TransformTranslationX(f32),
    TransformTranslationXTranslation(Transition),
}

impl Property {
    pub fn apply(self, style: &mut Style) {
        match self {
            Property::Opacity(opacity) => style.opacity = opacity,
            Property::OpacityTransition(transition) => style.opacity_transition = transition,
            Property::TransformTranslationX(transform) => style.transform_translation_x = transform,
            Property::TransformTranslationXTranslation(transition) => {
                style.transform_translation_x_transition = transition
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Error {
    UnknownProperty(Ident),
    UnexpectedTokenTree(TokenTree),
    InvalidFloat(Literal, ParseFloatError),
    InvalidOrigin(Selector),
}

impl Driver {
    fn parse_class_name<S>(&self, scanner: &mut S) -> Result<Selector, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match scanner.next() {
            Some(TokenTree::Ident(ident)) => Ok(Selector::ClassName(ident.to_string())),
            Some(tt) => Err(Error::UnexpectedTokenTree(tt)),
            _ => unimplemented!("Couldn't parse class name."),
        }
    }

    fn parse_selector<S>(&self, scanner: &mut S) -> Result<Selector, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match scanner.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '&' => Ok(Selector::Ampersand),
            Some(TokenTree::Punct(punct)) if punct.as_char() == ':' => self.parse_state(scanner),
            Some(TokenTree::Punct(punct)) if punct.as_char() == '.' => {
                self.parse_class_name(scanner)
            }
            Some(tt) => Err(Error::UnexpectedTokenTree(tt)),
            _ => unimplemented!("Couldn't parse selector."),
        }
    }

    fn parse_state<S>(&self, scanner: &mut S) -> Result<Selector, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match scanner.next() {
            Some(TokenTree::Ident(ident)) => {
                let state = ident.to_string();

                match state.as_str() {
                    "from" => match scanner.next() {
                        Some(TokenTree::Group(group))
                            if group.delimiter() == Delimiter::Parenthesis =>
                        {
                            let mut stream = group.stream().into_iter();
                            let subselector = self.parse_selector(&mut stream)?;
                            assert!(stream.next().is_none());

                            match subselector {
                                Selector::State(state) => Ok(Selector::FromState(state)),
                                Selector::ClassName(name) => Ok(Selector::FromClassName(name)),
                                _ => Err(Error::InvalidOrigin(subselector)),
                            }
                        }
                        Some(tt) => Err(Error::UnexpectedTokenTree(tt)),
                        _ => unreachable!("Couldn't parse from state."),
                    },
                    _ => Ok(Selector::State(ident.to_string())),
                }
            }
            Some(tt) => Err(Error::UnexpectedTokenTree(tt)),
            _ => unimplemented!("Couldn't parse state."),
        }
    }

    fn parse_dimension<S>(&self, value: &mut S) -> Result<f32, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match value.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == '-' => {
                self.parse_dimension(value).map(|dim| -dim)
            }
            Some(TokenTree::Literal(literal)) => {
                let lit = syn::parse_str::<syn::Lit>(&literal.to_string());

                match lit {
                    Ok(syn::Lit::Int(lit)) => match lit.suffix() {
                        "px" => Ok(lit.base10_parse::<f32>().unwrap()),
                        _ => unimplemented!("Unexpected literal: {:#?}", literal),
                    },
                    Ok(syn::Lit::Float(lit)) => match lit.suffix() {
                        "px" => Ok(lit.base10_parse::<f32>().unwrap()),
                        _ => unimplemented!("Unexpected literal: {:#?}", literal),
                    },
                    _ => unimplemented!("Unexpected literal: {:#?}", literal),
                }
            }
            token => unimplemented!("Unexpected token: {:#?}", token),
        }
    }

    fn parse_f32<S>(&self, value: &mut S) -> Result<f32, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        let literal = match value.next() {
            Some(TokenTree::Literal(literal)) => literal,
            Some(tt) => return Err(Error::UnexpectedTokenTree(tt)),
            _ => unreachable!(),
        };

        literal
            .to_string()
            .parse()
            .map_err(|error| Error::InvalidFloat(literal, error))
    }

    fn parse_bool<S>(&self, scanner: &mut S) -> Result<bool, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match scanner.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "true" => Ok(true),
            Some(TokenTree::Ident(ident)) if ident.to_string() == "false" => Ok(false),
            _ => unimplemented!("TODO: emit error."),
        }
    }

    fn parse_punct<S>(&self, scanner: &mut S, expected: char) -> Result<Punct, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match scanner.next() {
            Some(TokenTree::Punct(punct)) if punct.as_char() == expected => Ok(punct),
            _ => unimplemented!("TODO: emit error."),
        }
    }

    fn parse_transition<S>(&self, scanner: &mut S) -> Result<Transition, Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match scanner.next() {
            Some(TokenTree::Ident(ident)) if ident.to_string() == "ease_in_out" => {
                Ok(Transition::EaseInOut)
            }
            Some(TokenTree::Ident(ident)) if ident.to_string() == "step" => Ok(Transition::Step),
            Some(TokenTree::Ident(ident)) if ident.to_string() == "delay" => match scanner.next() {
                Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Parenthesis => {
                    let mut scanner = group.stream().into_iter();

                    let delay = self.parse_f32(&mut scanner)?;

                    assert!(scanner.next().is_none());

                    Ok(Transition::Delay(delay))
                }
                _ => unimplemented!("TODO: emit error."),
            },
            Some(TokenTree::Ident(ident)) if ident.to_string() == "spring" => {
                match scanner.next() {
                    Some(TokenTree::Group(group))
                        if group.delimiter() == Delimiter::Parenthesis =>
                    {
                        let mut scanner = group.stream().into_iter();

                        let stiffness = self.parse_f32(&mut scanner)?;
                        let _comma = self.parse_punct(&mut scanner, ',')?;

                        let damping = self.parse_f32(&mut scanner)?;
                        let _comma = self.parse_punct(&mut scanner, ',')?;

                        let mass = self.parse_f32(&mut scanner)?;
                        let _comma = self.parse_punct(&mut scanner, ',')?;

                        let allows_overdamping = self.parse_bool(&mut scanner)?;
                        let _comma = self.parse_punct(&mut scanner, ',')?;

                        let overshoot_clamping = self.parse_bool(&mut scanner)?;

                        assert!(scanner.next().is_none());

                        Ok(Transition::Spring(Spring {
                            stiffness,
                            damping,
                            mass,
                            allows_overdamping,
                            overshoot_clamping,
                        }))
                    }
                    Some(_) => unimplemented!("TODO: emit error."),
                    None => Ok(Transition::Spring(Spring::default())),
                }
            }
            Some(tt) => Err(Error::UnexpectedTokenTree(tt)),
            None => unimplemented!("TODO: emit error."),
        }
    }
}

impl casco::Driver for Driver {
    type Error = Error;
    type Selector = Selector;
    type Property = Property;

    fn parse_selector<S>(&self, scanner: &mut S) -> Result<Self::Selector, Self::Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        Driver::parse_selector(self, scanner)
    }

    fn parse_property<S>(&self, name: Ident, value: &mut S) -> Result<Self::Property, Self::Error>
    where
        S: Iterator<Item = TokenTree>,
    {
        match name.to_string().as_str() {
            "opacity" => self.parse_f32(value).map(|value| Property::Opacity(value)),
            "opacity_transition" => self
                .parse_transition(value)
                .map(|value| Property::OpacityTransition(value)),
            "transform_translation_x" => self
                .parse_dimension(value)
                .map(|value| Property::TransformTranslationX(value)),
            "transform_translation_x_transition" => self
                .parse_transition(value)
                .map(|value| Property::TransformTranslationXTranslation(value)),
            _ => Err(Error::UnknownProperty(name)),
        }
    }
}
