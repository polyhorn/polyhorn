use crate::styles::TextStyle;

/// Renders (rich) text to the screen.
pub struct Text {
    /// Controls the appearance and layout of this text. Note that the
    /// layout-related style properties are only honored if this text is
    /// contained within another view. They are not honored if this text is
    /// nested in another text.
    pub style: TextStyle,
}
