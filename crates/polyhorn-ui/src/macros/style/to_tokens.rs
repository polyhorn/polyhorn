use casco::stream::MultiSpan;
use num_traits::Float;
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, TokenStream, TokenTree};
use quote::{quote, ToTokens};
use std::iter::FromIterator;

use super::{Diagnostic, Driver, Error, ParseError};
use crate::color::Color;
use crate::font::{FontFamily, FontSize, FontStyle, FontWeight, GenericFontFamily};
use crate::geometry::{ByCorner, ByDirection, ByEdge, Dimension, Size};
use crate::layout::{LayoutAxisX, LayoutAxisY, LayoutDirection};
use crate::linalg::Transform3D;
use crate::styles::{
    Absolute, Align, Border, BorderStyle, FlexDirection, ImageStyle, Inherited, Justify, ObjectFit,
    Overflow, Position, Relative, TextAlign, TextStyle, Transform, ViewStyle, Visibility,
};

impl<T> ToTokens for Dimension<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Dimension::Auto => quote! { polyhorn::geometry::Dimension::Auto },
            Dimension::Undefined => quote! { polyhorn::geometry::Dimension::Undefined },
            Dimension::Points(points) => quote! { polyhorn::geometry::Dimension::Points(#points) },
            Dimension::Percentage(percentage) => {
                quote! { polyhorn::geometry::Dimension::Percentage(#percentage) }
            }
        })
    }
}

impl<T> ToTokens for LayoutAxisX<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            LayoutAxisX::DirectionDependent { leading, trailing } => quote! {
                polyhorn::layout::LayoutAxisX::DirectionDependent {
                    leading: #leading,
                    trailing: #trailing,
                }
            },
            LayoutAxisX::DirectionIndependent { left, right } => quote! {
                polyhorn::layout::LayoutAxisX::DirectionIndependent {
                    left: #left,
                    right: #right,
                }
            },
        })
    }
}

impl<T> ToTokens for LayoutAxisY<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let top = &self.top;
        let bottom = &self.bottom;

        tokens.extend(quote! {
            polyhorn::layout::LayoutAxisY {
                top: #top,
                bottom: #bottom,
            }
        })
    }
}

impl<T> ToTokens for ByEdge<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let horizontal = &self.horizontal;
        let vertical = &self.vertical;

        tokens.extend(quote! { polyhorn::geometry::ByEdge {
            horizontal: #horizontal,
            vertical: #vertical,
        } })
    }
}

impl ToTokens for Absolute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let distances = &self.distances;

        tokens.extend(quote! { polyhorn::styles::Absolute {
            distances: #distances
        } })
    }
}

impl ToTokens for Relative {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let flex_basis = &self.flex_basis;
        let flex_grow = &self.flex_grow;
        let flex_shrink = &self.flex_shrink;

        tokens.extend(quote! { polyhorn::styles::Relative {
            flex_basis: #flex_basis,
            flex_grow: #flex_grow,
            flex_shrink: #flex_shrink,
        } })
    }
}

impl ToTokens for Position {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Position::Absolute(absolute) => {
                quote! { polyhorn::styles::Position::Absolute(#absolute) }
            }
            Position::Relative(relative) => {
                quote! { polyhorn::styles::Position::Relative(#relative) }
            }
        })
    }
}

impl ToTokens for FlexDirection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FlexDirection::Column => quote! { polyhorn::styles::FlexDirection::Column },
            FlexDirection::ColumnReverse => {
                quote! { polyhorn::styles::FlexDirection::ColumnReverse }
            }
            FlexDirection::Row => quote! { polyhorn::styles::FlexDirection::Row },
            FlexDirection::RowReverse => quote! { polyhorn::styles::FlexDirection::RowReverse },
        })
    }
}

impl ToTokens for Align {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Align::FlexStart => quote! { polyhorn::styles::Align::FlexStart },
            Align::Center => quote! { polyhorn::styles::Align::Center },
            Align::FlexEnd => quote! { polyhorn::styles::Align::FlexEnd },
            Align::Stretch => quote! { polyhorn::styles::Align::Stretch },
            Align::SpaceAround => quote! { polyhorn::styles::Align::SpaceAround },
            Align::SpaceBetween => quote! { polyhorn::styles::Align::SpaceBetween },
        })
    }
}

