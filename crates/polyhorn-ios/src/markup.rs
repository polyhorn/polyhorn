use polyhorn_ios_sys::foundation::{
    NSAttributedString, NSAttributes, NSMutableParagraphStyle, NSTextAlignment,
};

use crate::{TextAlignment, TextStyle};

pub fn attributed_string(text: &str, style: &TextStyle) -> NSAttributedString {
    let mut paragraph_style = NSMutableParagraphStyle::new();
    paragraph_style.set_alignment(match style.alignment {
        TextAlignment::Left => NSTextAlignment::Left,
        TextAlignment::Center => NSTextAlignment::Center,
        TextAlignment::Right => NSTextAlignment::Right,
    });

    NSAttributedString::with_attributes(
        &text,
        NSAttributes {
            font: style.font.clone().unwrap_or_default().into(),
            foreground_color: style.color.clone().unwrap_or_default().into(),
            paragraph_style: paragraph_style.into(),
        },
    )
}
