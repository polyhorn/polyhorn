use crate::coregraphics::CGFloat;

/// The inset distances for views.
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct UIEdgeInsets {
    /// The top edge inset value.
    pub top: CGFloat,

    /// The left edge inset value.
    pub left: CGFloat,

    /// The bottom edge inset value.
    pub bottom: CGFloat,

    /// The right edge inset value.
    pub right: CGFloat,
}
