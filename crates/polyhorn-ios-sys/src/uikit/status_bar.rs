/// The style of the device's status bar.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum UIStatusBarStyle {
    /// A dark status bar, intended for use on light backgrounds.
    Default = 0,

    /// A light status bar, intended for use on dark backgrounds.
    LightContent = 1,

    /// A dark status bar, intended for use on light backgrounds.
    DarkContent = 3,
}

impl Default for UIStatusBarStyle {
    fn default() -> Self {
        UIStatusBarStyle::DarkContent
    }
}