impl ToTokens for Justify {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Justify::FlexStart => quote! { polyhorn::styles::Justify::FlexStart },
            Justify::Center => quote! { polyhorn::styles::Justify::Center },
            Justify::FlexEnd => quote! { polyhorn::styles::Justify::FlexEnd },
            Justify::SpaceAround => quote! { polyhorn::styles::Justify::SpaceAround },
            Justify::SpaceBetween => quote! { polyhorn::styles::Justify::SpaceBetween },
            Justify::SpaceEvenly => quote! { polyhorn::styles::Justify::SpaceEvenly },
        })
    }
}

impl ToTokens for LayoutDirection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            LayoutDirection::LTR => quote! { polyhorn::layout::LayoutDirection::LTR },
            LayoutDirection::RTL => quote! { polyhorn::layout::LayoutDirection::RTL },
        })
    }
}

impl ToTokens for Color {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let rgb = self.to_srgb();

        let red = (rgb.red * 255.0) as u8;
        let green = (rgb.green * 255.0) as u8;
        let blue = (rgb.blue * 255.0) as u8;
        let alpha = rgb.alpha;

        tokens.extend(quote! {
            polyhorn::color::Color::rgba(#red, #green, #blue, #alpha)
        });
    }
}

impl<T> ToTokens for Inherited<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Inherited::Inherited => quote! { polyhorn::styles::Inherited::Inherited },
            Inherited::Specified(specified) => {
                quote! { polyhorn::styles::Inherited::Specified(#specified) }
            }
        })
    }
}

impl ToTokens for GenericFontFamily {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            GenericFontFamily::Serif => quote! { polyhorn::font::GenericFontFamily::Serif },
            GenericFontFamily::SansSerif => quote! { polyhorn::font::GenericFontFamily::SansSerif },
            GenericFontFamily::Monospace => quote! { polyhorn::font::GenericFontFamily::Monospace },
        })
    }
}

impl<T> ToTokens for FontFamily<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontFamily::Generic(generic) => {
                quote! { polyhorn::font::FontFamily::Generic(#generic) }
            }
            FontFamily::Named(named) => quote! { polyhorn::font::FontFamily::Named(#named) },
        })
    }
}

impl ToTokens for FontWeight {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontWeight::Number(number) => quote! { polyhorn::font::FontWeight::Number(#number) },
            FontWeight::Thin => quote! { polyhorn::font::FontWeight::Thin },
            FontWeight::ExtraLight => quote! { polyhorn::font::FontWeight::ExtraLight },
            FontWeight::Light => quote! { polyhorn::font::FontWeight::Light },
            FontWeight::Normal => quote! { polyhorn::font::FontWeight::Normal },
            FontWeight::Medium => quote! { polyhorn::font::FontWeight::Medium },
            FontWeight::SemiBold => quote! { polyhorn::font::FontWeight::SemiBold },
            FontWeight::Bold => quote! { polyhorn::font::FontWeight::Bold },
            FontWeight::ExtraBold => quote! { polyhorn::font::FontWeight::ExtraBold },
            FontWeight::Black => quote! { polyhorn::font::FontWeight::Black },
        })
    }
}

impl ToTokens for FontStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontStyle::Normal => quote! { polyhorn::font::FontStyle::Normal },
            FontStyle::Oblique => quote! { polyhorn::font::FontStyle::Oblique },
            FontStyle::Italic => quote! { polyhorn::font::FontStyle::Italic },
        })
    }
}

impl ToTokens for FontSize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            FontSize::ExtraExtraSmall => quote! { polyhorn::font::FontSize::ExtraExtraSmall },
            FontSize::ExtraSmall => quote! { polyhorn::font::FontSize::ExtraSmall },
            FontSize::Small => quote! { polyhorn::font::FontSize::Small },
            FontSize::Medium => quote! { polyhorn::font::FontSize::Medium },
            FontSize::Large => quote! { polyhorn::font::FontSize::Large },
            FontSize::ExtraLarge => quote! { polyhorn::font::FontSize::ExtraLarge },
            FontSize::ExtraExtraLarge => quote! { polyhorn::font::FontSize::ExtraExtraLarge },
            FontSize::ExtraExtraExtraLarge => {
                quote! { polyhorn::font::FontSize::ExtraExtraExtraLarge }
            }
            FontSize::Dimension(dimension) => {
                quote! { polyhorn::font::FontSize::Dimension(#dimension) }
            }
        })
    }
}

impl ToTokens for TextAlign {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            TextAlign::Left => quote! { polyhorn::styles::TextAlign::Left },
            TextAlign::Center => quote! { polyhorn::styles::TextAlign::Center },
            TextAlign::Right => quote! { polyhorn::styles::TextAlign::Right },
        })
    }
}

