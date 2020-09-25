use casco::stream::{MultiSpan, TokenStream};
use derivative::Derivative;
use std::fmt::Debug;

use super::{PositionType, Property, PropertyValue, StyleCompound, StyleKind};
use crate::color::Color;
use crate::font::{FontFamily, FontSize, FontStyle, FontWeight};
use crate::geometry::{ByEdge, Size};
use crate::layout::{LayoutAxisX, LayoutAxisY, LayoutDirection};
use crate::styles::{
    Absolute, ImageStyle, ImageViewStyle, Inherited, ObjectFit, Position, Relative,
    ScrollableStyle, ScrollableViewStyle, TextAlign, TextStyle, ViewStyle,
};

/// Controls the appearance of a built-in component.
#[derive(Clone, Debug, PartialEq)]
pub enum Style {
    /// Controls the appearance of an Image.
    Image(ImageStyle),

    /// This is a union style of the Image and View styles.
    ImageView(ImageViewStyle),

    /// Controls the appearance of a Scrollable.
    Scrollable(ScrollableStyle),

    /// This is a union style of the Scrollable and View styles.
    ScrollableView(ScrollableViewStyle),

    /// Controls the appearance of a Text.
    Text(TextStyle<String>),

    /// Controls the appearance of a View.
    View(ViewStyle),

    /// Empty style.
    Unit,
}

/// This is a diagnostic that is emitted while building a style.
#[derive(Derivative)]
#[derivative(Debug, PartialEq(bound = "S::Span: PartialEq"))]
pub enum Diagnostic<S>
where
    S: TokenStream,
{
    /// This diagnostic is emitted when a property is overridden by another
    /// property.
    UnusedProperty(MultiSpan<S>),
}

/// This is a structure that is used to build the style from a series of
/// properties.
pub struct StyleBuilder<S>
where
    S: TokenStream,
{
    diagnostics: Vec<Diagnostic<S>>,
}

struct Tracked<T, S>
where
    S: TokenStream,
{
    value: T,
    previous_span: Option<MultiSpan<S>>,
}

