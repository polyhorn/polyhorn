//! Technical implementation of a subset of CSS Fonts Module Level 4 (WD).

use std::str::FromStr;
use strum_macros::EnumString;

use crate::geometry::Dimension;

/// Controls the thickness of outlines rendered to a text element.
#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
pub enum FontWeight {
    /// Accepts a custom normalized font weight scaled between 0 and 1.0
    #[strum(disabled)]
    Number(f32),

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 100.
    #[strum(serialize = "thin")]
    Thin,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 200. This weight is also known as ultra light.
    #[strum(serialize = "extra-light")]
    ExtraLight,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 300.
    #[strum(serialize = "light")]
    Light,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 400. This is the default font weight.
    #[strum(serialize = "normal")]
    Normal,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 500.
    #[strum(serialize = "medium")]
    Medium,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 600. This weight is also known as demi bold.
    #[strum(serialize = "semi-bold")]
    SemiBold,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 700.
    #[strum(serialize = "bold")]
    Bold,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 800. This weight is also known as ultra bold.
    #[strum(serialize = "extra-bold")]
    ExtraBold,

    /// This corresponds to the CSS font weight with the same name and has a
    /// numerical value of 900. This weight is also known as heavy.
    #[strum(serialize = "black")]
    Black,
}

impl Default for FontWeight {
    fn default() -> Self {
        FontWeight::Normal
    }
}

/// Polyhorn provides a few generic font families that can be used to get a
/// system-provided typeface in a particular category that looks best on a
/// user's display. The default generic font family is SansSerif.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum GenericFontFamily {
    /// This generic font style has serifs attached to the glyphs. Serifs are
    /// those pointy spikes at the extremes of (mostly) uppercase letters. This
    /// style is commonly used for newspaper headings.
    #[strum(serialize = "serif")]
    Serif,

    /// This generic font style has no serifs. This type of font is commonly
    /// used for the main text.
    #[strum(serialize = "sans-serif")]
    SansSerif,

    /// This generic font style has glyphs of equal width. This type of font is
    /// commonly used in code editors where each letter and symbol should have
    /// equal width.
    #[strum(serialize = "monospace")]
    Monospace,
}

impl Default for GenericFontFamily {
    fn default() -> Self {
        GenericFontFamily::SansSerif
    }
}

/// Controls the font family that produces the glyphs that are drawn for a text.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FontFamily<S = &'static str> {
    /// This is a system-dependent font family in the given generic category.
    Generic(GenericFontFamily),

    /// This is a specific font family referred to by its PostScript name.
    Named(S),
}

impl<S> FromStr for FontFamily<S> {
    type Err = <GenericFontFamily as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FontFamily::Generic(GenericFontFamily::from_str(s)?))
    }
}

/// Most fonts come in one to three styles: a normal style and one or two
/// slanted styles. The normal font style is the default font style used for
/// text. The oblique font style (if present) is a slanted version of the same
/// source glyphs. An italic font style is made from a separate set of source
/// glyphs.
///
/// Most fonts do not offer both oblique and italic font styles. Oblique fonts
/// are an inexpensive way to get a slanted version of a normal font, whereas
/// italic fonts might cost as much time to make as the original version.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum FontStyle {
    /// The normal font style is the default font style for text.
    #[strum(serialize = "normal")]
    Normal,

    /// The oblique font style is a slanted version from the same source glyphs.
    #[strum(serialize = "oblique")]
    Oblique,

    /// An italic font style is made from a separate set of source glyphs.
    #[strum(serialize = "italic")]
    Italic,
}

impl Default for FontStyle {
    fn default() -> Self {
        FontStyle::Normal
    }
}

/// Controls the size of text that is rendered to the screen.
#[derive(Copy, Clone, Debug, PartialEq, EnumString)]
pub enum FontSize {
    /// This font size is 3/5th of the medium font size.
    #[strum(serialize = "extra-extra-small")]
    ExtraExtraSmall,

    /// This font size is 3/4th of the medium font size.
    #[strum(serialize = "extra-small")]
    ExtraSmall,

    /// This font size is 8/9th of the medium font size.
    #[strum(serialize = "small")]
    Small,

    /// This is the default font size.
    #[strum(serialize = "medium")]
    Medium,

    /// This font size is 1.2x the medium font size.
    #[strum(serialize = "large")]
    Large,

    /// This font size is 1.5x the medium font size.
    #[strum(serialize = "extra-large")]
    ExtraLarge,

    /// This font size is 2x the medium font size.
    #[strum(serialize = "extra-extra-large")]
    ExtraExtraLarge,

    /// This font size is 3x the medium font size.
    #[strum(serialize = "extra-extra-extra-large")]
    ExtraExtraExtraLarge,

    /// This is a dimension (e.g. pixels or percentage). If the dimension
    /// resolves to either `undefined` or `auto`, the font size will be 0. If
    /// the dimension is a percentage, it will be relative to the parent text
    /// element's font size (not its width).
    #[strum(disabled)]
    Dimension(Dimension<f32>),
}

/// Controls the rendering of text to screen.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Font {
    /// Controls the font family that is used to render text to screen.
    pub family: FontFamily,

    /// Controls the font weight that is used to render text to screen.
    pub weight: FontWeight,

    /// Controls the font style that is used to render text to screen.
    pub style: FontStyle,

    /// Controls the font size that is used to render text to screen.
    pub size: FontSize,
}
