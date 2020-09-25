use std::fmt::Debug;

use super::{Style, Transitions};

pub trait Variants: Copy + Debug + Eq + 'static {
    /// This should return the variant that is used initially.
    fn initial() -> Self;

    /// This should return the variant that is used when the pointer is pressing
    /// on the view.
    fn press() -> Option<Self> {
        None
    }

    /// This should return the variant that is used when the view is unmounting
    /// and part of an `AnimatePresence` component.
    fn exit() -> Option<Self> {
        None
    }

    /// This should return the style that is used to render a view with the given
    /// variant.
    fn style(&self) -> Style;

    /// This should return the transition that is used to animate between the
    /// given variants.
    fn transitions(from: &Self, to: &Self) -> Transitions;
}