impl<S> StyleBuilder<S>
where
    S: TokenStream,
{
    /// Returns a new style builder.
    pub fn new() -> StyleBuilder<S> {
        StyleBuilder {
            diagnostics: vec![],
        }
    }

    fn track<T>(&mut self) -> Tracked<T, S>
    where
        T: Default,
    {
        Tracked {
            value: T::default(),
            previous_span: None,
        }
    }

    fn track_with<T>(&mut self, value: T) -> Tracked<T, S> {
        Tracked {
            value,
            previous_span: None,
        }
    }

    fn replace<T>(&mut self, tracker: &mut Tracked<T, S>, value: T, span: MultiSpan<S>) {
        if let Some(span) = tracker.previous_span {
            self.diagnostics.push(Diagnostic::UnusedProperty(span));
        }

        tracker.value = value;
        tracker.previous_span = Some(span);
    }

    fn build_image_style(&mut self, properties: &[Property<S>]) -> ImageStyle {
        let mut object_fit = self.track::<ObjectFit>();
        let mut tint_color = self.track::<Option<Color>>();

        for property in properties {
            match &property.value {
                PropertyValue::ObjectFit(fit) => {
                    self.replace(&mut object_fit, *fit, property.value_span);
                }
                PropertyValue::TintColor(color) => {
                    self.replace(&mut tint_color, Some(*color), property.value_span);
                }
                _ => {}
            }
        }

        ImageStyle {
            object_fit: object_fit.value,
            tint_color: tint_color.value,
        }
    }

    fn build_view_style(&mut self, properties: &[Property<S>]) -> ViewStyle {
        let mut position_type = self.track_with(PositionType::Relative);
        let mut top = self.track();
        let mut right = self.track();
        let mut bottom = self.track();
        let mut left = self.track();
        let mut flex_basis = self.track_with(Relative::default().flex_basis);
        let mut flex_grow = self.track_with(Relative::default().flex_grow);
        let mut flex_shrink = self.track_with(Relative::default().flex_shrink);
        let mut direction = self.track::<Inherited<LayoutDirection>>();
        let mut height = self.track_with(ViewStyle::default().size.height);
        let mut width = self.track_with(ViewStyle::default().size.width);
        let mut min_height = self.track_with(ViewStyle::default().min_size.height);
        let mut min_width = self.track_with(ViewStyle::default().min_size.width);
        let mut max_height = self.track_with(ViewStyle::default().max_size.height);
        let mut max_width = self.track_with(ViewStyle::default().max_size.width);
        let mut flex_direction = self.track_with(ViewStyle::default().flex_direction);
        let mut align_items = self.track_with(ViewStyle::default().align_items);
        let mut justify_content = self.track_with(ViewStyle::default().justify_content);
        let mut margin = self.track_with(ViewStyle::default().margin);
        let mut border = self.track_with(ViewStyle::default().border);
        let mut border_radius = self.track_with(ViewStyle::default().border_radius);
        let mut padding = self.track_with(ViewStyle::default().padding);
        let mut background_color = self.track_with(ViewStyle::default().background_color);
        let mut opacity = self.track_with(ViewStyle::default().opacity);
        let mut transform = self.track_with(ViewStyle::default().transform);
        let mut overflow = self.track_with(ViewStyle::default().overflow);
        let mut visibility = self.track_with(ViewStyle::default().visibility);

        for property in properties {
            match &property.value {
                PropertyValue::Position(value) => {
                    self.replace(&mut position_type, *value, property.value_span)
                }
                PropertyValue::Top(value) => self.replace(&mut top, *value, property.value_span),
                PropertyValue::Right(value) => {
                    self.replace(&mut right, *value, property.value_span)
                }
                PropertyValue::Bottom(value) => {
                    self.replace(&mut bottom, *value, property.value_span)
                }
                PropertyValue::Left(value) => self.replace(&mut left, *value, property.value_span),
                PropertyValue::FlexBasis(value) => {
                    self.replace(&mut flex_basis, *value, property.value_span)
                }
                PropertyValue::FlexGrow(value) => {
                    self.replace(&mut flex_grow, *value, property.value_span)
                }
                PropertyValue::FlexShrink(value) => {
                    self.replace(&mut flex_shrink, *value, property.value_span)
                }
                PropertyValue::Direction(value) => {
                    self.replace(&mut direction, *value, property.value_span)
                }
                PropertyValue::Height(value) => {
                    self.replace(&mut height, *value, property.value_span)
                }
                PropertyValue::Width(value) => {
                    self.replace(&mut width, *value, property.value_span)
                }
                PropertyValue::MinHeight(value) => {
                    self.replace(&mut min_height, *value, property.value_span)
                }
                PropertyValue::MinWidth(value) => {
                    self.replace(&mut min_width, *value, property.value_span)
                }
                PropertyValue::MaxHeight(value) => {
                    self.replace(&mut max_height, *value, property.value_span)
                }
                PropertyValue::MaxWidth(value) => {
                    self.replace(&mut max_width, *value, property.value_span)
                }
                PropertyValue::FlexDirection(value) => {
                    self.replace(&mut flex_direction, *value, property.value_span)
                }
                PropertyValue::AlignItems(value) => {
                    self.replace(&mut align_items, *value, property.value_span)
                }
                PropertyValue::JustifyContent(value) => {
                    self.replace(&mut justify_content, *value, property.value_span)
                }
                PropertyValue::Margin(value) => {
                    self.replace(&mut margin, *value, property.value_span)
                }
                PropertyValue::Border(value) => {
                    self.replace(&mut border, *value, property.value_span)
                }
                PropertyValue::BorderRadius(value) => {
                    self.replace(&mut border_radius, *value, property.value_span)
                }
                PropertyValue::Padding(value) => {
                    self.replace(&mut padding, *value, property.value_span)
                }
                PropertyValue::BackgroundColor(value) => {
                    self.replace(&mut background_color, *value, property.value_span)
                }
                PropertyValue::Opacity(value) => {
                    self.replace(&mut opacity, *value, property.value_span)
                }
                PropertyValue::Transform(value) => {
                    self.replace(&mut transform, *value, property.value_span)
                }
                PropertyValue::Overflow(value) => {
                    self.replace(&mut overflow, *value, property.value_span)
                }
                PropertyValue::Visibility(value) => {
                    self.replace(&mut visibility, *value, property.value_span)
                }
                _ => {}
            }
        }

        let position = match position_type.value {
            PositionType::Absolute => Position::Absolute({
                Absolute {
                    distances: ByEdge {
                        horizontal: LayoutAxisX::DirectionIndependent {
                            left: left.value,
                            right: right.value,
                        },
                        vertical: LayoutAxisY {
                            top: top.value,
                            bottom: bottom.value,
                        },
                    },
                }
            }),
            PositionType::Relative => Position::Relative({
                Relative {
                    flex_basis: flex_basis.value,
                    flex_grow: flex_grow.value,
                    flex_shrink: flex_shrink.value,
                }
            }),
        };

        ViewStyle {
            position,
            direction: direction.value,
            size: Size::new(width.value, height.value),
            min_size: Size::new(min_width.value, min_height.value),
            max_size: Size::new(max_width.value, max_height.value),
            flex_direction: flex_direction.value,
            align_items: align_items.value,
            justify_content: justify_content.value,
            margin: margin.value,
            border: border.value,
            border_radius: border_radius.value,
            padding: padding.value,
            background_color: background_color.value,
            opacity: opacity.value,
            transform: transform.value,
            overflow: overflow.value,
            visibility: visibility.value,
        }
    }

    fn build_text_style(&mut self, properties: &[Property<S>]) -> TextStyle<String> {
        let mut color = self.track::<Inherited<Color>>();
        let mut font_family = self.track::<Inherited<FontFamily<String>>>();
        let mut font_weight = self.track::<Inherited<FontWeight>>();
        let mut font_style = self.track::<Inherited<FontStyle>>();
        let mut font_size = self.track::<Inherited<FontSize>>();
        let mut text_align = self.track::<Inherited<TextAlign>>();

        for property in properties {
            match &property.value {
                PropertyValue::Color(value) => {
                    self.replace(&mut color, *value, property.value_span)
                }
                PropertyValue::FontFamily(value) => {
                    self.replace(&mut font_family, value.clone(), property.value_span)
                }
                PropertyValue::FontWeight(value) => {
                    self.replace(&mut font_weight, *value, property.value_span)
                }
                PropertyValue::FontStyle(value) => {
                    self.replace(&mut font_style, *value, property.value_span)
                }
                PropertyValue::FontSize(value) => {
                    self.replace(&mut font_size, *value, property.value_span)
                }
                PropertyValue::TextAlign(value) => {
                    self.replace(&mut text_align, *value, property.value_span)
                }
                _ => {}
            }
        }

        TextStyle {
            color: color.value,
            font_family: font_family.value,
            font_weight: font_weight.value,
            font_style: font_style.value,
            font_size: font_size.value,
            text_align: text_align.value,
        }
    }

    /// Builds a style from the given set of properties.
    pub fn build(mut self, properties: &[Property<S>]) -> (Style, Vec<Diagnostic<S>>) {
        let kinds = properties.iter().map(|prop| StyleKind::infer(prop));
        let compound = match StyleCompound::infer(kinds) {
            Some(compound) => compound,
            // TODO: this should emit a diagnostic.
            None => return (Style::Unit, vec![]),
        };

        let style = match compound {
            StyleCompound::Image => Style::Image(self.build_image_style(properties)),
            StyleCompound::ImageView => Style::ImageView(ImageViewStyle {
                image: self.build_image_style(properties),
                view: self.build_view_style(properties),
            }),
            StyleCompound::Text => Style::Text(self.build_text_style(properties)),
            StyleCompound::View => Style::View(self.build_view_style(properties)),
            _ => unimplemented!(),
        };

        (style, self.diagnostics)
    }
}
