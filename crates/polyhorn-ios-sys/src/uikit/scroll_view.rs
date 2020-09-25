/// The style of the scroll indicators. You can use these constants to set the
/// value of the `indicatorStyle` style.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum UIScrollViewIndicatorStyle {
    /// The default style of scroll indicator, which is black with a white
    /// border. This style is good against any content background.
    Default = 0,

    /// A style of indicator which is black and smaller than the default style.
    /// This style is good against a white content background.
    Black = 1,

    /// A style of indicator which is white and smaller than the default style.
    /// This style is good against a black content background.
    White = 2,
}

impl Default for UIScrollViewIndicatorStyle {
    fn default() -> Self {
        UIScrollViewIndicatorStyle::Default
    }
}
