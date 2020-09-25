//! Technical implementation of a subset of CSS Color Module Level 4 (WD).

use palette::rgb::{Rgb, Rgba};
use palette::{Alpha, ConvertInto, Hsl, Lab, Mix};

pub mod encoding;
mod named;

/// Nonlinear Display-P3.
pub type DisplayP3<T = f32> = Rgb<encoding::DisplayP3, T>;

/// Nonlinear Display-P3 with an alpha component.
pub type DisplayP3a<T = f32> = Rgba<encoding::DisplayP3, T>;

pub use palette::{Srgb, Srgba};

#[derive(Copy, Clone, Debug, PartialEq)]
enum ColorComponents {
    Transparent,

    /// sRGB is the default color space in CSS for colors specified with the
    /// `rgb(...)` or `rgba(...)` syntax. On iOS devices, it seems the default
    /// color space is (similar to) sRGB. As always, Apple is sparse on details.
    /// It apparently also is the default on Android devices. Note that is the
    /// default even for devices that support wide gamut: that's opt-in.
    StandardRGB(Srgb),

    /// Display P3 is another color space using the RGB model.
    DisplayP3(DisplayP3),
}

/// A color in a color-space with an associated alpha channel.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    components: ColorComponents,
    alpha: f32,
}

impl Color {
    /// This function returns a new translucent color in the sRGB color space
    /// with the given coordinates and alpha channel.
    pub fn rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
        Color {
            components: ColorComponents::StandardRGB(Srgb::new(
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
            )),
            alpha,
        }
    }

    /// This function returns a new opaque color in the sRGB color space with
    /// the given coordinates.
    pub fn rgb(red: u8, green: u8, blue: u8) -> Color {
        Self::rgba(red, green, blue, 1.0)
    }

    /// This function returns a new translucent color in the sRGB color space
    /// with the given hex-encoded coordinates.
    pub fn hexa(hex: u32, alpha: f32) -> Color {
        Self::rgba(
            ((hex >> 16) & 0xff) as u8,
            ((hex >> 8) & 0xff) as u8,
            ((hex >> 0) & 0xff) as u8,
            alpha,
        )
    }

    /// This function returns a new opaque color in the sRGB color space with
    /// the given hex-encoded coordinates.
    pub fn hex(hex: u32) -> Color {
        Self::hexa(hex, 1.0)
    }

    /// This function returns a new translucent color in the sRGB color space
    /// with the given coordinates after converting from the alternative HSL
    /// representation to RGB.
    pub fn hsla(hue: f32, saturation: f32, lightness: f32, alpha: f32) -> Color {
        Color {
            components: ColorComponents::StandardRGB(Hsl::new(hue, saturation, lightness).into()),
            alpha,
        }
    }

    /// This function returns a new opaque color in the sRGB color space with
    /// the given coordinates after converting from the alternative HSL
    /// representation to RGB.
    pub fn hsl(hue: f32, saturation: f32, lightness: f32) -> Color {
        Self::hsla(hue, saturation, lightness, 1.0)
    }

    /// This function returns a new translucent color in the Display-P3 color
    /// space with the given coordinates and alpha channel.
    pub fn display_p3_rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
        Color {
            components: ColorComponents::DisplayP3(DisplayP3::new(
                red as f32 / 255.0,
                green as f32 / 255.0,
                blue as f32 / 255.0,
            )),
            alpha,
        }
    }

    /// This functions returns a new opaque color in the Display-P3 color space
    /// with the given coordinates.
    pub fn display_p3_rgb(red: u8, green: u8, blue: u8) -> Color {
        Self::display_p3_rgba(red, green, blue, 1.0)
    }

    /// This function returns an unbiased transparent color. Specifically, when
    /// blending a non-transparent color with a transparent color, the resulting
    /// color will have the same color space and coordinate as the
    /// non-transparent color, and only the alpha channel will be blended.
    pub fn transparent() -> Color {
        Color {
            components: ColorComponents::Transparent,
            alpha: 0.0,
        }
    }

    /// This function returns the additional alpha-component of this color.
    pub fn alpha(&self) -> f32 {
        self.alpha
    }

    /// Blends the given color with another color using the given factor (between
    /// 0.0 and 1.0).
    pub fn mix(self, other: Color, factor: f32) -> Color {
        let factor = factor.max(0.0).min(1.0);

        Color {
            components: ColorComponents::StandardRGB(Srgb::from_linear(
                self.to_srgb()
                    .color
                    .into_linear()
                    .mix(&other.to_srgb().color.into_linear(), factor),
            )),
            alpha: self.alpha * factor + other.alpha * (1.0 - factor),
        }
    }

    /// This function converts the color to the Display-P3 color space (if
    /// necessary) and returns the result.
    pub fn to_display_p3(&self) -> DisplayP3a {
        Alpha {
            color: match self.components {
                ColorComponents::StandardRGB(rgb) => {
                    let lab: Lab = rgb.convert_into();
                    lab.convert_into()
                }
                ColorComponents::DisplayP3(rgb) => rgb,
                ColorComponents::Transparent => DisplayP3::new(0.0, 0.0, 0.0),
            },
            alpha: self.alpha,
        }
    }

    /// This function converts the color to the sRGB color space (if necessary)
    /// and returns the result.
    pub fn to_srgb(&self) -> Srgba {
        Alpha {
            color: match self.components {
                ColorComponents::StandardRGB(rgb) => rgb,
                ColorComponents::DisplayP3(rgb) => {
                    let lab: Lab = rgb.convert_into();
                    lab.convert_into()
                }
                ColorComponents::Transparent => Srgb::new(0.0, 0.0, 0.0),
            },
            alpha: self.alpha,
        }
    }

    /// Converts this color to sRGB and returns the resulting hex code.
    pub fn to_hex(&self) -> u32 {
        let srgb = self.to_srgb();

        (((srgb.red * 255.0) as u32) << 16)
            | (((srgb.green * 255.0) as u32) << 8)
            | (((srgb.blue * 255.0) as u32) << 0)
    }
}

pub use named::NamedColor;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srgb_to_display_p3() {
        let color = Color::rgb(255, 0, 0);
        let p3 = color.to_display_p3();

        assert!((p3.red - 234.0 / 255.0) < (1.0 / 255.0));
        assert!((p3.green - 51.0 / 255.0) < (1.0 / 255.0));
        assert!((p3.blue - 35.0 / 255.0) < (1.0 / 255.0));
    }

    #[test]
    fn test_display_p3_to_srgb() {
        let color = Color::display_p3_rgb(234, 51, 35);
        assert_eq!(color.to_srgb(), Srgba::new(1.0, 0.0, 0.0, 1.0));
    }

    #[test]
    fn test_named_color() {
        use NamedColor;

        let color = Color::coral();
        assert_eq!(color, Color::rgb(255, 127, 80));
    }

    #[test]
    fn test_gamma_correct_blending() {
        let blue = Color::hex(0x0073FF);
        let red = Color::hex(0xFF003D);
        let mix = blue.mix(red, 0.5);
        assert_eq!(mix.to_hex(), 0xBB52BF);
    }
}
