use casco::concrete::{Span, TokenStream};
use casco::stream::MultiSpan;

use super::{Diagnostic, Style, StyleBuilder};
use crate::color::{Color, NamedColor};
use crate::font::{FontFamily, FontSize, FontStyle, FontWeight, GenericFontFamily};
use crate::geometry::{ByEdge, Dimension, Size};
use crate::layout::{LayoutAxisX, LayoutAxisY, LayoutDirection};
use crate::macros::style::Driver;
use crate::styles::{
    Absolute, Align, Border, BorderStyle, FlexDirection, ImageStyle, Inherited, Justify, ObjectFit,
    Position, Relative, TextAlign, TextStyle, ViewStyle,
};

fn build(input: &str) -> (Style, Vec<Diagnostic<TokenStream>>) {
    let stream = casco::lexer::lex(input).unwrap();
    let mut driver = Driver::new();
    let stylesheet = casco::StyleSheet::parse(&mut driver, &stream.tokens).unwrap();
    let properties = stylesheet
        .items
        .into_iter()
        .filter_map(|item| match item {
            casco::Item::Property(property) => Some(property),
            _ => None,
        })
        .collect::<Vec<_>>();

    let builder = StyleBuilder::new();
    builder.build(&properties)
}

#[test]
fn test_unused_property_warning() {
    assert_eq!(
        build("tint-color: red; tint-color: green;"),
        (
            Style::Image(ImageStyle {
                tint_color: Some(Color::green()),
                ..Default::default()
            }),
            vec![Diagnostic::UnusedProperty(MultiSpan::single(Span::new(
                12, 15
            )))]
        )
    );
}

mod image {
    use super::*;

    #[test]
    fn test_object_fit() {
        assert_eq!(
            build("object-fit: contain;"),
            (
                Style::Image(ImageStyle {
                    object_fit: ObjectFit::Contain,
                    ..Default::default()
                }),
                vec![]
            )
        );
    }

    #[test]
    fn test_tint_color() {
        assert_eq!(
            build("tint-color: red;"),
            (
                Style::Image(ImageStyle {
                    tint_color: Some(Color::red()),
                    ..Default::default()
                }),
                vec![]
            )
        );
    }
}

mod text {
    use super::*;

    #[test]
    fn test_color() {
        assert_eq!(
            build("color: brown;"),
            (
                Style::Text(TextStyle {
                    color: Inherited::Specified(Color::brown()),
                    ..Default::default()
                }),
                vec![]
            )
        );
    }

    #[test]
    fn test_generic_font_family() {
        assert_eq!(
            build("font-family: monospace;"),
            (
                Style::Text(TextStyle {
                    font_family: Inherited::Specified(FontFamily::Generic(
                        GenericFontFamily::Monospace
                    )),
                    ..Default::default()
                }),
                vec![]
            )
        );
    }

