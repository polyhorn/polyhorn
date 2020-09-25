//! Imperative handles to reactive components.

mod image;
mod scrollable;
mod text;
mod view;

pub use image::ImageHandle;
pub use scrollable::ScrollableHandle;
pub use text::TextHandle;
pub use view::ViewHandle;

/// This trait is implemented by types that have an imperative handle.
pub trait Imperative {
    /// This is the handle of this imperative type.
    type Handle;
}
