use polyhorn_core::UseContext;
use std::ops::Deref;

use crate::geometry::ByEdge;
use crate::layout::{LayoutAxisX, LayoutAxisY};

/// Immutable structure that contains the layout direction independent insets
/// of the safe area of a view with respect to each edge of its rectangle. All
/// insets are positive numbers.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SafeAreaInsets(ByEdge<f32>);

impl SafeAreaInsets {
    /// Returns a new safe area insets structure with the given values in
    /// clockwise order starting with the top edge. The insets are layout
    /// direction independent.
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> SafeAreaInsets {
        SafeAreaInsets(ByEdge {
            horizontal: LayoutAxisX::DirectionIndependent { left, right },
            vertical: LayoutAxisY { top, bottom },
        })
    }
}

impl Deref for SafeAreaInsets {
    type Target = ByEdge<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Hook that is implemented by any type that can provide its safe area insets.
pub trait UseSafeAreaInsets {
    /// This function should returns the safe area insets of a view.
    fn use_safe_area_insets(&mut self) -> SafeAreaInsets;
}

impl<T> UseSafeAreaInsets for T
where
    T: UseContext,
{
    fn use_safe_area_insets(&mut self) -> SafeAreaInsets {
        self.use_context()
            .and_then(|context| context.to_owned())
            .unwrap_or(SafeAreaInsets(Default::default()))
    }
}

/// Hook that returns the safe area insets of a view.
#[macro_export]
macro_rules! use_safe_area_insets {
    ($manager:expr) => {
        $crate::hooks::UseSafeAreaInsets::use_safe_area_insets($manager)
    };
}
