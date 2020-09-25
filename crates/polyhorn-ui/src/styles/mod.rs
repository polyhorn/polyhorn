//! Styles for each reactive component.

mod flex;
mod image;
mod position;
mod scrollable;
mod text;
mod transform;
mod view;

pub use flex::{Align, FlexDirection, Justify};
pub use image::{ImageStyle, ImageViewStyle, ObjectFit};
pub use position::{Absolute, Position, Relative};
pub use scrollable::{ScrollableStyle, ScrollableViewStyle, ScrollbarColor};
pub use text::{TextAlign, TextStyle};
pub use transform::{Transform, TransformBuilder};
pub use view::{Border, BorderStyle, Overflow, ViewStyle, Visibility};

/// Represents a property that can optionally be inherited from a parent
/// element.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Inherited<T> {
    /// If inherited, the value of this property is taken from the parent
    /// element (transitively). If none of the parent elements have specified
    /// a value for this property, the default value is used.
    Inherited,

    /// If specified, this overrides any value from a parent element
    /// (transitively) for the same property.
    Specified(T),
}

impl<T> Default for Inherited<T> {
    fn default() -> Self {
        Inherited::Inherited
    }
}

impl<T> std::str::FromStr for Inherited<T>
where
    T: std::str::FromStr,
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inherit" => Ok(Inherited::Inherited),
            s => T::from_str(s)
                .map(|value| Inherited::Specified(value))
                .map_err(|_| ()),
        }
    }
}
