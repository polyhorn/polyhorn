use casco::stream::{MultiSpan, Spanned, TokenStream, TokenTree};
use derivative::Derivative;
use std::marker::PhantomData;

use super::{ParseError, Parser, PropertyValue};

/// This represents a single CSS-like property and contains the spans of the
/// name and value in case we need to emit diagnostics later on.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub struct Property<S>
where
    S: TokenStream,
{
    /// This is the span of the name tokens. For example, if the name consists
    /// of multiple identifiers separated by dashes, this will stretch multiple
    /// individual token spans.
    pub name_span: MultiSpan<S>,

    /// This is the span of the value tokens. For example, if the value consists
    /// of multiple tokens, this will stretch multiple individual token spans.
    pub value_span: MultiSpan<S>,

    /// This is the interpreted value of this property.
    pub value: PropertyValue,
}

#[derive(Debug)]
pub struct Rule;

/// This type implements casco's `Driver` trait and drives the parsing of our
/// domain-specific CSS properties.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Driver<S>(PhantomData<S>);

impl<S> Driver<S>
where
    S: TokenStream,
{
    /// Returns a new driver for the given concrete type of token stream.
    pub fn new() -> Driver<S> {
        Driver(PhantomData)
    }
}

impl<S> casco::Driver<S> for Driver<S>
where
    S: TokenStream,
{
    type Property = Property<S>;
    type Rule = Rule;
    type Error = Error<S>;

    fn parse_property(
        &mut self,
        name: &[TokenTree<S>],
        value: &[TokenTree<S>],
    ) -> Result<Self::Property, Self::Error> {
        let name_span = MultiSpan::new(name);
        let value_span = MultiSpan::new(value);

        let (name, remaining) = match casco::domain::name(name) {
            (Some(name), remaining) => (name, remaining),
            _ => return Err(Error::UnexpectedToken(value[0].span())),
        };

        if let Some(first) = remaining.first() {
            return Err(Error::UnexpectedToken(first.span()));
        }

        let value = match name.as_str() {
            "align-items" => PropertyValue::AlignItems(Parser::parse_enum(value)?),
            "background-color" => PropertyValue::BackgroundColor(Parser::parse_color(value)?),
            "border" => PropertyValue::Border(Parser::parse_border(value)?),
            "border-radius" => PropertyValue::BorderRadius(Parser::parse_border_radius(value)?),
            "bottom" => PropertyValue::Bottom(Parser::parse_dimension(value)?),
            "color" => PropertyValue::Color(Parser::parse_inherited_color(value)?),
            "direction" => PropertyValue::Direction(Parser::parse_enum(value)?),
            "flex-basis" => PropertyValue::FlexBasis(Parser::parse_dimension(value)?),
            "flex-direction" => PropertyValue::FlexDirection(Parser::parse_enum(value)?),
            "flex-grow" => PropertyValue::FlexGrow(Parser::parse_number(value)?),
            "flex-shrink" => PropertyValue::FlexShrink(Parser::parse_number(value)?),
            "font-family" => PropertyValue::FontFamily(Parser::parse_font_family(value)?),
            "font-size" => PropertyValue::FontSize(Parser::parse_font_size(value)?),
            "font-style" => PropertyValue::FontStyle(Parser::parse_enum(value)?),
            "font-weight" => PropertyValue::FontWeight(Parser::parse_font_weight(value)?),
            "height" => PropertyValue::Height(Parser::parse_dimension(value)?),
            "justify-content" => PropertyValue::JustifyContent(Parser::parse_enum(value)?),
            "left" => PropertyValue::Left(Parser::parse_dimension(value)?),
            "margin" => {
                PropertyValue::Margin(Parser::parse_by_edge(value, Parser::take_dimension)?)
            }
            "max-height" => PropertyValue::MaxHeight(Parser::parse_dimension(value)?),
            "max-width" => PropertyValue::MaxWidth(Parser::parse_dimension(value)?),
            "min-height" => PropertyValue::MinHeight(Parser::parse_dimension(value)?),
            "min-width" => PropertyValue::MinWidth(Parser::parse_dimension(value)?),
            "object-fit" => PropertyValue::ObjectFit(Parser::parse_enum(value)?),
            "opacity" => PropertyValue::Opacity(Parser::parse_number(value)?),
            "overflow" => PropertyValue::Overflow(Parser::parse_enum(value)?),
            "padding" => {
                PropertyValue::Padding(Parser::parse_by_edge(value, Parser::take_dimension)?)
            }
            "position" => PropertyValue::Position(Parser::parse_enum(value)?),
            "right" => PropertyValue::Right(Parser::parse_dimension(value)?),
            "text-align" => PropertyValue::TextAlign(Parser::parse_enum(value)?),
            "tint-color" => PropertyValue::TintColor(Parser::parse_color(value)?),
            "transform" => PropertyValue::Transform(Parser::parse_transform(value)?),
            "top" => PropertyValue::Top(Parser::parse_dimension(value)?),
            "visibility" => PropertyValue::Visibility(Parser::parse_enum(value)?),
            "width" => PropertyValue::Width(Parser::parse_dimension(value)?),
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
        preamble: &[TokenTree<S>],
        body: &S::Group,
    ) -> Result<Self::Rule, Vec<casco::Error<Self, S>>> {
        Err(vec![casco::Error::Domain(Error::UnexpectedRule(
            MultiSpan::new(preamble),
            body.span(),
        ))])
    }
}

/// This is an error that is emitted while parsing the domain-specific property
/// value types.
#[derive(Derivative)]
#[derivative(Debug(bound = ""))]
pub enum Error<S>
where
    S: TokenStream,
{
    /// This error is emitted when the parser encounters an unexpected token.
    UnexpectedToken(S::Span),

    /// This error is emitted when the parser encounters a property with a name
    /// that it does not recognize.
    UnrecognizedProperty(MultiSpan<S>),

    /// This error is emitted when the parser encounters a rule. Rules are
    /// currently ignored by this macro.
    UnexpectedRule(MultiSpan<S>, S::Span),

    /// Wraps an error that occurred while parsing a property value.
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
