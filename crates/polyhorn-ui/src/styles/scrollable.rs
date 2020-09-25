use super::ViewStyle;
use crate::geometry::{ByEdge, Dimension};

/// Color of the scroll bars of a Scrollable.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScrollbarColor {
    /// This variant lets the OS decide which color to use. The OS may look at
    /// the appearance mode chosen by the user (i.e. light vs. dark mode) and/or
    /// at the content that is shown in the scrollable (e.g. to determine which
    /// color would have the highest contrast with respect to the content
    /// underlying).
    Auto,

    /// This is a dark scroll bar. Use this only if the underlying content is
    /// light.
    Dark,

    /// This is a light scroll bar. Use this only if the underlying content is
    /// dark.
    Light,
}

impl Default for ScrollbarColor {
    fn default() -> Self {
        ScrollbarColor::Auto
    }
}

/// Controls the appearance of a Scrollable.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ScrollableStyle {
    /// Controls the distance between the scrollable content and each edge of a
    /// rectangle.
    pub scroll_padding: ByEdge<Dimension<f32>>,

    /// This field controls the appearance of scroll bars used in this
    /// Scrollable. When set to Default, the scroll bar's appearance is
    /// automatically updated when the OS indicates that the user has changed
    /// its appearance mode, or when the OS chooses its own style depending on
    /// the brightness of underlying content and recommends a different style
    /// than is currently shown.
    pub scrollbar_color: ScrollbarColor,

    /// Controls the distance between the scrollbars and each edge of a
    /// rectangle.
    pub scrollbar_padding: ByEdge<Dimension<f32>>,
}

/// This is a union style of the Scrollable and View styles.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ScrollableViewStyle {
    /// This style contains the properties that are only applicable to
    /// Scrollables.
    pub scrollable: ScrollableStyle,

    /// This style contains the properties that are only applicable to
    /// Views.
    pub view: ViewStyle,
}

impl From<ScrollableStyle> for ScrollableViewStyle {
    fn from(style: ScrollableStyle) -> Self {
        ScrollableViewStyle {
            scrollable: style,
            ..Default::default()
        }
    }
}

impl From<ViewStyle> for ScrollableViewStyle {
    fn from(style: ViewStyle) -> Self {
        ScrollableViewStyle {
            view: style,
            ..Default::default()
        }
    }
}
