use crate::style::Overflow;
use polyhorn_style::{AlignItems, Dimension, FlexDirection, JustifyContent, Position};

pub trait IntoYoga<T> {
    fn into_yoga(self) -> T;
}

impl IntoYoga<yoga::PositionType> for Position {
    fn into_yoga(self) -> yoga::PositionType {
        match self {
            Position::Absolute => yoga::PositionType::Absolute,
            Position::Relative => yoga::PositionType::Relative,
        }
    }
}

impl IntoYoga<yoga::FlexDirection> for FlexDirection {
    fn into_yoga(self) -> yoga::FlexDirection {
        match self {
            FlexDirection::Column => yoga::FlexDirection::Column,
            FlexDirection::Row => yoga::FlexDirection::Row,
        }
    }
}

impl IntoYoga<yoga::Align> for AlignItems {
    fn into_yoga(self) -> yoga::Align {
        match self {
            AlignItems::Center => yoga::Align::Center,
            AlignItems::Stretch => yoga::Align::Stretch,
            AlignItems::FlexStart => yoga::Align::FlexStart,
            AlignItems::FlexEnd => yoga::Align::FlexEnd,
        }
    }
}

impl IntoYoga<yoga::Justify> for JustifyContent {
    fn into_yoga(self) -> yoga::Justify {
        match self {
            JustifyContent::Center => yoga::Justify::Center,
            JustifyContent::FlexStart => yoga::Justify::FlexStart,
            JustifyContent::FlexEnd => yoga::Justify::FlexEnd,
            JustifyContent::SpaceBetween => yoga::Justify::SpaceBetween,
        }
    }
}

impl IntoYoga<yoga::StyleUnit> for Dimension {
    fn into_yoga(self) -> yoga::StyleUnit {
        match self {
            Dimension::Pixels(pixels) => yoga::StyleUnit::Point(pixels.into()),
            Dimension::Percent(percent) => yoga::StyleUnit::Percent((percent * 100.0).into()),
            Dimension::Auto => yoga::StyleUnit::Auto,
            Dimension::Undefined => yoga::StyleUnit::UndefinedValue,
        }
    }
}

impl IntoYoga<yoga::Overflow> for Overflow {
    fn into_yoga(self) -> yoga::Overflow {
        match self {
            Overflow::Visible => yoga::Overflow::Visible,
            Overflow::Scroll => yoga::Overflow::Scroll,
        }
    }
}
