//! Macros, types and traits that implement UI-related hooks for Polyhorn.

mod safe_area_insets;

pub use safe_area_insets::{SafeAreaInsets, UseSafeAreaInsets};

#[doc(inline)]
pub use crate::use_safe_area_insets;
