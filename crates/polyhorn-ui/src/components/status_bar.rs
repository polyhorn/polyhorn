/// Represents the style of the OS status bar.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum StatusBarStyle {
    /// Uses black text and symbols.
    DarkContent,

    /// Uses white text and symbols.
    LightContent,
}

impl Default for StatusBarStyle {
    fn default() -> Self {
        StatusBarStyle::DarkContent
    }
}

/// Controls the appearance of the system status bar on iOS and Android.
pub struct StatusBar {
    /// Controls the style (i.e. color of text and symbols) of the status bar.
    pub style: StatusBarStyle,
}
