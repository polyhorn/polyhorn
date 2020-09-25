use strum_macros::EnumString;

/// Controls the direction in which items are layed out.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum FlexDirection {
    /// Items are layed out vertically from top to bottom.
    #[strum(serialize = "column")]
    Column,

    /// Items are layed out vertically from bottom to top.
    #[strum(serialize = "column-reverse")]
    ColumnReverse,

    /// Items are layed out horizontally in the layout direction (either left to
    /// right or right to left).
    #[strum(serialize = "row")]
    Row,

    /// Items are layed out horizontally in the reverse layout direction (either
    /// right to left or left to right).
    #[strum(serialize = "row-reverse")]
    RowReverse,
}

/// Controls how items are aligned along the main axis of a flexbox.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum Align {
    /// Items are aligned at the start of the cross axis.
    #[strum(serialize = "flex-start")]
    FlexStart,

    /// Items are aligned at the center along the main axis.
    #[strum(serialize = "center")]
    Center,

    /// Items are aligned at the end of the cross axis.
    #[strum(serialize = "flex-end")]
    FlexEnd,

    /// Items are stretched between the start and end of the cross axis.
    #[strum(serialize = "stretch")]
    Stretch,

    /// Items are spaced evenly between, the first item starts at the start of
    /// the cross axis and the last item ends at the end of the cross axis.
    #[strum(serialize = "space-between")]
    SpaceBetween,

    /// Items are spaced evenly around, the first item starts after the start of
    /// the cross axis and the last item ends before the end of the cross axis.
    #[strum(serialize = "space-around")]
    SpaceAround,
}

/// Controls how content is justified along the cross axis of a flexbox.
#[derive(Copy, Clone, Debug, Eq, PartialEq, EnumString)]
pub enum Justify {
    /// Content is justified at the start of the main axis.
    #[strum(serialize = "flex-start")]
    FlexStart,

    /// Content is justified at the center along the cross axis.
    #[strum(serialize = "center")]
    Center,

    /// Content is justified at the end of the main axis.
    #[strum(serialize = "flex-end")]
    FlexEnd,

    /// Content is spaced evenly between, the first content starts at the start
    /// of the main axis and the last content ends at the end of the main axis.
    #[strum(serialize = "space-between")]
    SpaceBetween,

    /// Items are spaced evenly around, the first content starts after the start
    /// of the main axis and the last content ends before the end of the main
    /// axis.
    #[strum(serialize = "space-around")]
    SpaceAround,

    /// Items are evenly spaced: the space between items is equal to the space
    /// between the first and last item and both ends of the main axis
    /// respectively.
    #[strum(serialize = "space-evenly")]
    SpaceEvenly,
}