    #[test]
    fn test_named_font_family() {
        assert_eq!(
            build("font-family: \"Helvetica \\\"Neue\\\"\";"),
            (
                Style::Text(TextStyle {
                    font_family: Inherited::Specified(FontFamily::Named(
                        "Helvetica \"Neue\"".to_owned()
                    )),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_named_font_weight() {
        assert_eq!(
            build("font-weight: extra-bold;"),
            (
                Style::Text(TextStyle {
                    font_weight: Inherited::Specified(FontWeight::ExtraBold),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_integer_font_weight() {
        assert_eq!(
            build("font-weight: 450;"),
            (
                Style::Text(TextStyle {
                    font_weight: Inherited::Specified(FontWeight::Number(450.0)),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_font_style() {
        assert_eq!(
            build("font-style: oblique;"),
            (
                Style::Text(TextStyle {
                    font_style: Inherited::Specified(FontStyle::Oblique),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_named_font_size() {
        assert_eq!(
            build("font-size: extra-extra-large;"),
            (
                Style::Text(TextStyle {
                    font_size: Inherited::Specified(FontSize::ExtraExtraLarge),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_pixel_font_size() {
        assert_eq!(
            build("font-size: 12px;"),
            (
                Style::Text(TextStyle {
                    font_size: Inherited::Specified(FontSize::Dimension(Dimension::Points(12.0))),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_text_align() {
        assert_eq!(
            build("text-align: right;"),
            (
                Style::Text(TextStyle {
                    text_align: Inherited::Specified(TextAlign::Right),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }
}

mod view {
    use super::*;

    #[test]
    fn test_position() {
        assert_eq!(
            build("position: absolute;"),
            (
                Style::View(ViewStyle {
                    position: Position::Absolute(Default::default()),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_top() {
        assert_eq!(
            build("position: absolute; top: 20px;"),
            (
                Style::View(ViewStyle {
                    position: Position::Absolute(Absolute {
                        distances: ByEdge {
                            vertical: LayoutAxisY {
                                top: Dimension::Points(20.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_right() {
        assert_eq!(
            build("position: absolute; right: 30px;"),
            (
                Style::View(ViewStyle {
                    position: Position::Absolute(Absolute {
                        distances: ByEdge {
                            horizontal: LayoutAxisX::DirectionIndependent {
                                left: Dimension::Undefined,
                                right: Dimension::Points(30.0),
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_bottom() {
        assert_eq!(
            build("position: absolute; bottom: 40px;"),
            (
                Style::View(ViewStyle {
                    position: Position::Absolute(Absolute {
                        distances: ByEdge {
                            vertical: LayoutAxisY {
                                bottom: Dimension::Points(40.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_left() {
        assert_eq!(
            build("position: absolute; left: 50px;"),
            (
                Style::View(ViewStyle {
                    position: Position::Absolute(Absolute {
                        distances: ByEdge {
                            horizontal: LayoutAxisX::DirectionIndependent {
                                left: Dimension::Points(50.0),
                                right: Dimension::Undefined
                            },
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_flex_basis() {
        assert_eq!(
            build("flex-basis: 20px;"),
            (
                Style::View(ViewStyle {
                    position: Position::Relative(Relative {
                        flex_basis: Dimension::Points(20.0),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_flex_grow() {
        assert_eq!(
            build("flex-grow: 2.0;"),
            (
                Style::View(ViewStyle {
                    position: Position::Relative(Relative {
                        flex_grow: 2.0,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_flex_shrink() {
        assert_eq!(
            build("flex-shrink: 0.5;"),
            (
                Style::View(ViewStyle {
                    position: Position::Relative(Relative {
                        flex_shrink: 0.5,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_direction() {
        assert_eq!(
            build("direction: rtl;"),
            (
                Style::View(ViewStyle {
                    direction: Inherited::Specified(LayoutDirection::RTL),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_height() {
        assert_eq!(
            build("height: 15.5px;"),
            (
                Style::View(ViewStyle {
                    size: Size::new(Dimension::Auto, Dimension::Points(15.5)),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_width() {
        assert_eq!(
            build("width: 25.5px;"),
            (
                Style::View(ViewStyle {
                    size: Size::new(Dimension::Points(25.5), Dimension::Auto),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_min_height() {
        assert_eq!(
            build("min-height: 15.5px;"),
            (
                Style::View(ViewStyle {
                    min_size: Size::new(Dimension::Auto, Dimension::Points(15.5)),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_min_width() {
        assert_eq!(
            build("min-width: 25.5px;"),
            (
                Style::View(ViewStyle {
                    min_size: Size::new(Dimension::Points(25.5), Dimension::Auto),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_max_height() {
        assert_eq!(
            build("max-height: 15.5px;"),
            (
                Style::View(ViewStyle {
                    max_size: Size::new(Dimension::Auto, Dimension::Points(15.5)),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_max_width() {
        assert_eq!(
            build("max-width: 25.5px;"),
            (
                Style::View(ViewStyle {
                    max_size: Size::new(Dimension::Points(25.5), Dimension::Auto),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_flex_direction() {
        assert_eq!(
            build("flex-direction: column-reverse;"),
            (
                Style::View(ViewStyle {
                    flex_direction: FlexDirection::ColumnReverse,
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_align_items() {
        assert_eq!(
            build("align-items: flex-end;"),
            (
                Style::View(ViewStyle {
                    align_items: Align::FlexEnd,
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_justify_content() {
        assert_eq!(
            build("justify-content: space-evenly;"),
            (
                Style::View(ViewStyle {
                    justify_content: Justify::SpaceEvenly,
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_margin_single() {
        assert_eq!(
            build("margin: 5px;"),
            (
                Style::View(ViewStyle {
                    margin: ByEdge {
                        horizontal: LayoutAxisX::independent(
                            Dimension::Points(5.0),
                            Dimension::Points(5.0)
                        ),
                        vertical: LayoutAxisY {
                            top: Dimension::Points(5.0),
                            bottom: Dimension::Points(5.0),
                        }
                    },
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_margin_double() {
        assert_eq!(
            build("margin: 5px 10px;"),
            (
                Style::View(ViewStyle {
                    margin: ByEdge {
                        horizontal: LayoutAxisX::independent(
                            Dimension::Points(10.0),
                            Dimension::Points(10.0)
                        ),
                        vertical: LayoutAxisY {
                            top: Dimension::Points(5.0),
                            bottom: Dimension::Points(5.0),
                        }
                    },
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_margin_quadruple() {
        assert_eq!(
            build("margin: 1px 2px 3px 4px;"),
            (
                Style::View(ViewStyle {
                    margin: ByEdge {
                        horizontal: LayoutAxisX::independent(
                            Dimension::Points(4.0),
                            Dimension::Points(2.0)
                        ),
                        vertical: LayoutAxisY {
                            top: Dimension::Points(1.0),
                            bottom: Dimension::Points(3.0),
                        }
                    },
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_border() {
        let border = Border {
            width: Dimension::Points(1.0),
            style: BorderStyle::Solid,
            color: Color::red(),
        };

        assert_eq!(
            build("border: 1px solid red;"),
            (
                Style::View(ViewStyle {
                    border: ByEdge {
                        horizontal: LayoutAxisX::DirectionIndependent {
                            left: border,
                            right: border
                        },
                        vertical: LayoutAxisY {
                            top: border,
                            bottom: border,
                        }
                    },
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_padding_single() {
        assert_eq!(
            build("padding: 5px;"),
            (
                Style::View(ViewStyle {
                    padding: ByEdge {
                        horizontal: LayoutAxisX::independent(
                            Dimension::Points(5.0),
                            Dimension::Points(5.0)
                        ),
                        vertical: LayoutAxisY {
                            top: Dimension::Points(5.0),
                            bottom: Dimension::Points(5.0),
                        }
                    },
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_padding_double() {
        assert_eq!(
            build("padding: 5px 10px;"),
            (
                Style::View(ViewStyle {
                    padding: ByEdge {
                        horizontal: LayoutAxisX::independent(
                            Dimension::Points(10.0),
                            Dimension::Points(10.0)
                        ),
                        vertical: LayoutAxisY {
                            top: Dimension::Points(5.0),
                            bottom: Dimension::Points(5.0),
                        }
                    },
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_padding_quadruple() {
        assert_eq!(
            build("padding: 1px 2px 3px 4px;"),
            (
                Style::View(ViewStyle {
                    padding: ByEdge {
                        horizontal: LayoutAxisX::independent(
                            Dimension::Points(4.0),
                            Dimension::Points(2.0)
                        ),
                        vertical: LayoutAxisY {
                            top: Dimension::Points(1.0),
                            bottom: Dimension::Points(3.0),
                        }
                    },
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_background_color() {
        assert_eq!(
            build("background-color: beige;"),
            (
                Style::View(ViewStyle {
                    background_color: Color::beige(),
                    ..Default::default()
                }),
                vec![]
            )
        )
    }

    #[test]
    fn test_opacity() {
        assert_eq!(
            build("opacity: 0.5;"),
            (
                Style::View(ViewStyle {
                    opacity: 0.5,
                    ..Default::default()
                }),
                vec![]
            )
        )
    }
}
