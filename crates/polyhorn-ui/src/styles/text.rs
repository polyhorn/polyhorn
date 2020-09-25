use strum_macros::EnumString;

use super::Inherited;
use crate::color::Color;
use crate::font::{FontFamily, FontSize, FontStyle, FontWeight};

/// Controls the alignment of text in a container that is larger than the text
/// itself.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum TextAlign {
    /// Text is aligned to the left edge of the container.
    #[strum(serialize = "left")]
    Left,

    /// Text is aligned in the center of the container.
    #[strum(serialize = "center")]
    Center,

    /// Text is aligned to the right edge of the container.
    #[strum(serialize = "right")]
    Right,
}

/// Controls the appearance of a Text.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct TextStyle<S = &'static str> {
    /// This is the color that will be used to fill the text outlines. If not
    /// present, the Text component will inherit the text color of its parent.
    /// If the parent does not have a color, the default `Color::canvastext()`
    /// system color will be used. Note that the concrete value of this color
    /// is system-dependent and can vary depending on the user's appearance mode
    /// (i.e. light vs. dark mode).
    pub color: Inherited<Color>,

    /// This is the font family that will be used to render the text outlines.
    /// If not present, the Text component will inherit its font family from its
    /// parent. If the parent does not have a font family, the default
    /// `FontFamily::Generic(GenericFontFamily::SansSerif)` will be used. Note
    /// that the concrete value of this font family is system-dependent and can
    /// vary depending on the user's preferred fonts.
    pub font_family: Inherited<FontFamily<S>>,

    /// This is the font weight that will be used to render the text outlines.
    /// If not present, the Text component will inherit its font weight from its
    /// parent. If the parent does not have a font weight, the default
    /// `FontWeight::Normal` (400) will be used.
    pub font_weight: Inherited<FontWeight>,

    /// This is the font style that will be used to render the text outlines. If
    /// not present, the Text component will inherit its font style from its
    /// parent. If the parent does not have a font style, the default
    /// `FontStyle::Normal` will be used.
    pub font_style: Inherited<FontStyle>,

    /// This is the font size that will be used to render the text. If not
    /// present, the Text component will inherit its font size from its parent.
    /// If the parent does not have a font size, the default `FontSize:Medium`
    /// will be used. Note that the concrete value of this font size is
    /// system-dependent and can vary depending on a user's preferred font size.
    pub font_size: Inherited<FontSize>,

    /// Controls the alignment of text when it is rendered to a container that
    /// is larger than the rendered text.
    pub text_align: Inherited<TextAlign>,
}
