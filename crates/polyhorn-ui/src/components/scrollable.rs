use polyhorn_core::Reference;

use crate::handles::ScrollableHandle;
use crate::styles::ScrollableViewStyle;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ScrollDirection {
    Both,
    Horizontal,
    Vertical,
}

/// Implements scroll gestures and bars to accommodate layouts that exceed
/// screen or container dimensions.
#[derive(Clone)]
pub struct Scrollable {
    /// Controls the appearance of this Scrollable.
    pub style: ScrollableViewStyle,

    /// Controls the direction(s) in which this Scrollable can be scrolled.
    pub direction: ScrollDirection,

    /// Replaces the handle within this reference with a handle to this
    /// Scrollable that can be used to access its API in imperative code.
    pub reference: Option<Reference<Box<dyn ScrollableHandle>>>,
}

impl Default for Scrollable {
    fn default() -> Self {
        Scrollable {
            style: ScrollableViewStyle::default(),
            direction: ScrollDirection::Vertical,
            reference: None,
        }
    }
}
