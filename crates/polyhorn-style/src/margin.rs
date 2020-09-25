use super::Dimension;

#[derive(Copy, Clone, Debug)]
pub struct Margin {
    pub top: Dimension,
    pub trailing: Dimension,
    pub bottom: Dimension,
    pub leading: Dimension,
}

impl Default for Margin {
    fn default() -> Self {
        Margin {
            top: Dimension::Pixels(0.0),
            trailing: Dimension::Pixels(0.0),
            bottom: Dimension::Pixels(0.0),
            leading: Dimension::Pixels(0.0),
        }
    }
}

impl From<Dimension> for Margin {
    fn from(value: Dimension) -> Self {
        Margin {
            top: value,
            trailing: value,
            bottom: value,
            leading: value,
        }
    }
}

impl From<(Dimension, Dimension)> for Margin {
    fn from(value: (Dimension, Dimension)) -> Self {
        Margin {
            top: value.0,
            trailing: value.1,
            bottom: value.0,
            leading: value.1,
        }
    }
}

impl From<(Dimension, Dimension, Dimension, Dimension)> for Margin {
    fn from(value: (Dimension, Dimension, Dimension, Dimension)) -> Self {
        Margin {
            top: value.0,
            trailing: value.1,
            bottom: value.2,
            leading: value.3,
        }
    }
}
