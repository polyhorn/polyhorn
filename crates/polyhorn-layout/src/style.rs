use polyhorn_style::{AlignItems, Dimension, FlexDirection, JustifyContent, Position};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Insets<T> {
    pub top: T,
    pub trailing: T,
    pub bottom: T,
    pub leading: T,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Overflow {
    Visible,
    Scroll,
}

impl Default for Overflow {
    fn default() -> Self {
        Overflow::Visible
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Style {
    pub position: Position,
    pub flex_direction: FlexDirection,
    pub align_items: AlignItems,
    pub justify_content: JustifyContent,
    pub flex_basis: Dimension,
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub min_size: Size<Dimension>,
    pub size: Size<Dimension>,
    pub max_size: Size<Dimension>,
    pub padding: Insets<Dimension>,
    pub margin: Insets<Dimension>,
    pub overflow: Overflow,
}

impl Default for Style {
    fn default() -> Self {
        Style {
            position: Position::Relative,
            flex_basis: Dimension::Undefined,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_direction: FlexDirection::default(),
            align_items: AlignItems::default(),
            justify_content: JustifyContent::default(),
            padding: Insets::default(),
            margin: Insets::default(),
            min_size: Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
            size: Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
            max_size: Size {
                width: Dimension::auto(),
                height: Dimension::auto(),
            },
            overflow: Overflow::Visible,
        }
    }
}
