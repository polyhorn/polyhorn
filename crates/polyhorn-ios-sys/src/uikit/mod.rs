//! Bindings to UIKit.framework.
//!
//! ```objective-c
//! #import <UIKit/UIKit.h>
//! ```

mod application;
mod color;
mod font;
mod geometry;
mod image;
mod scroll_view;
mod status_bar;

pub use application::UIApplication;
pub use color::UIColor;
pub use font::UIFont;
pub use geometry::UIEdgeInsets;
pub use image::UIImage;
pub use scroll_view::UIScrollViewIndicatorStyle;
pub use status_bar::UIStatusBarStyle;
