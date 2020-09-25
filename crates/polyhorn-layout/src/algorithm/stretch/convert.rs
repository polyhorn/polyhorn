use stretch::geometry::{Rect, Size};
use stretch::node::MeasureFunc;
use stretch::style::{AlignItems, Dimension, FlexDirection, JustifyContent, PositionType, Style};

pub trait IntoStretch<T> {
    fn into_stretch(self) -> T;
}

impl IntoStretch<Style> for crate::Style {
    fn into_stretch(self) -> Style {
        Style {
            position_type: self.position.into_stretch(),
            flex_direction: self.flex_direction.into_stretch(),
            align_items: self.align_items.into_stretch(),
            justify_content: self.justify_content.into_stretch(),
            flex_basis: self.flex_basis.into_stretch(),
            flex_grow: self.flex_grow,
            flex_shrink: self.flex_shrink,
            min_size: Size {
                width: self.min_size.width.into_stretch(),
                height: self.min_size.height.into_stretch(),
            },
            size: Size {
                width: self.size.width.into_stretch(),
                height: self.size.height.into_stretch(),
            },
            max_size: Size {
                width: self.max_size.width.into_stretch(),
                height: self.max_size.height.into_stretch(),
            },
            padding: Rect {
                top: self.padding.top.into_stretch(),
                end: self.padding.trailing.into_stretch(),
                bottom: self.padding.bottom.into_stretch(),
                start: self.padding.leading.into_stretch(),
            },
            margin: Rect {
                top: self.margin.top.into_stretch(),
                end: self.margin.trailing.into_stretch(),
                bottom: self.margin.bottom.into_stretch(),
                start: self.margin.leading.into_stretch(),
            },
            ..Default::default()
        }
    }
}

impl IntoStretch<Dimension> for polyhorn_style::Dimension {
    fn into_stretch(self) -> Dimension {
        match self {
            polyhorn_style::Dimension::Auto => Dimension::Auto,
            polyhorn_style::Dimension::Undefined => Dimension::Undefined,
            polyhorn_style::Dimension::Pixels(pixels) => Dimension::Points(pixels),
            polyhorn_style::Dimension::Percent(percent) => Dimension::Percent(percent),
        }
    }
}

impl IntoStretch<PositionType> for polyhorn_style::Position {
    fn into_stretch(self) -> PositionType {
        match self {
            polyhorn_style::Position::Relative => PositionType::Relative,
            polyhorn_style::Position::Absolute => PositionType::Absolute,
        }
    }
}

impl IntoStretch<FlexDirection> for polyhorn_style::FlexDirection {
    fn into_stretch(self) -> FlexDirection {
        match self {
            polyhorn_style::FlexDirection::Column => FlexDirection::Column,
            polyhorn_style::FlexDirection::Row => FlexDirection::Row,
        }
    }
}

impl IntoStretch<AlignItems> for polyhorn_style::AlignItems {
    fn into_stretch(self) -> AlignItems {
        match self {
            polyhorn_style::AlignItems::Center => AlignItems::Center,
            polyhorn_style::AlignItems::Stretch => AlignItems::Stretch,
            polyhorn_style::AlignItems::FlexStart => AlignItems::FlexStart,
            polyhorn_style::AlignItems::FlexEnd => AlignItems::FlexEnd,
        }
    }
}

impl IntoStretch<JustifyContent> for polyhorn_style::JustifyContent {
    fn into_stretch(self) -> JustifyContent {
        match self {
            polyhorn_style::JustifyContent::Center => JustifyContent::Center,
            polyhorn_style::JustifyContent::FlexStart => JustifyContent::FlexStart,
            polyhorn_style::JustifyContent::FlexEnd => JustifyContent::FlexEnd,
            polyhorn_style::JustifyContent::SpaceBetween => JustifyContent::SpaceBetween,
        }
    }
}

impl IntoStretch<MeasureFunc> for crate::MeasureFunc {
    fn into_stretch(self) -> MeasureFunc {
        match self {
            crate::MeasureFunc::Boxed(measure) => {
                stretch::node::MeasureFunc::Boxed(Box::new(move |size| {
                    let size = measure(crate::Size {
                        width: match size.width {
                            stretch::number::Number::Defined(pixels) => {
                                polyhorn_style::Dimension::Pixels(pixels)
                            }
                            stretch::number::Number::Undefined => {
                                polyhorn_style::Dimension::Undefined
                            }
                        },
                        height: match size.height {
                            stretch::number::Number::Defined(pixels) => {
                                polyhorn_style::Dimension::Pixels(pixels)
                            }
                            stretch::number::Number::Undefined => {
                                polyhorn_style::Dimension::Undefined
                            }
                        },
                    });

                    stretch::geometry::Size {
                        width: size.width,
                        height: size.height,
                    }
                }))
            }
        }
    }
}