impl<T> ToTokens for TextStyle<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let color = &self.color;
        let font_family = &self.font_family;
        let font_weight = &self.font_weight;
        let font_style = &self.font_style;
        let font_size = &self.font_size;
        let text_align = &self.text_align;

        tokens.extend(quote! {
            polyhorn::styles::TextStyle {
                color: #color,
                font_family: #font_family,
                font_weight: #font_weight,
                font_style: #font_style,
                font_size: #font_size,
                text_align: #text_align,
            }
        })
    }
}

impl<T> ToTokens for Size<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let width = &self.width;
        let height = &self.height;

        tokens.extend(quote! {
            polyhorn::geometry::Size {
                width: #width,
                height: #height,
            }
        })
    }
}

impl<T> ToTokens for Transform3D<T>
where
    T: ToTokens + Float,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let entries = self
            .columns
            .iter()
            .map(|row| {
                quote! {
                    [#(#row,)*]
                }
            })
            .collect::<Vec<_>>();

        tokens.extend(quote! {
            polyhorn::linalg::Transform3D {
                columns: [#(#entries,)*],
            }
        })
    }
}

impl<T> ToTokens for Transform<T>
where
    T: ToTokens + Float,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let matrix = &self.matrix;
        let relative_translation = self.relative_translation;
        let relative_translation = [relative_translation.0, relative_translation.1];

        tokens.extend(quote! {
            polyhorn::styles::Transform {
                matrix: #matrix,
                relative_translation: (#(#relative_translation,)*),
            }
        })
    }
}

impl ToTokens for Overflow {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Overflow::Visible => quote! { polyhorn::styles::Overflow::Visible },
            Overflow::Scroll => quote! { polyhorn::styles::Overflow::Scroll },
            Overflow::Hidden => quote! { polyhorn::styles::Overflow::Hidden },
        })
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Visibility::Visible => quote! { polyhorn::styles::Visibility::Visible },
            Visibility::Hidden => quote! { polyhorn::styles::Visibility::Hidden },
        })
    }
}

impl ToTokens for BorderStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            BorderStyle::Solid => quote! { polyhorn::styles::BorderStyle::Solid },
            BorderStyle::Dashed => quote! { polyhorn::styles::BorderStyle::Dashed },
            BorderStyle::Dotted => quote! { polyhorn::styles::BorderStyle::Dotted },
        })
    }
}

impl ToTokens for Border {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let width = &self.width;
        let style = &self.style;
        let color = &self.color;

        tokens.extend(quote! {
            polyhorn::styles::Border {
                width: #width,
                style: #style,
                color: #color,
            }
        });
    }
}

impl<T> ToTokens for ByDirection<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let horizontal = &self.horizontal;
        let vertical = &self.vertical;

        tokens.extend(quote! {
            polyhorn::geometry::ByDirection {
                horizontal: #horizontal,
                vertical: #vertical,
            }
        });
    }
}

impl<T> ToTokens for ByCorner<T>
where
    T: ToTokens,
{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let all = &self.all;

        tokens.extend(quote! {
            polyhorn::geometry::ByCorner {
                all: #all,
            }
        });
    }
}

impl ToTokens for ViewStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let position = &self.position;
        let direction = &self.direction;
        let size = &self.size;
        let min_size = &self.size;
        let max_size = &self.max_size;
        let flex_direction = &self.flex_direction;
        let align_items = &self.align_items;
        let justify_content = &self.justify_content;
        let margin = &self.margin;
        let border = &self.border;
        let border_radius = &self.border_radius;
        let padding = &self.padding;
        let background_color = &self.background_color;
        let opacity = &self.opacity;
        let transform = &self.transform;
        let overflow = &self.overflow;
        let visibility = &self.visibility;

        tokens.extend(quote! {
            polyhorn::styles::ViewStyle {
                position: #position,
                direction: #direction,
                size: #size,
                min_size: #min_size,
                max_size: #max_size,
                flex_direction: #flex_direction,
                align_items: #align_items,
                justify_content: #justify_content,
                margin: #margin,
                border: #border,
                border_radius: #border_radius,
                padding: #padding,
                background_color: #background_color,
                opacity: #opacity,
                transform: [#(#transform),*],
                overflow: #overflow,
                visibility: #visibility,
            }
        })
    }
}

impl ToTokens for ObjectFit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            ObjectFit::Fill => quote! { polyhorn::styles::ObjectFit::Fill },
            ObjectFit::None => quote! { polyhorn::styles::ObjectFit::None },
            ObjectFit::Contain => quote! { polyhorn::styles::ObjectFit::Contain },
            ObjectFit::Cover => quote! { polyhorn::styles::ObjectFit::Cover },
        })
    }
}

