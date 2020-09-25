use super::Dimension;

#[derive(Copy, Clone, Debug, Default)]
pub struct Padding {
    pub top: Dimension,
    pub trailing: Dimension,
    pub bottom: Dimension,
    pub leading: Dimension,
}

impl From<Dimension> for Padding {
    fn from(value: Dimension) -> Self {
        Padding {
            top: value,
            trailing: value,
            bottom: value,
            leading: value,
        }
    }
}

impl From<(Dimension, Dimension)> for Padding {
    fn from(value: (Dimension, Dimension)) -> Self {
        Padding {
            top: value.0,
            trailing: value.1,
            bottom: value.0,
            leading: value.1,
        }
    }
}

impl From<(Dimension, Dimension, Dimension, Dimension)> for Padding {
    fn from(value: (Dimension, Dimension, Dimension, Dimension)) -> Self {
        Padding {
            top: value.0,
            trailing: value.1,
            bottom: value.2,
            leading: value.3,
        }
    }
}
