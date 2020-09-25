//! Bindings to CoreGraphics.framework.
//!
//! ```objective-c
//! #import <CoreGraphics/CoreGraphics.h>
//! ```

mod base;
mod geometry;

pub use base::CGFloat;
pub use geometry::{CGPoint, CGRect, CGSize};
