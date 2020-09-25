use super::Dimension;

#[derive(Copy, Clone, Debug)]
pub struct BorderRadius {
    pub top_leading: Dimension,
    pub top_trailing: Dimension,
    pub bottom_trailing: Dimension,
    pub bottom_leading: Dimension,
}

impl Default for BorderRadius {
    fn default() -> Self {
        BorderRadius {
            top_leading: Dimension::Pixels(0.0),
            top_trailing: Dimension::Pixels(0.0),
            bottom_trailing: Dimension::Pixels(0.0),
            bottom_leading: Dimension::Pixels(0.0),
        }
    }
}

impl From<Dimension> for BorderRadius {
    fn from(value: Dimension) -> Self {
        BorderRadius {
            top_leading: value,
            top_trailing: value,
            bottom_trailing: value,
            bottom_leading: value,
        }
    }
}

impl From<(Dimension, Dimension)> for BorderRadius {
    fn from(value: (Dimension, Dimension)) -> Self {
        BorderRadius {
            top_leading: value.0,
            top_trailing: value.1,
            bottom_trailing: value.0,
            bottom_leading: value.1,
        }
    }
}

impl From<(Dimension, Dimension, Dimension, Dimension)> for BorderRadius {
    fn from(value: (Dimension, Dimension, Dimension, Dimension)) -> Self {
        BorderRadius {
            top_leading: value.0,
            top_trailing: value.1,
            bottom_trailing: value.2,
            bottom_leading: value.3,
        }
    }
}