impl ToTokens for ImageStyle {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let tint_color = match &self.tint_color {
            Some(color) => quote! { Some(#color) },
            None => quote! { None },
        };
        let object_fit = &self.object_fit;

        tokens.extend(quote! {
            polyhorn::styles::ImageStyle {
                tint_color: #tint_color,
                object_fit: #object_fit,
            }
        })
    }
}

/// Utility type that implements `ToTokens` and can be constructed to generate
/// a compiler error with a particular span and message.
pub struct CompileError<'a> {
    span: casco::stream::MultiSpan<casco::proc_macro2::TokenStream>,
    message: &'a str,
}

impl<'a> CompileError<'a> {
    /// Returns a new compile error with the given span and message.
    pub fn new(
        span: casco::stream::MultiSpan<casco::proc_macro2::TokenStream>,
        message: &'a str,
    ) -> CompileError<'a> {
        CompileError { span, message }
    }
}

impl<'a> ToTokens for CompileError<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ident = Ident::new("compile_error", self.span.first().unwrap());

        let mut punct = Punct::new('!', Spacing::Alone);
        punct.set_span(self.span.last().unwrap());

        let literal = Literal::string(self.message);

        let mut group = Group::new(
            Delimiter::Parenthesis,
            TokenStream::from_iter(vec![TokenTree::Literal(literal)]),
        );

        group.set_span(self.span.last().unwrap());

        let mut semicolon = Punct::new(';', Spacing::Alone);
        semicolon.set_span(self.span.last().unwrap());

        tokens.extend(vec![
            TokenTree::Ident(ident),
            TokenTree::Punct(punct),
            TokenTree::Group(group),
            TokenTree::Punct(semicolon),
        ]);
    }
}

impl ToTokens for Diagnostic<casco::proc_macro2::TokenStream> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            &Diagnostic::UnusedProperty(span) => {
                let error = CompileError::new(
                    span,
                    "Property value is unused because it is later redefined.",
                );
                error.to_tokens(tokens)
            }
        }
    }
}

pub struct CascoError(
    pub casco::Error<Driver<casco::proc_macro2::TokenStream>, casco::proc_macro2::TokenStream>,
);

impl ToTokens for CascoError {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match &self.0 {
            casco::Error::Cascade(error) => match error {
                &casco::cascade::Error::MissingColon(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing colon.").to_tokens(tokens)
                }
                &casco::cascade::Error::MissingPropertyName(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing property name.")
                        .to_tokens(tokens)
                }
                &casco::cascade::Error::MissingPropertyValue(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing property value.")
                        .to_tokens(tokens)
                }
                &casco::cascade::Error::MissingRuleGroup(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing rule group.")
                        .to_tokens(tokens)
                }
                &casco::cascade::Error::MissingSelector(span) => {
                    CompileError::new(MultiSpan::single(span), "Missing selector.")
                        .to_tokens(tokens)
                }
            },
            casco::Error::Domain(error) => match error {
                &Error::UnexpectedRule(span, _) => {
                    CompileError::new(span, "Unexpected rule.").to_tokens(tokens)
                }
                &Error::UnexpectedToken(span) => {
                    CompileError::new(MultiSpan::single(span), "Unexpected token.")
                        .to_tokens(tokens)
                }
                &Error::UnrecognizedProperty(span) => {
                    CompileError::new(span, "Unrecognized property.").to_tokens(tokens)
                }
                Error::Parse(error) => match error {
                    &ParseError::Deprecated(span, message) => {
                        CompileError::new(span, message).to_tokens(tokens)
                    }
                    &ParseError::TooFewArguments(span) => {
                        CompileError::new(span, "Too few arguments.").to_tokens(tokens)
                    }
                    &ParseError::TooManyArguments(span) => {
                        CompileError::new(span, "Too many arguments.").to_tokens(tokens)
                    }
                    &ParseError::UnexpectedToken(span) => {
                        CompileError::new(MultiSpan::single(span), "Unexpected token.")
                            .to_tokens(tokens)
                    }
                    &ParseError::UnknownVariant(span) => {
                        CompileError::new(span, "Unknown variant.").to_tokens(tokens)
                    }
                    &ParseError::UnrecognizedUnit(span) => CompileError::new(
                        MultiSpan::single(span),
                        "Unrecognized CSS unit. Only `px` and `%` are supported at this moment.",
                    )
                    .to_tokens(tokens),
                },
            },
        }
    }
}
