use polyhorn_ios_sys::UIFont;

#[derive(Clone)]
pub struct Font(UIFont);

impl std::fmt::Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Font").finish()
    }
}

impl Font {
    pub fn system_font(size: f32) -> Font {
        Font(UIFont::system_font_of_size(size as _))
    }

    pub fn bold_system_font(size: f32) -> Font {
        Font(UIFont::bold_system_font_of_size(size as _))
    }
}

impl Default for Font {
    fn default() -> Self {
        Font::system_font(16.0)
    }
}

impl From<Font> for UIFont {
    fn from(font: Font) -> Self {
        font.0
    }
}
