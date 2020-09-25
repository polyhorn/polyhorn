use polyhorn_ios_sys::foundation::{
    NSAttributedString, NSAttributes, NSMutableParagraphStyle, NSTextAlignment,
};
use polyhorn_ui::color::{Color, NamedColor};
use polyhorn_ui::font::{Font, FontFamily, FontSize, FontStyle, FontWeight, GenericFontFamily};
use polyhorn_ui::styles::{Inherited, TextAlign, TextStyle};

use crate::raw::Convert;

/// Generates a new `NSAttributedString` for the given text with the given text
/// style.
pub fn attributed_string(text: &str, style: &TextStyle) -> NSAttributedString {
    let mut paragraph_style = NSMutableParagraphStyle::new();
    paragraph_style.set_alignment(match style.text_align {
        Inherited::Inherited => NSTextAlignment::Left,
        Inherited::Specified(specified) => match specified {
            TextAlign::Left => NSTextAlignment::Left,
            TextAlign::Center => NSTextAlignment::Center,
            TextAlign::Right => NSTextAlignment::Right,
        },
    });

    let font = Font {
        family: match style.font_family {
            Inherited::Inherited => FontFamily::Generic(GenericFontFamily::SansSerif),
            Inherited::Specified(family) => family,
        },
        weight: match style.font_weight {
            Inherited::Inherited => FontWeight::Normal,
            Inherited::Specified(weight) => weight,
        },
        style: match style.font_style {
            Inherited::Inherited => FontStyle::Normal,
            Inherited::Specified(style) => style,
        },
        size: match style.font_size {
            Inherited::Inherited => FontSize::Medium,
            Inherited::Specified(size) => size,
        },
    };

    let color = match style.color {
        Inherited::Inherited => Color::black(),
        Inherited::Specified(color) => color,
    }
    .convert();

    let style = paragraph_style.into();

    NSAttributedString::with_attributes(
        &text,
        NSAttributes {
            font: font.convert(),
            foreground_color: color,
            paragraph_style: style,
        },
    )
}
