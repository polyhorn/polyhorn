use crate::geometry::Dimension;
use crate::styles::{Align, FlexDirection, Justify, Overflow};

pub trait IntoYoga<T> {
    fn into_yoga(self) -> T;
}

impl IntoYoga<yoga::FlexDirection> for FlexDirection {
    fn into_yoga(self) -> yoga::FlexDirection {
        match self {
            FlexDirection::Column => yoga::FlexDirection::Column,
            FlexDirection::ColumnReverse => yoga::FlexDirection::ColumnReverse,
            FlexDirection::Row => yoga::FlexDirection::Row,
            FlexDirection::RowReverse => yoga::FlexDirection::RowReverse,
        }
    }
}

impl IntoYoga<yoga::Align> for Align {
    fn into_yoga(self) -> yoga::Align {
        match self {
            Align::FlexStart => yoga::Align::FlexStart,
            Align::Center => yoga::Align::Center,
            Align::FlexEnd => yoga::Align::FlexEnd,
            Align::Stretch => yoga::Align::Stretch,
            Align::SpaceAround => yoga::Align::SpaceAround,
            Align::SpaceBetween => yoga::Align::SpaceBetween,
        }
    }
}

impl IntoYoga<yoga::Justify> for Justify {
    fn into_yoga(self) -> yoga::Justify {
        match self {
            Justify::FlexStart => yoga::Justify::FlexStart,
            Justify::Center => yoga::Justify::Center,
            Justify::FlexEnd => yoga::Justify::FlexEnd,
            Justify::SpaceAround => yoga::Justify::SpaceAround,
            Justify::SpaceBetween => yoga::Justify::SpaceBetween,
            Justify::SpaceEvenly => yoga::Justify::SpaceEvenly,
        }
    }
}

impl IntoYoga<yoga::StyleUnit> for Dimension<f32> {
    fn into_yoga(self) -> yoga::StyleUnit {
        match self {
            Dimension::Points(pixels) => yoga::StyleUnit::Point(pixels.into()),
            Dimension::Percentage(percent) => yoga::StyleUnit::Percent((percent * 100.0).into()),
            Dimension::Auto => yoga::StyleUnit::Auto,
            Dimension::Undefined => yoga::StyleUnit::UndefinedValue,
        }
    }
}

impl IntoYoga<yoga::Overflow> for Overflow {
    fn into_yoga(self) -> yoga::Overflow {
        match self {
            Overflow::Visible => yoga::Overflow::Visible,
            Overflow::Hidden => yoga::Overflow::Hidden,
            Overflow::Scroll => yoga::Overflow::Scroll,
        }
    }
}
