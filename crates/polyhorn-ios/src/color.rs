use fsize::fsize;
use polyhorn_ios_sys::UIColor;

#[derive(Clone)]
pub struct Color(UIColor);

impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (red, green, blue, _) = self.0.get_components();

        f.debug_tuple("Color")
            .field(&red)
            .field(&green)
            .field(&blue)
            .finish()
    }
}

impl Color {
    pub fn from_rgba(red: u8, green: u8, blue: u8, alpha: f32) -> Color {
        Color(UIColor::new(
            red as fsize / 255.0,
            green as fsize / 255.0,
            blue as fsize / 255.0,
            alpha as fsize,
        ))
    }

    pub fn from_rgb(red: u8, green: u8, blue: u8) -> Color {
        Color::from_rgba(red, green, blue, 1.0)
    }

    pub fn from_hexa(hex: u32, alpha: f32) -> Color {
        Color::from_rgba(
            ((hex >> 16) & 0xff) as u8,
            ((hex >> 8) & 0xff) as u8,
            ((hex >> 0) & 0xff) as u8,
            alpha,
        )
    }

    pub fn from_hex(hex: u32) -> Color {
        Color::from_hexa(hex, 1.0)
    }

    pub fn transparent() -> Color {
        Color(UIColor::clear())
    }

    pub fn gray() -> Color {
        Color(UIColor::gray())
    }

    pub fn green() -> Color {
        Color(UIColor::green())
    }

    pub fn red() -> Color {
        Color(UIColor::red())
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::transparent()
    }
}

impl From<Color> for UIColor {
    fn from(color: Color) -> Self {
        color.0
    }
}
