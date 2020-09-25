use paste::paste;
use std::str::FromStr;

use super::Color;

macro_rules! color {
    ($hex:literal, $name:ident, $s:ident) => {
        paste! {
            #[doc = "Returns a CSS-compliant named color for `" $name "` in the sRGB color space."]
            fn $name() -> Self;
        }
    };
}

macro_rules! colors {
    ($s:ident) => {
        color!(0xF0F8FF, aliceblue, $s);
        color!(0xFAEBD7, antiquewhite, $s);
        color!(0x00FFFF, aqua, $s);
        color!(0x7FFFD4, aquamarine, $s);
        color!(0xF0FFFF, azure, $s);

        color!(0xF5F5DC, beige, $s);
        color!(0xFFE4C4, bisque, $s);
        color!(0x000000, black, $s);
        color!(0xFFEBCD, blanchedalmond, $s);
        color!(0x0000FF, blue, $s);
        color!(0x8A2BE2, blueviolet, $s);
        color!(0xA52A2A, brown, $s);
        color!(0xDEB887, burlywood, $s);

        color!(0x5F9EA0, cadetblue, $s);
        color!(0x7FFF00, chartreuse, $s);
        color!(0xD2691E, chocolate, $s);
        color!(0xFF7F50, coral, $s);
        color!(0x6495ED, cornflowerblue, $s);
        color!(0xFFF8DC, cornsilk, $s);
        color!(0xDC143C, crimson, $s);
        color!(0x00FFFF, cyan, $s);

        color!(0x00008B, darkblue, $s);
        color!(0x008B8B, darkcyan, $s);
        color!(0xB8860B, darkgoldenrod, $s);
        color!(0xA9A9A9, darkgray, $s);
        color!(0x006400, darkgreen, $s);
        color!(0xA9A9A9, darkgrey, $s);
        color!(0xBDB76B, darkkhaki, $s);
        color!(0x8B008B, darkmagenta, $s);
        color!(0x556B2F, darkolivegreen, $s);
        color!(0xFF8C00, darkorange, $s);
        color!(0x9932CC, darkorchid, $s);
        color!(0x8B0000, darkred, $s);
        color!(0xE9967A, darksalmon, $s);
        color!(0x8FBC8F, darkseagreen, $s);
        color!(0x483D8B, darkslateblue, $s);
        color!(0x2F4F4F, darkslategray, $s);
        color!(0x2F4F4F, darkslategrey, $s);
        color!(0x00CED1, darkturquoise, $s);
        color!(0x9400D3, darkviolet, $s);
        color!(0xFF1493, deeppink, $s);
        color!(0x00BFFF, deepskyblue, $s);
        color!(0x696969, dimgray, $s);
        color!(0x696969, dimgrey, $s);
        color!(0x1E90FF, dodgerblue, $s);

        color!(0xB22222, firebrick, $s);
        color!(0xFFFAF0, floralwhite, $s);
        color!(0x228B22, forestgreen, $s);
        color!(0xFF00FF, fuchsia, $s);

        color!(0xDCDCDC, gainsboro, $s);
        color!(0xF8F8FF, ghostwhite, $s);
        color!(0xFFD700, gold, $s);
        color!(0xDAA520, goldenrod, $s);
        color!(0x808080, gray, $s);
        color!(0x008000, green, $s);
        color!(0xADFF2F, greenyellow, $s);
        color!(0x808080, grey, $s);

        color!(0xF0FFF0, honeydew, $s);
        color!(0xFF69B4, hotpink, $s);

        color!(0xCD5C5C, indianred, $s);
        color!(0x4B0082, indigo, $s);
        color!(0xFFFFF0, ivory, $s);

        color!(0xF0E68C, khaki, $s);

        color!(0xE6E6FA, lavender, $s);
        color!(0xFFF0F5, lavenderblush, $s);
        color!(0x7CFC00, lawngreen, $s);
        color!(0xFFFACD, lemonchiffon, $s);
        color!(0xADD8E6, lightblue, $s);
        color!(0xF08080, lightcoral, $s);
        color!(0xE0FFFF, lightcyan, $s);
        color!(0xFAFAD2, lightgoldenrodyellow, $s);
        color!(0xD3D3D3, lightgray, $s);
        color!(0x90EE90, lightgreen, $s);
        color!(0xD3D3D3, lightgrey, $s);

        color!(0xFFB6C1, lightpink, $s);
        color!(0xFFA07A, lightsalmon, $s);
        color!(0x20B2AA, lightseagreen, $s);
        color!(0x87CEFA, lightskyblue, $s);
        color!(0x778899, lightslategray, $s);
        color!(0x778899, lightslategrey, $s);
        color!(0xB0C4DE, lightsteelblue, $s);
        color!(0xFFFFE0, lightyellow, $s);
        color!(0x00FF00, lime, $s);
        color!(0x32CD32, limegreen, $s);
        color!(0xFAF0E6, linen, $s);

        color!(0xFF00FF, magenta, $s);
        color!(0x800000, maroon, $s);
        color!(0x66CDAA, mediumaquamarine, $s);
        color!(0x0000CD, mediumblue, $s);
        color!(0xBA55D3, mediumorchid, $s);
        color!(0x9370DB, mediumpurple, $s);
        color!(0x3CB371, mediumseagreen, $s);
        color!(0x7B68EE, mediumslateblue, $s);
        color!(0x00FA9A, mediumspringgreen, $s);
        color!(0x48D1CC, mediumturquoise, $s);
        color!(0xC71585, mediumvioletred, $s);
        color!(0x191970, midnightblue, $s);
        color!(0xF5FFFA, mintcream, $s);
        color!(0xFFE4E1, mistyrose, $s);
        color!(0xFFE4B5, moccasin, $s);

        color!(0xFFDEAD, navajowhite, $s);
        color!(0x000080, navy, $s);

        color!(0xFDF5E6, oldlace, $s);
        color!(0x808000, olive, $s);
        color!(0x6B8E23, olivedrab, $s);
        color!(0xFFA500, orange, $s);
        color!(0xFF4500, orangered, $s);
        color!(0xDA70D6, orchid, $s);

        color!(0xEEE8AA, palegoldenrod, $s);
        color!(0x98FB98, palegreen, $s);
        color!(0xAFEEEE, paleturquoise, $s);
        color!(0xDB7093, palevioletred, $s);
        color!(0xFFEFD5, papayawhip, $s);
        color!(0xFFDAB9, peachpuff, $s);
        color!(0xCD853F, peru, $s);
        color!(0xFFC0CB, pink, $s);
        color!(0xDDA0DD, plum, $s);
        color!(0xB0E0E6, powderblue, $s);
        color!(0x800080, purple, $s);

        color!(0x663399, rebeccapurple, $s);
        color!(0xFF0000, red, $s);
        color!(0xBC8F8F, rosybrown, $s);
        color!(0x4169E1, royalblue, $s);

        color!(0x8B4513, saddlebrown, $s);
        color!(0xFA8072, salmon, $s);
        color!(0xF4A460, sandybrown, $s);
        color!(0x2E8B57, seagreen, $s);
        color!(0xFFF5EE, seashell, $s);
        color!(0xA0522D, sienna, $s);
        color!(0xC0C0C0, silver, $s);
        color!(0x87CEEB, skyblue, $s);
        color!(0x6A5ACD, slateblue, $s);
        color!(0x708090, slategray, $s);
        color!(0x708090, slategrey, $s);
        color!(0xFFFAFA, snow, $s);
        color!(0x00FF7F, springgreen, $s);
        color!(0x4682B4, steelblue, $s);
        color!(0xD2B48C, tan, $s);
        color!(0x008080, teal, $s);
        color!(0xD8BFD8, thistle, $s);
        color!(0xFF6347, tomato, $s);
        color!(0x40E0D0, turquoise, $s);
        color!(0xEE82EE, violet, $s);
        color!(0xF5DEB3, wheat, $s);
        color!(0xFFFFFF, white, $s);
        color!(0xF5F5F5, whitesmoke, $s);
        color!(0xFFFF00, yellow, $s);
        color!(0x9ACD32, yellowgreen, $s);
    };
}

/// Trait with a function for every CSS-compliant named color.
pub trait NamedColor: FromStr {
    colors!(none);
}

macro_rules! color {
    ($hex: literal, $name:ident, $s:ident) => {
        if $s == stringify!($name) {
            return Ok(Color::hex($hex));
        }
    };
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    UnknownColorName,
}

impl FromStr for Color {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        colors!(s);

        Err(Error::UnknownColorName)
    }
}

macro_rules! color {
    ($hex: literal, $name:ident, $s:ident) => {
        fn $name() -> Color {
            Color::hex($hex)
        }
    };
}

impl NamedColor for Color {
    colors!(none);
}
