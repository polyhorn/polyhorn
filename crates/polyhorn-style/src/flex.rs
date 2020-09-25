#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FlexDirection {
    Column,
    Row,
}

impl Default for FlexDirection {
    fn default() -> Self {
        FlexDirection::Column
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AlignItems {
    Center,
    Stretch,
    FlexStart,
    FlexEnd,
}

impl Default for AlignItems {
    fn default() -> Self {
        AlignItems::Stretch
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
}

impl Default for JustifyContent {
    fn default() -> Self {
        JustifyContent::FlexStart
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Position {
    Relative,
    Absolute,
}

impl Default for Position {
    fn default() -> Self {
        Position::Relative
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Hidden,
}

impl Visibility {
    pub fn is_hidden(&self) -> bool {
        self == &Visibility::Hidden
    }
}

impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
}
